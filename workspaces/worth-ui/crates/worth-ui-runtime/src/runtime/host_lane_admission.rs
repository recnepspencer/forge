use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::lane_admission::{
    WorthUiExtensionHookAdmissionPlanner, WorthUiLaneAdmissionPlanner,
};
use crate::runtime::plan_topology::WorthUiPlanTopologyAssembler;
use crate::runtime::{
    WorthUiExecutionLaneSupport, WorthUiExecutionPlan, WorthUiExecutionPlanInput,
    WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook, WorthUiLaneAdmission,
    WorthUiLaneAdmissionDenial, WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeHandleAllocation, WorthUiUnsupportedHookDenial,
};

impl WorthUiRuntimeHost {
    pub fn admit_execution_lanes(
        &self,
        plan_input: &WorthUiExecutionPlanInput,
        support: &WorthUiExecutionLaneSupport,
    ) -> Result<WorthUiLaneAdmission, WorthUiLaneAdmissionDenial> {
        WorthUiLaneAdmissionPlanner::admit(plan_input, support)
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
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        let lane_admission = WorthUiLaneAdmissionPlanner::admit(
            plan_input,
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .map_err(|_| {
            WorthUiPlanTopologyDenial::new(
                WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
                Default::default(),
            )
        })?;
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            plan_input,
            handle_allocation,
            &lane_admission,
        )
    }

    pub fn assemble_execution_plan_topology_with_lane_admission(
        &self,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            plan_input,
            handle_allocation,
            lane_admission,
        )
    }
}
