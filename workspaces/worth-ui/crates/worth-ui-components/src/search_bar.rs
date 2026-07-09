//! SearchBar — ⌘K search input with icon and keyboard badge.

use egui::{CornerRadius, Response, Stroke, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// Props for the search bar.
pub struct SearchBarProps {
    pub is_open: bool,
}

impl SearchBarProps {
    pub fn new(is_open: bool) -> Self {
        Self { is_open }
    }
}

/// Render the search bar. Returns click response.
pub fn fg_search_bar(ui: &mut Ui, theme: &WorthTheme, props: SearchBarProps) -> Response {
    let width = 280.0_f32;
    let height = 30.0_f32;
    let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let hovered = resp.hovered() || props.is_open;
        let bg = if hovered {
            theme.bg_raised
        } else {
            theme.bg_base
        };
        let border = if hovered {
            theme.border_subtle
        } else {
            theme.border_default
        };
        ui.painter().rect(
            rect,
            CornerRadius::same(theme.radius_md as u8),
            bg,
            Stroke::new(1.0, border),
            egui::StrokeKind::Outside,
        );

        // Search icon
        let icon_y = rect.center().y - 6.0;
        let icon_g = ui.fonts(|f| {
            f.layout_no_wrap(
                "🔍".to_string(),
                egui::FontId::proportional(11.0),
                theme.text_muted,
            )
        });
        ui.painter().galley(
            egui::Pos2::new(rect.min.x + 10.0, icon_y),
            icon_g,
            theme.text_muted,
        );

        // Placeholder
        let text_g = ui.fonts(|f| {
            f.layout_no_wrap(
                "Search operations…".to_string(),
                egui::FontId::proportional(theme.font_size_sm),
                theme.text_muted,
            )
        });
        ui.painter().galley(
            egui::Pos2::new(rect.min.x + 28.0, rect.center().y - text_g.size().y / 2.0),
            text_g,
            theme.text_muted,
        );

        // ⌘K badge
        let badge_g = ui.fonts(|f| {
            f.layout_no_wrap(
                "⌘K".to_string(),
                egui::FontId::proportional(10.0),
                theme.text_muted,
            )
        });
        let bw = badge_g.size().x + 8.0;
        let bh = badge_g.size().y + 4.0;
        let badge_rect = egui::Rect::from_min_size(
            egui::Pos2::new(rect.max.x - bw - 8.0, rect.center().y - bh / 2.0),
            Vec2::new(bw, bh),
        );
        ui.painter().rect(
            badge_rect,
            CornerRadius::same(3),
            theme.border_default,
            Stroke::NONE,
            egui::StrokeKind::Outside,
        );
        ui.painter().galley(
            badge_rect.min + Vec2::new(4.0, 2.0),
            badge_g,
            theme.text_muted,
        );
    }

    resp
}
