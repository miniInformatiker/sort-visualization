use std::time::Duration;

use eframe::egui::{
    self, Color32, FontId, Margin, Pos2, Rect, RichText, Rounding, Sense, Stroke, Vec2,
};
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
        apply_style(ctx);
        self.update_animation(ctx);

        egui::TopBottomPanel::top("header")
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(12, 15, 20))
                    .inner_margin(Margin::symmetric(18.0, 18.0)),
            )
            .show(ctx, |ui| {
                draw_header(ui, self);
            });

        egui::SidePanel::left("settings")
            .resizable(false)
            .exact_width(356.0)
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(12, 15, 20))
                    .inner_margin(Margin {
                        left: 18.0,
                        right: 8.0,
                        top: 18.0,
                        bottom: 18.0,
                    }),
            )
            .show(ctx, |ui| {
                draw_settings(ui, self);
            });

        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(Color32::from_rgb(12, 15, 20))
                    .inner_margin(Margin {
                        left: 8.0,
                        right: 18.0,
                        top: 18.0,
                        bottom: 18.0,
                    }),
            )
            .show(ctx, |ui| {
                draw_visualization(ui, self);
            });
    }
}

fn apply_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 10.0);
    style.spacing.button_padding = Vec2::new(14.0, 8.0);
    style.visuals = egui::Visuals::dark();
    style.visuals.panel_fill = Color32::from_rgb(12, 15, 20);
    style.visuals.window_fill = Color32::from_rgb(20, 24, 31);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(31, 37, 47);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(45, 54, 68);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(76, 129, 190);
    style.visuals.widgets.noninteractive.fg_stroke =
        Stroke::new(1.0, Color32::from_rgb(220, 226, 235));
    ctx.set_style(style);
}

fn draw_header(ui: &mut egui::Ui, app: &SortGuiApp) {
    card_frame(Color32::from_rgb(18, 22, 30)).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Sort Visualization")
                        .font(FontId::proportional(30.0))
                        .strong()
                        .color(Color32::from_rgb(244, 247, 250)),
                );
                ui.label(
                    RichText::new("Desktop- und Web-Visualizer fuer Sortieralgorithmen")
                        .color(Color32::from_rgb(145, 157, 174)),
                );
            });

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                let state = if app.playing { "Laeuft" } else { "Bereit" };
                status_chip(ui, state, Color32::from_rgb(112, 190, 130));
                status_chip(
                    ui,
                    &app.config.data_mode.to_string(),
                    Color32::from_rgb(112, 160, 220),
                );
                status_chip(
                    ui,
                    &app.config.algorithm.to_string(),
                    Color32::from_rgb(226, 175, 89),
                );
            });
        });

        ui.add_space(10.0);
        ui.add(
            egui::ProgressBar::new(app.progress())
                .desired_width(ui.available_width())
                .text(format!("{:.0}% abgeschlossen", app.progress() * 100.0))
                .fill(Color32::from_rgb(83, 154, 218)),
        );
    });
}

