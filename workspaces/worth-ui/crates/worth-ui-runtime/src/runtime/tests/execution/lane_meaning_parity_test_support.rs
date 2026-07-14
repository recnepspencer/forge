use super::activation_staging_test_support::{activation_staging_inputs, ActivationStagingInputs};
use super::allocation_planning_test_support::allocation_planning;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiNodeLifecycleTransition, WorthUiNodeReplacementClassification,
    WorthUiNodeReplacementCounters, WorthUiNodeReplacementPlan, WorthUiPlanExecutionLane,
    WorthUiPlanLanePartition, WorthUiPlanNode, WorthUiPlanNodeInputFamily, WorthUiPlanTopology,
    WorthUiQueryBindingComparison, WorthUiQueryLiveRebindPlan, WorthUiRuntime,
    WorthUiRuntimeImpactNarrowing,
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
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let active_plan = assemble_plan_from_pending_activation(&runtime, pending);
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
    let mut traversal_order = plan.topology().traversal_order().to_vec();
    let node = traversal_order
        .first()
        .expect("fixture includes a plan node")
        .clone();
    traversal_order[0] = WorthUiPlanNode::new(
        node.runtime_handle(),
        crate::runtime::WorthUiPlanNodeFamily::from_input_family(
            WorthUiPlanNodeInputFamily::Command,
        ),
        node.child_range(),
        node.region_structure(),
        node.egui_boundary().cloned(),
        node.render_resource_ref(),
    );
    WorthUiExecutionPlan::new(
        plan.handle_receipt(),
        WorthUiPlanTopology::new(traversal_order, plan.topology().child_ranges().to_vec()),
        plan.lane_partitions().to_vec(),
        plan.lookup_index().clone(),
        plan.counters(),
    )
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
                classification.identity_basis().to_owned(),
                classification.authored_provenance_digest(),
                transition,
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

fn assemble_plan_from_pending_activation(
    runtime: &WorthUiRuntime,
    pending: crate::runtime::WorthUiPendingActivation,
) -> WorthUiExecutionPlan {
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "lane-meaning.active");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handles allocate");
    runtime
        .assemble_execution_plan_topology(
            &runtime.detached_allocation_receipt_for_test(&planning),
            &allocation,
        )
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
    WorthUiExecutionPlan::new(
        plan.handle_receipt(),
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
