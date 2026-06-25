use egui::RichText;
use worth_ui::facade::{
    WorthUiMountedFlowAlign, WorthUiMountedInteractionNodeReceipt,
    WorthUiMountedInteractionStyleReceipt,
};

use super::receipt_color_translation::to_egui_color;

pub(super) fn render_live_view_actions_from_mounted(
    ui: &mut egui::Ui,
    align: WorthUiMountedFlowAlign,
    interactions: &[WorthUiMountedInteractionNodeReceipt],
    submissions: &mut Vec<WorthUiMountedInteractionNodeReceipt>,
) {
    let layout = match align {
        WorthUiMountedFlowAlign::End => egui::Layout::right_to_left(egui::Align::Center),
        WorthUiMountedFlowAlign::Start | WorthUiMountedFlowAlign::Center => {
            egui::Layout::left_to_right(egui::Align::Center)
        }
    };
    ui.with_layout(layout, |ui| {
        for row in interactions {
            render_live_view_interaction(ui, row, submissions);
        }
    });
}

fn render_live_view_interaction(
    ui: &mut egui::Ui,
    row: &WorthUiMountedInteractionNodeReceipt,
    submissions: &mut Vec<WorthUiMountedInteractionNodeReceipt>,
) {
    let style = row.style();
    let button = egui::Button::new(
        RichText::new(row.interaction().label()).color(to_egui_color(style.text_color())),
    )
    .fill(to_egui_color(style.background_color()))
    .stroke(egui::Stroke::new(
        style.border_width_points(),
        to_egui_color(style.border_color()),
    ))
    .corner_radius(egui::CornerRadius::same(style.radius_points() as u8))
    .min_size(interaction_min_size(style));
    let response = ui.add_enabled(row.is_enabled(), button);
    if response.clicked() {
        submissions.push(row.clone());
    }
}

fn interaction_min_size(style: &WorthUiMountedInteractionStyleReceipt) -> egui::Vec2 {
    egui::vec2(
        style.padding_left_points() + style.padding_right_points() + 40.0,
        style.padding_top_points() + style.padding_bottom_points() + 18.0,
    )
}
