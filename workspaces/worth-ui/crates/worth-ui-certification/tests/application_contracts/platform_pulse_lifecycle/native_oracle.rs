pub(super) const BLUE: egui::Color32 = egui::Color32::from_rgb(47, 129, 247);
pub(super) const GREEN: egui::Color32 = egui::Color32::from_rgb(63, 185, 80);

pub(super) fn assert_one_viewport_rect(
    shapes: &[egui::epaint::ClippedShape],
    color: egui::Color32,
) {
    assert_eq!(shapes.len(), 1);
    let shape = &shapes[0];
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0));
    assert_eq!(shape.clip_rect, viewport);
    let egui::epaint::Shape::Rect(rect) = &shape.shape else {
        panic!("the platform pulse native effect must remain one egui rectangle");
    };
    assert_eq!(rect.rect, viewport);
    assert_eq!(rect.fill, color);
}

pub(super) fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(160.0, 96.0),
        )),
        ..Default::default()
    }
}
