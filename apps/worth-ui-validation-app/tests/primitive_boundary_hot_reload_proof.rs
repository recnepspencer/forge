mod primitive_boundary_support;

use primitive_boundary_support::{
    activate_primitive_edit, assert_exact_projection_rows, assert_primitive_denial,
    assert_primitive_edit_rebinds, assert_receipt_consumes, primitive_surface_id,
    primitive_value_denial_for_edit, stable_primitive_inputs, PRIMITIVE_SURFACE,
};
use worth_ui::facade::{
    WorthUiAuthoredDeltaChangePosture, WorthUiComponentInteractionKind, WorthUiPrimitiveAlign,
    WorthUiPrimitiveFlowItemKind, WorthUiPrimitiveProjectionRebindStatus,
    WorthUiPrimitiveValueDenialCode, WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;
use worth_ui_validation_app::ValidationWorkbenchLaunch;

#[test]
fn authored_primitive_surface_resolves_through_projection_boundary() {
    let prepared = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_primitive_inputs())
        .expect("validation workbench should prepare");
    let surface_id = primitive_surface_id();

    let projection = prepared
        .runtime()
        .resolve_primitive_projection_for_target(
            &prepared
                .runtime()
                .bind_authored_primitive_proof_target(&surface_id)
                .expect("primitive projection target binds"),
            None,
        )
        .expect("primitive projection should resolve from authored source");
    let receipt = projection.primitive_receipt();

    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Unchanged
    );
    assert_eq!(projection.changed_rows(), &[]);
    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(receipt.component_id(), "worth.component.primitive_proof");
    assert_eq!(receipt.container().align(), WorthUiPrimitiveAlign::Center);
    assert_eq!(receipt.container().padding_points(), 32.0);
    assert_eq!(receipt.container().radius_points(), 8.0);
    assert_eq!(
        receipt.measurement().padding().token(),
        "validation.density.primitive.padding"
    );
    assert_eq!(
        receipt.interaction().interaction_id(),
        "worth.interaction.primitive.submit"
    );
    assert_eq!(
        receipt.motion().duration().token(),
        "validation.density.primitive.motion.fast"
    );
    assert_eq!(receipt.content().text(), "Worth primitive");
    assert_eq!(
        receipt.appearance().background_color().hex_triplet(),
        "#2f7de1"
    );
    assert_eq!(
        receipt.appearance().foreground_color().hex_triplet(),
        "#f7f1e8"
    );
    assert_receipt_consumes(
        &receipt.dependency_facts().cloned().collect::<Vec<_>>(),
        &[
            WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
            WorthUiRuntimeFactFamily::PrimitiveAppearance,
            WorthUiRuntimeFactFamily::PrimitiveAppearanceState,
            WorthUiRuntimeFactFamily::PrimitiveContainer,
            WorthUiRuntimeFactFamily::PrimitiveContent,
            WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
            WorthUiRuntimeFactFamily::PrimitiveInteraction,
            WorthUiRuntimeFactFamily::PrimitiveMeasurement,
            WorthUiRuntimeFactFamily::PrimitiveMotion,
        ],
    );
}

#[test]
fn primitive_prop_edits_rebind_projection_with_exact_rows_and_receipt_changes() {
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "content_text",
            "\"Hot primitive\"",
        ),
        WorthUiSemanticSliceId::PrimitiveContent,
        |receipt| assert_eq!(receipt.content().text(), "Hot primitive"),
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_padding",
            "validation.density.primitive.padding.fat",
        ),
        WorthUiSemanticSliceId::PrimitiveMeasurement,
        |receipt| assert_eq!(receipt.container().padding_points(), 56.0),
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_padding",
            "validation.density.primitive.padding.wide_shallow",
        ),
        WorthUiSemanticSliceId::PrimitiveMeasurement,
        |receipt| {
            let padding = receipt.container().padding_edges();
            assert_eq!(padding.top(), 8.0);
            assert_eq!(padding.right(), 64.0);
            assert_eq!(padding.bottom(), 8.0);
            assert_eq!(padding.left(), 64.0);
            assert_eq!(receipt.container().padding_points(), 64.0);
        },
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_payload",
            "\"submit.secondary\"",
        ),
        WorthUiSemanticSliceId::PrimitiveInteraction,
        |receipt| {
            assert!(receipt
                .interaction()
                .submit_payload()
                .fields()
                .iter()
                .any(|field| field.value().as_text() == "submit.secondary"));
        },
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_motion_duration",
            "validation.density.primitive.radius",
        ),
        WorthUiSemanticSliceId::PrimitiveMotion,
        |receipt| assert_eq!(receipt.motion().duration().points(), 8.0),
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, "primitive_align", "end"),
        WorthUiSemanticSliceId::PrimitiveContainer,
        |receipt| assert_eq!(receipt.container().align(), WorthUiPrimitiveAlign::End),
    );
    assert_primitive_edit_rebinds(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_background",
            "\"#193c9f\"",
        ),
        WorthUiSemanticSliceId::PrimitiveAppearance,
        |receipt| {
            assert_eq!(
                receipt.appearance().background_color().hex_triplet(),
                "#193c9f"
            );
        },
    );
}

