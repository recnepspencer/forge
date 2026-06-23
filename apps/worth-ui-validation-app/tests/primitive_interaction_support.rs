#![allow(dead_code)]

#[path = "validation_app_reload_fixture.rs"]
mod validation_app_reload_fixture;

use worth_ui::facade::{
    SurfaceId, WorthUiInteractionKind, WorthUiInteractionTarget, WorthUiMountedInteractionGesture,
    WorthUiRuntimeFactFamily, WorthUiSemanticSliceId,
};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationPreparedReload, ValidationReloadRequest,
    ValidationSourcePackage,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

use validation_app_reload_fixture::ValidationAppReloadFixture;

pub const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";

pub fn launch_interaction_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(normalized_interaction_sample_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench()
}

pub fn prepare_interaction_reload(
    workbench: &ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) -> ValidationPreparedReload {
    let inputs = normalized_interaction_sample_inputs();
    let mut source_text = inputs.source().source_text().to_owned();
    for edit in edits {
        source_text = edit
            .apply_to_source_text(&source_text)
            .expect("interaction edit applies to source");
    }
    workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(inputs.source().module_path(), source_text),
    )
}

pub fn assert_rebinds_primitive_interaction_fact(prepared: &ValidationPreparedReload) {
    let changed = prepared
        .changed_fact_mapping_receipt()
        .expect("interaction edit emits changed fact mapping");

    let mut saw_interaction_fact = false;
    for row in changed.rows() {
        if row.semantic_row().slice_id() == WorthUiSemanticSliceId::PrimitiveInteraction
            && row
                .changed_facts()
                .contains_family(WorthUiRuntimeFactFamily::PrimitiveInteraction)
        {
            saw_interaction_fact = true;
        }
    }
    assert!(saw_interaction_fact);
}

pub fn submit_centered_primitive(
    workbench: &mut ValidationRuntimeWorkbench,
) -> worth_ui::facade::WorthUiInteractionReceipt {
    let surface_id = primitive_surface_id();
    let proof = workbench
        .runtime()
        .resolve_primitive_proof(&surface_id)
        .expect("primitive proof resolves");
    let request = proof.interaction().activation_request(
        &surface_id,
        WorthUiMountedInteractionGesture::primary_click(),
    );
    workbench
        .submit_surface_interaction(request)
        .expect("generic interaction lane emits receipt")
}

pub fn assert_mounted_kind_target_emits(
    kind: WorthUiInteractionKind,
    target_prop: &str,
    target_value: &str,
    expected_target: WorthUiInteractionTarget,
) {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_disabled",
        "false",
    ))
    .expect("primitive disabled reset applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "interaction_kind",
        kind.token(),
    ))
    .expect("interaction kind edit applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "interaction_id",
        format!("worth.interaction.validation.{}", kind.token()),
    ))
    .expect("interaction id edit applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "interaction_readiness",
        "enabled",
    ))
    .expect("interaction readiness reset applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        target_prop,
        target_value,
    ))
    .expect("interaction target edit applies");

    let receipt = app
        .click_centered_primitive_for_proof()
        .expect("mounted primitive click emits through generic interaction lane");
    assert_eq!(receipt.kind(), kind);
    assert_eq!(receipt.target(), &expected_target);
    assert_eq!(
        receipt.interaction_id(),
        format!("worth.interaction.validation.{}", kind.token())
    );
    assert_eq!(app.last_primitive_interaction(), Some(&receipt));
}

pub fn primitive_surface_id() -> SurfaceId {
    SurfaceId::new(PRIMITIVE_SURFACE).expect("valid primitive surface id")
}

fn normalized_interaction_sample_inputs() -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let module_path = inputs.source().module_path().to_owned();
    let mut source_text = inputs.source().source_text().to_owned();
    for edit in [
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_kind",
            "submit",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_id",
            "worth.interaction.primitive.submit",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_payload",
            "\"submit.secondary\"",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_readiness",
            "enabled",
        ),
        ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "primitive_disabled",
            "false",
        ),
    ] {
        source_text = edit
            .apply_to_source_text(&source_text)
            .expect("normalizing primitive interaction sample source should apply");
    }
    source_text =
        ValidationAuthoredReloadEdit::remove_surface_prop(PRIMITIVE_SURFACE, "interaction_command")
            .apply_to_source_text(&source_text)
            .unwrap_or(source_text);
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}
