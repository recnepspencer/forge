//! FgInput — styled single-line text input.

use egui::{CornerRadius, Frame, Response, Stroke, Ui};
use forge_ui_theme::ForgeTheme;

/// Props for FgInput.
pub struct FgInput<'a> {
    pub label: Option<&'a str>,
    pub placeholder: &'a str,
    pub value: &'a mut String,
}

impl<'a> FgInput<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            label: None,
            placeholder: "",
            value,
        }
    }
    pub fn label(mut self, l: &'a str) -> Self {
        self.label = Some(l);
        self
    }
    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = p;
        self
    }
}

/// Render a styled text input. Returns the TextEdit response.
pub fn fg_input(ui: &mut Ui, theme: &ForgeTheme, props: FgInput<'_>) -> Response {
    if let Some(label) = props.label {
        ui.label(
            egui::RichText::new(label)
                .color(theme.text_secondary)
                .size(theme.font_size_sm),
        );
        ui.add_space(4.0);
    }

    Frame::new()
        .fill(theme.bg_base)
        .stroke(Stroke::new(1.0, theme.border_default))
        .corner_radius(CornerRadius::same(theme.radius_sm as u8))
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 6,
            bottom: 6,
        })
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::singleline(props.value)
                    .hint_text(props.placeholder)
                    .desired_width(f32::INFINITY)
                    .font(egui::FontId::proportional(theme.font_size_sm))
                    .text_color(theme.text_primary)
                    .frame(false),
            )
        })
        .inner
}
