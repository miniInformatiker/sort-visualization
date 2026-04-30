use std::io::{self, Stdout};
use std::time::Duration;

use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph, Wrap},
};

use crate::config::{Algorithm, Config, DataMode};
use crate::sorting::{SortFrame, build_frames};

pub fn run(config: Config) -> io::Result<()> {
    let mut terminal = setup_terminal()?;
    let result = run_app(&mut terminal, App::new(config));
    restore_terminal(&mut terminal)?;
    result
}

struct App {
    config: Config,
    frames: Vec<SortFrame>,
    step: usize,
    paused: bool,
    screen: Screen,
    selected_setting: Setting,
    clear_before_draw: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Menu,
    Running,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Setting {
    Algorithm,
    DataMode,
    Size,
    Delay,
    Start,
}

impl Setting {
    fn all() -> &'static [Setting] {
        &[
            Setting::Algorithm,
            Setting::DataMode,
            Setting::Size,
            Setting::Delay,
            Setting::Start,
        ]
    }

    fn label(self) -> &'static str {
        match self {
            Setting::Algorithm => "Algorithmus",
            Setting::DataMode => "Datenmodus",
            Setting::Size => "Anzahl Werte",
            Setting::Delay => "Geschwindigkeit",
            Setting::Start => "Visualisierung starten",
        }
    }
}

impl App {
    fn new(config: Config) -> Self {
        let frames = build_frames(&config);

        Self {
            config,
            frames,
            step: 0,
            paused: false,
            screen: Screen::Menu,
            selected_setting: Setting::Algorithm,
            clear_before_draw: true,
        }
    }

    fn current_frame(&self) -> &SortFrame {
        &self.frames[self.step]
    }

    fn progress(&self) -> f64 {
        if self.frames.len() <= 1 {
            1.0
        } else {
            self.step as f64 / (self.frames.len() - 1) as f64
        }
    }

    fn tick(&mut self) {
        if !self.paused && self.step + 1 < self.frames.len() {
            self.step += 1;
        }
    }

    fn restart(&mut self) {
        self.frames = build_frames(&self.config);
        self.step = 0;
        self.paused = false;
    }

    fn start_visualization(&mut self) {
        self.restart();
        self.screen = Screen::Running;
        self.clear_before_draw = true;
    }

    fn return_to_menu(&mut self) {
        self.screen = Screen::Menu;
        self.paused = false;
        self.clear_before_draw = true;
    }

    fn set_algorithm(&mut self, algorithm: Algorithm) {
        self.config.algorithm = algorithm;
        self.restart();
    }

    fn cycle_algorithm(&mut self, direction: isize) {
        let algorithms = Algorithm::all();
        let current = algorithms
            .iter()
            .position(|algorithm| *algorithm as u8 == self.config.algorithm as u8)
            .unwrap_or(0);
        let next = (current as isize + direction).rem_euclid(algorithms.len() as isize) as usize;
        self.set_algorithm(algorithms[next]);
    }

    fn cycle_mode(&mut self, direction: isize) {
        let modes = DataMode::all();
        let current = modes
            .iter()
            .position(|mode| *mode as u8 == self.config.data_mode as u8)
            .unwrap_or(0);
        let next = (current as isize + direction).rem_euclid(modes.len() as isize) as usize;
        self.config.data_mode = modes[next];
        self.restart();
    }

    fn change_size(&mut self, amount: isize) {
        self.config.size = (self.config.size as isize + amount).clamp(5, 60) as usize;
        self.restart();
    }

    fn change_delay(&mut self, amount: i64) {
        let current = self.config.delay.as_millis() as i64;
        self.config.delay = Duration::from_millis((current + amount).clamp(0, 2_000) as u64);
    }

    fn move_selection(&mut self, direction: isize) {
        let settings = Setting::all();
        let current = settings
            .iter()
            .position(|setting| *setting == self.selected_setting)
            .unwrap_or(0);
        let next = (current as isize + direction).rem_euclid(settings.len() as isize) as usize;
        self.selected_setting = settings[next];
    }

