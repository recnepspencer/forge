use egui::Ui;

use crate::runtime::{PreparedValidationWorkbenchLaunch, ValidationWorkbenchSnapshot};

use super::{validation_page_layout_renderer, ValidationWorkspaceState};

pub(crate) fn render_page_host(
    ui: &mut Ui,
    launch: &PreparedValidationWorkbenchLaunch,
    snapshot: ValidationWorkbenchSnapshot,
    state: &mut ValidationWorkspaceState,
) {
    validation_page_layout_renderer::render_page_host(ui, launch, snapshot, state);
}
