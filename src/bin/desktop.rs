use eframe::egui;
use sort_visualization::gui::SortGuiApp;

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sort Visualization")
            .with_inner_size([1000.0, 700.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Sort Visualization",
        options,
        Box::new(|_cc| Ok(Box::new(SortGuiApp::default()))),
    )
}
