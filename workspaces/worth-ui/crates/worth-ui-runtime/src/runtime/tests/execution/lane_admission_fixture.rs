use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::allocation_planning;
use crate::runtime::{
    UiAllocationCandidate, WorthUiComponentLoweringHook, WorthUiExecutionLaneSupport,
    WorthUiLaneAdmission, WorthUiPlanNodeInputFamily, WorthUiRuntime, WorthUiRuntimeFrameworkLoop,
    WorthUiRuntimeHandleAllocation,
};

pub(super) fn lane_fixture() -> (
    WorthUiRuntimeFrameworkLoop,
    crate::runtime::WorthUiExecutionPlanInput,
    UiAllocationCandidate,
    WorthUiRuntimeHandleAllocation,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(pending)
        .expect("plan input prepares");
    let planning = allocation_planning(&runtime, &plan_input, "lane-admission.fixture");
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handles allocate");
    (runtime, plan_input, planning, allocation)
}

pub(super) fn spoofed_query_lane_fixture() -> (
    WorthUiRuntimeFrameworkLoop,
    crate::runtime::WorthUiExecutionPlanInput,
    UiAllocationCandidate,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = plan_input_with_spoofed_query_lane_identity(&runtime, pending);
    let planning = allocation_planning(&runtime, &plan_input, "lane-admission.spoofed-query");
    (runtime, plan_input, planning)
}

pub(super) fn admit_lanes(
    runtime: &WorthUiRuntime,
    planning: &UiAllocationCandidate,
    support: &WorthUiExecutionLaneSupport,
) -> WorthUiLaneAdmission {
    let lowering_input = runtime.detached_allocation_lowering_input_for_test(planning);
    runtime
        .admit_execution_lanes(&lowering_input, support)
        .expect("lane admission succeeds")
}

fn plan_input_with_spoofed_query_lane_identity(
    runtime: &WorthUiRuntime,
    pending: crate::runtime::WorthUiPendingActivation,
) -> crate::runtime::WorthUiExecutionPlanInput {
    let hook = WorthUiComponentLoweringHook::registered(
        "component.local.spoofed_query_lane",
        WorthUiPlanNodeInputFamily::QueryViewBinding,
    );
    runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(pending, &[hook])
        .expect("spoofed query-lane input prepares before lane admission")
}
