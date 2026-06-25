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

pub fn activate_content_edits(
    edits: &[ValidationAuthoredReloadEdit],
) -> worth_ui::facade::WorthUiPrimitiveProjectionReceipt {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_content_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();
    let source_inputs = stable_content_inputs();
    let mut source = source_inputs.source().source_text().to_owned();
    for edit in edits {
        source = edit
            .apply_to_source_text(&source)
            .expect("structured content edit should apply to sample source");
    }
    let prepared_reload = workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(source_inputs.source().module_path(), source),
    );
    let mapping = prepared_reload
        .changed_fact_mapping_receipt()
        .cloned()
        .expect("content prop edit should produce changed fact mapping");
    workbench
        .activate_reload(prepared_reload)
        .expect("content source edit should activate");
    workbench
        .runtime()
        .resolve_primitive_projection_for_target(&primitive_target(&workbench), Some(&mapping))
        .expect("primitive projection should resolve after content reload")
}

pub fn stable_source_text_with_content_edits(edits: &[(&str, &str)]) -> String {
    let mut source_text = VALIDATION_SAMPLE_SOURCE.to_owned();
    for (key, value) in stable_content_props()
        .into_iter()
        .chain(edits.iter().copied())
    {
        source_text = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source_text)
            .expect("stable source edit should apply");
    }
    source_text
}

pub fn assert_content_projection_rebound(
    projection: &worth_ui::facade::WorthUiPrimitiveProjectionReceipt,
) {
    assert_eq!(
        projection.rebind_status(),
        WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_projection_row(
        projection.changed_rows(),
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_projection_row(
        projection.changed_rows(),
        WorthUiSemanticSliceId::PrimitiveContent,
        WorthUiRuntimeFactFamily::PrimitiveContent,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
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
        .expect("content primitive projection target binds")
}

fn stable_content_inputs() -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let source = inputs.source();
    let module_path = source.module_path().to_owned();
    let source_text = stable_source_text_with_content_edits(&[]);
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}

fn stable_content_props() -> [(&'static str, &'static str); 10] {
    [
        ("primitive_text", "\"Worth primitive\""),
        ("primitive_align", "center"),
        ("primitive_padding", "validation.density.primitive.padding"),
        ("primitive_radius", "validation.density.primitive.radius"),
        ("flow_kind", "inline"),
        ("flow_gap", "validation.density.primitive.flow.gap.default"),
        ("content_kind", "inline"),
        ("content_order", "\"icon,text\""),
        ("content_text", "\"Worth primitive\""),
        ("content_icon", "worth.icon.action.plus"),
    ]
}

fn assert_projection_row(
    rows: &[worth_ui::facade::WorthUiPrimitiveChangedFactEvidenceRow],
    expected_slice: WorthUiSemanticSliceId,
    expected_family: WorthUiRuntimeFactFamily,
    expected_posture: WorthUiAuthoredDeltaChangePosture,
) {
    let row = rows
        .iter()
        .find(|row| row.semantic_slice() == expected_slice)
        .expect("expected content projection row");
    assert_eq!(row.subject_surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(row.change_posture(), expected_posture);
    assert!(
        row.changed_facts()
            .iter()
            .any(|fact| fact.family() == expected_family),
        "expected changed fact family {}",
        expected_family.token()
    );
}
