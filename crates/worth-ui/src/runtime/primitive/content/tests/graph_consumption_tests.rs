use crate::runtime::{
    WorthUiPrimitiveContentAccessibilityParticipation, WorthUiPrimitiveContentGraphPosture,
    WorthUiPrimitiveContentIconRenderPosture, WorthUiPrimitiveContentItemKind,
    WorthUiPrimitiveContentParticipationPosture, WorthUiQueryGraphObligationSemantic,
    WorthUiRuntimeFactId,
};

use super::support::{content_source, runtime_for_source, surface_id};

#[test]
fn content_anatomy_receipt_is_shared_graph_consumption_surface() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("content_accessibility_name", "\"Submit form\""),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");
    let anatomy = primitive.content().anatomy_receipt();

    assert_eq!(anatomy.item_count(), 2);
    assert_eq!(
        anatomy.accessibility(),
        WorthUiPrimitiveContentAccessibilityParticipation::Named
    );
    assert_eq!(
        anatomy.items()[0].item_kind(),
        WorthUiPrimitiveContentItemKind::Icon
    );
    assert_eq!(
        anatomy.items()[0].participation(),
        WorthUiPrimitiveContentParticipationPosture::Present
    );
    assert_eq!(
        anatomy.items()[0].accessibility(),
        WorthUiPrimitiveContentAccessibilityParticipation::Hidden
    );
    assert_eq!(
        anatomy.items()[0].native_vector(),
        Some(WorthUiPrimitiveContentIconRenderPosture::NativeVector)
    );
    assert!(anatomy.items()[1].baseline_points() > 0.0);
}

#[test]
fn content_receipt_carries_query_execution_for_content_consumers() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Fallback\""),
        ("content_icon", "worth.icon.action.fallback"),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");
    let execution = primitive.content().query_graph_execution_receipt();
    let semantics = execution
        .rows()
        .iter()
        .map(|row| row.semantic())
        .collect::<Vec<_>>();

    assert_eq!(execution.selected_obligation_count(), 6);
    assert_eq!(
        execution.touch_descriptor().surface_id(),
        WorthUiRuntimeFactId::primitive_content(surface_id().as_str()).identity()
    );
    for expected in WorthUiQueryGraphObligationSemantic::PRIMITIVE_CONTENT_ANATOMY {
        assert!(
            semantics.contains(&expected),
            "missing content graph semantic {expected:?}"
        );
    }
    assert_eq!(
        support_status_for(
            &execution,
            WorthUiQueryGraphObligationSemantic::ContentVectorPosture
        ),
        "diagnostic-only",
        "fallback posture must be graph-visible, not renderer folklore"
    );
}

#[test]
fn draw_plan_content_path_uses_proved_content_anatomy() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");
    let proved_content = primitive.content().proved_anatomy();
    let draw_plan = primitive.draw_plan(400.0, 240.0);

    assert_eq!(draw_plan.counters().content_item_count(), 2);
    assert_eq!(
        draw_plan.item_frames().len(),
        proved_content.anatomy().item_count()
    );
    assert_eq!(
        proved_content
            .query_graph_execution()
            .selected_obligation_count(),
        6
    );
    assert_eq!(
        proved_content
            .query_graph_execution()
            .touch_descriptor()
            .surface_id(),
        WorthUiRuntimeFactId::primitive_content(surface_id().as_str()).identity()
    );
}

#[test]
fn denied_content_posture_marks_schema_obligation_unsupported() {
    let touch = crate::runtime::WorthUiQueryGraphTouchDescriptor::primitive_content_anatomy(
        "worth.surface.denied.content",
        [WorthUiRuntimeFactId::primitive_content(
            "worth.surface.denied.content",
        )],
        WorthUiPrimitiveContentGraphPosture::Denied,
    );
    let execution = crate::runtime::WorthUiQueryGraphExecutionReceipt::primitive_content_anatomy(
        touch,
        crate::runtime::WorthUiQueryGraphOperatingWorld::runtime_preview(),
    );

    assert_eq!(
        support_status_for(
            &execution,
            WorthUiQueryGraphObligationSemantic::ContentSchemaAdmission
        ),
        "unsupported"
    );
}

#[test]
fn authored_absent_presence_removes_layout_items_without_renderer_branching() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("content_presence", "absent"),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");

    assert_eq!(
        primitive.content().participation(),
        WorthUiPrimitiveContentParticipationPosture::Absent
    );
    assert!(primitive.content().items().is_empty());
    assert_eq!(primitive.content().anatomy_receipt().item_count(), 0);
    assert!(primitive.draw_plan(400.0, 240.0).item_frames().is_empty());
}

#[test]
fn hidden_from_accessibility_presence_is_not_collapsed_into_absent_or_paint_hidden() {
    let runtime = runtime_for_source(content_source(&[
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Submit\""),
        ("content_icon", "worth.icon.action.plus"),
        ("content_accessibility_name", "\"Submit form\""),
        ("content_presence", "hidden_from_accessibility"),
    ]));
    let primitive = runtime
        .resolve_primitive_proof(&surface_id())
        .expect("primitive proof resolves");
    let anatomy = primitive.content().anatomy_receipt();

    assert_eq!(
        primitive.content().participation(),
        WorthUiPrimitiveContentParticipationPosture::HiddenFromAccessibility
    );
    assert_eq!(primitive.content().items().len(), 2);
    assert_eq!(
        anatomy.accessibility(),
        WorthUiPrimitiveContentAccessibilityParticipation::Hidden
    );
    assert_eq!(
        anatomy.items()[1].participation(),
        WorthUiPrimitiveContentParticipationPosture::HiddenFromAccessibility
    );
}

fn support_status_for(
    receipt: &crate::runtime::WorthUiQueryGraphExecutionReceipt,
    semantic: WorthUiQueryGraphObligationSemantic,
) -> &str {
    receipt
        .rows()
        .iter()
        .find(|row| row.semantic() == semantic)
        .expect("content graph row exists")
        .support_status()
}
