//! FeatureRow — sidebar feature list item.

use crate::icons::{FgIcon, IconStore};
use egui::{Color32, CornerRadius, Pos2, Response, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// Props for a feature row.
pub struct FeatureRowProps<'a> {
    pub name: &'a str,
    pub icon: FgIcon,
    pub status_color: Color32,
    pub is_selected: bool,
}

impl<'a> FeatureRowProps<'a> {
    pub fn new(name: &'a str, status_color: Color32, is_selected: bool) -> Self {
        Self {
            name,
            icon: FgIcon::Box,
            status_color,
            is_selected,
        }
    }
    pub fn icon(mut self, icon: FgIcon) -> Self {
        self.icon = icon;
        self
    }
}

/// Render a feature row. Returns click response.
pub fn fg_feature_row(
    ui: &mut Ui,
    theme: &WorthTheme,
    icons: &IconStore,
    props: FeatureRowProps<'_>,
) -> Response {
    let row_h = 28.0;
    let avail_w = ui.available_width();
    let (row_rect, row_resp) =
        ui.allocate_exact_size(Vec2::new(avail_w, row_h), egui::Sense::click());

    if ui.is_rect_visible(row_rect) {
        let painter = ui.painter();
        let rr = CornerRadius::same(theme.radius_sm as u8);

        // Inset background
        let bg_rect = egui::Rect::from_min_max(
            Pos2::new(row_rect.min.x + 6.0, row_rect.min.y + 1.0),
            Pos2::new(row_rect.max.x - 6.0, row_rect.max.y - 1.0),
        );

        if props.is_selected {
            painter.rect_filled(bg_rect, rr, theme.accent_subtle);
            // Left accent stripe
            painter.rect_filled(
                egui::Rect::from_min_size(bg_rect.min, Vec2::new(3.0, bg_rect.height())),
                CornerRadius::same(2),
                theme.accent_primary,
            );
        } else if row_resp.hovered() {
            painter.rect_filled(bg_rect, rr, Color32::from_white_alpha(7));
        }

        // Icon + label via child UI
        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(bg_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        child.add_space(8.0);
        icons.draw(&mut child, props.icon, 14.0, props.status_color);
        child.add_space(6.0);
        let label_color = if props.is_selected {
            theme.text_primary
        } else {
            theme.text_secondary
        };
        child.label(
            egui::RichText::new(props.name)
                .color(label_color)
                .size(theme.font_size_sm),
        );
    }

    row_resp
}
