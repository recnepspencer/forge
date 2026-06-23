use eframe::egui::{self, Button, Frame, RichText, Stroke};

use crate::{ValidationManualFlowId, ValidationManualFlowMatrixRenderPlan};

pub fn render_manual_flow_matrix(
    ui: &mut egui::Ui,
    plan: &ValidationManualFlowMatrixRenderPlan,
) -> Option<ValidationManualFlowId> {
    ui.heading("Manual Verification Matrix");
    ui.label("Run a named flow, then compare the visible product proof against the typed expectation row.");
    ui.separator();

    let style = plan.style();
    let mut requested = None;
    for row in plan.rows() {
        Frame::group(ui.style())
            .fill(style.menu_fill())
            .stroke(Stroke::new(
                style.border_width_points(),
                style.border_color(),
            ))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(row.title())
                            .strong()
                            .color(style.text_color()),
                    );
                    let button = Button::new("Run flow")
                        .fill(style.menu_active_fill())
                        .stroke(Stroke::new(
                            style.border_width_points(),
                            style.border_color(),
                        ));
                    if ui.add(button).clicked() {
                        requested = Some(row.flow_id());
                    }
                });
                ui.monospace(format!("Authored input: {}", row.authored_input()));
                ui.monospace(format!("Expected status: {}", row.expected_status()));
                ui.monospace(format!("Observed status: {}", row.observed_status()));
                ui.monospace(format!(
                    "Expected visible result: {}",
                    row.expected_visible_result()
                ));
                ui.monospace(format!(
                    "Observed visible result: {}",
                    row.observed_visible_result()
                ));
                ui.monospace(format!(
                    "Expected counters: {}",
                    row.expected_counter_posture()
                ));
                ui.monospace(format!(
                    "Observed counters: {}",
                    row.observed_counter_posture()
                ));
                ui.monospace(format!(
                    "Observed counter details: {}",
                    row.observed_counter_details()
                ));
                ui.monospace(format!(
                    "Expected replay posture: {}",
                    row.expected_replay_posture()
                ));
                ui.monospace(format!(
                    "Observed replay posture: {}",
                    row.observed_replay_posture()
                ));
                ui.monospace(format!(
                    "Observed projection digest: {}",
                    row.observed_projection_digest()
                ));
                render_list(ui, "Expected changed facts", row.expected_changed_facts());
                render_list(ui, "Observed changed facts", row.observed_changed_facts());
                render_list(
                    ui,
                    "Expected rebuilt projections",
                    row.expected_rebuilt_projections(),
                );
                render_list(
                    ui,
                    "Observed rebuilt projections",
                    row.observed_rebuilt_projections(),
                );
                render_list(
                    ui,
                    "Expected preserved projections",
                    row.expected_preserved_projections(),
                );
                render_list(
                    ui,
                    "Observed preserved projections",
                    row.observed_preserved_projections(),
                );
                let verdict = if row.matches_expectation() {
                    "Expectation satisfied"
                } else {
                    "Expectation not yet satisfied"
                };
                ui.label(RichText::new(verdict).color(style.text_color()));
            });
        ui.separator();
    }
    requested
}

fn render_list(ui: &mut egui::Ui, label: &str, values: &[String]) {
    if values.is_empty() {
        ui.monospace(format!("{label}: none"));
        return;
    }
    ui.monospace(format!("{label}:"));
    for value in values {
        ui.monospace(format!("  {value}"));
    }
}
