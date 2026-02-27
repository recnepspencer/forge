//! FgCard — surface/raised/accent card container.

use egui::{CornerRadius, Frame, InnerResponse, Stroke, Ui};
use forge_ui_theme::ForgeTheme;

/// Card visual weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FgCardVariant {
    /// bg_surface + border_subtle
    #[default]
    Flat,
    /// bg_raised + border_default
    Raised,
    /// accent_subtle + accent border
    Accent,
}

/// Props for FgCard.
pub struct FgCard {
    pub variant: FgCardVariant,
}

impl FgCard {
    pub fn flat() -> Self {
        Self {
            variant: FgCardVariant::Flat,
        }
    }
    pub fn raised() -> Self {
        Self {
            variant: FgCardVariant::Raised,
        }
    }
    pub fn accent() -> Self {
        Self {
            variant: FgCardVariant::Accent,
        }
    }
}

/// Render a card container. Returns the inner response.
pub fn fg_card<R>(
    ui: &mut Ui,
    theme: &ForgeTheme,
    props: FgCard,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let (fill, stroke_color) = match props.variant {
        FgCardVariant::Flat => (theme.bg_surface, theme.border_subtle),
        FgCardVariant::Raised => (theme.bg_raised, theme.border_default),
        FgCardVariant::Accent => (theme.accent_subtle, theme.accent_primary),
    };

    Frame::new()
        .fill(fill)
        .corner_radius(CornerRadius::same(theme.radius_md as u8))
        .inner_margin(egui::Margin::same(16))
        .stroke(Stroke::new(1.0, stroke_color))
        .show(ui, add_contents)
}
