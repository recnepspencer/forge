use crate::runtime::{
    WorthUiCanvasSpatialFrameDenial, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiFrameworkTurnExecution,
};

impl WorthUiFrameworkTurnExecution<'_> {
    pub(crate) fn execute_active_canvas_spatial_frame(
        &self,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        self.runtime
            .active
            .active_plan_ref()
            .execute_canvas_spatial(target)
    }
}
