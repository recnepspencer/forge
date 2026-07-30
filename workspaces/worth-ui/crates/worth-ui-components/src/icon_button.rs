//! FgIconButton — clickable icon with hover background.

use crate::icons::{FgIcon, IconStore};
use egui::{CornerRadius, Response, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// Props for FgIconButton.
pub struct FgIconButton {
    pub icon: FgIcon,
    pub size: f32,
    pub tint: Option<egui::Color32>,
}

impl FgIconButton {
    pub fn new(icon: FgIcon) -> Self {
        Self {
            icon,
            size: 16.0,
            tint: None,
        }
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn tint(mut self, c: egui::Color32) -> Self {
        self.tint = Some(c);
        self
    }
}

/// Render a clickable icon button. Returns click response.
pub fn fg_icon_button(
    ui: &mut Ui,
    theme: &WorthTheme,
    icons: &IconStore,
    props: FgIconButton,
) -> Response {
    let padding = 6.0;
    let outer = props.size + padding * 2.0;
    let (rect, resp) = ui.allocate_exact_size(Vec2::splat(outer), egui::Sense::click());

    if ui.is_rect_visible(rect) {
        if resp.hovered() {
            ui.painter().rect_filled(
                rect,
                CornerRadius::same(theme.radius_sm as u8),
                theme.bg_raised,
            );
        }
        let tint = props.tint.unwrap_or(theme.text_secondary);
        if let Some(tex) = icons.textures.get(&props.icon) {
            let icon_pos = rect.center() - Vec2::splat(props.size / 2.0);
            let sized = egui::load::SizedTexture::new(tex.id(), [props.size, props.size]);
            let img = egui::Image::from_texture(sized).tint(tint);
            img.paint_at(
                ui,
                egui::Rect::from_min_size(icon_pos, Vec2::splat(props.size)),
            );
        } else {
            let g = ui.fonts_mut(|f| {
                f.layout_no_wrap(
                    props.icon.glyph().to_string(),
                    egui::FontId::proportional(props.size * 0.85),
                    tint,
                )
            });
            ui.painter().galley(rect.center() - g.size() / 2.0, g, tint);
        }
    }

    resp
}
