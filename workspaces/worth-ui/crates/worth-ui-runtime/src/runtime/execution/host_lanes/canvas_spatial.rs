use crate::runtime::canvas_spatial_lane::{
    WorthUiCanvasSpatialFrameExecutor, WorthUiCanvasSpatialPlanBuilder,
};
use crate::runtime::{
    WorthUiCanvasSpatialFrameDenial, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial,
    WorthUiExecutionPlan, WorthUiExtensionHookAdmission, WorthUiLaneAdmission,
    WorthUiRuntimeHandleAllocation,
};
use crate::runtime::{WorthUiFrameworkTurnExecution, WorthUiRuntime};

impl WorthUiRuntime {
    pub fn prepare_canvas_spatial_plan(
        &self,
        execution_plan: &WorthUiExecutionPlan,
        handle_allocation: &WorthUiRuntimeHandleAllocation,
        lane_admission: &WorthUiLaneAdmission,
        hook_admissions: &[WorthUiExtensionHookAdmission],
    ) -> Result<WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial> {
        WorthUiCanvasSpatialPlanBuilder::build(
            execution_plan,
            handle_allocation,
            lane_admission,
            hook_admissions,
        )
    }

    #[cfg(test)]
    pub fn execute_canvas_spatial_frame(
        &self,
        canvas_plan: &WorthUiCanvasSpatialPlan,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        WorthUiCanvasSpatialFrameExecutor::execute(canvas_plan, target)
    }
}

impl WorthUiFrameworkTurnExecution<'_> {
    pub fn execute_canvas_spatial_frame(
        &self,
        canvas_plan: &WorthUiCanvasSpatialPlan,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        WorthUiCanvasSpatialFrameExecutor::execute(canvas_plan, target)
    }
}
