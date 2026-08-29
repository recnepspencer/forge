use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiMotionInterruptedInspectionReason {
    RetargetedFromCurrentPresentation,
    RestartedFromSemanticPredecessor,
    FinishThenApply,
    SnappedToTarget,
    CancelledAndDropped,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiMotionInterruptedInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    reason: UiMotionInterruptedInspectionReason,
    successor_revision: u64,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiMotionInterruptedInspectionSummary {
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        reason: UiMotionInterruptedInspectionReason,
        successor_revision: u64,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            reason,
            successor_revision,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn reason(self) -> UiMotionInterruptedInspectionReason {
        self.reason
    }
    pub const fn successor_revision(self) -> u64 {
        self.successor_revision
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}
