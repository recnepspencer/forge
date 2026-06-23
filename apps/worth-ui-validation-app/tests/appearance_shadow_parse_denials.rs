use worth_ui::facade::{
    WorthUiAppearanceShadowParseDenialCode, WorthUiCapabilityReloadDenialCode,
    WorthUiCapabilityReloadStage, WorthUiCapabilityReloadStatus,
};
use worth_ui_validation_app::reload::ValidationSourcePackage;
use worth_ui_validation_app::reload::{
    ValidationAppearanceSource, ValidationReloadInput, ValidationReloadTick,
    ValidationRuntimeReloadTickOutcome,
};
use worth_ui_validation_app::{
    ValidationRuntimeWorkbench, ValidationWorkbenchAuthoredInputs, ValidationWorkbenchLaunch,
};

#[test]
fn invalid_shadow_color_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #zzzzzzff 0px 1px 3px 0px",
        WorthUiAppearanceShadowParseDenialCode::InvalidColor,
    );
}

#[test]
fn invalid_shadow_offset_x_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #102030ff 2rem 1px 3px 0px",
        WorthUiAppearanceShadowParseDenialCode::InvalidOffsetX,
    );
}

#[test]
fn invalid_shadow_offset_y_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #102030ff 0px 2rem 3px 0px",
        WorthUiAppearanceShadowParseDenialCode::InvalidOffsetY,
    );
}

#[test]
fn invalid_shadow_blur_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #102030ff 0px 1px 2rem 0px",
        WorthUiAppearanceShadowParseDenialCode::InvalidBlur,
    );
}

#[test]
fn invalid_shadow_spread_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #102030ff 0px 1px 3px 2rem",
        WorthUiAppearanceShadowParseDenialCode::InvalidSpread,
    );
}

#[test]
fn invalid_shadow_arity_reports_typed_denial() {
    assert_shadow_denial(
        "validation.appearance.header.panel_shadow = #102030ff 0px 1px 3px",
        WorthUiAppearanceShadowParseDenialCode::InvalidArity,
    );
}

fn assert_shadow_denial(source_text: &str, expected: WorthUiAppearanceShadowParseDenialCode) {
    let mut workbench = runtime_workbench();
    let before_snapshot = workbench.runtime().inspect_active().snapshot_digest();

    let outcome = workbench.apply_reload_tick(ValidationReloadTick::Changed(
        ValidationReloadInput::HeaderAppearance(ValidationAppearanceSource::from_observed_file(
            "apps/worth-ui-validation-app/theme/header.appearance",
            source_text,
        )),
    ));

    let ValidationRuntimeReloadTickOutcome::AppearanceReloaded { evidence, .. } = outcome else {
        panic!("invalid shadow appearance should still report typed capability evidence");
    };
    assert_eq!(
        evidence.status(),
        WorthUiCapabilityReloadStatus::Denied(WorthUiCapabilityReloadStage::AppearanceSourceParse)
    );
    assert_eq!(
        evidence.denial_code(),
        Some(WorthUiCapabilityReloadDenialCode::AppearanceShadow(
            expected
        ))
    );
    assert_eq!(
        workbench.runtime().inspect_active().snapshot_digest(),
        before_snapshot
    );
}

fn runtime_workbench() -> ValidationRuntimeWorkbench {
    ValidationWorkbenchLaunch::new()
        .prepare_from_authored_inputs(ValidationWorkbenchAuthoredInputs::new(
            ValidationSourcePackage::sample(),
        ))
        .expect("validation app launches through Worth UI")
        .into_runtime_workbench()
}
