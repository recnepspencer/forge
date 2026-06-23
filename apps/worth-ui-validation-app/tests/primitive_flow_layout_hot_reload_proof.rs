mod primitive_flow_layout_support;
mod validation_app_reload_fixture;

use primitive_flow_layout_support::{
    activate_flow_edit, activate_flow_edits, assert_flow_edit_rebinds,
    assert_flow_projection_rows_for_mixed_authored_prop_edit, assert_projection_row,
    stable_source_text_with_edits, PRIMITIVE_SURFACE,
};
use worth_ui::facade::{
    WorthUiAuthoredDeltaChangePosture, WorthUiFlowLayoutCrossAlign, WorthUiFlowLayoutKind,
    WorthUiPrimitiveContentItem, WorthUiPrimitiveFlowItemKind,
    WorthUiPrimitiveProjectionRebindStatus, WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

#[test]
fn flow_layout_edits_rebind_projection_with_exact_rows_and_receipt_changes() {
    assert_flow_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "flow_gap",
            "validation.density.primitive.flow.gap.compact",
        ),
        |receipt| {
            assert_eq!(
                receipt.flow_layout().gap_token(),
                "validation.density.primitive.flow.gap.compact"
            );
            assert_eq!(receipt.flow_layout().gap_points(), 6.0);
        },
    );
    assert_flow_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "flow_padding",
            "validation.density.primitive.flow.padding.fat",
        ),
        |receipt| {
            assert_eq!(
                receipt.flow_layout().padding_token(),
                "validation.density.primitive.flow.padding.fat"
            );
            assert_eq!(receipt.flow_layout().padding_points(), 48.0);
        },
    );
    assert_flow_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_align", "end"),
        |receipt| {
            assert_eq!(
                receipt.dependency_facts().collect::<Vec<_>>().len(),
                10,
                "flow layout participates in primitive dependency contract"
            );
        },
    );
    assert_flow_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "flow_cross_align",
            "baseline",
        ),
        |receipt| {
            assert_eq!(
                receipt.flow_layout().cross_align(),
                WorthUiFlowLayoutCrossAlign::Baseline
            );
        },
    );
}

#[test]
fn switching_inline_to_stack_changes_flow_receipt_and_draw_plan() {
    let projection = activate_flow_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon",
            "worth.icon.action.plus",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_kind", "stack"),
    ]);
    let receipt = projection.primitive_receipt();
    let draw_plan = receipt.draw_plan(1000.0, 600.0);

    assert_eq!(receipt.flow_layout().kind(), WorthUiFlowLayoutKind::Stack);
    assert_eq!(draw_plan.counters().layout_item_count(), 2);
    assert_eq!(draw_plan.item_frames().len(), 2);
    assert_eq!(
        draw_plan.item_frames()[0].item_kind(),
        WorthUiPrimitiveFlowItemKind::Icon
    );
    assert_eq!(
        draw_plan.item_frames()[1].item_kind(),
        WorthUiPrimitiveFlowItemKind::Text
    );
    assert!(draw_plan.item_frames()[0].frame().y() < draw_plan.item_frames()[1].frame().y());
    assert_flow_projection_rows_for_mixed_authored_prop_edit(projection.changed_rows());
}

#[test]
fn flow_draw_plan_comes_from_flow_receipt_not_renderer_constants() {
    let projection = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_align",
        "end",
    ));
    let draw_plan = projection.primitive_receipt().draw_plan(1000.0, 600.0);
    let counters = draw_plan.counters();

    assert_frame_matches_text_content_and_padding(projection.primitive_receipt(), &draw_plan);
    assert_eq!(draw_plan.frame().x() + draw_plan.frame().width(), 1000.0);
    assert_eq!(counters.content_item_count(), 1);
    assert_eq!(counters.layout_item_count(), 1);
    assert_eq!(draw_plan.item_frames().len(), 1);
    assert_eq!(
        draw_plan.item_frames()[0].item_kind(),
        WorthUiPrimitiveFlowItemKind::Text
    );
    assert!(draw_plan.item_frames()[0].frame().x() > 0.0);
    assert!(draw_plan.item_frames()[0].frame().y() > 0.0);
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.artifact_scan_count(), 0);
}

