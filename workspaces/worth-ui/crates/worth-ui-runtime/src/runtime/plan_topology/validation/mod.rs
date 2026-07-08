use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiExecutionLaneDescriptor, WorthUiLaneAdmission,
    WorthUiPlanNodeInput, WorthUiPlanNodeInputFamily, WorthUiPlanTopologyCounters,
    WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason, WorthUiRuntimeHandleAllocation,
    WorthUiRuntimeHandleAllocationBasis,
};

pub(crate) fn verify_handle_allocation_receipt(
    allocation_planning: &WorthUiAllocationPlanning,
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    let basis = WorthUiRuntimeHandleAllocationBasis::from_allocation_planning(allocation_planning);
    if handle_allocation.receipt().certifies_basis(&basis) {
        Ok(())
    } else {
        Err(denial(
            WorthUiPlanTopologyDenialReason::HandleAllocationReceiptMismatch,
            *counters,
        ))
    }
}

pub(crate) fn verify_lane_admission(
    node_inputs: &[WorthUiPlanNodeInput],
    lane_admission: &WorthUiLaneAdmission,
    expected_basis_digest: u64,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    if lane_admission.plan_input_basis_digest() != expected_basis_digest {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
            *counters,
        ));
    }
    if lane_admission.counters().plan_node_count() != node_inputs.len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
            *counters,
        ));
    }
    for node_input in node_inputs {
        let descriptor = WorthUiExecutionLaneDescriptor::from_node_input(node_input);
        if !lane_admission.includes_lane(descriptor.lane()) {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
                *counters,
            ));
        }
    }
    Ok(())
}

pub(crate) fn verify_runtime_handles(
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    if handle_allocation.runtime_handles().len() != node_inputs.len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingRuntimeHandle,
            *counters,
        ));
    }
    let generation = handle_allocation.receipt().plan_generation();
    for (position, (node_input, runtime_handle)) in node_inputs
        .iter()
        .zip(handle_allocation.runtime_handles())
        .enumerate()
    {
        let plan_index = u32::try_from(position).map_err(|_| {
            denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                *counters,
            )
        })?;
        if runtime_handle.plan_index() != plan_index
            || runtime_handle.plan_generation() != generation
        {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                *counters,
            ));
        }
        if runtime_handle.family() != node_input.family() {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::RuntimeHandleFamilyMismatch,
                *counters,
            ));
        }
    }
    Ok(())
}

pub(crate) fn verify_child_range_handles(
    node_inputs: &[WorthUiPlanNodeInput],
    handle_allocation: &WorthUiRuntimeHandleAllocation,
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<(), WorthUiPlanTopologyDenial> {
    counters.record_validation();
    let generation = handle_allocation.receipt().plan_generation();
    let expected_child_range_plan_indexes =
        expected_child_range_plan_indexes(node_inputs, counters)?;
    if expected_child_range_plan_indexes.len() != handle_allocation.child_range_handles().len() {
        return Err(denial(
            WorthUiPlanTopologyDenialReason::MissingChildOrLaneLink,
            *counters,
        ));
    }
    for (expected_plan_index, handle) in expected_child_range_plan_indexes
        .iter()
        .zip(handle_allocation.child_range_handles())
    {
        let plan_index = handle.plan_index() as usize;
        let Some(node_input) = node_inputs.get(plan_index) else {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::OrphanedChildRangeHandle,
                *counters,
            ));
        };
        if handle.plan_index() != *expected_plan_index
            || handle.plan_generation() != generation
            || node_input.family() != WorthUiPlanNodeInputFamily::ChildRange
        {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::OrphanedChildRangeHandle,
                *counters,
            ));
        }
    }
    Ok(())
}

fn expected_child_range_plan_indexes(
    node_inputs: &[WorthUiPlanNodeInput],
    counters: &mut WorthUiPlanTopologyCounters,
) -> Result<Vec<u32>, WorthUiPlanTopologyDenial> {
    node_inputs
        .iter()
        .enumerate()
        .filter_map(|(position, node_input)| {
            matches!(node_input.family(), WorthUiPlanNodeInputFamily::ChildRange)
                .then_some(position)
        })
        .map(|position| {
            u32::try_from(position).map_err(|_| {
                denial(
                    WorthUiPlanTopologyDenialReason::RuntimeHandleOutOfBounds,
                    *counters,
                )
            })
        })
        .collect()
}

pub(crate) fn denial(
    reason: WorthUiPlanTopologyDenialReason,
    counters: WorthUiPlanTopologyCounters,
) -> WorthUiPlanTopologyDenial {
    WorthUiPlanTopologyDenial::new(reason, counters)
}