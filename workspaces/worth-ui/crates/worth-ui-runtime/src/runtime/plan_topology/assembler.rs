use crate::runtime::{
    UiCommittedAllocationLoweringInput, WorthUiExecutionPlan, WorthUiLaneAdmission,
    WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial, WorthUiRuntimeHandleAllocation,
};

use super::assembly::construct_execution_plan;
use super::validation::{
    verify_child_range_handles, verify_handle_allocation_receipt, verify_lane_admission,
    verify_runtime_handles,
};

pub(crate) struct WorthUiPlanTopologyAssembler;

impl WorthUiPlanTopologyAssembler {
    pub(crate) fn assemble_with_lane_admission(
        lowering_input: &UiCommittedAllocationLoweringInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        let mut counters = WorthUiPlanTopologyCounters::default();
        let allocation_receipt = lowering_input.receipt();
        let committed_allocation = allocation_receipt.committed_allocation();
        let node_inputs = committed_allocation.node_inputs();
        verify_handle_allocation_receipt(committed_allocation, handle_allocation, &mut counters)?;
        verify_lane_admission(
            node_inputs,
            lane_admission,
            crate::runtime::WorthUiRuntimeHandleAllocationBasis::from_committed_allocation(
                committed_allocation,
            )
            .digest(),
            &mut counters,
        )?;
        verify_runtime_handles(node_inputs, handle_allocation, &mut counters)?;
        verify_child_range_handles(node_inputs, handle_allocation, &mut counters)?;
        construct_execution_plan(node_inputs, handle_allocation, counters)
    }
}
