use super::{
    WorthQueryAdmittedWorkflowEffect, WorthQueryLoweredWorkflowPlan,
    WorthQueryPromotionEligibility, WorthQueryWorkflowAftermath, WorthQueryWorkflowCounters,
    WorthQueryWorkflowExecution, WorthQueryWorkflowStop,
};
use crate::runtime::WorthQueryPreviewOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryWorkflowAdvisoryKind {
    LowerRuntime,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryWorkflowAdvisory {
    kind: WorthQueryWorkflowAdvisoryKind,
}

impl WorthQueryWorkflowAdvisory {
    pub fn kind(&self) -> WorthQueryWorkflowAdvisoryKind {
        self.kind
    }
}

pub struct WorthQueryWorkflowCompletion {
    advisories: Vec<WorthQueryWorkflowAdvisory>,
    eligibility: WorthQueryPromotionEligibility,
    execution: WorthQueryWorkflowExecution,
    aftermath: WorthQueryWorkflowAftermath,
    preview_outcome: WorthQueryPreviewOutcome,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryWorkflowCompletion {
    pub fn advisories(&self) -> &[WorthQueryWorkflowAdvisory] {
        &self.advisories
    }

    pub fn promotion_eligibility(&self) -> &WorthQueryPromotionEligibility {
        &self.eligibility
    }

    pub fn execution(&self) -> &WorthQueryWorkflowExecution {
        &self.execution
    }

    pub fn admitted_effect(&self) -> &WorthQueryAdmittedWorkflowEffect {
        self.execution.admitted_effect()
    }

    pub fn lowered_plan(&self) -> &WorthQueryLoweredWorkflowPlan {
        self.execution.lowered_plan()
    }

    pub fn aftermath(&self) -> &WorthQueryWorkflowAftermath {
        &self.aftermath
    }

    pub fn preview_outcome(&self) -> &WorthQueryPreviewOutcome {
        &self.preview_outcome
    }

    pub fn counters(&self) -> &WorthQueryWorkflowCounters {
        &self.counters
    }

    pub(crate) fn new(
        eligibility: WorthQueryPromotionEligibility,
        admitted_effect: WorthQueryAdmittedWorkflowEffect,
        lowered_plan: WorthQueryLoweredWorkflowPlan,
        aftermath: WorthQueryWorkflowAftermath,
        preview_outcome: WorthQueryPreviewOutcome,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            advisories: Vec::new(),
            eligibility,
            execution: WorthQueryWorkflowExecution::new(admitted_effect, lowered_plan),
            aftermath,
            preview_outcome,
            counters,
        }
    }
}

pub enum WorthQueryWorkflowOutcome {
    Completed(WorthQueryWorkflowCompletion),
    Stopped(WorthQueryWorkflowStop),
}

impl WorthQueryWorkflowOutcome {
    pub fn completed(&self) -> Option<&WorthQueryWorkflowCompletion> {
        match self {
            Self::Completed(completion) => Some(completion),
            Self::Stopped(_) => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryWorkflowStop> {
        match self {
            Self::Completed(_) => None,
            Self::Stopped(stop) => Some(stop),
        }
    }
}
