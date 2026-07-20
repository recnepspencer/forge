use super::activation_staging_test_support::activation_staging_inputs;
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiPlanLoweringDenialReason, WorthUiPlanNodeInputFamily,
};

#[test]
fn same_staged_artifact_produces_same_plan_input() {
    let left = activation_staging_inputs();
    let (left_runtime, left_pending) = left.into_runtime_and_pending();
    let left_input = left_runtime
        .prepare_execution_plan_input(left_pending)
        .expect("plan input prepares");

    let right = activation_staging_inputs();
    let (right_runtime, right_pending) = right.into_runtime_and_pending();
    let right_input = right_runtime
        .prepare_execution_plan_input(right_pending)
        .expect("plan input prepares");

    assert_eq!(left_input.basis(), right_input.basis());
    assert_eq!(left_input.context(), right_input.context());
    assert_eq!(left_input.node_inputs(), right_input.node_inputs());
    assert_eq!(left_input.counters(), right_input.counters());
}

#[test]
fn unchanged_replacement_plan_input_materializes_no_candidate_wide_rows() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");
    let counters = plan_input.counters();

    assert!(plan_input.node_inputs().is_empty());
    assert_eq!(counters.staged_node_input_count(), 0);
    assert_eq!(counters.query_binding_input_count(), 0);
    assert!(plan_input.basis().candidate_node_input_count() > 0);
    assert!(plan_input.basis().query_binding_input_count() > 0);
    assert!(counters.reconciliation_receipt_input_count() > 0);
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.registry_string_lookup_count(), 0);
}

#[test]
fn preserved_query_binding_is_carried_by_successor_cardinality_not_a_delta_row() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");

    assert_eq!(plan_input.basis().query_binding_input_count(), 1);
    assert!(plan_input
        .node_inputs()
        .iter()
        .all(|input| input.query_binding_identity().is_none()));
    assert_eq!(plan_input.counters().query_binding_input_count(), 0);
}

#[test]
fn preserved_query_posture_does_not_force_candidate_wide_relowering() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");

    assert_eq!(plan_input.basis().query_binding_input_count(), 1);
    assert!(plan_input.node_inputs().is_empty());
    assert_eq!(plan_input.counters().query_binding_input_count(), 0);
}

#[test]
fn plan_lowering_rejects_stale_pending_activation() {
    let inputs = activation_staging_inputs();
    let (mut runtime, pending) = inputs.into_runtime_and_pending();
    runtime.advance_frame_epoch_for_test();

    let denial = runtime
        .prepare_execution_plan_input(pending)
        .expect_err("stale pending activation denies");

    assert_eq!(
        denial.reason(),
        WorthUiPlanLoweringDenialReason::StalePendingActivation
    );
    assert_ne!(denial.pending_frame_epoch(), denial.active_frame_epoch());
    assert_eq!(denial.counters().epoch_verification_count(), 1);
    assert_eq!(denial.counters().readiness_verification_count(), 0);
    assert_no_plan_construction_work(denial.counters());
}

#[test]
fn component_lowering_hook_cannot_emit_unregistered_plan_node_family() {
    let inputs = activation_staging_inputs();
    let registered_hook = WorthUiComponentLoweringHook::registered(
        "platform.component_hook.a11y",
        WorthUiPlanNodeInputFamily::Accessibility,
    );
    let hook = WorthUiComponentLoweringHook::unregistered_for_test(
        "platform.component_hook.rogue",
        "platform.local.executable_node",
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();

    let denial = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &[registered_hook, hook],
        )
        .expect_err("unregistered hook family denies");

    assert_eq!(
        denial.reason(),
        WorthUiPlanLoweringDenialReason::UnregisteredPlanNodeFamily
    );
    assert_eq!(denial.counters().rejected_component_hook_count(), 1);
    assert_eq!(denial.counters().readiness_verification_count(), 1);
    assert_no_plan_construction_work(denial.counters());
}

#[test]
fn component_lowering_hook_may_emit_registered_observation_input() {
    let inputs = activation_staging_inputs();
    let hook = WorthUiComponentLoweringHook::registered(
        "platform.component_hook.a11y",
        WorthUiPlanNodeInputFamily::Accessibility,
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();

    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(pending, &[hook])
        .expect("registered hook family prepares");

    assert!(plan_input.node_inputs().iter().any(|input| {
        input.identity_basis() == "platform.component_hook.a11y"
            && input.family() == WorthUiPlanNodeInputFamily::Accessibility
    }));
    assert_eq!(plan_input.counters().component_hook_input_count(), 1);
}

fn assert_no_plan_construction_work(counters: crate::runtime::WorthUiPlanLoweringCounters) {
    assert_eq!(counters.staged_node_input_count(), 0);
    assert_eq!(counters.query_binding_input_count(), 0);
    assert_eq!(counters.reconciliation_receipt_input_count(), 0);
    assert_eq!(counters.component_hook_input_count(), 0);
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.registry_string_lookup_count(), 0);
}