#[test]
fn primitive_submit_uses_generic_runtime_interaction_lane() {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_primitive_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();

    let receipt = workbench
        .submit_component_interaction(
            &primitive_surface_id(),
            WorthUiComponentInteractionKind::Submit,
        )
        .expect("primitive submit should emit sealed interaction receipt");

    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(receipt.component_id(), "worth.component.primitive_proof");
    assert_eq!(
        receipt.interaction_id(),
        "worth.interaction.primitive.submit"
    );
    assert_eq!(
        receipt.payload().kind(),
        WorthUiComponentInteractionKind::Submit
    );
    assert!(receipt
        .payload()
        .fields()
        .iter()
        .any(|field| field.name() == "payload" && field.value().as_text() == "submit.primary"));
}

#[test]
fn removing_primitive_prop_rebinds_projection_to_defaulted_receipt() {
    let projection = activate_primitive_edit(ValidationAuthoredReloadEdit::remove_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_foreground",
    ));

    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_exact_projection_rows(
        projection.changed_rows(),
        WorthUiSemanticSliceId::PrimitiveAppearance,
        WorthUiAuthoredDeltaChangePosture::Removed,
    );
    assert_eq!(
        projection
            .primitive_receipt()
            .appearance()
            .foreground_color()
            .hex_triplet(),
        "#f7f1e8"
    );
}

#[test]
fn malformed_primitive_values_are_rejected_before_rendering() {
    assert_primitive_denial(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_background",
            "\"blue\"",
        ),
        "primitive_background",
        "blue",
        WorthUiPrimitiveValueDenialCode::InvalidColorHex,
        WorthUiSemanticSliceId::PrimitiveAppearance,
        WorthUiRuntimeFactFamily::PrimitiveAppearance,
    );
    assert_primitive_denial(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_align",
            "wide",
        ),
        "primitive_align",
        "wide",
        WorthUiPrimitiveValueDenialCode::InvalidAlignKeyword,
        WorthUiSemanticSliceId::PrimitiveContainer,
        WorthUiRuntimeFactFamily::PrimitiveContainer,
    );
    assert_primitive_denial(
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_padding",
            "fat",
        ),
        "primitive_padding",
        "fat",
        WorthUiPrimitiveValueDenialCode::InvalidMeasurementToken,
        WorthUiSemanticSliceId::PrimitiveMeasurement,
        WorthUiRuntimeFactFamily::PrimitiveMeasurement,
    );
}

#[test]
fn primitive_value_denial_digest_tracks_structured_basis() {
    let first = primitive_value_denial_for_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_background",
        "\"blue\"",
    ));
    let second = primitive_value_denial_for_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_background",
        "\"purple-ish\"",
    ));

    assert_eq!(first.prop_key(), second.prop_key());
    assert_eq!(first.schema_id(), second.schema_id());
    assert_ne!(first.raw_value(), second.raw_value());
    assert_ne!(first.denial_digest(), second.denial_digest());
}

#[test]
fn primitive_draw_plan_comes_from_receipt_not_renderer_constants() {
    let projection = activate_primitive_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "flow_align",
        "end",
    ));
    let draw_plan = projection.primitive_receipt().draw_plan(1000.0, 600.0);
    let counters = draw_plan.counters();

    assert!((draw_plan.frame().width() - 190.0).abs() < 0.01);
    assert_eq!(draw_plan.frame().height(), 128.0);
    assert!((draw_plan.frame().x() - (1000.0 - 190.0)).abs() < 0.01);
    assert_eq!(draw_plan.frame().y(), 236.0);
    assert_eq!(counters.content_item_count(), 1);
    assert_eq!(counters.layout_item_count(), 1);
    assert_eq!(draw_plan.item_frames().len(), 1);
    assert_eq!(
        draw_plan.item_frames()[0].item_kind(),
        WorthUiPrimitiveFlowItemKind::Text
    );
    assert!((draw_plan.item_frames()[0].frame().x() - 32.0).abs() < 0.01);
    assert!((draw_plan.item_frames()[0].frame().y() - 56.5).abs() < 0.01);
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.artifact_scan_count(), 0);
    assert_eq!(draw_plan.receipt().content().text(), "Worth primitive");
}
