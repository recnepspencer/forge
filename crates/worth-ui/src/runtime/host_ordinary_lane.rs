use crate::runtime::host::WorthUiRuntimeHost;
use crate::runtime::ordinary_lane::{
    WorthUiOrdinaryLaneFrameExecutor, WorthUiOrdinaryLanePlanBuilder,
};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiOrdinaryFrameTarget,
    WorthUiOrdinaryLaneFrameDenial, WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLanePlan,
    WorthUiOrdinaryLanePlanDenial,
};

impl WorthUiRuntimeHost {
    pub fn prepare_ordinary_lane_plan(
        &self,
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial> {
        WorthUiOrdinaryLanePlanBuilder::build(execution_plan, lane_admission)
    }

    pub fn execute_ordinary_lane_frame(
        &self,
        ordinary_plan: &WorthUiOrdinaryLanePlan,
        target: WorthUiOrdinaryFrameTarget,
    ) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
        WorthUiOrdinaryLaneFrameExecutor::execute(ordinary_plan, target)
    }
}
