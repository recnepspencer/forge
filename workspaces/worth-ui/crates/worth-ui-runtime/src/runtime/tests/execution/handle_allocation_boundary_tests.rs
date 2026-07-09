use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis,
    admitted_measurement_basis_with_font_seed, allocation_planning,
};
use super::durable_state_inventory_test_support::platform_inventory;
use super::identity_match_graph_test_support::{
    artifact_from_nodes, identity_match_app, runtime_and_narrowing, surface_node,
};
use super::node_replacement_classification_test_support::{
    lane_affecting_impact_for, lane_narrowing_for,
};
use crate::runtime::{
    WorthUiComponentLoweringHook, WorthUiNodeLifecycleTransition, WorthUiPlanNodeInputFamily,
    WorthUiRuntimeHandleAllocationDenialReason,
};

#[test]
fn equivalent_plan_inputs_allocate_equivalent_runtime_handles() {
    let left = runtime_handle_allocation();
    let right = runtime_handle_allocation();

    assert_eq!(left.basis(), right.basis());
    assert_eq!(left.receipt(), right.receipt());
    assert_eq!(left.family_widths(), right.family_widths());
    assert_eq!(left.counters(), right.counters());
    assert_eq!(left.runtime_handles(), right.runtime_handles());
    assert_eq!(left.view_binding_handles(), right.view_binding_handles());
}

#[test]
fn handle_allocation_performs_no_source_parse_or_registry_lookup() {
    let allocation = runtime_handle_allocation();
    let counters = allocation.counters();

    assert_eq!(
        counters.plan_node_input_count(),
        allocation.runtime_handles().len()
    );
    assert!(counters.plan_node_input_count() > 0);
    assert!(counters.collision_check_count() > 0);
    assert_eq!(counters.source_parse_count(), 0);
    assert_eq!(counters.registry_string_lookup_count(), 0);
    assert_eq!(counters.broad_registry_scan_count(), 0);
}

#[test]
fn query_view_binding_handle_preserves_query_owned_evidence_boundary() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("handle-allocation.query-view-binding");
    let neighborhood = admitted_allocation_neighborhood("handle-allocation.query-view-binding");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
    let query_input_count = planning
        .node_inputs()
        .expect("admitted planning exposes node inputs")
        .iter()
        .filter(|input| {
            input.query_binding_identity().is_some() && input.query_binding_posture().is_some()
        })
        .count();

    let allocation = runtime
        .allocate_runtime_handles(&planning)
        .expect("handles allocate");

    assert_eq!(query_input_count, allocation.view_binding_handles().len());
    assert_eq!(
        allocation.family_widths().view_binding_handle_count(),
        allocation.view_binding_handles().len()
    );
    assert_eq!(
        allocation.counters().view_binding_handle_count(),
        allocation.view_binding_handles().len()
    );
}

#[test]
fn handle_allocation_reports_cardinality_and_collision_denials() {
    let inputs = activation_staging_inputs();
    let duplicate_hook = WorthUiComponentLoweringHook::registered(
        "platform.component_hook.duplicate",
        WorthUiPlanNodeInputFamily::ComponentInvocation,
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &[duplicate_hook.clone(), duplicate_hook],
        )
        .expect("plan input prepares with duplicated hook claims");
    let planning = allocation_planning(
        &runtime,
        &plan_input,
        "handle-allocation.duplicate.component",
    );

    let denial = runtime
        .allocate_runtime_handles(&planning)
        .expect_err("duplicate component handle claim denies");

    assert_eq!(
        denial.reason(),
        WorthUiRuntimeHandleAllocationDenialReason::DuplicatePlanLocalHandleClaim
    );
    assert_eq!(denial.counters().collision_denial_count(), 1);
    assert!(denial.counters().collision_check_count() > 1);
    assert_eq!(denial.counters().plan_node_input_count(), 0);
    assert_eq!(denial.counters().component_handle_count(), 0);
    assert_eq!(denial.counters().view_binding_handle_count(), 0);
}

