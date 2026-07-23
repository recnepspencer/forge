use crate::runtime::WorthUiFrameworkTurnExecution;
use crate::runtime::{
    WorthUiOrdinaryFrameTarget, WorthUiOrdinaryLaneFrameDenial, WorthUiOrdinaryLaneFrameReceipt,
};

impl WorthUiFrameworkTurnExecution<'_> {
    pub(crate) fn execute_active_ordinary_frame(
        &self,
        target: WorthUiOrdinaryFrameTarget,
    ) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .execute_ordinary(target)
    }

    pub(crate) fn active_plan_digest(&self) -> u64 {
        self.runtime.active.active_plan_ref().digest().as_u64()
    }

    pub(crate) fn active_frame_epoch(&self) -> crate::runtime::WorthUiRuntimeFrameEpoch {
        self.runtime.active.frame_epoch()
    }
}
