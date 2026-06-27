use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::virtualized_data_lane::{
    WorthUiVirtualizedDataFrameExecutor, WorthUiVirtualizedDataPlanBuilder,
};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiVirtualizedDataFrameDenial,
    WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameTarget,
    WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial,
};

impl WorthUiRuntimeHost {
    pub fn prepare_virtualized_data_plan(
        &self,
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiVirtualizedDataPlan, WorthUiVirtualizedDataPlanDenial> {
        WorthUiVirtualizedDataPlanBuilder::build(execution_plan, lane_admission)
    }

    pub fn execute_virtualized_data_frame(
        &self,
        data_plan: &WorthUiVirtualizedDataPlan,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        WorthUiVirtualizedDataFrameExecutor::execute(data_plan, target)
    }
}
