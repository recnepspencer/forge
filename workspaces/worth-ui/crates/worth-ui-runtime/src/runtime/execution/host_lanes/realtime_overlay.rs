use crate::runtime::{
    WorthUiFrameworkTurnExecution, WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameReceipt,
    WorthUiRealtimeFrameTarget,
};

impl WorthUiFrameworkTurnExecution<'_> {
    pub(crate) fn execute_active_realtime_frame(
        &self,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .execute_realtime(target)
    }
}
