#![allow(dead_code)]

use worth_ui::facade::{SurfaceId, WorthUiPrimitiveProjectionReceipt, WorthUiPrimitiveProofDenial};
use worth_ui_validation_app::reload::{
    ValidationAuthoredReloadEdit, ValidationReloadRequest, ValidationSourcePackage,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

pub const PRIMITIVE_SURFACE: &str = "worth.surface.preview.primitive.proof";

pub fn launch_stable_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(stable_appearance_state_inputs())
        .expect("validation workbench should prepare")
        .into_runtime_workbench()
}

pub fn prepare_reload_for_edits(
    workbench: &ValidationRuntimeWorkbench,
    edits: &[ValidationAuthoredReloadEdit],
) -> worth_ui::facade::WorthUiValidationPreparedReload {
    let source_inputs = stable_appearance_state_inputs();
    workbench.runtime().prepare_validation_reload(
        workbench.runtime().active_capability_snapshot(),
        ValidationReloadRequest::from_source_module(
            source_inputs.source().module_path(),
            source_text_after_edits(edits),
        ),
    )
}

pub fn activate_prepared_reload(
    workbench: &mut ValidationRuntimeWorkbench,
    prepared_reload: worth_ui::facade::WorthUiValidationPreparedReload,
) {
    workbench
        .activate_reload(prepared_reload)
        .expect("appearance-state source edit should activate");
}

pub fn resolve_projection(
    workbench: &ValidationRuntimeWorkbench,
    mapping: Option<&worth_ui::facade::WorthUiValidationChangedFactMappingReceipt>,
) -> Result<WorthUiPrimitiveProjectionReceipt, WorthUiPrimitiveProofDenial> {
    workbench
        .runtime()
        .resolve_primitive_projection(&primitive_surface_id(), mapping)
}

fn primitive_surface_id() -> SurfaceId {
    SurfaceId::new(PRIMITIVE_SURFACE).expect("valid primitive surface id")
}

fn stable_appearance_state_inputs() -> ValidationWorkbenchAuthoredInputs {
    let inputs = ValidationWorkbenchAuthoredInputs::sample();
    let source = inputs.source();
    let module_path = source.module_path().to_owned();
    let mut source_text = source.source_text().to_owned();
    for (key, value) in stable_props() {
        source_text = ValidationAuthoredReloadEdit::set_surface_prop(PRIMITIVE_SURFACE, key, value)
            .apply_to_source_text(&source_text)
            .expect("stable appearance-state input edit should apply");
    }
    inputs.with_source(ValidationSourcePackage::new(module_path, source_text))
}

fn source_text_after_edits(edits: &[ValidationAuthoredReloadEdit]) -> String {
    let source_inputs = stable_appearance_state_inputs();
    let mut source = source_inputs.source().source_text().to_owned();
    for edit in edits {
        source = edit
            .apply_to_source_text(&source)
            .expect("structured appearance-state edit should apply");
    }
    source
}

fn stable_props() -> [(&'static str, &'static str); 21] {
    [
        ("primitive_text", "\"Submit\""),
        ("primitive_background", "\"#2f7de1\""),
        ("primitive_foreground", "\"#ffffff\""),
        ("primitive_disabled", "false"),
        ("primitive_selected", "false"),
        ("interaction_kind", "submit"),
        ("interaction_id", "worth.interaction.primitive.submit"),
        ("interaction_payload", "\"submit.primary\""),
        (
            "interaction_target",
            "worth.surface.preview.primitive.proof",
        ),
        ("interaction_readiness", "enabled"),
        ("flow_kind", "inline"),
        ("flow_gap", "validation.density.primitive.flow.gap.default"),
        (
            "flow_padding",
            "validation.density.primitive.flow.padding.default",
        ),
        (
            "appearance_rest_background",
            "validation.theme.header.menu.active",
        ),
        ("appearance_rest_text_color", "validation.theme.header.text"),
        ("appearance_rest_icon_color", "validation.theme.header.text"),
        (
            "appearance_rest_border_width",
            "validation.density.primitive.border.none",
        ),
        (
            "appearance_rest_radius",
            "validation.density.primitive.radius",
        ),
        ("appearance_pressed_background", "\"#ffffff\""),
        ("appearance_pressed_text_color", "\"#2f7de1\""),
        ("appearance_pressed_icon_color", "\"#2f7de1\""),
    ]
}
