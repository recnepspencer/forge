use crate::runtime::{
    WorthUiFrameworkTurnExecution, WorthUiVirtualizedDataFrameDenial,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
};

impl WorthUiFrameworkTurnExecution<'_> {
    pub(crate) fn execute_active_virtualized_data_frame(
        &self,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .execute_virtualized(&self.runtime.query_binding, target)
    }
}
