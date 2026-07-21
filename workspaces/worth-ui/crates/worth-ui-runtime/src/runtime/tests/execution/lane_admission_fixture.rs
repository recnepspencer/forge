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
    let plan_input = inputs
        .runtime
        .prepare_reconstructive_plan_input_for_test(&inputs.admitted, &[]);
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let planning = allocation_planning(&runtime, &pending, "lane-admission.fixture");
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input.clone());
    let allocation = runtime
        .allocate_runtime_handles(&facts)
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
    let plan_input = plan_input_with_spoofed_query_lane_identity(&runtime, &pending);
    let planning = allocation_planning(&runtime, &pending, "lane-admission.spoofed-query");
    (runtime, plan_input, planning)
}

pub(super) fn admit_lanes(
    runtime: &WorthUiRuntime,
    planning: &UiAllocationCandidate,
    plan_input: &crate::runtime::WorthUiExecutionPlanInput,
    support: &WorthUiExecutionLaneSupport,
) -> WorthUiLaneAdmission {
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(planning, plan_input.clone());
    runtime
        .admit_execution_lanes(&facts, support)
        .expect("lane admission succeeds")
}

fn plan_input_with_spoofed_query_lane_identity(
    runtime: &WorthUiRuntime,
    pending: &crate::runtime::WorthUiPendingActivation,
) -> crate::runtime::WorthUiExecutionPlanInput {
    let hook = WorthUiComponentLoweringHook::registered(
        "component.local.spoofed_query_lane",
        WorthUiPlanNodeInputFamily::QueryViewBinding,
    );
    runtime
        .prepare_execution_plan_input_with_component_hooks_for_test(pending, &[hook])
        .expect("spoofed query-lane input prepares before lane admission")
}
