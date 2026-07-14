use super::activation_staging_test_support::activation_staging_inputs;
use super::durable_state_inventory_test_support::platform_inventory;
use crate::runtime::{
    WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiPendingActivation,
};

pub(super) struct LaneChangeActivationInputs {
    pub(super) runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pub(super) pending: WorthUiPendingActivation,
    pub(super) admitted_catalog: crate::graph::UiAdmittedAllocationCatalogBasisSet,
    pub(super) node_plan: WorthUiNodeReplacementPlan,
    pub(super) narrowing: crate::runtime::WorthUiRuntimeImpactNarrowing,
    pub(super) query_comparison: crate::runtime::WorthUiQueryBindingComparison,
    pub(super) query_rebind: crate::runtime::WorthUiQueryLiveRebindPlan,
}

pub(super) fn lane_change_activation_inputs() -> LaneChangeActivationInputs {
    let inputs = activation_staging_inputs();
    let runtime = inputs.runtime;
    let node_plan = lane_change_plan(&inputs.node_plan);
    let inventory = platform_inventory(&runtime)
        .build_for_replacement(&node_plan)
        .expect("inventory builds");
    let reconciliation = runtime
        .reconcile_durable_state(&node_plan, &inventory)
        .expect("state reconciliation succeeds");
    let query_comparison = runtime
        .compare_query_bindings(&node_plan, &inputs.narrowing, &inputs.admitted)
        .expect("query comparison succeeds");
    let query_rebind = runtime
        .plan_query_live_rebinds(
            &query_comparison,
            &node_plan,
            &inputs.narrowing,
            &inputs.admitted,
        )
        .expect("query rebind succeeds");
    let lowering_input = runtime.prepare_pending_execution_plan_lowering_input(
        &node_plan,
        &reconciliation,
        &query_rebind,
    );
    let pending = runtime
        .stage_replacement_activation(
            inputs.admitted,
            &inputs.impact,
            &inputs.narrowing,
            &node_plan,
            Some(&reconciliation),
            Some(&query_rebind),
            Some(&lowering_input),
        )
        .expect("lane-change activation stages");
    let (measurement_basis, graph_snapshot, selected_obligations) =
        crate::runtime::tests::allocation_planning_test_support::admitted_planning_admission(
            "frame-validation.lane-change",
            "operator:stack",
        );
    let admitted_catalog = graph_snapshot
        .admit_allocation_catalog_basis_set(vec![(measurement_basis, selected_obligations)])
        .expect("graph admits complete lane-change catalog");

    LaneChangeActivationInputs {
        runtime,
        pending,
        admitted_catalog,
        node_plan,
        narrowing: inputs.narrowing,
        query_comparison,
        query_rebind,
    }
}

fn lane_change_plan(plan: &WorthUiNodeReplacementPlan) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    let classifications = plan
        .classifications()
        .iter()
        .map(|classification| {
            counters.record_transition(WorthUiNodeLifecycleTransition::LaneChange);
            WorthUiNodeReplacementClassification::new(
                classification.identity_basis().to_owned(),
                classification.authored_provenance_digest(),
                WorthUiNodeLifecycleTransition::LaneChange,
                classification.active_kind(),
                classification.candidate_kind(),
                classification.active_durable_state_eligible(),
                classification.candidate_durable_state_eligible(),
                classification.active_resize_contract_id().cloned(),
                classification.candidate_resize_contract_id().cloned(),
                classification.active_resize_permission().cloned(),
                classification.candidate_resize_permission().cloned(),
                classification.active_resize_shape_digest(),
                classification.candidate_resize_shape_digest(),
            )
        })
        .collect();
    WorthUiNodeReplacementPlan::new(
        plan.active_artifact_digest(),
        plan.candidate_artifact_digest(),
        classifications,
        counters,
    )
}
