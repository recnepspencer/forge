//! Forge Query todo showcase executable boundary.

mod showcase;
mod todo;
mod ui;

use eframe::egui;
use showcase::TodoShowcaseApp;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Momentum")
            .with_inner_size([1380.0, 860.0])
            .with_min_inner_size([980.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Momentum",
        native_options,
        Box::new(|cc| Ok(Box::new(TodoShowcaseApp::new(cc)))),
    )
}