    fn adjust_selected(&mut self, direction: isize) {
        match self.selected_setting {
            Setting::Algorithm => self.cycle_algorithm(direction),
            Setting::DataMode => self.cycle_mode(direction),
            Setting::Size => self.change_size(direction),
            Setting::Delay => self.change_delay((direction as i64) * -10),
            Setting::Start => {}
        }
    }
}

fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout))
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()
}

fn run_app(terminal: &mut Terminal<CrosstermBackend<Stdout>>, mut app: App) -> io::Result<()> {
    loop {
        if app.clear_before_draw {
            terminal.clear()?;
            app.clear_before_draw = false;
        }

        terminal.draw(|frame| draw_ui(frame, &app))?;
        let poll_delay = if app.screen == Screen::Menu {
            Duration::from_millis(250)
        } else {
            app.config.delay
        };

        if event::poll(poll_delay)? {
            if let Event::Key(key) = event::read()? {
                if handle_key(key.code, &mut app) {
                    return Ok(());
                }
            }
        } else if app.screen == Screen::Running {
            app.tick();
        }
    }
}

fn handle_key(code: KeyCode, app: &mut App) -> bool {
    match app.screen {
        Screen::Menu => handle_menu_key(code, app),
        Screen::Running => handle_running_key(code, app),
    }
}

fn handle_menu_key(code: KeyCode, app: &mut App) -> bool {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => return true,
        KeyCode::Up | KeyCode::Char('k') => app.move_selection(-1),
        KeyCode::Down | KeyCode::Char('j') => app.move_selection(1),
        KeyCode::Left | KeyCode::Char('-') => app.adjust_selected(-1),
        KeyCode::Right | KeyCode::Char('+') | KeyCode::Char('=') => app.adjust_selected(1),
        KeyCode::Enter => app.start_visualization(),
        KeyCode::Char('1') => app.set_algorithm(Algorithm::Bubble),
        KeyCode::Char('2') => app.set_algorithm(Algorithm::Selection),
        KeyCode::Char('3') => app.set_algorithm(Algorithm::Insertion),
        KeyCode::Char('4') => app.set_algorithm(Algorithm::Quick),
        _ => {}
    }

    false
}

fn handle_running_key(code: KeyCode, app: &mut App) -> bool {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => app.return_to_menu(),
        KeyCode::Char(' ') => app.paused = !app.paused,
        KeyCode::Char('r') => app.restart(),
        KeyCode::Left => app.cycle_algorithm(-1),
        KeyCode::Right => app.cycle_algorithm(1),
        KeyCode::Char('m') => app.cycle_mode(1),
        KeyCode::Char('+') | KeyCode::Char('=') => app.change_size(1),
        KeyCode::Char('-') => app.change_size(-1),
        KeyCode::Char('[') => app.change_delay(10),
        KeyCode::Char(']') => app.change_delay(-10),
        KeyCode::Char('1') => app.set_algorithm(Algorithm::Bubble),
        KeyCode::Char('2') => app.set_algorithm(Algorithm::Selection),
        KeyCode::Char('3') => app.set_algorithm(Algorithm::Insertion),
        KeyCode::Char('4') => app.set_algorithm(Algorithm::Quick),
        _ => {}
    }

    false
}

fn draw_ui(frame: &mut Frame<'_>, app: &App) {
    if app.screen == Screen::Menu {
        draw_menu(frame, app);
        return;
    }

    let outer = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(8),
            Constraint::Length(4),
        ])
        .split(frame.area());

    draw_header(frame, outer[0], app);

    let body = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(outer[1]);

    draw_bars(frame, body[0], app.current_frame());
    draw_details(frame, body[1], app);
    draw_footer(frame, outer[2]);
}

