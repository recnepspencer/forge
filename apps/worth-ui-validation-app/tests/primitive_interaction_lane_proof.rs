mod primitive_interaction_support;
mod validation_app_reload_fixture;

use worth_ui::facade::{
    WorthUiInteractionKind, WorthUiInteractionOperabilityBasis, WorthUiInteractionSubmissionDenial,
    WorthUiInteractionTarget, WorthUiMountedInteractionActivation,
    WorthUiQueryGraphCanonicalObligationKind, WorthUiQueryGraphObligationSemantic,
    WorthUiRuntimeFactFamily,
};
use worth_ui_validation_app::reload::ValidationAuthoredReloadEdit;
use worth_ui_validation_app::runtime_workbench::ValidationComponentInteractionApplicationDenial;

use primitive_interaction_support::{
    assert_rebinds_primitive_interaction_fact, launch_interaction_workbench,
    mounted_interaction_plan_for_surface, prepare_interaction_reload, primitive_surface_id,
    submit_centered_primitive, PRIMITIVE_SURFACE,
};

#[test]
fn click_emits_generic_submit_receipt_with_active_authored_payload() {
    let mut workbench = launch_interaction_workbench();
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(receipt.kind(), WorthUiInteractionKind::Submit);
    assert_eq!(receipt.surface_id(), PRIMITIVE_SURFACE);
    assert_eq!(
        receipt
            .payload()
            .field("payload")
            .expect("submit receipt carries payload")
            .as_text(),
        "submit.secondary"
    );
}

#[test]
fn payload_edit_changes_next_receipt_and_rebinds_interaction_fact() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_payload",
            "\"submit.changed\"",
        )],
    );
    assert_rebinds_primitive_interaction_fact(&prepared);

    workbench
        .activate_reload(prepared)
        .expect("interaction payload reload activates");
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(
        receipt
            .payload()
            .field("payload")
            .expect("submit receipt carries payload")
            .as_text(),
        "submit.changed"
    );
}

#[test]
fn target_edit_changes_next_receipt_without_component_code_rebuild() {
    let mut workbench = launch_interaction_workbench();
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_target",
            "worth.surface.preview.primitive.alternate",
        )],
    );
    assert_rebinds_primitive_interaction_fact(&prepared);

    workbench
        .activate_reload(prepared)
        .expect("interaction target reload activates");
    let receipt = submit_centered_primitive(&mut workbench);

    assert_eq!(
        receipt.target(),
        &WorthUiInteractionTarget::Surface("worth.surface.preview.primitive.alternate".to_owned())
    );
}

#[test]
fn mounted_interaction_plan_selects_graph_activation_obligations() {
    let workbench = launch_interaction_workbench();
    let surface_id = primitive_surface_id();
    let plan = mounted_interaction_plan_for_surface(&workbench, &surface_id);

    assert!(matches!(
        plan.activation(),
        WorthUiMountedInteractionActivation::Eligible(_)
    ));
    assert_query_obligation_selected(
        &plan,
        WorthUiQueryGraphObligationSemantic::ActivationEligibility,
    );
    assert_query_obligation_selected(
        &plan,
        WorthUiQueryGraphObligationSemantic::CapabilitySupport,
    );
    assert_query_obligation_selected(
        &plan,
        WorthUiQueryGraphObligationSemantic::InteractionContainment,
    );
    assert_query_obligation_not_applicable(
        &plan,
        WorthUiQueryGraphObligationSemantic::CommandSupport,
    );
    assert_query_obligation_not_applicable(
        &plan,
        WorthUiQueryGraphObligationSemantic::InteractionFocusability,
    );
    assert_eq!(
        plan.operability()
            .query_graph_execution()
            .selected_obligation_count(),
        WorthUiQueryGraphObligationSemantic::MOUNTED_INTERACTION_ACTIVATION.len()
    );
    assert!(plan
        .operability()
        .query_graph_execution()
        .rows()
        .iter()
        .any(|row| row.canonical_kind()
            == WorthUiQueryGraphCanonicalObligationKind::BlockingInvariant
            && row.semantic() == WorthUiQueryGraphObligationSemantic::ActivationEligibility
            && row.support_status() == "supported"));
    assert_ne!(plan.operability().query_graph_execution_digest(), 0);
    let WorthUiMountedInteractionActivation::Eligible(eligible) = plan.activation() else {
        panic!("mounted interaction should be eligible");
    };
    assert_eq!(
        eligible.query_graph_execution_digest(),
        plan.operability().query_graph_execution_digest()
    );
    assert!(plan
        .operability()
        .consumed_facts()
        .iter()
        .any(|fact| fact.family() == WorthUiRuntimeFactFamily::PrimitiveInteraction));
}

