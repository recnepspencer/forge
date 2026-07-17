use crate::runtime::{
    WorthUiDurableResizeInputPosture, WorthUiDurableStateFamilyId,
    WorthUiTransientInteractionPolicy, WorthUiTransientInteractionState,
};

use super::durable_state_inventory_test_support::platform_inventory;
use super::durable_state_reconciliation_test_support::{
    deterministic_reconciliation_inputs, lane_change_inputs, splitter_replace_inputs,
};
use super::identity_match_graph_test_support::{
    artifact_from_nodes, component_node, identity_match_app, runtime_and_narrowing,
    splitter_surface_node_with_authored_provenance_digest,
};
use super::node_replacement_classification_test_support::{narrowing_for, no_op_impact_for};

#[test]
fn splitter_position_state_participates_only_through_admitted_runtime_resize_seam() {
    let (runtime, plan, inventory) = deterministic_reconciliation_inputs();

    assert_eq!(
        inventory.transient(WorthUiTransientInteractionState::DragCapture),
        WorthUiTransientInteractionPolicy::Drop
    );

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("reconciliation succeeds");

    let admitted = reconciliation
        .admitted_durable_resize_input("surface:main")
        .expect("surface splitter state should admit durable resize input");
    assert_eq!(
        admitted.family_id(),
        &WorthUiDurableStateFamilyId::SplitterPosition
    );
    assert_eq!(
        admitted.posture(),
        WorthUiDurableResizeInputPosture::AdmittedPlanningTimeOnly
    );
    assert!(admitted.is_planning_time_only());
    assert_eq!(
        admitted.resize_permission(),
        &crate::capability::MosaicResizePermission::user_resizable()
    );
    assert!(
        reconciliation
            .admitted_durable_resize_input("component:dashboard")
            .is_none(),
        "non-surface nodes must not manufacture durable resize planning authority"
    );
}

#[test]
fn lane_change_splitter_state_remaps_explicitly() {
    let (runtime, plan, inventory) = lane_change_inputs();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("lane change reconciliation succeeds");
    let resize_input = reconciliation
        .durable_resize_input("surface:stable")
        .expect("lane-changed surface keeps an explicit durable resize decision");

    assert_eq!(
        resize_input.posture(),
        WorthUiDurableResizeInputPosture::RemappedForChangedResizeLane
    );
    assert!(resize_input.is_planning_time_only());
    assert_eq!(
        reconciliation
            .receipt_for(
                "surface:stable",
                &WorthUiDurableStateFamilyId::SplitterPosition
            )
            .expect("splitter receipt exists")
            .outcome(),
        crate::runtime::WorthUiDurableStateReconciliationOutcome::Recreate
    );
}

#[test]
fn incompatible_splitter_shape_change_denies_explicitly() {
    let (runtime, plan, inventory) = splitter_replace_inputs();

    let reconciliation = runtime
        .reconcile_durable_state(&plan, &inventory)
        .expect("replace reconciliation succeeds");
    let resize_input = reconciliation
        .durable_resize_input("surface:main")
        .expect("replaced surface keeps an explicit durable resize denial");

    assert_eq!(
        resize_input.posture(),
        WorthUiDurableResizeInputPosture::DeniedIncompatibleCarryForwardShape
    );
    assert!(reconciliation
        .admitted_durable_resize_input("surface:main")
        .is_none());
    assert_eq!(
        reconciliation
            .receipt_for(
                "surface:main",
                &WorthUiDurableStateFamilyId::SplitterPosition
            )
            .expect("splitter replacement receipt exists")
            .outcome(),
        crate::runtime::WorthUiDurableStateReconciliationOutcome::Drop
    );
}

#[test]
fn ordinary_planning_consumes_runtime_resize_witness_instead_of_legacy_support_marker() {
    let (measurement_basis, graph_snapshot, selected) =
        crate::runtime::tests::allocation_planning_test_support::admitted_planning_admission(
            "runtime-durable-resize-split",
            "operator:split",
        );
    let neighborhood = selected
        .admit_allocation_neighborhood(&graph_snapshot, &measurement_basis)
        .expect("selected graph touch admits the preliminary split neighborhood");
    let authored_provenance_digest = neighborhood
        .members()
        .iter()
        .find(|member| {
            matches!(
                member.role(),
                crate::evidence::UiAllocationNeighborhoodMemberRole::Root
            )
        })
        .expect("graph-admitted split neighborhood has a root")
        .authored_provenance_digest();
    let (runtime, pending, _) =
        splitter_pending_activation_with_provenance(authored_provenance_digest);

    let direct_constraints = measurement_basis
        .admit_allocation_constraint_set(&neighborhood)
        .expect("split neighborhood should still admit without minting durable resize authority");
    assert!(
        direct_constraints.propagation_edges().iter().all(|edge| {
            edge.family() != crate::evidence::UiConstraintPropagationEdgeFamily::DurableResizeInput
        }),
        "legacy graph support must not manufacture durable resize authority on its own"
    );

    let input = runtime
        .admit_planning_lane_input(&pending, &graph_snapshot, measurement_basis, &selected)
        .expect("canonical admission augments and re-admits durable resize truth");
    let planning = runtime.plan_allocation(input);
    assert!(planning.is_admitted());
    let constraints = planning
        .allocation_constraint_set()
        .expect("planning should preserve admitted constraint set");
    let durable_edge = constraints
        .propagation_edges()
        .iter()
        .find(|edge| {
            edge.family() == crate::evidence::UiConstraintPropagationEdgeFamily::DurableResizeInput
        })
        .expect("runtime witness should mint durable resize edge on ordinary lane");

    match durable_edge.payload() {
        crate::evidence::UiConstraintPropagationEdgePayload::DurableResizeInput {
            planning_time_only,
            ..
        } => assert!(planning_time_only),
        other => panic!("expected durable resize payload, got {other:?}"),
    }
}

pub(crate) fn splitter_pending_activation_with_provenance(
    authored_provenance_digest: u64,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    crate::runtime::WorthUiPendingActivation,
    u64,
) {
    let app = identity_match_app();
    let active = artifact_from_nodes([(
        "app/main.wui",
        vec![
            component_node("component:dashboard", 0),
            splitter_surface_node_with_authored_provenance_digest(
                "surface:main",
                "workspace.surface.main",
                "workspace.sizing.splitter.main",
                1,
                authored_provenance_digest,
            ),
        ],
    )]);
    let candidate = active.clone();
    let (runtime, admitted, narrowing) = runtime_and_narrowing(&app, active, candidate);
    let identity_report = runtime
        .build_identity_match_graph(&narrowing, &admitted)
        .expect("identity graph builds");
    let impact = no_op_impact_for(&identity_report);
    let narrowed = narrowing_for(&identity_report);
    let node_plan = runtime
        .classify_node_replacements(&impact, &narrowed, &identity_report)
        .expect("node replacement plan builds");
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &narrowed, &admitted)
        .expect("query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(&query_comparison, &node_plan, &narrowed, &admitted)
        .expect("query rebind plan succeeds");
    let lowering_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    let pending = runtime
        .stage_replacement_activation(
            admitted,
            &impact,
            &narrowed,
            &node_plan,
            crate::runtime::WorthUiActivationStagingPlans::new(
                Some(&reconciliation),
                Some(&query_rebind),
                Some(&lowering_input),
            ),
        )
        .expect("pending activation stages");
    let authored_provenance_digest = reconciliation
        .admitted_durable_resize_input("surface:main")
        .and_then(|input| input.authored_provenance_digest())
        .expect("admitted resize witness should preserve authored provenance");

    (runtime, pending, authored_provenance_digest)
}
