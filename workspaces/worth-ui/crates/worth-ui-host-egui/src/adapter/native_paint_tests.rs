use super::{UiEguiPreparedFilledRect, UiEguiPreparedNativePaintCommand};

#[test]
fn one_surface_paints_sorted_semantic_rows_back_to_front() {
    let context = egui::Context::default();
    let viewport = egui::Rect::from_min_size(egui::Pos2::ZERO, egui::vec2(160.0, 96.0));
    let target = egui::Rect::from_min_size(egui::pos2(48.0, 24.0), egui::vec2(64.0, 48.0));
    let commands = [
        UiEguiPreparedNativePaintCommand::FilledRect(filled_rect(
            viewport,
            egui::Color32::from_rgb(47, 129, 247),
        )),
        UiEguiPreparedNativePaintCommand::FilledRect(filled_rect(
            target,
            egui::Color32::from_rgb(242, 204, 96),
        )),
    ];
    let layer = egui::LayerId::new(egui::Order::Middle, egui::Id::new("ordered-surface"));
    let output = context.run_ui(raw_input(), |_| {
        let painter = context.layer_painter(layer);
        for command in &commands {
            command.paint(&painter);
        }
    });
    let observed = output
        .shapes
        .iter()
        .map(|shape| {
            let egui::epaint::Shape::Rect(rect) = &shape.shape else {
                panic!("native paint should emit rectangles");
            };
            (rect.rect, rect.fill)
        })
        .collect::<Vec<_>>();
    assert_eq!(
        observed,
        vec![
            (viewport, egui::Color32::from_rgb(47, 129, 247)),
            (target, egui::Color32::from_rgb(242, 204, 96)),
        ]
    );
}

fn filled_rect(rect: egui::Rect, color: egui::Color32) -> UiEguiPreparedFilledRect {
    UiEguiPreparedFilledRect {
        rect,
        clip_rect: rect,
        color,
    }
}

fn raw_input() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(160.0, 96.0),
        )),
        ..Default::default()
    }
}
