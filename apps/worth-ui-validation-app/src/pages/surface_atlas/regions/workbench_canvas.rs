use egui::Ui;

use crate::shell::ValidationRunSummary;
use crate::theme::ValidationWorkbenchTheme;

pub fn render(ui: &mut Ui, theme: &ValidationWorkbenchTheme, summary: &ValidationRunSummary) {
    ui.visuals_mut().widgets.noninteractive.bg_fill = theme.editor_canvas();
    ui.heading("Surface atlas");
    ui.label("Native Worth UI validation workbench");
    ui.horizontal(|ui| {
        ui.label("Artifact");
        ui.monospace(summary.runtime_observation().artifact_digest().to_string());
    });
    ui.horizontal(|ui| {
        ui.label("Plan");
        ui.monospace(
            summary
                .runtime_observation()
                .active_plan_digest()
                .to_string(),
        );
    });
}
