use super::{WorthUiVirtualizedDataFrameExecutor, WorthUiVirtualizedDataPlanBuilder};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiVirtualizedDataFrameDenial,
    WorthUiVirtualizedDataFrameDenialReason, WorthUiVirtualizedDataFrameReceipt,
    WorthUiVirtualizedDataFrameTarget, WorthUiVirtualizedDataPlan,
    WorthUiVirtualizedDataPlanDenial, WorthUiVirtualizedDataPlanDenialReason,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiVirtualizedPlanAvailability {
    Executable,
    QueryFree,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiActiveVirtualizedDataPlanPosture {
    Executable(Box<WorthUiVirtualizedDataPlan>),
    QueryFree,
}

impl WorthUiActiveVirtualizedDataPlanPosture {
    pub(crate) fn lower(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<Self, WorthUiVirtualizedDataPlanDenial> {
        if execution_plan
            .regional_family_count(crate::runtime::WorthUiPlanNodeInputFamily::QueryViewBinding)
            == 0
        {
            return Ok(Self::QueryFree);
        }
        match WorthUiVirtualizedDataPlanBuilder::build(execution_plan, lane_admission) {
            Ok(plan) => Ok(Self::Executable(Box::new(plan))),
            Err(denial)
                if denial.reason()
                    == WorthUiVirtualizedDataPlanDenialReason::NoVirtualizedDataRows =>
            {
                Ok(Self::QueryFree)
            }
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn availability(&self) -> WorthUiVirtualizedPlanAvailability {
        match self {
            Self::Executable(_) => WorthUiVirtualizedPlanAvailability::Executable,
            Self::QueryFree => WorthUiVirtualizedPlanAvailability::QueryFree,
        }
    }

    pub(crate) fn execute(
        &self,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        target: WorthUiVirtualizedDataFrameTarget,
    ) -> Result<WorthUiVirtualizedDataFrameReceipt, WorthUiVirtualizedDataFrameDenial> {
        match self {
            Self::Executable(plan) => {
                WorthUiVirtualizedDataFrameExecutor::execute(plan, query_binding, target)
            }
            Self::QueryFree => Err(WorthUiVirtualizedDataFrameDenial::new(
                WorthUiVirtualizedDataFrameDenialReason::ActivePlanIsQueryFree,
                Some(target.handle().plan_index()),
                Default::default(),
            )),
        }
    }

    pub(crate) fn summary(
        &self,
        query_binding: &worth_ui_query_binding::WorthUiRuntimeQueryBinding,
        request: crate::runtime::WorthUiVirtualizedPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiVirtualizedPlanSummary,
        crate::runtime::WorthUiVirtualizedPlanSummaryDenial,
    > {
        match self {
            Self::Executable(plan) => crate::runtime::WorthUiVirtualizedPlanSummary::from_plan(
                plan,
                query_binding,
                request,
            ),
            Self::QueryFree => {
                Err(crate::runtime::WorthUiVirtualizedPlanSummaryDenial::ActivePlanIsQueryFree)
            }
        }
    }
}
