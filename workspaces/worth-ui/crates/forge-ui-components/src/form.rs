//! FgForm — padded form container with a button footer.
//!
//! The form provides consistent padding, and a dedicated bottom section where
//! buttons (passed via closure) are laid out horizontally right-aligned.

use egui::{CornerRadius, Frame, Stroke, Ui};
use forge_ui_theme::ForgeTheme;

/// Render a form container.
///
/// `add_fields` draws the form body (inputs, dropdowns, etc.).
/// `add_buttons` draws the footer buttons (right-aligned).
pub fn fg_form<R>(
    ui: &mut Ui,
    theme: &ForgeTheme,
    add_fields: impl FnOnce(&mut Ui),
    add_buttons: impl FnOnce(&mut Ui) -> R,
) -> R {
    Frame::new()
        .fill(theme.bg_surface)
        .stroke(Stroke::new(1.0, theme.border_subtle))
        .corner_radius(CornerRadius::same(theme.radius_md as u8))
        .inner_margin(egui::Margin::same(20))
        .show(ui, |ui| {
            // ── Form fields ──────────────────────────────────────
            add_fields(ui);

            // ── Divider ──────────────────────────────────────────
            ui.add_space(16.0);
            ui.separator();
            ui.add_space(12.0);

            // ── Button footer (right-aligned) ────────────────────
            ui.with_layout(
                egui::Layout::right_to_left(egui::Align::Center),
                add_buttons,
            )
            .inner
        })
        .inner
}
