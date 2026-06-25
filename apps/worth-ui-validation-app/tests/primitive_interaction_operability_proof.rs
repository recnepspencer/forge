mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{
    WorthUiInteractionOperabilityBasis, WorthUiInteractionOperabilityPosture,
    WorthUiMountedInteractionActivation, WorthUiPrimitiveActivationPosture,
    WorthUiPrimitiveFocusPosture, WorthUiPrimitiveOperabilityBasis,
    WorthUiPrimitiveOperabilityPosture, WorthUiPrimitiveResolvedCursorPosture,
    WorthUiQueryGraphObligationSemantic, WorthUiUserIntentOperationFamily,
    WorthUiUserIntentTargetDenial,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;

use primitive_interaction_support::{
    launch_interaction_workbench, prepare_interaction_reload, PRIMITIVE_SURFACE,
};
use validation_app_reload_fixture::ValidationAppReloadFixture;

const UNMOUNTED_BUTTON_SURFACE: &str = "worth.surface.preview.button.proof";

#[test]
fn disabled_unmounted_button_surface_cannot_disable_visible_primitive_target() {
    let fixture = ValidationAppReloadFixture::new();
    let mut app = fixture.build_app();
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "primitive_disabled",
        "false",
    ))
    .expect("visible primitive disabled reset applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        PRIMITIVE_SURFACE,
        "interaction_readiness",
        "enabled",
    ))
    .expect("visible primitive readiness reset applies");
    app.apply_authored_reload_edit(ValidationAuthoredReloadEdit::set_surface_prop(
        UNMOUNTED_BUTTON_SURFACE,
        "primitive_disabled",
        "true",
    ))
    .expect("unmounted authored surface edit applies");

    let primitive = app
        .centered_primitive_proof()
        .expect("visible primitive proof still resolves");
    assert_eq!(primitive.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(
        primitive.target_binding().surface_id().as_str(),
        PRIMITIVE_SURFACE
    );
    assert_eq!(
        primitive.target_binding().operation_family(),
        WorthUiUserIntentOperationFamily::PrimitiveProof
    );
    assert_eq!(
        primitive.interaction().operability().posture(),
        WorthUiPrimitiveOperabilityPosture::Enabled
    );
    assert_eq!(
        primitive.target_binding().counters().source_reparse_count(),
        0
    );
    assert_eq!(
        primitive
            .target_binding()
            .counters()
            .page_slot_lookup_count(),
        1
    );
    assert_eq!(
        primitive.target_binding().counters().page_slot_scan_count(),
        0
    );
    assert_eq!(
        primitive.target_binding().counters().artifact_scan_count(),
        0
    );
}

#[test]
fn mounted_interaction_plan_carries_visible_target_binding_digest() {
    let fixture = ValidationAppReloadFixture::new();
    let app = fixture.build_app();
    let primitive = app
        .centered_primitive_proof()
        .expect("visible primitive proof resolves");
    let target = primitive
        .target_binding()
        .for_mounted_interaction(app.workbench().runtime().graph_authority());
    let target_digest = target.binding_digest();
    let plan = app
        .workbench()
        .runtime()
        .resolve_mounted_interaction_plan_for_target(target)
        .expect("target-bound mounted interaction plan resolves");

    assert_eq!(plan.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(plan.target_binding_digest(), Some(target_digest));
}

#[test]
fn missing_visible_target_denial_carries_query_graph_obligations() {
    let fixture = ValidationAppReloadFixture::new();
    let app = fixture.build_app();
    let denial = app
        .workbench()
        .runtime()
        .bind_visible_primitive_proof_target(app.workbench().page_host_plan(), "missing_slot")
        .expect_err("missing visible slot must deny target binding");
    let WorthUiUserIntentTargetDenial::MissingSlot { .. } = denial else {
        panic!("missing slot should stay typed");
    };
    let execution = denial.query_graph_execution();
    assert_ne!(execution.execution_digest(), 0);
    assert!(
        execution.rows().iter().any(|row| {
            row.semantic() == WorthUiQueryGraphObligationSemantic::TargetBindingPosture
                && row.support_status() == "unsupported"
        }),
        "non-bound target posture must be represented as Query-selected obligation evidence"
    );
}

#[test]
fn disabled_readiness_denies_mounted_click_in_runtime_lane() {
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
    assert!(operability.disabled_posture());
    let affordance = primitive.interaction().affordance();
    assert_eq!(
        affordance.activation_posture(),
        WorthUiPrimitiveActivationPosture::Denied
    );
    assert_eq!(affordance.operability(), operability);
    assert_eq!(
        affordance.cursor(),
        WorthUiPrimitiveResolvedCursorPosture::NotAllowed
    );
    assert_eq!(affordance.focus(), WorthUiPrimitiveFocusPosture::None);

    let denial = app
        .click_centered_primitive_for_proof()
        .expect_err("runtime denies disabled mounted interaction");
    assert!(denial.interaction_submission_denial().is_none());
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
    assert!(operability.disabled_posture());
}

#[test]
fn focus_interaction_requires_focusable_primitive_posture() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_kind",
                "focus",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "interaction_focus_target",
                "worth.focus.validation.primary",
            ),
            ValidationAuthoredReloadEdit::set_surface_prop(
                PRIMITIVE_SURFACE,
                "primitive_focus",
                "none",
            ),
        ],
    );
    workbench
        .activate_reload(prepared)
        .expect("focus interaction reload activates");
    let surface_id = primitive_interaction_support::primitive_surface_id();
    let target = workbench
        .runtime()
        .bind_authored_primitive_proof_target(&surface_id)
        .expect("focus target binds");
    let proof = workbench
        .runtime()
        .resolve_primitive_proof_for_target(&target)
        .expect("focus primitive proof resolves");
    let plan = workbench
        .runtime()
        .resolve_mounted_interaction_plan_for_target(
            proof
                .target_binding()
                .for_mounted_interaction(workbench.runtime().graph_authority()),
        )
        .expect("focus interaction plan resolves");

    assert_eq!(
        plan.operability().posture(),
        WorthUiInteractionOperabilityPosture::Unsupported
    );
    assert_eq!(
        plan.operability().basis(),
        WorthUiInteractionOperabilityBasis::NonFocusableTarget
    );
    assert!(matches!(
        plan.activation(),
        WorthUiMountedInteractionActivation::Denied(_)
    ));
}
