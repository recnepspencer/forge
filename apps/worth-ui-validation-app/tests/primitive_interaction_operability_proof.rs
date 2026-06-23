mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{
    WorthUiInteractionSubmissionDenial, WorthUiPrimitiveFocusPosture,
    WorthUiPrimitiveOperabilityBasis, WorthUiPrimitiveOperabilityPosture,
    WorthUiPrimitiveResolvedCursorPosture,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

use primitive_interaction_support::PRIMITIVE_SURFACE;
use validation_app_reload_fixture::ValidationAppReloadFixture;

#[test]
fn disabled_readiness_denies_mounted_click_in_runtime_lane() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "interaction_readiness",
        "disabled",
    ))
    .expect("disabled readiness edit applies");

    let primitive = app
        .centered_primitive_proof()
        .expect("disabled primitive proof resolves");
    let operability = primitive.interaction().operability();
    assert_eq!(
        operability.posture(),
        WorthUiPrimitiveOperabilityPosture::Disabled
    );
    assert_eq!(
        operability.basis(),
        WorthUiPrimitiveOperabilityBasis::InteractionReadinessDisabled
    );
    assert!(!operability.can_activate());
    assert!(!operability.can_focus());
    assert!(operability.disabled_posture());
    let affordance = primitive.interaction().affordance();
    assert!(!affordance.can_activate());
    assert_eq!(affordance.operability(), operability);
    assert_eq!(
        affordance.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::NotAllowed
    );
    assert_eq!(affordance.focus(), WorthUiPrimitiveFocusPosture::None);
    assert!(affordance.disabled_posture());

    let denial = app
        .click_centered_primitive_for_proof()
        .expect_err("runtime denies disabled mounted interaction");
    assert!(matches!(
        denial.interaction_submission_denial(),
        Some(WorthUiInteractionSubmissionDenial::DisabledInteraction { .. })
    ));
    assert!(app.last_primitive_interaction().is_none());
}

#[test]
fn primitive_disabled_resolves_the_same_operability_category() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_disabled",
        "true",
    ))
    .expect("primitive disabled edit applies");

    let primitive = app
        .centered_primitive_proof()
        .expect("disabled primitive proof resolves");
    let operability = primitive.interaction().operability();
    assert_eq!(
        operability.posture(),
        WorthUiPrimitiveOperabilityPosture::Disabled
    );
    assert_eq!(
        operability.basis(),
        WorthUiPrimitiveOperabilityBasis::PrimitiveDisabled
    );
    assert!(!operability.can_activate());
    assert!(operability.disabled_posture());
}
