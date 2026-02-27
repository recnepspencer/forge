//! Sidebar organism — feature tree panel.

use eframe::egui;
use egui::{Frame, Stroke};
use forge_ui_components::{fg_feature_row, FeatureRowProps, FgIcon, IconStore};
use forge_ui_state::AppState;

/// Draw the left sidebar with the feature tree.
pub fn draw_sidebar(ctx: &egui::Context, state: &mut AppState, icons: &IconStore) {
    let t_sidebar = state.theme.bg_sidebar;
    let t_border = state.theme.border_subtle;
    egui::SidePanel::left("sidebar")
        .default_width(220.0)
        .min_width(160.0)
        .frame(
            Frame::new()
                .fill(t_sidebar)
                .stroke(Stroke::new(1.0, t_border)),
        )
        .show(ctx, |ui| {
            let t = &state.theme;
            ui.add_space(10.0);

            // Section label
            ui.horizontal(|ui| {
                ui.add_space(14.0);
                ui.label(
                    egui::RichText::new("FEATURE TREE")
                        .color(t.text_muted)
                        .size(t.font_size_xs)
                        .strong(),
                );
            });
            ui.add_space(6.0);
            ui.separator();
            ui.add_space(4.0);

            let features = state.model.features().to_vec();

            for feature in &features {
                let is_selected = state.model.selected() == Some(feature.id);

                let status_color = match &feature.status {
                    forge_ui_types::FeatureStatus::Exact => t.success,
                    forge_ui_types::FeatureStatus::NearBoundary => t.warning,
                    forge_ui_types::FeatureStatus::Error(_) => t.danger,
                    forge_ui_types::FeatureStatus::Pending => t.text_muted,
                };

                let props = FeatureRowProps::new(&feature.name, status_color, is_selected)
                    .icon(FgIcon::Box);
                let resp = fg_feature_row(ui, t, icons, props);
                if resp.clicked() {
                    state.model.select(feature.id);
                }
            }
        });
}
