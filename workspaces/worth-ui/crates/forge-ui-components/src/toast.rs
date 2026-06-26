//! FgToast — dismissible toast banner.

use egui::Context;
use forge_ui_theme::ForgeTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FgToastVariant {
    #[default]
    Info,
    Success,
    Warning,
    Error,
}

pub struct FgToast<'a> {
    pub variant: FgToastVariant,
    pub title: &'a str,
    pub message: &'a str,
    pub dismissible: bool,
}

impl<'a> FgToast<'a> {
    pub fn new(message: &'a str) -> Self {
        Self {
            variant: FgToastVariant::Info,
            title: "",
            message,
            dismissible: false,
        }
    }

    pub fn variant(mut self, v: FgToastVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn dismissible(mut self, d: bool) -> Self {
        self.dismissible = d;
        self
    }
}

/// Renders a toast banner. Returns a Response; if `dismissible` is true,
/// the caller should check `response.secondary_clicked()` or a returned `bool`
/// to remove the toast from state.
pub fn fg_toast(
    ctx: &Context,
    theme: &ForgeTheme,
    icons: &crate::IconStore,
    id_source: impl std::hash::Hash,
    props: FgToast<'_>,
) -> bool {
    let (bg, accent, icon) = match props.variant {
        FgToastVariant::Info => (theme.info_surface, theme.info, crate::FgIcon::Info),
        FgToastVariant::Success => (theme.success_surface, theme.success, crate::FgIcon::Check),
        FgToastVariant::Warning => (theme.warning_surface, theme.warning, crate::FgIcon::Warning),
        FgToastVariant::Error => (theme.danger_surface, theme.danger, crate::FgIcon::X),
    };

    let mut dismissed = false;
    let frame = egui::Frame::new()
        .fill(bg)
        .inner_margin(egui::Margin::symmetric(
            theme.sp(3) as i8,
            theme.sp(2) as i8,
        ))
        .corner_radius(egui::CornerRadius::same(theme.radius_md as u8))
        .stroke(egui::Stroke::new(1.0, accent))
        .shadow(egui::Shadow {
            offset: [0, 8],
            blur: 16,
            spread: 0,
            color: egui::Color32::from_black_alpha(40),
        });

    egui::Area::new(egui::Id::new(id_source))
        .anchor(egui::Align2::RIGHT_BOTTOM, egui::vec2(-24.0, -24.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            frame.show(ui, |ui| {
                ui.horizontal(|ui| {
                    icons.draw(ui, icon, theme.font_size_md, accent);
                    ui.add_space(8.0);
                    ui.vertical(|ui| {
                        if !props.title.is_empty() {
                            ui.label(
                                egui::RichText::new(props.title)
                                    .color(accent)
                                    .size(theme.font_size_sm)
                                    .strong(),
                            );
                        }
                        ui.label(
                            egui::RichText::new(props.message)
                                .color(theme.text_secondary)
                                .size(theme.font_size_sm),
                        );
                    });
                    if props.dismissible {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            let (rect, resp) = ui.allocate_exact_size(
                                egui::Vec2::splat(theme.font_size_sm),
                                egui::Sense::click(),
                            );
                            let col = if resp.hovered() {
                                theme.text_primary
                            } else {
                                theme.text_secondary
                            };
                            icons.draw_in_rect(ui, crate::FgIcon::X, rect, col);
                            if resp.clicked() {
                                dismissed = true;
                            }
                        });
                    }
                });
            });
        });

    dismissed
}