fn draw_menu(frame: &mut Frame<'_>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(2),
            Constraint::Length(15),
            Constraint::Length(5),
            Constraint::Min(2),
        ])
        .split(frame.area());

    let panel = centered_rect(66, 15, chunks[1]);
    let lines = Setting::all()
        .iter()
        .map(|setting| {
            let selected = *setting == app.selected_setting;
            let marker = if selected { "> " } else { "  " };
            let value = setting_value(*setting, app);
            let style = if selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            if *setting == Setting::Start {
                Line::from(vec![Span::styled(
                    format!("{marker}{}", setting.label()),
                    style,
                )])
            } else {
                Line::from(vec![
                    Span::styled(format!("{marker}{:<17}", setting.label()), style),
                    Span::styled(value, style),
                ])
            }
        })
        .collect::<Vec<_>>();

    let menu = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Einstellungen"),
        )
        .alignment(Alignment::Left);

    frame.render_widget(menu, panel);

    let help = Paragraph::new(vec![
        Line::raw("↑/↓ auswaehlen | ←/→ aendern | Enter starten"),
        Line::raw("1-4 Algorithmus direkt | q/Esc beenden"),
    ])
    .block(Block::default().borders(Borders::ALL).title("Tasten"))
    .alignment(Alignment::Center);

    frame.render_widget(help, chunks[2]);
}

fn centered_rect(percent_x: u16, height: u16, area: Rect) -> Rect {
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(height.min(area.height)),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(vertical[1])[1]
}

fn setting_value(setting: Setting, app: &App) -> String {
    match setting {
        Setting::Algorithm => app.config.algorithm.to_string(),
        Setting::DataMode => app.config.data_mode.to_string(),
        Setting::Size => app.config.size.to_string(),
        Setting::Delay => format!("{} ms", app.config.delay.as_millis()),
        Setting::Start => String::new(),
    }
}

fn draw_header(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let status = if app.paused { "pausiert" } else { "laeuft" };
    let title = Line::from(vec![
        Span::styled(
            "Sort Visualization",
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::raw(app.config.algorithm.to_string()),
        Span::raw(format!(" | {} | {}", app.config.data_mode, status)),
    ]);

    let gauge = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title(title))
        .gauge_style(Style::default().fg(Color::Green))
        .ratio(app.progress());

    frame.render_widget(gauge, area);
}

fn draw_bars(frame: &mut Frame<'_>, area: Rect, sort_frame: &SortFrame) {
    let max = sort_frame.values.iter().copied().max().unwrap_or(1);
    let width = area.width.saturating_sub(10).max(1) as usize;
    let lines = sort_frame
        .values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let bar_width = value * width / max;
            let marker = if sort_frame.active.contains(&index) {
                ">"
            } else {
                " "
            };
            let color = if sort_frame.active.contains(&index) {
                Color::Yellow
            } else {
                Color::Blue
            };

            Line::from(vec![
                Span::styled(marker, Style::default().fg(Color::Yellow)),
                Span::raw(format!(" {:>2} ", value)),
                Span::styled("█".repeat(bar_width), Style::default().fg(color)),
            ])
        })
        .collect::<Vec<_>>();

    let paragraph = Paragraph::new(lines)
        .block(Block::default().borders(Borders::ALL).title("Werte"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_details(frame: &mut Frame<'_>, area: Rect, app: &App) {
    let current = app.current_frame();
    let details = vec![
        Line::from(vec![Span::styled(
            &current.message,
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::raw(""),
        Line::raw(format!("Algorithmus: {}", app.config.algorithm)),
        Line::raw(format!("Daten:       {}", app.config.data_mode)),
        Line::raw(format!("Werte:       {}", app.config.size)),
        Line::raw(format!("Delay:       {} ms", app.config.delay.as_millis())),
        Line::raw(format!(
            "Schritt:     {}/{}",
            app.step + 1,
            app.frames.len()
        )),
    ];

    let paragraph = Paragraph::new(details)
        .block(Block::default().borders(Borders::ALL).title("Status"))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn draw_footer(frame: &mut Frame<'_>, area: Rect) {
    let lines = vec![
        Line::raw("q/Esc Menue | Space Pause | r Neustart | ←/→ Algorithmus | 1-4 Direktwahl"),
        Line::raw("m Datenmodus | +/- Werte | [/ ] langsamer/schneller"),
    ];

    let paragraph =
        Paragraph::new(lines).block(Block::default().borders(Borders::ALL).title("Tasten"));

    frame.render_widget(paragraph, area);
}
