//! FgModal — screen overlay popover container.
//!
//! The modal is just the overlay + centered card. Content is defined by the caller
//! via a closure. The modal does not manage its own open/close state — the caller
//! controls visibility with a bool.

use egui::{Color32, CornerRadius, Frame, Stroke};
use worth_ui_theme::WorthTheme;

pub struct FgModalResponse<R> {
    pub inner: egui::InnerResponse<R>,
    pub outside_clicked: bool,
}

/// Render a modal overlay. Only call when the modal should be visible.
/// Returns the inner response from the content closure.
pub fn fg_modal<R>(
    ctx: &egui::Context,
    theme: &WorthTheme,
    id: &str,
    width: f32,
    add_contents: impl FnOnce(&mut egui::Ui) -> R,
) -> FgModalResponse<R> {
    // Dark scrim
    let scrim_resp = egui::Area::new(egui::Id::new(format!("{id}_scrim")))
        .fixed_pos(egui::Pos2::ZERO)
        .order(egui::Order::Foreground)
        .interactable(true)
        .show(ctx, |ui| {
            let screen = ctx.screen_rect();
            let (rect, resp) = ui.allocate_exact_size(screen.size(), egui::Sense::click());
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(140));
            resp
        });

    // Content card
    let inner = egui::Area::new(egui::Id::new(id))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            Frame::new()
                .fill(theme.bg_raised)
                .stroke(Stroke::new(1.0, theme.border_default))
                .corner_radius(CornerRadius::same(theme.radius_lg as u8))
                .inner_margin(egui::Margin::same(24))
                .shadow(egui::Shadow {
                    offset: [0, 12],
                    blur: 40,
                    spread: 4,
                    color: Color32::from_black_alpha(100),
                })
                .show(ui, |ui| {
                    ui.set_width(width);
                    add_contents(ui)
                })
                .inner
        });

    FgModalResponse {
        outside_clicked: scrim_resp.inner.clicked() && !inner.response.hovered(),
        inner,
    }
}
