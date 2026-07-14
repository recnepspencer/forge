use crate::runtime::virtualized_data_lane::{
    WorthUiVirtualizedDataFrameExecutor, WorthUiVirtualizedDataPlanBuilder,
};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiVirtualizedDataFrameDenial,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial,
};
use crate::runtime::{WorthUiFrameworkTurnExecution, WorthUiRuntime};

impl WorthUiRuntime {
    pub fn prepare_virtualized_data_plan(
        &self,
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial> {
        WorthUiVirtualizedDataPlanBuilder::build(execution_plan, lane_admission)
    }

    #[cfg(test)]
    pub fn execute_virtualized_data_frame(
        &self,
        data_plan: &WorthUiVirtualizedDataPlan,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        WorthUiVirtualizedDataFrameExecutor::execute(data_plan, target)
    }
}

impl WorthUiFrameworkTurnExecution<'_> {
    pub fn execute_virtualized_data_frame(
        &self,
        data_plan: &WorthUiVirtualizedDataPlan,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        WorthUiVirtualizedDataFrameExecutor::execute(data_plan, target)
    }
}