fn draw_settings(ui: &mut egui::Ui, app: &mut SortGuiApp) {
    card_frame(Color32::from_rgb(20, 25, 33)).show(ui, |ui| {
        ui.label(
            RichText::new("Steuerung")
                .font(FontId::proportional(22.0))
                .strong()
                .color(Color32::from_rgb(240, 244, 248)),
        );
        ui.label(
            RichText::new("Waehle Daten, Tempo und Algorithmus.")
                .color(Color32::from_rgb(143, 154, 170)),
        );
        ui.add_space(14.0);

        let old_algorithm = app.config.algorithm;
        ui.label(RichText::new("Algorithmus").strong());
        ui.vertical(|ui| {
            for algorithm in Algorithm::all() {
                if selectable_row(
                    ui,
                    app.config.algorithm == *algorithm,
                    &algorithm.to_string(),
                )
                .clicked()
                {
                    app.config.algorithm = *algorithm;
                }
            }
        });

        ui.add_space(12.0);
        let old_mode = app.config.data_mode;
        ui.label(RichText::new("Datenmodus").strong());
        ui.horizontal_wrapped(|ui| {
            for mode in DataMode::all() {
                if mode_button(ui, app.config.data_mode == *mode, &mode.to_string()).clicked() {
                    app.config.data_mode = *mode;
                }
            }
        });

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            draw_metric(ui, "Werte", &app.config.size.to_string());
            draw_metric(ui, "Delay", &format!("{} ms", app.config.delay.as_millis()));
        });

        let size_changed = ui
            .add(
                egui::Slider::new(&mut app.config.size, 5..=60)
                    .text("Anzahl")
                    .show_value(false),
            )
            .changed();

        let mut delay_ms = app.config.delay.as_millis() as u64;
        let delay_changed = ui
            .add(
                egui::Slider::new(&mut delay_ms, 0..=2_000)
                    .text("Tempo")
                    .show_value(false),
            )
            .changed();
        if delay_changed {
            app.config.delay = Duration::from_millis(delay_ms);
        }

        if old_algorithm != app.config.algorithm || old_mode != app.config.data_mode || size_changed
        {
            app.rebuild();
        }

        ui.add_space(16.0);
        ui.horizontal(|ui| {
            let primary = if app.playing { "Pause" } else { "Start" };
            if primary_button(ui, primary).clicked() {
                app.playing = !app.playing;
                app.last_tick = Instant::now();
            }
            if ui.button("Neu starten").clicked() {
                app.restart();
            }
        });

        ui.add_space(18.0);
        ui.separator();
        ui.add_space(8.0);
        ui.label(
            RichText::new(&app.current_frame().message)
                .strong()
                .color(Color32::from_rgb(235, 199, 122)),
        );
        ui.label(
            RichText::new(format!("Schritt {} von {}", app.step + 1, app.frames.len()))
                .color(Color32::from_rgb(145, 157, 174)),
        );
    });
}

fn draw_visualization(ui: &mut egui::Ui, app: &SortGuiApp) {
    card_frame(Color32::from_rgb(17, 21, 28)).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(
                RichText::new("Live-Ansicht")
                    .font(FontId::proportional(22.0))
                    .strong()
                    .color(Color32::from_rgb(240, 244, 248)),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(format!("{} Frames", app.frames.len()))
                        .color(Color32::from_rgb(142, 153, 168)),
                );
            });
        });
        ui.add_space(10.0);
        draw_bars(ui, app.current_frame());
    });
}

fn card_frame(fill: Color32) -> egui::Frame {
    egui::Frame::none()
        .fill(fill)
        .rounding(Rounding::same(14.0))
        .stroke(Stroke::new(1.0, Color32::from_rgb(43, 51, 64)))
        .inner_margin(Margin::same(18.0))
}

fn status_chip(ui: &mut egui::Ui, text: &str, color: Color32) {
    egui::Frame::none()
        .fill(color.gamma_multiply(0.18))
        .rounding(Rounding::same(999.0))
        .stroke(Stroke::new(1.0, color.gamma_multiply(0.55)))
        .inner_margin(Margin::symmetric(12.0, 6.0))
        .show(ui, |ui| {
            ui.label(
                RichText::new(text)
                    .color(Color32::from_rgb(232, 238, 244))
                    .strong(),
            );
        });
}

fn selectable_row(ui: &mut egui::Ui, selected: bool, text: &str) -> egui::Response {
    let fill = if selected {
        Color32::from_rgb(52, 91, 132)
    } else {
        Color32::from_rgb(27, 33, 43)
    };
    let stroke = if selected {
        Stroke::new(1.0, Color32::from_rgb(105, 168, 226))
    } else {
        Stroke::new(1.0, Color32::from_rgb(48, 57, 70))
    };

    egui::Frame::none()
        .fill(fill)
        .rounding(Rounding::same(8.0))
        .stroke(stroke)
        .inner_margin(Margin::symmetric(12.0, 8.0))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.label(
                RichText::new(text)
                    .strong()
                    .color(Color32::from_rgb(235, 240, 245)),
            );
        })
        .response
        .interact(Sense::click())
}

