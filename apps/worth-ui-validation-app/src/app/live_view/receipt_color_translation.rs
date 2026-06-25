use egui::Color32;

pub(super) fn to_egui_color(color: worth_ui::facade::WorthUiPrimitiveColor) -> Color32 {
    if color.is_transparent() {
        Color32::TRANSPARENT
    } else {
        Color32::from_rgb(color.red(), color.green(), color.blue())
    }
}
