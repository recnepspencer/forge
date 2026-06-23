use crate::runtime::WorthUiPrimitiveFlowItemKind;

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn stacked_layout_uses_content_metrics_without_inline_baseline_offsets() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "stack"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        (
            "content_icon_size",
            "validation.density.primitive.content.icon.large",
        ),
        ("flow_kind", "stack"),
        ("flow_cross_align", "baseline"),
    ]));

    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("stacked primitive resolves");
    let draw_plan = primitive.draw_plan(400.0, 240.0);
    let frames = draw_plan.item_frames();

    assert_eq!(frames[0].item_kind(), WorthUiPrimitiveFlowItemKind::Icon);
    assert_eq!(frames[1].item_kind(), WorthUiPrimitiveFlowItemKind::Text);
    assert_eq!(frames[0].frame().x(), frames[1].frame().x());
    assert!(frames[0].frame().y() < frames[1].frame().y());
    assert_eq!(primitive.content().items()[0].baseline_points(), 16.0);
    assert!((primitive.content().items()[1].baseline_points() - 11.7).abs() < 0.001);
}
