//! Titlebar organism — logo + page tabs + search + theme toggle.

use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Stroke, Vec2};
use forge_ui_components::{
    fg_icon_button, fg_page_tab, fg_search_bar, FgIcon, FgIconButton, FgPageTab, IconStore,
    SearchBarProps,
};
use forge_ui_state::AppState;

use crate::Page;

/// Draw the titlebar. Mutates `page` when a tab is clicked.
pub fn draw_titlebar(
    ctx: &egui::Context,
    state: &mut AppState,
    icons: &IconStore,
    page: &mut Page,
) {
    let t = state.theme.clone();
    let palette_open = state.palette.open;
    let theme_kind = state.theme_kind;

    egui::TopBottomPanel::top("titlebar")
        .exact_height(44.0)
        .frame(
            Frame::new()
                .fill(t.bg_surface)
                .stroke(Stroke::new(1.0, t.border_subtle)),
        )
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(12.0);

                // ── Logo pill ─────────────────────────────────────
                let logo_text = "◆ Forge";
                let galley = ui.fonts(|f| {
                    f.layout_no_wrap(
                        logo_text.to_string(),
                        egui::FontId::proportional(t.font_size_md),
                        Color32::WHITE,
                    )
                });
                let pad = Vec2::new(10.0, 5.0);
                let (logo_rect, _) =
                    ui.allocate_exact_size(galley.size() + pad * 2.0, egui::Sense::hover());
                if ui.is_rect_visible(logo_rect) {
                    ui.painter().rect_filled(
                        logo_rect,
                        CornerRadius::same(t.radius_sm as u8),
                        t.accent_primary,
                    );
                    ui.painter()
                        .galley(logo_rect.min + pad, galley, Color32::WHITE);
                }
                ui.add_space(12.0);

                // ── Page tabs (using FgPageTab atom) ──────────────
                let pages = [(Page::Editor, "Editor"), (Page::TestBoard, "Test Board")];
                let current = *page;
                for (p, label) in &pages {
                    let resp = fg_page_tab(ui, &t, FgPageTab::new(label, current == *p));
                    if resp.clicked() {
                        *page = *p;
                    }
                }
                ui.add_space(4.0);

                // ── Filename chip (editor only) ──────────────────
                if current == Page::Editor {
                    Frame::new()
                        .fill(t.bg_raised)
                        .stroke(Stroke::new(1.0, t.border_default))
                        .corner_radius(CornerRadius::same(t.radius_sm as u8))
                        .inner_margin(egui::Margin {
                            left: 10,
                            right: 10,
                            top: 3,
                            bottom: 3,
                        })
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new("my_model.fg ●")
                                    .color(t.text_secondary)
                                    .size(t.font_size_sm),
                            );
                        });
                }

                // ── Search bar (using molecule) ──────────────────
                let search_resp = fg_search_bar(ui, &t, SearchBarProps::new(palette_open));
                if search_resp.clicked() {
                    state.palette.toggle();
                }

                // ── Theme toggle (right-aligned, using FgIconButton atom) ──
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(16.0);
                    let theme_icon = match theme_kind {
                        forge_ui_state::ThemeKind::Dark => FgIcon::Sun,
                        forge_ui_state::ThemeKind::Light => FgIcon::Moon,
                    };
                    let resp = fg_icon_button(
                        ui,
                        &t,
                        icons,
                        FgIconButton::new(theme_icon).tint(t.text_secondary),
                    );
                    if resp.clicked() {
                        state.toggle_theme();
                    }
                });
            });
        });
}
