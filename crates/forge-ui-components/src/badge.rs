//! FgBadge — small inline status chip.

use egui::{Response, Ui, Vec2};
use forge_ui_theme::ForgeTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FgBadgeVariant {
    /// Green — resolved exactly.
    Exact,
    /// Yellow — near a tolerance boundary.
    NearBoundary,
    /// Red — error state.
    Error,
    /// Grey — not yet evaluated.
    Pending,
    /// Uses accent colour.
    Info,
}

pub struct FgBadge<'a> {
    pub label:   &'a str,
    pub variant: FgBadgeVariant,
}

pub fn fg_badge(ui: &mut Ui, theme: &ForgeTheme, props: FgBadge<'_>) -> Response {
    let (bg, fg) = match props.variant {
        FgBadgeVariant::Exact        => (theme.success_surface, theme.success),
        FgBadgeVariant::NearBoundary => (theme.warning_surface, theme.warning),
        FgBadgeVariant::Error        => (theme.danger_surface,  theme.danger),
        FgBadgeVariant::Pending      => (theme.bg_raised,       theme.text_muted),
        FgBadgeVariant::Info         => (theme.info_surface,    theme.info),
    };

    let galley = ui.fonts(|f| {
        f.layout_no_wrap(props.label.to_string(), egui::FontId::proportional(theme.font_size_xs), fg)
    });
    let h_pad = theme.sp(1);
    let v_pad = 2.0_f32;
    let size = Vec2::new(galley.size().x + h_pad * 2.0, galley.size().y + v_pad * 2.0);
    let (rect, response) = ui.allocate_exact_size(size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        let painter = ui.painter();
        painter.rect(rect, egui::CornerRadius::same(theme.radius_pill as u8), bg, egui::Stroke::NONE, egui::StrokeKind::Outside);

        painter.galley(rect.min + Vec2::new(h_pad, v_pad), galley, fg);
    }
    response
}
