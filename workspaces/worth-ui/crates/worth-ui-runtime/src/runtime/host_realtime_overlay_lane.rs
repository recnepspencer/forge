use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::realtime_overlay_lane::{WorthUiHudPlanBuilder, WorthUiRealtimeFrameExecutor};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiExtensionHookAdmission, WorthUiHighFrequencyFramePolicy,
    WorthUiHudPlan, WorthUiHudPlanDenial, WorthUiLaneAdmission, WorthUiRealtimeFrameDenial,
    WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameTarget, WorthUiRuntimeHandleAllocation,
};

impl WorthUiRuntimeHost {
    pub fn prepare_hud_plan(
        &self,
        execution_plan: &WorthUiExecutionPlan,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
        hook_admissions: &[WorthUiExtensionHookAdmission],
        frame_policy: WorthUiHighFrequencyFramePolicy,
    ) -> Result<WorthUiHudPlan, WorthUiHudPlanDenial> {
        WorthUiHudPlanBuilder::build(
            execution_plan,
            handle_allocation,
            lane_admission,
            hook_admissions,
            frame_policy,
        )
    }

    pub fn execute_realtime_frame(
        &self,
        hud_plan: &WorthUiHudPlan,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        WorthUiRealtimeFrameExecutor::execute(hud_plan, target)
    }
}
