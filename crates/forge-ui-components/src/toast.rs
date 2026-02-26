//! FgToast — dismissible toast banner.

use egui::{Response, Ui};
use forge_ui_theme::ForgeTheme;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FgToastVariant { #[default] Info, Success, Warning, Error }

pub struct FgToast<'a> {
    pub variant:     FgToastVariant,
    pub title:       &'a str,
    pub message:     &'a str,
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
}

/// Renders a toast banner. Returns a Response; if `dismissible` is true,
/// the caller should check `response.secondary_clicked()` or a returned `bool`
/// to remove the toast from state.
pub fn fg_toast(ui: &mut Ui, theme: &ForgeTheme, props: FgToast<'_>) -> bool {
    let (bg, accent, icon) = match props.variant {
        FgToastVariant::Info    => (theme.info_surface,    theme.info,    "ℹ"),
        FgToastVariant::Success => (theme.success_surface, theme.success, "✓"),
        FgToastVariant::Warning => (theme.warning_surface, theme.warning, "⚠"),
        FgToastVariant::Error   => (theme.danger_surface,  theme.danger,  "✕"),
    };

    let mut dismissed = false;
    let frame = egui::Frame::new()
        .fill(bg)
        .inner_margin(egui::Margin::symmetric(theme.sp(3) as i8, theme.sp(2) as i8))
        .corner_radius(egui::CornerRadius::same(theme.radius_md as u8))
        .stroke(egui::Stroke::new(1.0, accent));

    frame.show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new(icon).color(accent).size(theme.font_size_md));
            ui.vertical(|ui| {
                if !props.title.is_empty() {
                    ui.label(egui::RichText::new(props.title)
                        .color(accent)
                        .size(theme.font_size_sm)
                        .strong());
                }
                ui.label(egui::RichText::new(props.message)
                    .color(theme.text_secondary)
                    .size(theme.font_size_sm));
            });
            if props.dismissible {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button(egui::RichText::new("✕").color(theme.text_muted).size(theme.font_size_sm)).clicked() {
                        dismissed = true;
                    }
                });
            }
        });
    });

    dismissed
}
