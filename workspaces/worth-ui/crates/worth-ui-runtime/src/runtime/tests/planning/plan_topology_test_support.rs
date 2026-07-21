use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{admitted_planning_admission, allocation_planning};
use crate::runtime::planning::execution_plan_input::WorthUiExecutionPlanInputPreparer;
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
    topology_fixture_from_activation(inputs)
}

pub(super) fn topology_fixture_with_app(
    app: crate::facade::WorthUiApp,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
    UiAllocationCandidate,
    WorthUiRuntimeHandleAllocation,
) {
    topology_fixture_from_activation(
        super::activation_staging_test_support::activation_staging_inputs_for(app),
    )
}

fn topology_fixture_from_activation(
    inputs: super::activation_staging_test_support::ActivationStagingInputs,
) -> (
    crate::runtime::WorthUiRuntimeFrameworkLoop,
    WorthUiExecutionPlanInput,
    UiAllocationCandidate,
    WorthUiRuntimeHandleAllocation,
) {
    let plan_input = WorthUiExecutionPlanInputPreparer::prepare_launch(
        inputs.admitted.artifact_bundle().artifact(),
        inputs.admitted.artifact_bundle().artifact_digest(),
        inputs.runtime.frame_epoch(),
        inputs.app.prepared_authority().query_binding_plan(),
    );
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let (measurement_basis, snapshot, selected) =
        admitted_planning_admission("plan-topology.fixture", "operator:stack");
    let planning = runtime.plan_allocation(
        runtime
            .admit_planning_lane_input(&pending, &snapshot, measurement_basis, &selected)
            .expect("topology fixture planning admits through graph authority"),
    );
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(&planning, plan_input.clone());
    let allocation = runtime
        .allocate_runtime_handles(&facts)
        .expect("handles allocate");
    (runtime, plan_input, planning, allocation)
}

pub(super) fn topology_planning(label: &str) -> UiAllocationCandidate {
    let (runtime, pending) = activation_staging_inputs().into_runtime_and_pending();
    allocation_planning(&runtime, &pending, label)
}

pub(super) fn allocate_handles(
    planning: &UiAllocationCandidate,
    plan_input: &WorthUiExecutionPlanInput,
) -> WorthUiRuntimeHandleAllocation {
    let runtime = fresh_runtime();
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(planning, plan_input.clone());
    runtime
        .allocate_runtime_handles(&facts)
        .expect("handles allocate")
}

pub(super) fn assemble(
    planning: &UiAllocationCandidate,
    plan_input: &WorthUiExecutionPlanInput,
    allocation: &WorthUiRuntimeHandleAllocation,
) -> crate::runtime::WorthUiExecutionPlan {
    let runtime = fresh_runtime();
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(planning, plan_input.clone());
    runtime
        .assemble_execution_plan_topology(&facts, allocation)
        .expect("topology assembles")
}

pub(super) fn assemble_err(
    planning: &UiAllocationCandidate,
    plan_input: &WorthUiExecutionPlanInput,
    allocation: &WorthUiRuntimeHandleAllocation,
) -> crate::runtime::WorthUiPlanTopologyDenial {
    let runtime = fresh_runtime();
    let facts =
        runtime.detached_execution_plan_lowering_facts_for_test(planning, plan_input.clone());
    runtime
        .assemble_execution_plan_topology(&facts, allocation)
        .expect_err("topology assembly denies")
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
    rebuilt_allocation(allocation, runtime_handles)
}

pub(super) fn allocation_with_child_ranges(
    allocation: &WorthUiRuntimeHandleAllocation,
    child_range_handles: Vec<WorthUiChildRangeHandle>,
) -> WorthUiRuntimeHandleAllocation {
    let mut replacements = child_range_handles.into_iter();
    let mut runtime_handles = Vec::new();
    for handle in allocation.runtime_handles().iter().copied() {
        if handle.family() != WorthUiPlanNodeInputFamily::ChildRange {
            runtime_handles.push(handle);
        } else if let Some(replacement) = replacements.next() {
            runtime_handles.push(crate::runtime::WorthUiRuntimeHandle::new(
                WorthUiPlanNodeInputFamily::ChildRange,
                replacement.plan_index(),
                replacement.slot_generation(),
                replacement.arena_identity(),
            ));
        }
    }
    runtime_handles.extend(replacements.map(|replacement| {
        crate::runtime::WorthUiRuntimeHandle::new(
            WorthUiPlanNodeInputFamily::ChildRange,
            replacement.plan_index(),
            replacement.slot_generation(),
            replacement.arena_identity(),
        )
    }));
    rebuilt_allocation(allocation, runtime_handles)
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
) -> WorthUiRuntimeHandleAllocation {
    WorthUiRuntimeHandleAllocation::new(
        crate::runtime::execution::handle_allocation::WorthUiRuntimeHandleAllocationInput {
            basis: allocation.basis().clone(),
            receipt: allocation.receipt(),
            family_widths: allocation.family_widths(),
            counters: allocation.counters(),
            runtime_handles,
        },
    )
}
