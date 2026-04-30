use std::time::Duration;

use eframe::egui::{self, Color32, Pos2, Rect, RichText, Sense, Stroke, Vec2};
use web_time::Instant;

use crate::config::{Algorithm, Config, DEFAULT_DELAY_MS, DEFAULT_SIZE, DataMode};
use crate::sorting::{SortFrame, build_frames};

pub struct SortGuiApp {
    config: Config,
    frames: Vec<SortFrame>,
    step: usize,
    playing: bool,
    last_tick: Instant,
}

impl Default for SortGuiApp {
    fn default() -> Self {
        let config = Config {
            algorithm: Algorithm::Bubble,
            data_mode: DataMode::Random,
            size: DEFAULT_SIZE,
            delay: Duration::from_millis(DEFAULT_DELAY_MS),
        };
        let frames = build_frames(&config);

        Self {
            config,
            frames,
            step: 0,
            playing: false,
            last_tick: Instant::now(),
        }
    }
}

impl SortGuiApp {
    fn current_frame(&self) -> &SortFrame {
        &self.frames[self.step]
    }

    fn progress(&self) -> f32 {
        if self.frames.len() <= 1 {
            1.0
        } else {
            self.step as f32 / (self.frames.len() - 1) as f32
        }
    }

    fn rebuild(&mut self) {
        self.frames = build_frames(&self.config);
        self.step = 0;
        self.last_tick = Instant::now();
    }

    fn restart(&mut self) {
        self.rebuild();
        self.playing = true;
    }

    fn tick(&mut self) {
        if self.step + 1 < self.frames.len() {
            self.step += 1;
        } else {
            self.playing = false;
        }
    }

    fn update_animation(&mut self, ctx: &egui::Context) {
        if !self.playing {
            return;
        }

        if self.last_tick.elapsed() >= self.config.delay {
            self.tick();
            self.last_tick = Instant::now();
        }

        ctx.request_repaint_after(self.config.delay.max(Duration::from_millis(16)));
    }
}

impl eframe::App for SortGuiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.update_animation(ctx);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("Sort Visualization");
                ui.separator();
                ui.label(format!(
                    "{} | {} | Schritt {}/{}",
                    self.config.algorithm,
                    self.config.data_mode,
                    self.step + 1,
                    self.frames.len()
                ));
            });
            ui.add(egui::ProgressBar::new(self.progress()).show_percentage());
        });

        egui::SidePanel::left("settings_panel")
            .resizable(false)
            .default_width(260.0)
            .show(ctx, |ui| {
                ui.heading("Einstellungen");
                ui.add_space(8.0);

                let old_algorithm = self.config.algorithm as u8;
                egui::ComboBox::from_label("Algorithmus")
                    .selected_text(self.config.algorithm.to_string())
                    .show_ui(ui, |ui| {
                        for algorithm in Algorithm::all() {
                            ui.selectable_value(
                                &mut self.config.algorithm,
                                *algorithm,
                                algorithm.to_string(),
                            );
                        }
                    });

                let old_mode = self.config.data_mode as u8;
                egui::ComboBox::from_label("Datenmodus")
                    .selected_text(self.config.data_mode.to_string())
                    .show_ui(ui, |ui| {
                        for mode in DataMode::all() {
                            ui.selectable_value(
                                &mut self.config.data_mode,
                                *mode,
                                mode.to_string(),
                            );
                        }
                    });

                let size_changed = ui
                    .add(egui::Slider::new(&mut self.config.size, 5..=60).text("Werte"))
                    .changed();

                let mut delay_ms = self.config.delay.as_millis() as u64;
                let delay_changed = ui
                    .add(egui::Slider::new(&mut delay_ms, 0..=2_000).text("Delay ms"))
                    .changed();
                if delay_changed {
                    self.config.delay = Duration::from_millis(delay_ms);
                }

                if old_algorithm != self.config.algorithm as u8
                    || old_mode != self.config.data_mode as u8
                    || size_changed
                {
                    self.rebuild();
                }

                ui.add_space(12.0);
                ui.horizontal(|ui| {
                    if ui.button("Start").clicked() {
                        self.playing = true;
                        self.last_tick = Instant::now();
                    }
                    if ui.button("Pause").clicked() {
                        self.playing = false;
                    }
                    if ui.button("Neu").clicked() {
                        self.restart();
                    }
                });

                ui.add_space(16.0);
                ui.label(RichText::new(&self.current_frame().message).strong());
                ui.label(format!("Fortschritt: {:.0}%", self.progress() * 100.0));
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            draw_bars(ui, self.current_frame());
        });
    }
}

fn draw_bars(ui: &mut egui::Ui, frame: &SortFrame) {
    let available = ui.available_size();
    let desired = Vec2::new(available.x.max(320.0), available.y.max(280.0));
    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 6.0, Color32::from_rgb(18, 22, 28));
    painter.rect_stroke(rect, 6.0, Stroke::new(1.0, Color32::from_gray(70)));

    if frame.values.is_empty() {
        return;
    }

    let max_value = frame.values.iter().copied().max().unwrap_or(1) as f32;
    let count = frame.values.len() as f32;
    let gap = 4.0;
    let bar_width = ((rect.width() - gap * (count + 1.0)) / count).max(2.0);
    let bottom = rect.bottom() - 24.0;
    let max_height = (rect.height() - 52.0).max(1.0);

    for (index, value) in frame.values.iter().enumerate() {
        let left = rect.left() + gap + index as f32 * (bar_width + gap);
        let height = (*value as f32 / max_value) * max_height;
        let bar_rect = Rect::from_min_max(
            Pos2::new(left, bottom - height),
            Pos2::new(left + bar_width, bottom),
        );
        let active = frame.active.contains(&index);
        let fill = if active {
            Color32::from_rgb(245, 190, 75)
        } else {
            Color32::from_rgb(80, 160, 220)
        };

        painter.rect_filled(bar_rect, 3.0, fill);
        painter.text(
            Pos2::new(left + bar_width / 2.0, bottom + 6.0),
            egui::Align2::CENTER_TOP,
            value.to_string(),
            egui::FontId::monospace(11.0),
            Color32::from_gray(210),
        );
    }
}
