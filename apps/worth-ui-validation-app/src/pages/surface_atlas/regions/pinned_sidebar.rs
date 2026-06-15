use egui::Ui;

use crate::pages::surface_atlas::SurfaceAtlasTopologySnapshot;

pub fn render(ui: &mut Ui, topology: &SurfaceAtlasTopologySnapshot) {
    ui.heading("Pinned sidebar");
    for region in topology.regions().iter().take(4) {
        ui.monospace(region.stable_id());
    }
}
