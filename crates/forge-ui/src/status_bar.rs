//! Status bar organism — bottom telemetry strip.

use eframe::egui;
use egui::{Frame, Stroke};
use forge_ui_state::AppState;

/// Draw the bottom status bar.
pub fn draw_status_bar(ctx: &egui::Context, state: &AppState) {
    let t = &state.theme;
    egui::TopBottomPanel::bottom("statusbar")
        .exact_height(24.0)
        .frame(Frame::new()
            .fill(t.bg_surface)
            .stroke(Stroke::new(1.0, t.border_subtle)))
        .show(ctx, |ui| {
            let tel = &state.telemetry;
            ui.horizontal_centered(|ui| {
                ui.add_space(12.0);
                ui.label(egui::RichText::new(format!(
                    "{} faces · {} verts · {} edges",
                    tel.face_count, tel.vertex_count, tel.edge_count,
                )).color(t.text_muted).size(t.font_size_xs));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new(format!("Exact · {:.1}ms · v0.1", tel.last_op_ms))
                        .color(t.text_muted).size(t.font_size_xs));
                });
            });
        });
}
