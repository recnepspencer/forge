use worth_ui::facade::{
    SurfaceId, WorthUiAuthoredDeltaChangePosture, WorthUiPrimitiveProjectionRebindStatus,
    WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationReloadRequest, ValidationSourcePackage,
};
use worth_ui_validation_app::sample_source::VALIDATION_SAMPLE_SOURCE;
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

pub const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";

pub fn assert_flow_edit_rebinds(
    edit: ValidationAuthoredReloadEdit,
    assert_receipt: impl FnOnce(&worth_ui::facade::WorthUiPrimitiveProofReceipt),
) {
    let projection = activate_flow_edit(edit);

    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_eq!(projection.rebind_plan().rebuilt_facts().len(), 5);
    assert_eq!(projection.rebind_plan().preserved_facts().len(), 8);
    assert_exact_flow_projection_rows(projection.changed_rows());
    assert_receipt(projection.primitive_receipt());
}

pub fn activate_flow_edit(
    edit: ValidationAuthoredReloadEdit,
) -> worth_ui::facade::WorthUiPrimitiveProjectionReceipt {
    activate_flow_edits(&[edit])
}

pub fn activate_flow_edits(
    edits: &[ValidationAuthoredReloadEdit],
) -> worth_ui::facade::WorthUiPrimitiveProjectionReceipt {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_flow_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();
    let source_inputs = stable_flow_inputs();
    let mut source = source_inputs.source().source_text().to_owned();
    for edit in edits {
        source = edit
            .apply_to_source_text(&source)
            .expect("structured flow edit should apply to sample source");
    }
    let prepared_reload = workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(source_inputs.source().module_path(), source),
    );
    let mapping = prepared_reload
        .changed_fact_mapping_receipt()
        .cloned()
        .expect("flow prop edit should produce changed fact mapping");
    workbench
        .activate_reload(prepared_reload)
        .expect("flow source edit should activate");
    workbench
        .runtime()
        .resolve_primitive_projection_for_target(&primitive_target(&workbench), Some(&mapping))
        .expect("primitive projection should resolve after flow reload")
}

pub fn assert_exact_flow_projection_rows(
    rows: &[worth_ui::facade::WorthUiPrimitiveChangedFactEvidenceRow],
) {
    assert_eq!(rows.len(), 2);
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveDrawPlan,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveEventRegion,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
}

pub fn assert_flow_projection_rows_for_mixed_authored_prop_edit(
    rows: &[worth_ui::facade::WorthUiPrimitiveChangedFactEvidenceRow],
) {
    assert_eq!(rows.len(), 3);
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveFlowLayout,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveDrawPlan,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveFlowLayout,
        WorthUiRuntimeFactFamily::PrimitiveEventRegion,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::PrimitiveContent,
        WorthUiRuntimeFactFamily::PrimitiveContent,
        WorthUiAuthoredDeltaChangePosture::Added,
    );
}

pub fn stable_source_text_with_edits(edits: &[(&str, &str)]) -> String {
    let mut source_text = VALIDATION_SAMPLE_SOURCE.to_owned();
    for (key, value) in stable_flow_props().into_iter().chain(edits.iter().copied()) {
        source_text = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source_text)
            .expect("stable source edit should apply");
    }
    for removed_key in ["icon", "content_icon"] {
        if !edits.iter().any(|(key, _)| *key == removed_key) {
            source_text =
                ValidationAuthoredReloadEdit::remove_surface_prop(PRIMITIVE_SURFACE, removed_key)
                    .apply_to_source_text(&source_text)
                    .unwrap_or(source_text);
        }
    }
    source_text
}

fn primitive_surface_id() -> SurfaceId {
    SurfaceId::new(PRIMITIVE_SURFACE).expect("valid primitive surface id")
}

fn primitive_target(
    workbench: &worth_ui_validation_app::ValidationRuntimeWorkbench,
) -> worth_ui::facade::WorthUiPrimitiveProofTargetBinding {
    workbench
        .runtime()
        .bind_authored_primitive_proof_target(&primitive_surface_id())
        .expect("flow primitive projection target binds")
}

fn stable_flow_inputs() -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let source = inputs.source();
    let module_path = source.module_path().to_owned();
    let mut source_text = source.source_text().to_owned();
    for (key, value) in stable_flow_props() {
        source_text = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source_text)
            .expect("stable flow input edit should apply");
    }
    for removed_key in ["icon", "content_icon"] {
        source_text =
            ValidationAuthoredReloadEdit::remove_surface_prop(PRIMITIVE_SURFACE, removed_key)
                .apply_to_source_text(&source_text)
                .unwrap_or(source_text);
    }
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}

fn stable_flow_props() -> [(&'static str, &'static str); 27] {
    [
        ("primitive_text", "\"Worth primitive\""),
        ("primitive_align", "center"),
        ("primitive_padding", "validation.density.primitive.padding"),
        ("primitive_radius", "validation.density.primitive.radius"),
        ("primitive_background", "\"#2f7de1\""),
        ("primitive_foreground", "\"#f7f1e8\""),
        ("primitive_interaction", "submit"),
        ("primitive_cursor", "pointer"),
        ("primitive_focus", "focusable"),
        (
            "primitive_interaction_id",
            "worth.interaction.primitive.submit",
        ),
        ("primitive_submit_payload", "\"submit.primary\""),
        ("primitive_motion", "transition"),
        ("primitive_motion_target", "primitive_background"),
        (
            "primitive_motion_duration",
            "validation.density.primitive.motion.fast",
        ),
        ("primitive_motion_easing", "standard"),
        ("flow_kind", "inline"),
        ("flow_gap", "validation.density.primitive.flow.gap.default"),
        (
            "flow_padding",
            "validation.density.primitive.flow.padding.default",
        ),
        ("flow_align", "center"),
        ("flow_cross_align", "center"),
        ("flow_fit", "hug"),
        ("flow_fill", "none"),
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Worth primitive\""),
        (
            "content_text_size",
            "validation.density.primitive.content.text.default",
        ),
        (
            "content_icon_size",
            "validation.density.primitive.content.icon.large",
        ),
    ]
}

pub fn assert_projection_row(
    rows: &[worth_ui::facade::WorthUiPrimitiveChangedFactEvidenceRow],
    expected_slice: WorthUiSemanticSliceId,
    expected_family: WorthUiRuntimeFactFamily,
    expected_posture: WorthUiAuthoredDeltaChangePosture,
) {
    let row = rows
        .iter()
        .find(|row| row.semantic_slice() == expected_slice)
        .expect("expected flow projection row");
    assert_eq!(row.subject_surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(row.change_posture(), expected_posture);
    assert!(
        row.changed_facts()
            .iter()
            .any(|fact| fact.family() == expected_family && fact.identity() == PRIMITIVE_SURFACE),
        "expected row to contain {expected_family:?} for {PRIMITIVE_SURFACE}"
    );
}
