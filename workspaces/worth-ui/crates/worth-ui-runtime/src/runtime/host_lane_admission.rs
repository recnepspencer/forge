use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::lane_admission::{
    WorthUiExtensionHookAdmissionPlanner, WorthUiLaneAdmissionPlanner,
};
use crate::runtime::plan_topology::WorthUiPlanTopologyAssembler;
use crate::runtime::{
    WorthUiAllocationPlanning, WorthUiExecutionLaneSupport, WorthUiExecutionPlan,
    WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook, WorthUiLaneAdmission,
    WorthUiLaneAdmissionDenial, WorthUiPlanTopologyDenial, WorthUiPlanTopologyDenialReason,
    WorthUiRuntimeHandleAllocation, WorthUiUnsupportedHookDenial,
};

impl WorthUiRuntimeHost {
    pub fn admit_execution_lanes(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
        support: &WorthUiExecutionLaneSupport,
    ) -> Result<WorthUiLaneAdmission, WorthUiLaneAdmissionDenial> {
        WorthUiLaneAdmissionPlanner::admit(allocation_planning, support)
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
        allocation_planning: &WorthUiAllocationPlanning,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        if !allocation_planning.is_admitted() {
            return Err(WorthUiPlanTopologyDenial::new(
                WorthUiPlanTopologyDenialReason::AllocationPlanningDenied,
                Default::default(),
            ));
        }
        let lane_admission = WorthUiLaneAdmissionPlanner::admit(
            allocation_planning,
            &WorthUiExecutionLaneSupport::platform_default(),
        )
        .map_err(|_| {
            WorthUiPlanTopologyDenial::new(
                WorthUiPlanTopologyDenialReason::LaneAdmissionMismatch,
                Default::default(),
            )
        })?;
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            allocation_planning,
            handle_allocation,
            &lane_admission,
        )
    }

    pub fn assemble_execution_plan_topology_with_lane_admission(
        &self,
        allocation_planning: &WorthUiAllocationPlanning,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        if !allocation_planning.is_admitted() {
            return Err(WorthUiPlanTopologyDenial::new(
                WorthUiPlanTopologyDenialReason::AllocationPlanningDenied,
                Default::default(),
            ));
        }
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            allocation_planning,
            handle_allocation,
            lane_admission,
        )
    }
}
