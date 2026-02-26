//! FgButton — the primary interactive component.

use egui::{Response, Ui, Vec2};
use forge_ui_theme::ForgeTheme;

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
    pub label:    &'a str,
    pub variant:  FgButtonVariant,
    pub size:     FgButtonSize,
    pub disabled: bool,
    pub loading:  bool,
}

impl<'a> FgButton<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, variant: FgButtonVariant::Primary, size: FgButtonSize::Md, disabled: false, loading: false }
    }
    pub fn variant(mut self, v: FgButtonVariant) -> Self { self.variant = v; self }
    pub fn size(mut self, s: FgButtonSize) -> Self { self.size = s; self }
    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }
}

/// Render a styled Forge button.
pub fn fg_button(ui: &mut Ui, theme: &ForgeTheme, props: FgButton<'_>) -> Response {
    let (h_pad, v_pad, font_size) = match props.size {
        FgButtonSize::Sm => (theme.sp(1), theme.sp(0), theme.font_size_sm),
        FgButtonSize::Md => (theme.sp(3), theme.sp(1), theme.font_size_md),
        FgButtonSize::Lg => (theme.sp(5), theme.sp(2), theme.font_size_lg),
    };

    let (bg, text_col, border) = if props.disabled || props.loading {
        (theme.bg_raised, theme.text_muted, theme.border_subtle)
    } else {
        match props.variant {
            FgButtonVariant::Primary   => (theme.accent_primary, theme.text_inverse, theme.accent_primary),
            FgButtonVariant::Secondary => (theme.bg_raised, theme.text_primary, theme.border_default),
            FgButtonVariant::Ghost     => (egui::Color32::TRANSPARENT, theme.text_primary, egui::Color32::TRANSPARENT),
            FgButtonVariant::Danger    => (theme.danger_surface, theme.danger, theme.danger),
            FgButtonVariant::Link      => (egui::Color32::TRANSPARENT, theme.accent_primary, egui::Color32::TRANSPARENT),
        }
    };

    let label_text = if props.loading {
        format!("⏳ {}", props.label)
    } else {
        props.label.to_string()
    };

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(label_text.clone(), egui::FontId::proportional(font_size), text_col)
    });
    let size = Vec2::new(galley.size().x + h_pad * 2.0, galley.size().y + v_pad * 2.0);
    let (rect, mut response) = ui.allocate_exact_size(size, egui::Sense::click());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        let rounding = egui::CornerRadius::same(theme.radius_md as u8);

        let actual_bg = if response.hovered() && !props.disabled {
            match props.variant {
                FgButtonVariant::Primary => theme.accent_hover,
                _ => theme.accent_hover,
            }
        } else {
            bg
        };

        painter.rect(rect, rounding, actual_bg, egui::Stroke::new(1.0, border), egui::StrokeKind::Outside);
        painter.galley(rect.min + Vec2::new(h_pad, v_pad), galley, text_col);
    }

    if props.disabled || props.loading { response = response.on_disabled_hover_text("Unavailable"); }
    response
}
