use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{admitted_planning_admission, allocation_planning};
use crate::runtime::{
    UiAllocationCandidate, WorthUiChildRangeHandle, WorthUiExecutionPlanInput,
    WorthUiPlanNodeInputFamily, WorthUiRuntimeHandleAllocation,
};

pub(super) fn topology_fixture() -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
    UiAllocationCandidate,
    WorthUiRuntimeHandleAllocation,
) {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let (measurement_basis, snapshot, selected) =
        admitted_planning_admission("plan-topology.fixture", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("topology fixture planning admits through graph authority"),
    );
    let allocation = runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(&planning))
        .expect("handles allocate");
    (runtime, plan_input, planning, allocation)
}

pub(super) fn topology_planning(
    plan_input: &WorthUiExecutionPlanInput,
    label: &str,
) -> UiAllocationCandidate {
    let (runtime, _, _, _) = topology_fixture();
    allocation_planning(&runtime, plan_input, label)
}

pub(super) fn allocate_handles(planning: &UiAllocationCandidate) -> WorthUiRuntimeHandleAllocation {
    let runtime = fresh_runtime();
    runtime
        .allocate_runtime_handles(&runtime.detached_allocation_receipt_for_test(planning))
        .expect("handles allocate")
}

pub(super) fn assemble(
    planning: &UiAllocationCandidate,
    allocation: &WorthUiRuntimeHandleAllocation,
) -> crate::runtime::WorthUiExecutionPlan {
    let runtime = fresh_runtime();
    runtime
        .assemble_execution_plan_topology(
            &runtime.detached_allocation_lowering_input_for_test(planning),
            allocation,
        )
        .expect("topology assembles")
}

pub(super) fn assemble_err(
    planning: &UiAllocationCandidate,
    allocation: &WorthUiRuntimeHandleAllocation,
) -> crate::runtime::WorthUiPlanTopologyDenial {
    let runtime = fresh_runtime();
    runtime
        .assemble_execution_plan_topology(
            &runtime.detached_allocation_lowering_input_for_test(planning),
            allocation,
        )
        .expect_err("topology assembly denies")
}

pub(super) fn plan_input_without_first_egui_boundary(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    replace_first_matching_input(
        plan_input,
        |input| {
            matches!(
                input.family(),
                WorthUiPlanNodeInputFamily::ComponentInvocation
                    | WorthUiPlanNodeInputFamily::LayoutRegion
                    | WorthUiPlanNodeInputFamily::QueryViewBinding
                    | WorthUiPlanNodeInputFamily::TokenStyle
                    | WorthUiPlanNodeInputFamily::DiagnosticsRef
                    | WorthUiPlanNodeInputFamily::EguiBoundaryRef
            )
        },
        |input| input.without_egui_boundary_for_test(),
    )
}

pub(super) fn plan_input_without_first_region_structure(
    plan_input: WorthUiExecutionPlanInput,
) -> WorthUiExecutionPlanInput {
    replace_first_matching_input(
        plan_input,
        |input| {
            matches!(
                input.family(),
                WorthUiPlanNodeInputFamily::ComponentInvocation
                    | WorthUiPlanNodeInputFamily::LayoutRegion
                    | WorthUiPlanNodeInputFamily::QueryViewBinding
            ) && input.topology_input().structure_declared()
        },
        |input| input.without_topology_input_for_test(),
    )
}

pub(super) fn allocation_with_runtime_handles(
    allocation: &WorthUiRuntimeHandleAllocation,
    runtime_handles: Vec<crate::runtime::WorthUiRuntimeHandle>,
) -> WorthUiRuntimeHandleAllocation {
    rebuilt_allocation(
        allocation,
        runtime_handles,
        allocation.child_range_handles().to_vec(),
    )
}

pub(super) fn allocation_with_child_ranges(
    allocation: &WorthUiRuntimeHandleAllocation,
    child_range_handles: Vec<WorthUiChildRangeHandle>,
) -> WorthUiRuntimeHandleAllocation {
    rebuilt_allocation(
        allocation,
        allocation.runtime_handles().to_vec(),
        child_range_handles,
    )
}

fn fresh_runtime() -> crate::runtime::WorthUiRuntimeFrameworkLoop {
    activation_staging_inputs().into_runtime_and_pending().0
}

fn replace_first_matching_input(
    plan_input: WorthUiExecutionPlanInput,
    matches_input: impl Fn(&crate::runtime::WorthUiPlanNodeInput) -> bool,
    replace: impl FnOnce(crate::runtime::WorthUiPlanNodeInput) -> crate::runtime::WorthUiPlanNodeInput,
) -> WorthUiExecutionPlanInput {
    let mut node_inputs = plan_input.node_inputs().to_vec();
    let index = node_inputs
        .iter()
        .position(matches_input)
        .expect("fixture includes the required topology input");
    node_inputs[index] = replace(node_inputs[index].clone());
    WorthUiExecutionPlanInput::new(
        plan_input.basis().clone(),
        plan_input.context().clone(),
        node_inputs,
        plan_input.counters(),
    )
}

fn rebuilt_allocation(
    allocation: &WorthUiRuntimeHandleAllocation,
    runtime_handles: Vec<crate::runtime::WorthUiRuntimeHandle>,
    child_range_handles: Vec<WorthUiChildRangeHandle>,
) -> WorthUiRuntimeHandleAllocation {
    WorthUiRuntimeHandleAllocation::new(
        crate::runtime::execution::handle_allocation::WorthUiRuntimeHandleAllocationInput {
            basis: allocation.basis().clone(),
            receipt: allocation.receipt(),
            family_widths: allocation.family_widths(),
            counters: allocation.counters(),
            runtime_handles,
            component_handles: allocation.component_handles().to_vec(),
            command_handles: allocation.command_handles().to_vec(),
            token_handles: allocation.token_handles().to_vec(),
            child_range_handles,
            view_binding_handles: allocation.view_binding_handles().to_vec(),
            lane_handles: allocation.lane_handles().to_vec(),
            state_slot_handles: allocation.state_slot_handles().to_vec(),
        },
    )
}
