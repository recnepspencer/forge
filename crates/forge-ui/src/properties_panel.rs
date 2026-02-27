//! Properties panel — displays selected feature properties.

use eframe::egui;
use egui::{CornerRadius, Frame};
use forge_ui_state::AppState;

/// Draw the properties panel content.
pub fn draw_properties_panel(ui: &mut egui::Ui, state: &mut AppState) {
    let t = &state.theme;
    ui.add_space(4.0);

    if let Some(_id) = state.model.selected() {
        if let Some(plane) = state.model.planes().iter().next() {
            Frame::new()
                .fill(t.accent_subtle)
                .corner_radius(CornerRadius::same(t.radius_sm as u8))
                .inner_margin(egui::Margin {
                    left: 8,
                    right: 8,
                    top: 3,
                    bottom: 3,
                })
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("Selected Plane")
                            .color(t.accent_primary)
                            .size(t.font_size_xs)
                            .strong(),
                    );
                });
            ui.add_space(10.0);

            for (label, value) in [
                ("Normal X", format!("{:.4}", plane.normal[0])),
                ("Normal Y", format!("{:.4}", plane.normal[1])),
                ("Normal Z", format!("{:.4}", plane.normal[2])),
                ("Offset D", format!("{:.4}", plane.offset)),
            ] {
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    ui.label(
                        egui::RichText::new(label)
                            .color(t.text_muted)
                            .size(t.font_size_sm),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new(value)
                                .color(t.text_primary)
                                .size(t.font_size_sm)
                                .monospace(),
                        );
                    });
                });
            }
        }
    } else {
        ui.add_space(24.0);
        ui.vertical_centered(|ui| {
            ui.label(
                egui::RichText::new("No selection")
                    .color(t.text_secondary)
                    .size(t.font_size_md)
                    .strong(),
            );
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("Click a feature in the tree\nor a shape in the viewport.")
                    .color(t.text_muted)
                    .size(t.font_size_sm),
            );
        });
    }
}
