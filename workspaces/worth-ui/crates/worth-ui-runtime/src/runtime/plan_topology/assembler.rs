use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiExecutionPlan, WorthUiLaneAdmission,
    WorthUiPlanTopologyCounters, WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeHandleAllocation,
};

use super::assembly::construct_execution_plan;
use super::validation::{
    denial, verify_child_range_handles, verify_handle_allocation_receipt, verify_lane_admission,
    verify_runtime_handles,
};

pub(crate) struct WorthUiPlanTopologyAssembler;

impl WorthUiPlanTopologyAssembler {
    pub(crate) fn assemble_with_lane_admission(
        allocation_planning: &WorthUiAllocationPlanning,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        let mut counters = WorthUiPlanTopologyCounters::default();
        if !allocation_planning.is_admitted() {
            return Err(denial(
                WorthUiPlanTopologyDenialReason::AllocationPlanningDenied,
                counters,
            ));
        }
        let node_inputs = allocation_planning
            .node_inputs()
            .expect("admitted allocation planning must expose lowered node inputs");
        verify_handle_allocation_receipt(allocation_planning, handle_allocation, &mut counters)?;
        verify_lane_admission(
            node_inputs,
            lane_admission,
            handle_allocation.receipt().basis_digest(),
            &mut counters,
        )?;
        verify_runtime_handles(node_inputs, handle_allocation, &mut counters)?;
        verify_child_range_handles(node_inputs, handle_allocation, &mut counters)?;
        construct_execution_plan(node_inputs, handle_allocation, counters)
    }
}