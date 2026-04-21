//! FgButton — the primary interactive component.

use egui::{Response, Ui, Vec2};
use forge_ui_theme::ForgeTheme;

use crate::IconStore;

/// Darken a color by `amount` (0..1).
fn darken(c: egui::Color32, amount: f32) -> egui::Color32 {
    let f = 1.0 - amount;
    egui::Color32::from_rgb(
        (c.r() as f32 * f) as u8,
        (c.g() as f32 * f) as u8,
        (c.b() as f32 * f) as u8,
    )
}

/// Lighten a color by `amount` (0..1).
fn lighten(c: egui::Color32, amount: f32) -> egui::Color32 {
    egui::Color32::from_rgb(
        (c.r() as f32 + (255.0 - c.r() as f32) * amount) as u8,
        (c.g() as f32 + (255.0 - c.g() as f32) * amount) as u8,
        (c.b() as f32 + (255.0 - c.b() as f32) * amount) as u8,
    )
}

/// Button variant controls colour and visual weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FgButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Danger,
    Link,
}

/// Button size controls padding and font size.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FgButtonSize {
    Sm,
    #[default]
    Md,
    Lg,
}

/// Props for FgButton.
pub struct FgButton<'a> {
    pub label: &'a str,
    pub variant: FgButtonVariant,
    pub size: FgButtonSize,
    pub disabled: bool,
    pub loading: bool,
    pub width: Option<f32>,
}

impl<'a> FgButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: FgButtonVariant::Primary,
            size: FgButtonSize::Md,
            disabled: false,
            loading: false,
            width: None,
        }
    }
    pub fn variant(mut self, v: FgButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn size(mut self, s: FgButtonSize) -> Self {
        self.size = s;
        self
    }
    pub fn disabled(mut self, d: bool) -> Self {
        self.disabled = d;
        self
    }
    pub fn loading(mut self, l: bool) -> Self {
        self.loading = l;
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
}

pub fn fg_button(
    ui: &mut Ui,
    theme: &ForgeTheme,
    icons: &IconStore,
    props: FgButton<'_>,
) -> Response {
    let (h_pad, v_pad, font_size) = match props.size {
        FgButtonSize::Sm => (theme.sp(1), theme.sp(0), theme.font_size_sm),
        FgButtonSize::Md => (theme.sp(3), theme.sp(1), theme.font_size_md),
        FgButtonSize::Lg => (theme.sp(5), theme.sp(2), theme.font_size_lg),
    };

    let (bg, text_col, border) = if props.disabled || props.loading {
        (theme.bg_raised, theme.text_muted, theme.border_subtle)
    } else {
        match props.variant {
            FgButtonVariant::Primary => (
                theme.accent_primary,
                theme.text_inverse,
                theme.accent_primary,
            ),
            FgButtonVariant::Secondary => {
                (theme.bg_raised, theme.text_primary, theme.border_default)
            }
            FgButtonVariant::Ghost => (
                egui::Color32::TRANSPARENT,
                theme.text_primary,
                egui::Color32::TRANSPARENT,
            ),
            FgButtonVariant::Danger => (theme.danger_surface, theme.danger, theme.danger),
            FgButtonVariant::Link => (
                egui::Color32::TRANSPARENT,
                theme.accent_primary,
                egui::Color32::TRANSPARENT,
            ),
        }
    };

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(
            props.label.to_string(),
            egui::FontId::proportional(font_size),
            text_col,
        )
    });

    let mut content_width = if props.loading {
        galley.size().x + font_size + 8.0
    } else {
        galley.size().x
    };
    if props.label.is_empty() && props.loading {
        content_width = font_size;
    }

    let natural_width = content_width + h_pad * 2.0;
    let final_width = props.width.unwrap_or(natural_width).max(natural_width);

    let size = Vec2::new(final_width, galley.size().y.max(font_size) + v_pad * 2.0);
    let (rect, mut response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let rounding = egui::CornerRadius::same(theme.radius_md as u8);

        let is_pressed = response.is_pointer_button_down_on() && !props.disabled && !props.loading;
        let is_hovered = response.hovered() && !props.disabled && !props.loading;

        let actual_bg = if is_pressed {
            match props.variant {
                FgButtonVariant::Primary => darken(theme.accent_primary, 0.2),
                FgButtonVariant::Secondary => darken(theme.bg_raised, 0.15),
                FgButtonVariant::Ghost => egui::Color32::from_white_alpha(12),
                FgButtonVariant::Danger => darken(theme.danger, 0.2),
                FgButtonVariant::Link => egui::Color32::from_white_alpha(8),
            }
        } else if is_hovered {
            match props.variant {
                FgButtonVariant::Primary => theme.accent_hover,
                FgButtonVariant::Secondary => lighten(theme.bg_raised, 0.1),
                FgButtonVariant::Ghost => egui::Color32::from_white_alpha(8),
                FgButtonVariant::Danger => lighten(theme.danger_surface, 0.1),
                FgButtonVariant::Link => egui::Color32::TRANSPARENT,
            }
        } else {
            bg
        };

        // Pressed scale-down effect: inset rect by 1px
        let draw_rect = if is_pressed { rect.shrink(1.0) } else { rect };

        ui.painter().rect(
            draw_rect,
            rounding,
            actual_bg,
            egui::Stroke::new(1.0, border),
            egui::StrokeKind::Outside,
        );

        let extra_space = final_width - natural_width;
        let align_offset = Vec2::new(extra_space / 2.0, 0.0);

        let text_offset = if is_pressed {
            Vec2::new(h_pad, v_pad + 0.5)
        } else {
            Vec2::new(h_pad, v_pad)
        };
        let mut content_start = draw_rect.min + text_offset + align_offset;

        let content_height = galley.size().y.max(font_size);
        let icon_y_offset = ((content_height - font_size) / 2.0).max(0.0);
        let text_y_offset = ((content_height - galley.size().y) / 2.0).max(0.0);

        if props.loading {
            let icon_pos = content_start + Vec2::new(0.0, icon_y_offset);
            let icon_rect = egui::Rect::from_min_size(icon_pos, Vec2::splat(font_size));
            ui.ctx().request_repaint(); // Needs continuous repaint for animation
            let t = ui.input(|i| i.time);
            let angle = (t * std::f64::consts::PI * 2.0) as f32; // 1 rotation per second
            icons.draw_rotated(ui, crate::FgIcon::LoaderCircle, icon_rect, text_col, angle);

            if !props.label.is_empty() {
                content_start.x += font_size + 8.0;
                let text_pos = content_start + Vec2::new(0.0, text_y_offset);
                ui.painter().galley(text_pos, galley, text_col);
            }
        } else {
            let text_pos = content_start + Vec2::new(0.0, text_y_offset);
            ui.painter().galley(text_pos, galley, text_col);
        }
    }

    if props.disabled || props.loading {
        response = response.on_disabled_hover_text("Unavailable");
    }
    response
}
