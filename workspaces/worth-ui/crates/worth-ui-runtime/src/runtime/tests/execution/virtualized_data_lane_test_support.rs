use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use crate::runtime::{
    WorthUiExecutionLane, WorthUiExecutionLaneSupport, WorthUiExecutionPlan,
    WorthUiExecutionPlanInput, WorthUiLaneAdmission, WorthUiPlanNodeInputFamily,
    WorthUiQueryLaneSupportLinks, WorthUiRuntimeHandleAllocation, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial,
};

pub(super) fn virtualized_data_fixture() -> VirtualizedDataFixture {
    virtualized_data_context().into_fixture()
}

pub(super) fn virtualized_data_denial_for_missing_support(
    removed_lane: WorthUiExecutionLane,
) -> WorthUiVirtualizedDataPlanDenial {
    let context = virtualized_data_context();
    let admission = if removed_lane == WorthUiExecutionLane::QueryBound {
        admission_without_query_bound(&context)
    } else {
        let planning = allocation_planning(
            &context.runtime,
            &context.plan_input,
            "virtualized-data.missing-support",
        );
        let lowering_input = context
            .runtime
            .detached_allocation_lowering_input_for_test(&planning);
        context
            .runtime
            .admit_execution_lanes(
                &lowering_input,
                &WorthUiExecutionLaneSupport::without_lane_for_test(removed_lane),
            )
            .expect("narrower lane admission succeeds")
    };

    context
        .runtime
        .prepare_virtualized_data_plan(&context.execution_plan, &admission)
        .expect_err("virtualized data plan rejects missing support")
}

pub(super) fn virtualized_data_denial_for_stale_lane_admission() -> WorthUiVirtualizedDataPlanDenial
{
    let context = virtualized_data_context();
    let drifted_plan_input = plan_input_with_duplicate_query_input(&context.plan_input);
    let receipt_runtime = fresh_runtime();
    let drifted_planning = allocation_planning(
        &receipt_runtime,
        &drifted_plan_input,
        "virtualized-data.stale-admission",
    );
    let stale_admission = receipt_runtime
        .admit_execution_lanes(
            &receipt_runtime.detached_allocation_lowering_input_for_test(&drifted_planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("drifted lane admission still has data and Query support");

    context
        .runtime
        .prepare_virtualized_data_plan(&context.execution_plan, &stale_admission)
        .expect_err("virtualized data plan rejects stale lane admission")
}

pub(super) struct VirtualizedDataFixture {
    pub(super) runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    pub(super) data_plan: WorthUiVirtualizedDataPlan,
    pub(super) allocation: WorthUiRuntimeHandleAllocation,
    pub(super) query_links: Vec<WorthUiQueryLaneSupportLinks>,
}

struct VirtualizedDataContext {
    runtime: crate::runtime::WorthUiRuntimeFrameworkLoop,
    plan_input: WorthUiExecutionPlanInput,
    allocation: WorthUiRuntimeHandleAllocation,
    execution_plan: WorthUiExecutionPlan,
    lane_admission: WorthUiLaneAdmission,
    data_plan: WorthUiVirtualizedDataPlan,
}

impl VirtualizedDataContext {
    fn into_fixture(self) -> VirtualizedDataFixture {
        VirtualizedDataFixture {
            runtime: self.runtime,
            data_plan: self.data_plan,
            allocation: self.allocation,
            query_links: self.lane_admission.query_support_links().to_vec(),
        }
    }
}

fn virtualized_data_context() -> VirtualizedDataContext {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("execution plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "virtualized-data.fixture");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handle allocation succeeds");
    let lane_admission = runtime
        .admit_execution_lanes(
            &runtime.detached_allocation_lowering_input_for_test(&planning),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .expect("lane admission succeeds");
    let execution_plan = runtime
        .assemble_execution_plan_topology_with_lane_admission(
            &runtime.detached_allocation_lowering_input_for_test(&planning),
            &allocation,
            &lane_admission,
        )
        .expect("execution plan topology assembles");
    let data_plan = runtime
        .prepare_virtualized_data_plan(&execution_plan, &lane_admission)
        .expect("virtualized data plan prepares");

    assert!(data_plan.counters().data_plan_row_count() > 0);
    assert!(!lane_admission.query_support_links().is_empty());

    VirtualizedDataContext {
        runtime,
        plan_input,
        allocation,
        execution_plan,
        lane_admission,
        data_plan,
    }
}

fn admission_without_query_bound(context: &VirtualizedDataContext) -> WorthUiLaneAdmission {
    let no_query_plan_input = plan_input_without_family(
        &context.plan_input,
        WorthUiPlanNodeInputFamily::QueryViewBinding,
    );
    let receipt_runtime = fresh_runtime();
    let planning = allocation_planning(
        &receipt_runtime,
        &no_query_plan_input,
        "virtualized-data.without-query-bound",
    );
    let lowering_input = receipt_runtime.detached_allocation_lowering_input_for_test(&planning);
    receipt_runtime
        .admit_execution_lanes(
            &lowering_input,
            &WorthUiExecutionLaneSupport::without_lane_for_test(WorthUiExecutionLane::QueryBound),
        )
        .expect("query-free input can be admitted without QueryBound support")
}

fn fresh_runtime() -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    activation_staging_inputs().into_runtime_and_pending().0
}

fn plan_input_without_family(
    plan_input: &WorthUiExecutionPlanInput,
    family: WorthUiPlanNodeInputFamily,
) -> WorthUiExecutionPlanInput {
    let node_inputs = plan_input
        .node_inputs()
        .iter()
        .filter(|node_input| node_input.family() != family)
        .cloned()
        .collect::<Vec<_>>();
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn plan_input_with_duplicate_query_input(
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    let duplicated_query_input = plan_input
        .node_inputs()
        .iter()
        .find(|node_input| node_input.family() == WorthUiPlanNodeInputFamily::QueryViewBinding)
        .expect("fixture has a Query input to duplicate")
        .clone();
    let mut node_inputs = plan_input.node_inputs().to_vec();
    node_inputs.push(duplicated_query_input);
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}
