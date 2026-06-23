use worth_ui::facade::{
    SurfaceId, WorthUiAuthoredDeltaChangePosture, WorthUiPrimitiveProjectionReceipt,
    WorthUiPrimitiveProofDenial, WorthUiPrimitiveValueDenialCode,
    WorthUiPrimitiveValueDenialReceipt, WorthUiRuntimeFactFamily, WorthUiRuntimeFactId,
    WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationReloadRequest, ValidationSourcePackage,
};
use worth_ui_validation_app::{ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch};

pub const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";

pub fn primitive_surface_id() -> SurfaceId {
    SurfaceId::new(PRIMITIVE_SURFACE).expect("valid primitive surface id")
}

pub fn assert_primitive_edit_rebinds(
    edit: ValidationAuthoredReloadEdit,
    expected_primitive_slice: WorthUiSemanticSliceId,
    assert_receipt: impl FnOnce(&worth_ui::facade::WorthUiPrimitiveProofReceipt),
) {
    let projection = activate_primitive_edit(edit);

    assert_eq!(
        projection.rebind_status(),
        worth_ui::facade::WorthUiPrimitiveProjectionRebindStatus::Rebound
    );
    assert_exact_projection_rows(
        projection.changed_rows(),
        expected_primitive_slice,
        WorthUiAuthoredDeltaChangePosture::Changed,
    );
    assert_receipt(projection.primitive_receipt());
}

pub fn activate_primitive_edit(
    edit: ValidationAuthoredReloadEdit,
) -> WorthUiPrimitiveProjectionReceipt {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_primitive_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();
    let source_inputs = stable_primitive_inputs();
    let source = edit
        .apply_to_source_text(source_inputs.source().source_text())
        .expect("structured primitive edit should apply to sample source");
    let prepared_reload = workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(source_inputs.source().module_path(), source),
    );
    let mapping = prepared_reload
        .changed_fact_mapping_receipt()
        .cloned()
        .expect("primitive prop edit should produce changed fact mapping");
    workbench
        .activate_reload(prepared_reload)
        .expect("primitive source edit should activate");
    workbench
        .runtime()
        .resolve_primitive_projection(&primitive_surface_id(), Some(&mapping))
        .expect("primitive projection should resolve after reload")
}

pub fn assert_primitive_denial(
    edit: ValidationAuthoredReloadEdit,
    expected_prop_key: &str,
    expected_raw_value: &str,
    expected_code: WorthUiPrimitiveValueDenialCode,
    expected_slice: WorthUiSemanticSliceId,
    expected_family: WorthUiRuntimeFactFamily,
) {
    let receipt = primitive_value_denial_for_edit(edit);
    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(receipt.prop_key(), expected_prop_key);
    assert_eq!(receipt.raw_value(), expected_raw_value);
    assert_eq!(receipt.denial_code(), expected_code);
    assert_eq!(receipt.semantic_slice(), expected_slice);
    assert_eq!(receipt.fact_family(), expected_family);
    assert!(!receipt.schema_id().is_empty());
    assert!(!receipt.expected_shape().is_empty());
    assert!(!receipt.examples().is_empty());
    assert!(receipt.source_span().is_some());
    assert_ne!(receipt.denial_digest(), 0);
}

pub fn primitive_value_denial_for_edit(
    edit: ValidationAuthoredReloadEdit,
) -> WorthUiPrimitiveValueDenialReceipt {
    let mut workbench = ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_primitive_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench();
    let source_inputs = stable_primitive_inputs();
    let source = edit
        .apply_to_source_text(source_inputs.source().source_text())
        .expect("structured primitive edit should apply to sample source");
    let prepared_reload = workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(source_inputs.source().module_path(), source),
    );
    workbench
        .activate_reload(prepared_reload)
        .expect("malformed primitive value should still reload as authored source");
    let denial = workbench
        .runtime()
        .resolve_primitive_projection(&primitive_surface_id(), None)
        .expect_err("primitive projection should reject malformed primitive value");

    value_denial_receipt(&denial).clone()
}

pub fn assert_exact_projection_rows(
    rows: &[worth_ui::facade::WorthUiPrimitiveChangedFactEvidenceRow],
    expected_primitive_slice: WorthUiSemanticSliceId,
    expected_posture: WorthUiAuthoredDeltaChangePosture,
) {
    assert_eq!(rows.len(), 2);
    assert_projection_row(
        rows,
        WorthUiSemanticSliceId::AuthoredSurfaceInstanceProps,
        WorthUiRuntimeFactFamily::AuthoredSurfaceProps,
        expected_posture,
    );
    assert_projection_row(
        rows,
        expected_primitive_slice,
        primitive_fact_family(expected_primitive_slice),
        expected_posture,
    );
}

pub fn assert_receipt_consumes(
    facts: &[WorthUiRuntimeFactId],
    expected: &[WorthUiRuntimeFactFamily],
) {
    assert_eq!(facts.len(), expected.len());
    for expected_family in expected {
        assert!(
            facts.iter().any(|fact| {
                fact.family() == *expected_family && fact.identity() == PRIMITIVE_SURFACE
            }),
            "expected dependency fact family {} for primitive surface",
            expected_family.token()
        );
    }
}

pub fn stable_primitive_inputs() -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let source = inputs.source();
    let module_path = source.module_path().to_owned();
    let mut source_text = source.source_text().to_owned();
    for (key, value) in [
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
    ] {
        source_text = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source_text)
            .expect("stable primitive input edit should apply");
    }
    source_text = ValidationAuthoredReloadEdit::remove_surface_prop(PRIMITIVE_SURFACE, "icon")
        .apply_to_source_text(&source_text)
        .unwrap_or(source_text);
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}

fn value_denial_receipt(
    denial: &WorthUiPrimitiveProofDenial,
) -> &WorthUiPrimitiveValueDenialReceipt {
    let WorthUiPrimitiveProofDenial::InvalidAuthoredPrimitiveValues { report } = denial else {
        panic!("expected primitive value denial report, got {denial:?}");
    };
    report
        .status()
        .denial_set()
        .expect("primitive denial report carries denial set")
        .denials()
        .first()
        .expect("primitive denial set is non-empty")
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
        .expect("expected primitive projection row");
    assert_eq!(row.subject_surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(row.change_posture(), expected_posture);
    assert_eq!(row.changed_facts().len(), 1);
    assert_eq!(row.changed_facts()[0].family(), expected_family);
    assert_eq!(row.changed_facts()[0].identity(), PRIMITIVE_SURFACE);
}

fn primitive_fact_family(slice: WorthUiSemanticSliceId) -> WorthUiRuntimeFactFamily {
    match slice {
        WorthUiSemanticSliceId::PrimitiveContent => WorthUiRuntimeFactFamily::PrimitiveContent,
        WorthUiSemanticSliceId::PrimitiveContainer => WorthUiRuntimeFactFamily::PrimitiveContainer,
        WorthUiSemanticSliceId::PrimitiveMeasurement => {
            WorthUiRuntimeFactFamily::PrimitiveMeasurement
        }
        WorthUiSemanticSliceId::PrimitiveAppearance => {
            WorthUiRuntimeFactFamily::PrimitiveAppearance
        }
        WorthUiSemanticSliceId::PrimitiveInteraction => {
            WorthUiRuntimeFactFamily::PrimitiveInteraction
        }
        WorthUiSemanticSliceId::PrimitiveMotion => WorthUiRuntimeFactFamily::PrimitiveMotion,
        _ => panic!("slice is not a primitive fact family"),
    }
}