fn mode_button(ui: &mut egui::Ui, selected: bool, text: &str) -> egui::Response {
    let button = egui::Button::new(RichText::new(text).strong())
        .fill(if selected {
            Color32::from_rgb(79, 117, 84)
        } else {
            Color32::from_rgb(31, 37, 47)
        })
        .stroke(Stroke::new(
            1.0,
            if selected {
                Color32::from_rgb(126, 196, 137)
            } else {
                Color32::from_rgb(52, 61, 73)
            },
        ))
        .rounding(Rounding::same(8.0));
    ui.add(button)
}

fn primary_button(ui: &mut egui::Ui, text: &str) -> egui::Response {
    ui.add(
        egui::Button::new(RichText::new(text).strong().color(Color32::WHITE))
            .fill(Color32::from_rgb(58, 124, 196))
            .stroke(Stroke::new(1.0, Color32::from_rgb(106, 173, 235)))
            .rounding(Rounding::same(9.0)),
    )
}

fn draw_metric(ui: &mut egui::Ui, label: &str, value: &str) {
    egui::Frame::none()
        .fill(Color32::from_rgb(25, 31, 41))
        .rounding(Rounding::same(10.0))
        .inner_margin(Margin::symmetric(12.0, 10.0))
        .show(ui, |ui| {
            ui.set_min_width(126.0);
            ui.label(RichText::new(label).color(Color32::from_rgb(134, 147, 164)));
            ui.label(
                RichText::new(value)
                    .font(FontId::proportional(22.0))
                    .strong()
                    .color(Color32::from_rgb(242, 246, 250)),
            );
        });
}

fn draw_bars(ui: &mut egui::Ui, frame: &SortFrame) {
    let available = ui.available_size();
    let desired = Vec2::new(available.x.max(360.0), available.y.max(360.0));
    let (rect, _) = ui.allocate_exact_size(desired, Sense::hover());
    let painter = ui.painter_at(rect);

    painter.rect_filled(rect, 12.0, Color32::from_rgb(13, 17, 24));
    painter.rect_stroke(rect, 12.0, Stroke::new(1.0, Color32::from_rgb(38, 47, 61)));

    if frame.values.is_empty() {
        return;
    }

    let max_value = frame.values.iter().copied().max().unwrap_or(1) as f32;
    let count = frame.values.len() as f32;
    let gap = 5.0;
    let bar_width = ((rect.width() - gap * (count + 1.0)) / count).max(2.0);
    let bottom = rect.bottom() - 34.0;
    let max_height = (rect.height() - 70.0).max(1.0);

    for line in 0..4 {
        let y = rect.top() + 24.0 + line as f32 * (max_height / 4.0);
        painter.line_segment(
            [
                Pos2::new(rect.left() + 18.0, y),
                Pos2::new(rect.right() - 18.0, y),
            ],
            Stroke::new(1.0, Color32::from_rgb(28, 35, 45)),
        );
    }

    for (index, value) in frame.values.iter().enumerate() {
        let left = rect.left() + gap + index as f32 * (bar_width + gap);
        let height = (*value as f32 / max_value) * max_height;
        let bar_rect = Rect::from_min_max(
            Pos2::new(left, bottom - height),
            Pos2::new(left + bar_width, bottom),
        );
        let active = frame.active.contains(&index);
        let fill = if active {
            Color32::from_rgb(242, 184, 82)
        } else {
            Color32::from_rgb(75, 150, 214)
        };

        painter.rect_filled(bar_rect, 5.0, fill);
        if bar_width >= 14.0 {
            painter.text(
                Pos2::new(left + bar_width / 2.0, bottom + 8.0),
                egui::Align2::CENTER_TOP,
                value.to_string(),
                FontId::monospace(11.0),
                Color32::from_rgb(175, 184, 196),
            );
        }
    }
}
