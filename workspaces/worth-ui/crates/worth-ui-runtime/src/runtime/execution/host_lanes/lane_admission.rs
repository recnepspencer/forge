use crate::runtime::lane_admission::{
    WorthUiExtensionHookAdmissionPlanner, WorthUiLaneAdmissionPlanner,
};
use crate::runtime::plan_topology::WorthUiPlanTopologyAssembler;
use crate::runtime::WorthUiRuntime;
use crate::runtime::{
    UiCommittedAllocationLoweringInput, WorthUiExecutionLaneSupport, WorthUiExecutionPlan,
    WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook, WorthUiLaneAdmission,
    WorthUiLaneAdmissionDenial, WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeHandleAllocation, WorthUiUnsupportedHookDenial,
};

impl WorthUiRuntime {
    pub fn admit_execution_lanes(
        &self,
        lowering_input: &UiCommittedAllocationLoweringInput,
        support: &WorthUiExecutionLaneSupport,
    ) -> Result<WorthUiLaneAdmission, WorthUiLaneAdmissionDenial> {
        WorthUiLaneAdmissionPlanner::admit(lowering_input.receipt().committed_allocation(), support)
    }

    pub fn admit_extension_hook(
        &self,
        lane_admission: &WorthUiLaneAdmission,
        hook: WorthUiLaneAdapterHook,
    ) -> Result<WorthUiExtensionHookAdmission, WorthUiUnsupportedHookDenial> {
        WorthUiExtensionHookAdmissionPlanner::admit(lane_admission, hook)
    }

    pub fn assemble_execution_plan_topology(
        &self,
        lowering_input: &UiCommittedAllocationLoweringInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        let lane_admission = WorthUiLaneAdmissionPlanner::admit(
            lowering_input.receipt().committed_allocation(),
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .map_err(|_| {
            WorthUiPlanTopologyDenial::new(
                WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
                Default::default(),
            )
        })?;
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            lowering_input,
            handle_allocation,
            &lane_admission,
        )
    }

    pub fn assemble_execution_plan_topology_with_lane_admission(
        &self,
        lowering_input: &UiCommittedAllocationLoweringInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            lowering_input,
            handle_allocation,
            lane_admission,
        )
    }
}
