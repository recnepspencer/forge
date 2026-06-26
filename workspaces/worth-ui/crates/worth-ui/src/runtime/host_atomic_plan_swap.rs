use crate::runtime::atomic_plan_swap::WorthUiAtomicPlanSwap;
#[cfg(test)]
use crate::runtime::atomic_plan_swap::WorthUiPlanSwapFailureInjection;
use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiFrameBoundary, WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback,
    WorthUiReadyActivation,
};

impl WorthUiRuntimeHost {
    pub fn swap_ready_activation_at_frame_boundary(
        &mut self,
        ready_activation: WorthUiReadyActivation,
        candidate_plan: WorthUiExecutionPlan,
        boundary: WorthUiFrameBoundary,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback> {
        let runtime_frame_epoch = self.frame_epoch();
        self.record_last_valid_from_active_for_swap();
        WorthUiAtomicPlanSwap::swap(
            self.active_state_for_swap_mut(),
            ready_activation,
            candidate_plan,
            boundary,
            runtime_frame_epoch,
        )
    }

    #[cfg(test)]
    pub(crate) fn swap_ready_activation_at_frame_boundary_with_injection_for_test(
        &mut self,
        ready_activation: WorthUiReadyActivation,
        candidate_plan: WorthUiExecutionPlan,
        boundary: WorthUiFrameBoundary,
        injection: WorthUiPlanSwapFailureInjection,
    ) -> Result<WorthUiPlanSwapReceipt, WorthUiPlanSwapRollback> {
        let runtime_frame_epoch = self.frame_epoch();
        self.record_last_valid_from_active_for_swap();
        WorthUiAtomicPlanSwap::swap_with_injection(
            self.active_state_for_swap_mut(),
            ready_activation,
            candidate_plan,
            boundary,
            runtime_frame_epoch,
            injection,
        )
    }
}
