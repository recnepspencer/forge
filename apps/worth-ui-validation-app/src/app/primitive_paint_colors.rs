use egui::Color32;

pub(crate) fn color_from_primitive_with_opacity(
    color: worth_ui::facade::WorthUiPrimitiveColor,
    opacity: f32,
) -> Color32 {
    if color.is_transparent() {
        return Color32::TRANSPARENT;
    }
    Color32::from_rgba_premultiplied(
        color.red(),
        color.green(),
        color.blue(),
        opacity_to_alpha(opacity),
    )
}

fn opacity_to_alpha(opacity: f32) -> u8 {
    (opacity.clamp(0.0, 1.0) * 255.0).round() as u8
}