#[test]
fn flow_padding_expands_frame_and_offsets_item_frames() {
    let compact = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_padding",
        "validation.density.primitive.flow.padding.compact",
    ));
    let fat = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_padding",
        "validation.density.primitive.flow.padding.fat",
    ));

    let compact_plan = compact.primitive_receipt().draw_plan(1000.0, 600.0);
    let fat_plan = fat.primitive_receipt().draw_plan(1000.0, 600.0);

    assert_eq!(
        compact.primitive_receipt().flow_layout().padding_points(),
        16.0
    );
    assert_eq!(fat.primitive_receipt().flow_layout().padding_points(), 48.0);
    assert_eq!(
        fat_plan.frame().width() - compact_plan.frame().width(),
        64.0
    );
    assert_eq!(
        fat_plan.frame().height() - compact_plan.frame().height(),
        64.0
    );
    assert_eq!(
        fat_plan.item_frames()[0].frame().x() - compact_plan.item_frames()[0].frame().x(),
        32.0
    );
}

#[test]
fn flow_padding_preserves_independent_edge_values() {
    let projection = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_padding",
        "validation.density.primitive.flow.padding.wide_shallow",
    ));
    let receipt = projection.primitive_receipt();
    let padding = receipt.flow_layout().padding_edges();
    let draw_plan = receipt.draw_plan(1000.0, 600.0);

    assert_eq!(padding.top(), 8.0);
    assert_eq!(padding.right(), 64.0);
    assert_eq!(padding.bottom(), 8.0);
    assert_eq!(padding.left(), 64.0);
    let text = receipt.content().items()[0].width_points();
    assert!((draw_plan.frame().width() - (text.clamp(120.0, 360.0) + 128.0)).abs() < 0.01);
    assert_eq!(draw_plan.frame().height(), 80.0);
    assert!(draw_plan.item_frames()[0].frame().x() >= padding.left());
    assert!(draw_plan.item_frames()[0].frame().y() >= padding.top());
}

#[test]
fn baseline_cross_alignment_uses_receipt_metrics() {
    let projection = activate_flow_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon",
            "worth.icon.action.plus",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "flow_cross_align",
            "baseline",
        ),
    ]);
    let draw_plan = projection.primitive_receipt().draw_plan(1000.0, 600.0);
    let icon = draw_plan.item_frames()[0].frame();
    let text = draw_plan.item_frames()[1].frame();

    assert_eq!(
        projection.primitive_receipt().flow_layout().cross_align(),
        WorthUiFlowLayoutCrossAlign::Baseline
    );
    let icon_baseline = projection.primitive_receipt().content().items()[0].baseline_points();
    let text_baseline = projection.primitive_receipt().content().items()[1].baseline_points();
    assert!(
        (icon.y() + icon_baseline - (text.y() + text_baseline)).abs() < 0.01,
        "baseline uses deterministic icon/text metrics from resolved content receipts"
    );
}

