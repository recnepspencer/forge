use egui::Ui;

use crate::pages::surface_atlas::SurfaceAtlasViewport;

pub fn render(ui: &mut Ui, viewport: SurfaceAtlasViewport) {
    ui.heading("Stacked scroll panes");
    egui::ScrollArea::vertical()
        .id_salt("surface-atlas.stacked-scroll-panes")
        .max_height(96.0)
        .show(ui, |ui| {
            ui.label(format!("Viewport: {viewport:?}"));
            ui.label("Runtime receipts");
            ui.label("Expected observations");
            ui.label("Diagnostics slots");
        });
}