#[test]
fn query_execution_digest_changes_when_operability_state_changes() {
    let mut workbench = launch_interaction_workbench();
    let surface_id = primitive_surface_id();
    let enabled_plan = mounted_interaction_plan_for_surface(&workbench, &surface_id);
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_readiness",
            "disabled",
        )],
    );
    workbench
        .activate_reload(prepared)
        .expect("disabled interaction reload activates");
    let disabled_plan = mounted_interaction_plan_for_surface(&workbench, &surface_id);

    assert_ne!(
        enabled_plan.operability().query_graph_execution_digest(),
        disabled_plan.operability().query_graph_execution_digest()
    );
    assert_eq!(
        disabled_plan.operability().basis(),
        WorthUiInteractionOperabilityBasis::InteractionReadinessDisabled
    );
    assert!(disabled_plan
        .operability()
        .query_graph_execution()
        .rows()
        .iter()
        .any(
            |row| row.semantic() == WorthUiQueryGraphObligationSemantic::ActivationEligibility
                && row.support_status() == "unsupported"
        ));
}

#[test]
fn stale_activation_receipt_cannot_emit_after_payload_reload() {
    let mut workbench = launch_interaction_workbench();
    let surface_id = primitive_surface_id();
    let plan = mounted_interaction_plan_for_surface(&workbench, &surface_id);
    let WorthUiMountedInteractionActivation::Eligible(stale_eligible) = plan.activation().clone()
    else {
        panic!("initial interaction should be eligible");
    };
    let prepared = prepare_interaction_reload(
        &workbench,
        &[ValidationAuthoredReloadEdit::set_surface_prop(
            PRIMITIVE_SURFACE,
            "interaction_payload",
            "\"submit.after.stale\"",
        )],
    );

    workbench
        .activate_reload(prepared)
        .expect("payload reload activates");
    let denial = workbench
        .submit_mounted_interaction(stale_eligible)
        .expect_err("stale eligible receipt must not emit after reload");

    assert!(matches!(
        denial,
        ValidationComponentInteractionApplicationDenial::Interaction(
            WorthUiInteractionSubmissionDenial::StaleActivationReceipt { .. }
        )
    ));
}

fn assert_query_obligation_selected(
    plan: &worth_ui::facade::WorthUiMountedInteractionPlan,
    semantic: WorthUiQueryGraphObligationSemantic,
) {
    assert!(
        plan.operability()
            .query_graph_execution()
            .rows()
            .iter()
            .any(|row| row.semantic() == semantic && row.support_status() == "supported"),
        "expected selected Query graph obligation {semantic:?}"
    );
}

fn assert_query_obligation_not_applicable(
    plan: &worth_ui::facade::WorthUiMountedInteractionPlan,
    semantic: WorthUiQueryGraphObligationSemantic,
) {
    assert!(
        plan.operability()
            .query_graph_execution()
            .rows()
            .iter()
            .any(|row| row.semantic() == semantic && row.support_status() == "not-applicable"),
        "expected not-applicable Query graph obligation {semantic:?}"
    );
}
