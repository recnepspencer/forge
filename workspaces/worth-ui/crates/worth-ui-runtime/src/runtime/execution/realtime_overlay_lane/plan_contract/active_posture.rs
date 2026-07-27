use crate::runtime::{
    WorthUiExecutionPlan, WorthUiHudPlan, WorthUiHudPlanDenial, WorthUiHudPlanDenialReason,
    WorthUiLaneAdmission, WorthUiRealtimeFrameDenial, WorthUiRealtimeFrameReceipt,
    WorthUiRealtimeFrameTarget, WorthUiRendererSurfaceHandle,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiRealtimePlanAvailability {
    Executable,
    NotDeclared,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiActiveRealtimePlanPosture {
    Executable(Box<WorthUiHudPlan>),
    NotDeclared,
}

impl WorthUiActiveRealtimePlanPosture {
    pub(crate) fn lower(
        execution_plan: &WorthUiExecutionPlan,
        lane_admission: &WorthUiLaneAdmission,
        host_binding: crate::facade::WorthUiHostPlanBinding,
    ) -> Result<Self, WorthUiHudPlanDenial> {
        match super::WorthUiHudPlanBuilder::build(execution_plan, lane_admission, host_binding) {
            Ok(plan) => Ok(Self::Executable(Box::new(plan))),
            Err(denial) if denial.reason() == WorthUiHudPlanDenialReason::NoHudRows => {
                Ok(Self::NotDeclared)
            }
            Err(denial) => Err(denial),
        }
    }

    pub(crate) fn availability(&self) -> WorthUiRealtimePlanAvailability {
        match self {
            Self::Executable(_) => WorthUiRealtimePlanAvailability::Executable,
            Self::NotDeclared => WorthUiRealtimePlanAvailability::NotDeclared,
        }
    }

    pub(crate) fn first_handle(&self) -> Option<WorthUiRendererSurfaceHandle> {
        match self {
            Self::Executable(plan) => plan
                .first_row()
                .map(|row| row.renderer_surface_admission().handle()),
            Self::NotDeclared => None,
        }
    }

    pub(crate) fn execute(
        &self,
        target: WorthUiRealtimeFrameTarget,
    ) -> Result<WorthUiRealtimeFrameReceipt, WorthUiRealtimeFrameDenial> {
        match self {
            Self::Executable(plan) => super::WorthUiRealtimeFrameExecutor::execute(plan, target),
            Self::NotDeclared => Err(WorthUiRealtimeFrameDenial::new(
                crate::runtime::WorthUiRealtimeFrameDenialReason::TargetNotInHudPlan,
                None,
                Default::default(),
            )),
        }
    }

    #[cfg(any(test, feature = "certification-support"))]
    pub(crate) fn summary(
        &self,
        handle: WorthUiRendererSurfaceHandle,
    ) -> Result<
        crate::runtime::WorthUiRealtimeTargetSummary,
        crate::runtime::WorthUiRealtimeInspectionDenial,
    > {
        match self {
            Self::Executable(plan) => super::summary::summarize(plan, handle),
            Self::NotDeclared => Err(crate::runtime::WorthUiRealtimeInspectionDenial::new(
                crate::runtime::WorthUiHandleResolutionOutcome::TargetMissing,
            )),
        }
    }
}
