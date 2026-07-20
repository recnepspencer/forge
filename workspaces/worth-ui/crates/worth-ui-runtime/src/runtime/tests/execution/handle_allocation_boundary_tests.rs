use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_measurement_basis, admitted_measurement_basis_with_font_seed, allocation_planning,
    independent_allocation_planning, planning_graph_authority,
};
use super::durable_state_inventory_test_support::platform_inventory;
use super::identity_match_graph_test_support::{
    artifact_from_nodes, identity_match_app, runtime_and_narrowing, surface_node,
};
use super::node_replacement_classification_test_support::{
    lane_affecting_impact_for, lane_narrowing_for,
};
use crate::runtime::{
    UiAllocationReceiptCommitDenial, UiAllocationReuseDenial, WorthUiComponentLoweringHook,
    WorthUiNodeLifecycleTransition, WorthUiPlanNodeInputFamily,
    WorthUiRuntimeHandleAllocationDenialReason,
};

#[test]
fn equivalent_plan_inputs_preserve_layout_without_sharing_session_authority() {
    let left = runtime_handle_allocation();
    let right = runtime_handle_allocation();

    assert_eq!(left.basis(), right.basis());
    assert_eq!(
        left.receipt().basis_digest(),
        right.receipt().basis_digest()
    );
    assert_ne!(
        left.receipt().arena_identity(),
        right.receipt().arena_identity()
    );
    assert_eq!(left.family_widths(), right.family_widths());
    assert_eq!(left.counters(), right.counters());
    assert_eq!(
        left.runtime_handles()
            .iter()
            .map(|handle| (handle.family(), handle.plan_index()))
            .collect::<Vec<_>>(),
        right
            .runtime_handles()
            .iter()
            .map(|handle| (handle.family(), handle.plan_index()))
            .collect::<Vec<_>>()
    );
    assert_ne!(left.runtime_handles(), right.runtime_handles());
    assert_ne!(
        left.view_binding_handles().collect::<Vec<_>>(),
        right.view_binding_handles().collect::<Vec<_>>()
    );
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
    let plan_input = inputs
        .runtime
        .prepare_reconstructive_plan_input_for_test(&inputs.admitted, &[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("handle-allocation.query-view-binding");
    let (snapshot, selected) =
        planning_graph_authority("handle-allocation.query-view-binding", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("query handle planning admits through graph authority"),
    );
    let query_input_count = plan_input
        .node_inputs()
        .iter()
        .filter(|input| {
            input.query_binding_identity().is_some() && input.query_binding_posture().is_some()
        })
        .count();

    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);
    let allocation = runtime
        .allocate_runtime_handles(&facts)
        .expect("handles allocate");

    assert_eq!(query_input_count, allocation.view_binding_handles().count());
    assert_eq!(
        allocation.family_widths().view_binding_handle_count(),
        allocation.view_binding_handles().count()
    );
    assert_eq!(
        allocation.counters().view_binding_handle_count(),
        allocation.view_binding_handles().count()
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
            &pending,
            &[duplicate_hook.clone(), duplicate_hook],
        )
        .expect("plan input prepares with duplicated hook claims");
    let planning = independent_allocation_planning("handle-allocation.duplicate.component");
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);

    let denial = runtime
        .allocate_runtime_handles(&facts)
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
            &pending,
            &[duplicate_hook.clone(), duplicate_hook],
        )
        .expect("plan input prepares with duplicated token claims");
    let planning = allocation_planning(&runtime, &pending, "handle-allocation.duplicate.token");
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);

    let denial = runtime
        .allocate_runtime_handles(&facts)
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
fn same_lowered_topology_cannot_bypass_unsupported_partial_reuse() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let (snapshot, selected) =
        planning_graph_authority("handle-allocation.plan-drift", "operator:stack");
    let first_planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(
                &pending,
                &snapshot,
                admitted_measurement_basis_with_font_seed("handle-allocation.plan-drift", 100),
                &selected,
            )
            .expect("first measurement basis admits through candidate projection authority"),
    );
    let second_planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(
                &pending,
                &snapshot,
                admitted_measurement_basis_with_font_seed("handle-allocation.plan-drift", 240),
                &selected,
            )
            .expect("changed measurement basis admits through candidate projection authority"),
    );

    assert_eq!(
        first_planning
            .planning()
            .projection()
            .map(|projection| projection.evidence_digest()),
        second_planning
            .planning()
            .projection()
            .map(|projection| projection.evidence_digest())
    );

    let first_receipt = runtime
        .commit_allocation_candidate_for_test(first_planning)
        .expect("first planning meaning commits");
    let first_facts = runtime.execution_plan_lowering_facts_below_authority_for_test(
        first_receipt
            .lowering_input()
            .expect("first committed receipt remains fresh"),
        plan_input,
    );
    runtime
        .allocate_runtime_handles(&first_facts)
        .expect("first handles allocate");
    let denial = runtime
        .commit_allocation_candidate_for_test(second_planning)
        .expect_err("changed same-scope meaning must not mint a reusable receipt");
    assert!(matches!(
        denial,
        UiAllocationReceiptCommitDenial::ReuseDenied(report)
            if report.denial() == Some(UiAllocationReuseDenial::UnsupportedPartialReuse)
    ));
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
    assert_eq!(second.family_widths().state_slot_handle_count(), 0);
}

