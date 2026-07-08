use crate::runtime::frame_activation_gate::WorthUiFrameActivationGate;
use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiActivationGateDenial, WorthUiActivationGateReceipt, WorthUiExecutionPlan,
    WorthUiExecutionPlanInput, WorthUiFrameBoundary, WorthUiLaneParityReport,
    WorthUiPendingActivation, WorthUiReadyActivation, WorthUiRuntimeHandleAllocation,
};

impl WorthUiRuntimeHost {
    pub fn safe_frame_boundary(&self) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::safe_to_activate(self.frame_epoch())
    }

    pub fn prepare_ready_activation(
        &self,
        pending_activation: WorthUiPendingActivation,
        plan_input: &WorthUiExecutionPlanInput,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        candidate_plan: &WorthUiExecutionPlan,
        lane_parity_report: Option<&WorthUiLaneParityReport>,
    ) -> Result<WorthUiReadyActivation, WorthUiActivationGateDenial> {
        WorthUiReadyActivation::prepare(
            pending_activation,
            plan_input,
            handle_allocation,
            candidate_plan,
            lane_parity_report,
        )
    }

    pub fn activate_ready_at_frame_boundary(
        &self,
        ready_activation: WorthUiReadyActivation,
        boundary: WorthUiFrameBoundary,
    ) -> Result<WorthUiActivationGateReceipt, WorthUiActivationGateDenial> {
        WorthUiFrameActivationGate::activate_at_boundary(
            self.inspect_active(),
            &ready_activation,
            boundary,
            self.frame_epoch(),
        )
    }

    #[cfg(test)]
    pub(crate) fn traversal_frame_boundary_for_test(&self) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::traversal_in_progress_for_test(self.frame_epoch())
    }

    #[cfg(test)]
    pub(crate) fn safe_frame_boundary_for_epoch_for_test(
        &self,
        frame_epoch: crate::runtime::WorthUiRuntimeFrameEpoch,
    ) -> WorthUiFrameBoundary {
        WorthUiFrameBoundary::safe_to_activate(frame_epoch)
    }
}
