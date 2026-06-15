use egui::Ui;

use crate::pages::surface_atlas::SurfaceAtlasFixtureEvidence;

pub fn render(ui: &mut Ui, evidence: &SurfaceAtlasFixtureEvidence) {
    ui.heading("Run timeline");
    ui.label("Fixture rows cannot complete validation");
    ui.monospace(format!("{:?}", evidence.mark_success()));
}
