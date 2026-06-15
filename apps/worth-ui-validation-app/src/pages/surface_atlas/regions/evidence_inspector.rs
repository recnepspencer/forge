use egui::Ui;

use crate::pages::surface_atlas::SurfaceAtlasFixtureEvidence;

pub fn render(ui: &mut Ui, evidence: &SurfaceAtlasFixtureEvidence) {
    ui.heading("Evidence inspector");
    ui.label(evidence.label());
    for family in evidence.display_families() {
        ui.monospace(format!("{family:?}"));
    }
}