fn runtime_handle_allocation() -> crate::runtime::WorthUiRuntimeHandleAllocation {
    let inputs = activation_staging_inputs();
    let plan_input = inputs.reconstructive_plan_input(&[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let measurement_basis = admitted_measurement_basis("handle-allocation.runtime");
    let (snapshot, selected) =
        planning_graph_authority("handle-allocation.runtime", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("runtime handle planning admits through graph authority"),
    );
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);
    runtime
        .allocate_runtime_handles(&facts)
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
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowing,
            &node_plan,
            crate::runtime::WorthUiActivationStagingPlans::new(
                Some(&reconciliation_plan),
                Some(&query_rebind_plan),
            ),
        )
        .expect("lane-change activation staging succeeds");
    let measurement_basis = admitted_measurement_basis("handle-allocation.lane-change");
    let (snapshot, selected) =
        planning_graph_authority("handle-allocation.lane-change", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("lane-change handle planning admits through graph authority"),
    );
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("lane-change plan input prepares after planning");
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);

    runtime
        .allocate_runtime_handles(&facts)
        .expect("lane-change handles allocate")
}

#[test]
fn compact_handle_capacity_denies_before_index_or_generation_wraparound() {
    use crate::runtime::handle_allocation::WorthUiHandleCapacity;
    use crate::runtime::WorthUiHandleCapacityExhaustion;

    assert_eq!(
        WorthUiHandleCapacity::plan_index(u32::MAX as usize),
        Ok(u32::MAX)
    );
    if usize::BITS > u32::BITS {
        assert_eq!(
            WorthUiHandleCapacity::plan_index(u32::MAX as usize + 1),
            Err(WorthUiHandleCapacityExhaustion::PlanIndex)
        );
    }
    assert_eq!(
        WorthUiHandleCapacity::stable_slot(u64::from(u32::MAX) + 1),
        Err(WorthUiHandleCapacityExhaustion::StableSlot)
    );
    assert_eq!(
        WorthUiHandleCapacity::next_slot_generation(u64::MAX),
        Err(WorthUiHandleCapacityExhaustion::SlotGeneration)
    );
    assert_eq!(
        WorthUiHandleCapacity::child_range(u32::MAX as usize),
        Ok(u32::MAX)
    );
    if usize::BITS > u32::BITS {
        assert_eq!(
            WorthUiHandleCapacity::child_range((u32::MAX as usize) + 1),
            Err(WorthUiHandleCapacityExhaustion::ChildRange)
        );
    }
}
