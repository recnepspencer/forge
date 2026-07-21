use crate::runtime::{
    WorthUiCanvasSpatialFrameDenial, WorthUiCanvasSpatialFrameReceipt,
    WorthUiCanvasSpatialFrameTarget, WorthUiCanvasSpatialPlan, WorthUiCanvasSpatialPlanDenial,
    WorthUiCanvasSpatialPlanDenialReason, WorthUiExecutionPlan, WorthUiLaneAdmission,
    WorthUiLaneHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCanvasSpatialPlanAvailability {
    Executable,
    NotDeclared,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiActiveCanvasSpatialPlanPosture {
    Executable(Box<WorthUiCanvasSpatialPlan>),
    NotDeclared,
}

impl WorthUiActiveCanvasSpatialPlanPosture {
    pub(crate) fn lower(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Result<Self, WorthUiCanvasSpatialPlanDenial> {
        match super::WorthUiCanvasSpatialPlanBuilder::build(
            execution_plan,
            lane_admission,
            host_binding,
        ) {
            Ok(plan) => Ok(Self::Executable(Box::new(plan))),
            Err(denial)
                if denial.reason() == WorthUiCanvasSpatialPlanDenialReason::NoCanvasSpatialRows =>
            {
                Ok(Self::NotDeclared)
            }
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn availability(&self) -> WorthUiCanvasSpatialPlanAvailability {
        match self {
            Self::Executable(_) => WorthUiCanvasSpatialPlanAvailability::Executable,
            Self::NotDeclared => WorthUiCanvasSpatialPlanAvailability::NotDeclared,
        }
    }

    pub(crate) fn first_handle(&self) -> Option<WorthUiLaneHandle> {
        match self {
            Self::Executable(plan) => plan.first_row().map(|row| row.lane_handle()),
            Self::NotDeclared => None,
        }
    }

    pub(crate) fn execute(
        &self,
        target: WorthUiCanvasSpatialFrameTarget,
    ) -> Result<WorthUiCanvasSpatialFrameReceipt, WorthUiCanvasSpatialFrameDenial> {
        match self {
            Self::Executable(plan) => {
                super::WorthUiCanvasSpatialFrameExecutor::execute(plan, target)
            }
            Self::NotDeclared => Err(WorthUiCanvasSpatialFrameDenial::new(
                crate::runtime::WorthUiCanvasSpatialFrameDenialReason::TargetNotInCanvasSpatialPlan,
                None,
                Default::default(),
            )),
        }
    }

    pub(crate) fn summary(
        &self,
        handle: WorthUiLaneHandle,
    ) -> Result<
        crate::runtime::WorthUiCanvasSpatialTargetSummary,
        crate::runtime::WorthUiCanvasSpatialInspectionDenial,
    > {
        match self {
            Self::Executable(plan) => super::summary::summarize(plan, handle),
            Self::NotDeclared => Err(crate::runtime::WorthUiCanvasSpatialInspectionDenial::new(
                crate::runtime::WorthUiHandleResolutionOutcome::TargetMissing,
            )),
        }
    }
}
