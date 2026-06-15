use egui::Ui;

use crate::pages::surface_atlas::SurfaceAtlasControlState;
use crate::theme::ValidationWorkbenchTheme;

pub fn render(ui: &mut Ui, theme: &ValidationWorkbenchTheme, controls: &SurfaceAtlasControlState) {
    ui.heading("Theme and density");
    ui.horizontal(|ui| {
        ui.label("Density");
        ui.monospace(format!("{:?}", controls.density()));
    });
    ui.horizontal(|ui| {
        ui.label("Theme revision");
        ui.monospace(controls.theme_revision().to_string());
    });
    ui.colored_label(theme.accent(), "Token-driven VS Code-like dark palette");
}
