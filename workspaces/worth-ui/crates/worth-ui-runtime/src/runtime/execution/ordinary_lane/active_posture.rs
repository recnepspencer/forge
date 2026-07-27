use super::{WorthUiOrdinaryLaneFrameExecutor, WorthUiOrdinaryLanePlanBuilder};
use crate::runtime::{
    WorthUiExecutionPlan, WorthUiLaneAdmission, WorthUiOrdinaryFrameTarget,
    WorthUiOrdinaryLaneFrameDenial, WorthUiOrdinaryLaneFrameDenialReason,
    WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLanePlan, WorthUiOrdinaryLanePlanDenial,
    WorthUiOrdinaryLanePlanDenialReason,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiOrdinaryPlanAvailability {
    Executable,
    NonExecutableBootstrap,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiActiveOrdinaryPlanPosture {
    Executable(Box<WorthUiOrdinaryLanePlan>),
    NonExecutableBootstrap,
}

impl WorthUiActiveOrdinaryPlanPosture {
    pub(crate) fn lower(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
    ) -> Result<Self, WorthUiOrdinaryLanePlanDenial> {
        match WorthUiOrdinaryLanePlanBuilder::build(execution_plan, lane_admission) {
            Ok(plan) => Ok(Self::Executable(Box::new(plan))),
            Err(denial)
                if denial.reason() == WorthUiOrdinaryLanePlanDenialReason::NoOrdinaryRows =>
            {
                Ok(Self::NonExecutableBootstrap)
            }
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn availability(&self) -> WorthUiOrdinaryPlanAvailability {
        match self {
            Self::Executable(_) => WorthUiOrdinaryPlanAvailability::Executable,
            Self::NonExecutableBootstrap => WorthUiOrdinaryPlanAvailability::NonExecutableBootstrap,
        }
    }

    pub(crate) fn execute(
        &self,
        target: WorthUiOrdinaryFrameTarget,
    ) -> Result<WorthUiOrdinaryLaneFrameReceipt, WorthUiOrdinaryLaneFrameDenial> {
        match self {
            Self::Executable(plan) => WorthUiOrdinaryLaneFrameExecutor::execute(plan, target),
            Self::NonExecutableBootstrap => Err(WorthUiOrdinaryLaneFrameDenial::new(
                WorthUiOrdinaryLaneFrameDenialReason::ActivePlanNotOrdinaryExecutable,
                None,
                Default::default(),
            )),
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn summary(
        &self,
        request: crate::runtime::WorthUiOrdinaryPlanSummaryRequest,
    ) -> Result<
        crate::runtime::WorthUiOrdinaryPlanSummary,
        crate::runtime::WorthUiOrdinaryPlanSummaryDenial,
    > {
        match self {
            Self::Executable(plan) => {
                crate::runtime::WorthUiOrdinaryPlanSummary::from_plan(plan, request)
            }
            Self::NonExecutableBootstrap => Err(
                crate::runtime::WorthUiOrdinaryPlanSummaryDenial::ActivePlanNotOrdinaryExecutable,
            ),
        }
    }
}
