//! FgChip — small inline pill for labels, badges, and metadata.

use egui::{CornerRadius, Frame, Response, Stroke, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// Props for FgChip.
pub struct FgChip<'a> {
    pub label: &'a str,
    /// Optional dot color rendered before the label.
    pub dot: Option<egui::Color32>,
}

impl<'a> FgChip<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, dot: None }
    }
    pub fn dot(mut self, color: egui::Color32) -> Self {
        self.dot = Some(color);
        self
    }
}

/// Render a chip pill. Returns the response for click detection.
pub fn fg_chip(ui: &mut Ui, theme: &WorthTheme, props: FgChip<'_>) -> Response {
    Frame::new()
        .fill(theme.bg_raised)
        .stroke(Stroke::new(1.0, theme.border_default))
        .corner_radius(CornerRadius::same(theme.radius_sm as u8))
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 3,
            bottom: 3,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                if let Some(dot_col) = props.dot {
                    let (r, _) = ui.allocate_exact_size(Vec2::new(6.0, 6.0), egui::Sense::hover());
                    ui.painter().circle_filled(r.center(), 3.0, dot_col);
                    ui.add_space(4.0);
                }
                ui.label(
                    egui::RichText::new(props.label)
                        .color(theme.text_secondary)
                        .size(theme.font_size_sm),
                );
            });
        })
        .response
}
