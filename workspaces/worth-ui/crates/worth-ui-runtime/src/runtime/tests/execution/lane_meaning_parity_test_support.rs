use super::activation_staging_test_support::{activation_staging_inputs, ActivationStagingInputs};
use super::allocation_planning_test_support::allocation_planning;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiPlanExecutionLane,
    WorthUiPlanLanePartition, WorthUiPlanNodeInputFamily, WorthUiQueryBindingComparison,
    WorthUiQueryLiveRebindPlan, WorthUiRuntime, WorthUiRuntimeImpactNarrowing,
};

pub(super) struct QueryPreservingLaneChangeFixture {
    pub(super) runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pub(super) narrowing: WorthUiRuntimeImpactNarrowing,
    pub(super) node_plan: WorthUiNodeReplacementPlan,
    pub(super) query_comparison: WorthUiQueryBindingComparison,
    pub(super) query_rebind_plan: WorthUiQueryLiveRebindPlan,
    pub(super) active_plan: WorthUiExecutionPlan,
    pub(super) candidate_plan: WorthUiExecutionPlan,
}

pub(super) fn query_preserving_lane_change_fixture() -> QueryPreservingLaneChangeFixture {
    let inputs = activation_staging_inputs();
    let query_comparison = compare_queries(&inputs);
    let node_plan = lane_change_plan(&inputs.node_plan);
    let narrowing = inputs.narrowing.clone();
    let query_rebind_plan = inputs.query_rebind_plan.clone();
    let plan_input = inputs
        .runtime
        .prepare_reconstructive_plan_input_for_test(&inputs.admitted, &[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let active_plan = assemble_plan_from_pending_activation(&runtime, pending, plan_input);
    let candidate_plan = plan_with_first_node_moved_to_next_lane(&active_plan);
    QueryPreservingLaneChangeFixture {
        runtime,
        narrowing,
        node_plan,
        query_comparison,
        query_rebind_plan,
        active_plan,
        candidate_plan,
    }
}

pub(super) fn plan_with_command_semantics_changed(
    plan: &WorthUiExecutionPlan,
) -> WorthUiExecutionPlan {
    plan.with_test_first_regional_family(WorthUiPlanNodeInputFamily::Command)
}

fn compare_queries(inputs: &ActivationStagingInputs) -> WorthUiQueryBindingComparison {
    inputs
        .runtime
        .compare_query_bindings(&inputs.node_plan, &inputs.narrowing, &inputs.admitted)
        .expect("query comparison succeeds")
}

fn lane_change_plan(plan: &WorthUiNodeReplacementPlan) -> WorthUiNodeReplacementPlan {
    let mut counters = WorthUiNodeReplacementCounters::default();
    let classifications = plan
        .classifications()
        .iter()
        .enumerate()
        .map(|(index, classification)| {
            let transition = if index == 0 {
                WorthUiNodeLifecycleTransition::LaneChange
            } else {
                classification.transition()
            };
            counters.record_transition(transition);
            WorthUiNodeReplacementClassification::new(
                crate::runtime::replacement::node_classification::WorthUiNodeReplacementClassificationInput {
                    identity_basis: classification.identity_basis().to_owned(),
                    authored_provenance_digest: classification.authored_provenance_digest(),
                    transition,
                    active_kind: classification.active_kind(),
                    candidate_kind: classification.candidate_kind(),
                    active_durable_state_eligible: classification.active_durable_state_eligible(),
                    candidate_durable_state_eligible: classification.candidate_durable_state_eligible(),
                    active_resize_contract_id: classification.active_resize_contract_id().cloned(),
                    candidate_resize_contract_id: classification.candidate_resize_contract_id().cloned(),
                    active_resize_permission: classification.active_resize_permission().cloned(),
                    candidate_resize_permission: classification.candidate_resize_permission().cloned(),
                    active_resize_shape_digest: classification.active_resize_shape_digest(),
                    candidate_resize_shape_digest: classification.candidate_resize_shape_digest(),
                },
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

fn assemble_plan_from_pending_activation(
    runtime: &WorthUiRuntime,
    pending: crate::runtime::WorthUiPendingActivation,
    plan_input: crate::runtime::WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlan {
    let planning = allocation_planning(runtime, &pending, "lane-meaning.active");
    let facts = runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input);
    let allocation = runtime
        .allocate_runtime_handles(&facts)
        .expect("handles allocate");
    runtime
        .assemble_execution_plan_topology(&facts, &allocation)
        .expect("topology assembles")
}

fn plan_with_first_node_moved_to_next_lane(plan: &WorthUiExecutionPlan) -> WorthUiExecutionPlan {
    let mut partitions = plan.lane_partitions().to_vec();
    let source = partitions
        .iter()
        .position(|partition| !partition.plan_indexes().is_empty())
        .expect("fixture has a populated lane");
    let current_lane = partitions[source].lane();
    let indexes = partitions[source].plan_indexes().to_vec();
    partitions[source] = WorthUiPlanLanePartition::new(alternate_lane(current_lane), indexes);
    plan.with_test_parts(
        plan.topology().clone(),
        partitions,
        plan.lookup_index().clone(),
        plan.counters(),
    )
}

fn alternate_lane(lane: WorthUiPlanExecutionLane) -> WorthUiPlanExecutionLane {
    match lane {
        WorthUiPlanExecutionLane::QueryView => WorthUiPlanExecutionLane::UiStructure,
        _ => WorthUiPlanExecutionLane::QueryView,
    }
}
