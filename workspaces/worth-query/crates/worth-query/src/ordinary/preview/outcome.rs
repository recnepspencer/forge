use crate::ordinary::workflow::{
    WorthQueryLoweredWorkflowPlan, WorthQueryWorkflowAftermath, WorthQueryWorkflowCompletion,
    WorthQueryWorkflowCounters, WorthQueryWorkflowStop,
};
use crate::runtime::WorthQueryPreviewOutcome;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPreviewCompletionFamily {
    ReadOnly,
    PromotionEligible,
}

pub struct WorthQueryReadOnlyPreviewCompletion {
    lowered_plan: WorthQueryLoweredWorkflowPlan,
    aftermath: WorthQueryWorkflowAftermath,
    preview_outcome: WorthQueryPreviewOutcome,
    counters: WorthQueryWorkflowCounters,
}

impl WorthQueryReadOnlyPreviewCompletion {
    pub fn family(&self) -> WorthQueryPreviewCompletionFamily {
        WorthQueryPreviewCompletionFamily::ReadOnly
    }

    pub fn lowered_plan(&self) -> &WorthQueryLoweredWorkflowPlan {
        &self.lowered_plan
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
        lowered_plan: WorthQueryLoweredWorkflowPlan,
        aftermath: WorthQueryWorkflowAftermath,
        preview_outcome: WorthQueryPreviewOutcome,
        counters: WorthQueryWorkflowCounters,
    ) -> Self {
        Self {
            lowered_plan,
            aftermath,
            preview_outcome,
            counters,
        }
    }
}

pub enum WorthQueryPreviewJourneyOutcome {
    ReadOnlyCompleted(WorthQueryReadOnlyPreviewCompletion),
    PromotionCompleted(WorthQueryWorkflowCompletion),
    Stopped(WorthQueryWorkflowStop),
}

impl WorthQueryPreviewJourneyOutcome {
    pub fn read_only_completion(&self) -> Option<&WorthQueryReadOnlyPreviewCompletion> {
        match self {
            Self::ReadOnlyCompleted(completion) => Some(completion),
            _ => None,
        }
    }

    pub fn promotion_completion(&self) -> Option<&WorthQueryWorkflowCompletion> {
        match self {
            Self::PromotionCompleted(completion) => Some(completion),
            _ => None,
        }
    }

    pub fn stop(&self) -> Option<&WorthQueryWorkflowStop> {
        match self {
            Self::Stopped(stop) => Some(stop),
            _ => None,
        }
    }
}