#[test]
fn row_column_grid_spacer_fit_and_fill_are_receipt_planned() {
    let row = activate_flow_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_kind", "row"),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon",
            "worth.icon.action.plus",
        ),
    ]);
    let row_plan = row.primitive_receipt().draw_plan(1000.0, 600.0);
    assert_eq!(row_plan.item_frames().len(), 2);
    assert!(row_plan.item_frames()[0].frame().x() < row_plan.item_frames()[1].frame().x());

    let column = activate_flow_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_kind", "column"),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_icon",
            "worth.icon.action.plus",
        ),
    ]);
    let column_plan = column.primitive_receipt().draw_plan(1000.0, 600.0);
    assert!(column_plan.item_frames()[0].frame().y() < column_plan.item_frames()[1].frame().y());

    let grid = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_kind",
        "grid",
    ));
    assert_eq!(
        grid.primitive_receipt()
            .draw_plan(1000.0, 600.0)
            .frame()
            .height(),
        160.0
    );

    let spacer = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_kind",
        "spacer",
    ));
    let spacer_plan = spacer.primitive_receipt().draw_plan(1000.0, 600.0);
    assert_eq!(spacer_plan.item_frames().len(), 0);
    assert_eq!(spacer_plan.frame().width(), 8.0);

    let fill = activate_flow_edits(&[
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_fit", "fill"),
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "flow_fill", "both"),
    ]);
    let fill_plan = fill.primitive_receipt().draw_plan(1000.0, 600.0);
    assert_eq!(fill_plan.frame().width(), 1000.0);
    assert_eq!(fill_plan.frame().height(), 600.0);
}

#[test]
fn icon_text_anatomy_hot_reloads_through_authored_surface_props() {
    let projection = activate_flow_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "content_icon",
        "worth.icon.action.plus",
    ));
    let receipt = projection.primitive_receipt();
    let draw_plan = receipt.draw_plan(1000.0, 600.0);

    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_projection_row(
        projection.changed_rows(),
        WorthUiSemanticSliceId::PrimitiveContent,
        WorthUiRuntimeFactFamily::PrimitiveContent,
        WorthUiAuthoredDeltaChangePosture::Added,
    );
    assert!(receipt.content().items().iter().any(|item| {
        matches!(
            item,
            WorthUiPrimitiveContentItem::Icon(icon) if icon.icon_id() == "worth.icon.action.plus"
        )
    }));
    assert_eq!(draw_plan.counters().content_item_count(), 2);
    assert_eq!(draw_plan.counters().layout_item_count(), 2);
    assert_eq!(draw_plan.item_frames().len(), 2);
    assert_eq!(
        draw_plan.item_frames()[0].item_kind(),
        WorthUiPrimitiveFlowItemKind::Icon
    );
    assert_eq!(
        draw_plan.item_frames()[1].item_kind(),
        WorthUiPrimitiveFlowItemKind::Text
    );
}

#[test]
fn observed_source_reload_updates_centered_primitive_flow_receipt() {
    let fixture = validation_app_reload_fixture::ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    fixture.write_source(&stable_source_text_with_edits(&[
        ("content_icon", "worth.icon.action.plus"),
        ("flow_kind", "stack"),
        ("flow_gap", "validation.density.primitive.flow.gap.compact"),
    ]));
    app.run_one_reload_observation_cycle();

    let receipt = app
        .centered_primitive_proof()
        .expect("observed source reload should update primitive proof");
    let draw_plan = receipt.draw_plan(1000.0, 600.0);

    assert_eq!(receipt.flow_layout().kind(), WorthUiFlowLayoutKind::Stack);
    assert_eq!(receipt.flow_layout().gap_points(), 6.0);
    assert_eq!(draw_plan.item_frames().len(), 2);
    assert!(draw_plan.item_frames()[0].frame().y() < draw_plan.item_frames()[1].frame().y());
}

fn assert_frame_matches_text_content_and_padding(
    receipt: &worth_ui::facade::WorthUiPrimitiveProofReceipt,
    draw_plan: &worth_ui::facade::WorthUiPrimitiveDrawPlan,
) {
    let item = &receipt.content().items()[0];
    let padding = receipt.flow_layout().padding_edges();
    let expected_width = item.width_points().clamp(120.0, 360.0) + padding.horizontal();
    let expected_height = item.height_points().max(64.0) + padding.vertical();
    assert!((draw_plan.frame().width() - expected_width).abs() < 0.01);
    assert!((draw_plan.frame().height() - expected_height).abs() < 0.01);
}
