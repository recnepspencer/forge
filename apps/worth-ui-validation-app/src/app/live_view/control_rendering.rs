use egui::{Color32, ComboBox, RichText, TextEdit};
use worth_ui::facade::{
    WorthUiLiveViewControlEditabilityPosture, WorthUiLiveViewControlHostFrameKind,
    WorthUiLiveViewControlHostFrameReceipt, WorthUiLiveViewControlHostFrameStyleReceipt,
    WorthUiLiveViewStateEditIntent, WorthUiLiveViewStateValue, WorthUiMountedControlNodeReceipt,
};

use super::receipt_color_translation::to_egui_color;

pub(super) fn render_live_view_control(
    ui: &mut egui::Ui,
    control: &WorthUiMountedControlNodeReceipt,
    intents: &mut Vec<WorthUiLiveViewStateEditIntent>,
) {
    let binding = control.state_binding();
    let frame = control.host_frame();
    ui.push_id(control.composition_child_binding().binding_digest(), |ui| {
        ui.vertical(|ui| {
            ui.label(
                RichText::new(frame.label()).color(to_egui_color(frame.style().foreground_color())),
            );
            match frame.kind() {
                WorthUiLiveViewControlHostFrameKind::TextInput => {
                    render_live_view_text_input(ui, frame, binding, intents);
                }
                WorthUiLiveViewControlHostFrameKind::DropdownInput => {
                    render_live_view_select_control(ui, frame, binding, intents);
                }
            }
        });
    });
}

fn render_live_view_text_input(
    ui: &mut egui::Ui,
    frame: &WorthUiLiveViewControlHostFrameReceipt,
    binding: &worth_ui::facade::WorthUiLiveViewStateBindingReceipt,
    intents: &mut Vec<WorthUiLiveViewStateEditIntent>,
) {
    let mut value = frame.value_text().to_owned();
    let before = value.clone();
    render_control_frame(ui, frame.style(), |ui| {
        ui.visuals_mut().override_text_color = Some(Color32::from_rgb(17, 24, 39));
        let edit = TextEdit::singleline(&mut value)
            .frame(false)
            .interactive(frame.editability().is_editable());
        ui.add(edit);
    });
    if value != before {
        intents.push(binding.edit(WorthUiLiveViewStateValue::text(value)));
    }
}

fn render_live_view_select_control(
    ui: &mut egui::Ui,
    frame: &WorthUiLiveViewControlHostFrameReceipt,
    binding: &worth_ui::facade::WorthUiLiveViewStateBindingReceipt,
    intents: &mut Vec<WorthUiLiveViewStateEditIntent>,
) {
    let mut value = frame.value_text().to_owned();
    let before = value.clone();
    render_control_frame(ui, frame.style(), |ui| {
        let combo = ComboBox::from_id_salt(frame.control_id());
        let selected_text = if value.is_empty() {
            "-"
        } else {
            value.as_str()
        };
        combo.selected_text(selected_text).show_ui(ui, |ui| {
            for option in frame.options() {
                ui.selectable_value(&mut value, option.value().to_owned(), option.label());
            }
        });
    });
    if frame.editability() != WorthUiLiveViewControlEditabilityPosture::Editable {
        return;
    }
    if value != before {
        intents.push(binding.edit(WorthUiLiveViewStateValue::text(value)));
    }
}

fn render_control_frame(
    ui: &mut egui::Ui,
    style: &WorthUiLiveViewControlHostFrameStyleReceipt,
    add_contents: impl FnOnce(&mut egui::Ui),
) {
    egui::Frame::new()
        .fill(to_egui_color(style.background_color()))
        .stroke(egui::Stroke::new(
            style.border_width_points(),
            to_egui_color(style.border_color()),
        ))
        .corner_radius(egui::CornerRadius::same(style.radius_points() as u8))
        .inner_margin(egui::Margin {
            left: style.padding_left_points() as i8,
            right: style.padding_right_points() as i8,
            top: style.padding_top_points() as i8,
            bottom: style.padding_bottom_points() as i8,
        })
        .show(ui, |ui| {
            add_contents(ui);
        });
}
