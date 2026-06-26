//! FgTextArea — styled multiline text input.

use egui::{CornerRadius, Frame, Response, Stroke, Ui};
use forge_ui_theme::ForgeTheme;

/// Props for FgTextArea.
pub struct FgTextArea<'a> {
    pub label: Option<&'a str>,
    pub placeholder: &'a str,
    pub value: &'a mut String,
    pub rows: usize,
}

impl<'a> FgTextArea<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            label: None,
            placeholder: "",
            value,
            rows: 4,
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
    pub fn rows(mut self, r: usize) -> Self {
        self.rows = r;
        self
    }
}

/// Render a styled multiline text area. Returns the TextEdit response.
pub fn fg_textarea(ui: &mut Ui, theme: &ForgeTheme, props: FgTextArea<'_>) -> Response {
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
            top: 8,
            bottom: 8,
        })
        .show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(props.value)
                    .hint_text(props.placeholder)
                    .desired_width(f32::INFINITY)
                    .desired_rows(props.rows)
                    .font(egui::FontId::proportional(theme.font_size_sm))
                    .text_color(theme.text_primary)
                    .frame(false),
            )
        })
        .inner
}
