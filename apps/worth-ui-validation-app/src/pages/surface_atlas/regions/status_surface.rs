use egui::Ui;

use crate::shell::ValidationRunSummary;

pub fn render(ui: &mut Ui, run_summary: &ValidationRunSummary) {
    ui.heading("Status surface");
    ui.horizontal(|ui| {
        ui.label("Active plan");
        ui.monospace(
            run_summary
                .runtime_observation()
                .active_plan_digest()
                .to_string(),
        );
    });
}
