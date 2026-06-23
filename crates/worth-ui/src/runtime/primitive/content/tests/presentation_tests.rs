use crate::runtime::{WorthUiPrimitiveFlowItemKind, WorthUiRuntimeHost};

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn primitive_draw_plan_uses_content_receipt_order_for_item_frames() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,spacer,text,divider\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.check"),
        (
            "content_spacer_size",
            "validation.density.primitive.content.spacer.default",
        ),
        (
            "content_divider_thickness",
            "validation.density.primitive.content.divider.default",
        ),
        ("flow_kind", "inline"),
        ("flow_gap", "validation.density.primitive.flow.gap.compact"),
        (
            "flow_padding",
            "validation.density.primitive.flow.padding.compact",
        ),
    ]));

    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof receipt resolves");
    let draw_plan = primitive.draw_plan(400.0, 240.0);
    assert_eq!(
        draw_plan
            .item_frames()
            .iter()
            .map(|frame| frame.item_kind())
            .collect::<Vec<_>>(),
        vec![
            WorthUiPrimitiveFlowItemKind::Icon,
            WorthUiPrimitiveFlowItemKind::Spacer,
            WorthUiPrimitiveFlowItemKind::Text,
            WorthUiPrimitiveFlowItemKind::Divider,
        ]
    );
}

#[test]
fn same_content_receipt_survives_different_flow_arrangements() {
    let inline = primitive_for_flow_kind("inline");
    let stack = primitive_for_flow_kind("stack");

    assert_eq!(
        inline.content().receipt_digest(),
        stack.content().receipt_digest()
    );
    assert_ne!(
        inline.draw_plan(400.0, 240.0).item_frames(),
        stack.draw_plan(400.0, 240.0).item_frames()
    );
}

#[test]
fn baseline_alignment_uses_deterministic_content_metrics() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        (
            "content_icon_size",
            "validation.density.primitive.content.icon.large",
        ),
        ("flow_kind", "inline"),
        ("flow_cross_align", "baseline"),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof receipt resolves");
    let draw_plan = primitive.draw_plan(400.0, 240.0);
    let frames = draw_plan.item_frames();
    let icon_baseline = primitive.content().items()[0].baseline_points();
    let text_baseline = primitive.content().items()[1].baseline_points();
    let icon_absolute_baseline = frames[0].frame().y() + icon_baseline;
    let text_absolute_baseline = frames[1].frame().y() + text_baseline;

    assert_eq!(icon_baseline, 16.0);
    assert!((text_baseline - 11.7).abs() < 0.001);
    assert!(
        (icon_absolute_baseline - text_absolute_baseline).abs() < 0.001,
        "baseline alignment should be derived from deterministic content receipt metrics"
    );
}

fn primitive_for_flow_kind(flow_kind: &str) -> crate::runtime::WorthUiPrimitiveProofReceipt {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("flow_kind", flow_kind),
    ]));
    resolve_primitive(runtime)
}

fn resolve_primitive(runtime: WorthUiRuntimeHost) -> crate::runtime::WorthUiPrimitiveProofReceipt {
    runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof receipt resolves")
}
