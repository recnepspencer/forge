pub(super) const BLUE: egui::Color32 = egui::Color32::from_rgb(47, 129, 247);
pub(super) const GREEN: egui::Color32 = egui::Color32::from_rgb(63, 185, 80);
const TARGET: egui::Color32 = egui::Color32::from_rgb(242, 204, 96);

pub(super) fn assert_background_and_target(
    shapes: &[egui::epaint::ClippedShape],
    background_color: egui::Color32,
) {
    assert_eq!(shapes.len(), 2);
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0));
    let target = egui::Rect::from_min_size(egui::pos2(48.0, 24.0), egui::vec2(64.0, 48.0));
    let observed = shapes
        .iter()
        .map(|shape| {
            let egui::epaint::Shape::Rect(rect) = &shape.shape else {
                panic!("the platform pulse native effects must be egui rectangles");
            };
            assert_eq!(shape.clip_rect, rect.rect);
            (rect.rect, rect.fill)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![(viewport, background_color), (target, TARGET)]
    );
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
