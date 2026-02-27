//! Right drawer organism — tabbed panel with Properties and Chat.

use eframe::egui;
use egui::{CornerRadius, Frame, Pos2, Rect, Stroke, Vec2};
use forge_ui_state::AppState;

use crate::chat_panel;
use crate::properties_panel;

/// Draw the right-side tabbed drawer.
pub fn draw_right_drawer(ctx: &egui::Context, state: &mut AppState) {
    let t_surface = state.theme.bg_surface;
    let t_border = state.theme.border_subtle;
    egui::SidePanel::right("right_drawer")
        .default_width(308.0)
        .frame(
            Frame::new()
                .fill(t_surface)
                .stroke(Stroke::new(1.0, t_border)),
        )
        .show(ctx, |ui| {
            let t = &state.theme;

            // ── Tab bar ───────────────────────────────────────────
            let tab_h = 38.0;
            let avail_w = ui.available_width();
            let (tab_bar_rect, _) =
                ui.allocate_exact_size(Vec2::new(avail_w, tab_h), egui::Sense::hover());

            // Underline
            ui.painter().rect_filled(
                Rect::from_min_size(
                    Pos2::new(tab_bar_rect.min.x, tab_bar_rect.max.y - 1.0),
                    Vec2::new(avail_w, 1.0),
                ),
                0.0,
                t.border_subtle,
            );

            let tabs = [
                (forge_ui_state::DrawerTab::Properties, "Properties"),
                (forge_ui_state::DrawerTab::Chat, "Chat"),
            ];
            let tab_w = avail_w / tabs.len() as f32;

            for (idx, (tab, label)) in tabs.iter().enumerate() {
                let is_active = state.drawer.active_tab == *tab;
                let tab_x = tab_bar_rect.min.x + idx as f32 * tab_w;
                let this_rect = Rect::from_min_size(
                    Pos2::new(tab_x, tab_bar_rect.min.y),
                    Vec2::new(tab_w, tab_h),
                );

                let text_color = if is_active {
                    t.text_primary
                } else {
                    t.text_muted
                };
                let galley = ui.fonts(|f| {
                    f.layout_no_wrap(
                        label.to_string(),
                        egui::FontId::proportional(t.font_size_sm),
                        text_color,
                    )
                });
                let text_pos =
                    this_rect.center() - Vec2::new(galley.size().x / 2.0, galley.size().y / 2.0);
                ui.painter().galley(text_pos, galley, text_color);

                if is_active {
                    ui.painter().rect_filled(
                        Rect::from_min_size(
                            Pos2::new(this_rect.min.x + 16.0, this_rect.max.y - 2.0),
                            Vec2::new(this_rect.width() - 32.0, 2.0),
                        ),
                        CornerRadius::same(1),
                        t.accent_primary,
                    );
                }

                let resp = ui.interact(
                    this_rect,
                    egui::Id::new(format!("tab_{idx}")),
                    egui::Sense::click(),
                );
                if resp.clicked() {
                    state.drawer.active_tab = *tab;
                }
            }

            ui.add_space(8.0);

            match state.drawer.active_tab {
                forge_ui_state::DrawerTab::Properties => {
                    properties_panel::draw_properties_panel(ui, state)
                }
                forge_ui_state::DrawerTab::Chat => chat_panel::draw_chat_panel(ui, state),
            }
        });
}
