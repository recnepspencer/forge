use egui::Ui;

use crate::shell::ValidationRunSummary;

pub fn render(ui: &mut Ui, run_summary: &ValidationRunSummary) {
    ui.heading("Scenario list");
    ui.monospace(run_summary.selected_scenario());
}