#[test]
fn handle_allocation_rejects_non_component_plan_local_claim_collisions() {
    let inputs = activation_staging_inputs();
    let duplicate_hook = WorthUiComponentLoweringHook::registered(
        "platform.token_hook.duplicate",
        WorthUiPlanNodeInputFamily::TokenStyle,
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(
            pending,
            &[duplicate_hook.clone(), duplicate_hook],
        )
        .expect("plan input prepares with duplicated token claims");
    let planning = allocation_planning(&runtime, &plan_input, "handle-allocation.duplicate.token");

    let denial = runtime
        .allocate_runtime_handles(&planning)
        .expect_err("duplicate token handle claim denies");

    assert_eq!(
        denial.reason(),
        WorthUiRuntimeHandleAllocationDenialReason::DuplicatePlanLocalHandleClaim
    );
    assert_eq!(denial.counters().collision_denial_count(), 1);
    assert_eq!(denial.counters().plan_node_input_count(), 0);
    assert_eq!(denial.counters().token_handle_count(), 0);
}

#[test]
fn same_lowered_topology_but_changed_planning_semantics_requires_distinct_plan_receipt() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");
    let neighborhood = admitted_allocation_neighborhood("handle-allocation.plan-drift");
    let first_planning = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input.clone(),
        &admitted_measurement_basis_with_font_seed("handle-allocation.plan-drift", 100),
        &neighborhood,
    );
    let second_planning = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &admitted_measurement_basis_with_font_seed("handle-allocation.plan-drift", 240),
        &neighborhood,
    );

    assert_eq!(first_planning.node_inputs(), second_planning.node_inputs());

    let first = runtime
        .allocate_runtime_handles(&first_planning)
        .expect("first handles allocate");
    let second = runtime
        .allocate_runtime_handles(&second_planning)
        .expect("second handles allocate");

    assert_eq!(
        first.basis().active_artifact_digest(),
        second.basis().active_artifact_digest()
    );
    assert_eq!(
        first.basis().candidate_artifact_digest(),
        second.basis().candidate_artifact_digest()
    );
    assert_eq!(first.basis().frame_epoch(), second.basis().frame_epoch());
    assert_eq!(
        first.basis().plan_node_input_count(),
        second.basis().plan_node_input_count()
    );
    assert_eq!(
        first.basis().query_binding_input_count(),
        second.basis().query_binding_input_count()
    );
    assert_ne!(
        first.basis().allocation_planning_identity_digest(),
        second.basis().allocation_planning_identity_digest()
    );
    assert!(!first.receipt().certifies_basis(second.basis()));
    assert_ne!(first.receipt(), second.receipt());
}

#[test]
fn handle_reuse_after_lane_change_requires_new_plan_receipt() {
    let first = runtime_handle_allocation();
    let second = runtime_handle_allocation_for_lane_change();

    assert!(!first.receipt().certifies_basis(second.basis()));
    assert!(second.receipt().certifies_basis(second.basis()));
    assert_ne!(first.receipt(), second.receipt());
    assert_eq!(
        second.counters().state_slot_handle_count(),
        second.family_widths().state_slot_handle_count()
    );
    assert!(second.family_widths().state_slot_handle_count() > 0);
}

fn runtime_handle_allocation() -> crate::runtime::WorthUiRuntimeHandleAllocation {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("handle-allocation.runtime");
    let neighborhood = admitted_allocation_neighborhood("handle-allocation.runtime");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);
    runtime
        .allocate_runtime_handles(&planning)
        .expect("handles allocate")
}

fn runtime_handle_allocation_for_lane_change() -> crate::runtime::WorthUiRuntimeHandleAllocation {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![surface_node("surface:stable", "workspace.surface.main", 0)],
    )]);
    let candidate = active.clone();
    let (runtime, admitted, identity_narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&identity_narrowing, &admitted)
        .expect("identity graph builds");
    let impact = lane_affecting_impact_for(&identity_report);
    let narrowing = lane_narrowing_for(&identity_report);
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowing, &identity_report)
        .expect("lane-change node plan builds");
    assert_eq!(
        node_plan.transition_for_identity("surface:stable"),
        Some(WorthUiNodeLifecycleTransition::LaneChange)
    );
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("state inventory builds");
    let reconciliation_plan = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowing, &admitted)
        .expect("query comparison succeeds");
    let query_rebind_plan = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowing, &admitted)
        .expect("query rebind planning succeeds");
    let pending_execution_plan_lowering_input = runtime
        .prepare_pending_execution_plan_lowering_input(
            &node_plan,
            &reconciliation_plan,
            &query_rebind_plan,
        );
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            Some(&reconciliation_plan),
            Some(&query_rebind_plan),
            Some(&pending_execution_plan_lowering_input),
        )
        .expect("lane-change activation staging succeeds");
    let measurement_basis = admitted_measurement_basis("handle-allocation.lane-change");
    let neighborhood = admitted_allocation_neighborhood("handle-allocation.lane-change");
    let planning = runtime.plan_allocation(&pending, &measurement_basis, &neighborhood);

    runtime
        .allocate_runtime_handles(&planning)
        .expect("lane-change handles allocate")
}
