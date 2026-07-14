//! FgPageTab — navigation tab with active underline.

use egui::{CornerRadius, Response, Stroke, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// Props for FgPageTab.
pub struct FgPageTab<'a> {
    pub label: &'a str,
    pub is_active: bool,
}

impl<'a> FgPageTab<'a> {
    pub fn new(label: &'a str, is_active: bool) -> Self {
        Self { label, is_active }
    }
}

/// Render a page navigation tab. Returns click response.
pub fn fg_page_tab(ui: &mut Ui, theme: &WorthTheme, props: FgPageTab<'_>) -> Response {
    let text_color = if props.is_active {
        theme.accent_primary
    } else {
        theme.text_secondary
    };

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            props.label.to_string(),
            egui::FontId::proportional(theme.font_size_sm),
            text_color,
        )
    });
    let pad = Vec2::new(10.0, 4.0);
    let size = galley.size() + pad * 2.0;
    let (rect, resp) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        if resp.hovered() && !props.is_active {
            ui.painter().rect_filled(
                rect,
                CornerRadius::same(theme.radius_sm as u8),
                theme.bg_raised,
            );
        }
        ui.painter().galley(rect.min + pad, galley, text_color);

        if props.is_active {
            let y = rect.max.y - 1.0;
            ui.painter().hline(
                rect.min.x + 4.0..=rect.max.x - 4.0,
                y,
                Stroke::new(2.0, theme.accent_primary),
            );
        }
    }

    resp
}
