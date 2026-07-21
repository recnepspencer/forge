use crate::runtime::execution::lane_admission::WorthUiExtensionHookAdmissionPlanner;
#[cfg(test)]
use crate::runtime::execution::lane_admission::WorthUiLaneAdmissionPlanner;
use crate::runtime::planning::plan_topology::WorthUiPlanTopologyAssembler;
use crate::runtime::planning::WorthUiExecutionPlanLoweringFacts;
use crate::runtime::WorthUiRuntime;
#[cfg(test)]
use crate::runtime::{WorthUiExecutionLaneSupport, WorthUiLaneAdmissionDenial};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiExtensionHookAdmission, WorthUiLaneAdapterHook,
    WorthUiLaneAdmission, WorthUiPlanTopologyDenial, WorthUiRuntimeHandleAllocation,
    WorthUiUnsupportedHookDenial,
};

impl WorthUiRuntime {
    #[cfg(test)]
    pub(crate) fn admit_execution_lanes(
        &self,
        authority: &WorthUiExecutionPlanLoweringFacts,
        support: &WorthUiExecutionLaneSupport,
    ) -> Result<WorthUiLaneAdmission, WorthUiLaneAdmissionDenial> {
        WorthUiLaneAdmissionPlanner::admit(authority, support)
    }

    pub fn admit_extension_hook(
        &self,
        lane_admission: &WorthUiLaneAdmission,
        hook: WorthUiLaneAdapterHook,
    ) -> Result<WorthUiExtensionHookAdmission, WorthUiUnsupportedHookDenial> {
        WorthUiExtensionHookAdmissionPlanner::admit(lane_admission, hook)
    }

    #[cfg(test)]
    pub(crate) fn assemble_execution_plan_topology(
        &self,
        authority: &WorthUiExecutionPlanLoweringFacts,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        self.assemble_execution_plan_topology_with_admission(authority, handle_allocation)
            .map(|(plan, _)| plan)
    }

    pub(crate) fn assemble_execution_plan_topology_with_admission(
        &self,
        authority: &WorthUiExecutionPlanLoweringFacts,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
    ) -> Result<(WorthUiExecutionPlan, WorthUiLaneAdmission), WorthUiPlanTopologyDenial> {
        if authority.region_delta().is_none() {
            return WorthUiPlanTopologyAssembler::assemble_from_authority_with_lane_admission(
                authority,
                handle_allocation,
            );
        }
        let proof = self
            .active
            .predecessor_region_proof(authority)
            .map_err(|_| {
                WorthUiPlanTopologyDenial::new(
                    crate::runtime::WorthUiPlanTopologyDenialReason::RegionalSuccessorMismatch,
                    Default::default(),
                )
            })?;
        WorthUiPlanTopologyAssembler::assemble_successor_from_authority_with_lane_admission(
            authority,
            handle_allocation,
            proof,
        )
    }

    #[cfg(test)]
    pub(crate) fn assemble_execution_plan_topology_with_lane_admission(
        &self,
        authority: &WorthUiExecutionPlanLoweringFacts,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiExecutionPlan, WorthUiPlanTopologyDenial> {
        WorthUiPlanTopologyAssembler::assemble_with_lane_admission(
            authority,
            handle_allocation,
            lane_admission,
        )
    }
}
