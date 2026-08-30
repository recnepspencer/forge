use super::{UiRuntimeServiceInspectionCost, UiRuntimeServiceInspectionSource};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFocusMoveInspectionCause {
    Direct,
    KeyboardTraversal,
    RovingMovement,
    PortalInitial,
    PortalRestoration,
    RebindPreserved,
    RebindFallback,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFocusMoveInspectionOutcome {
    Moved,
    Unchanged,
    Cleared,
    NoEligibleParticipant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiFocusRestorationFailureInspectionReason {
    NoEligibleParticipant,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiFocusMovedInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    previous_mounted_instance: Option<u64>,
    current_mounted_instance: Option<u64>,
    cause: UiFocusMoveInspectionCause,
    outcome: UiFocusMoveInspectionOutcome,
    participants_visited: u32,
    cost: UiRuntimeServiceInspectionCost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiFocusRestorationFailedInspectionSummary {
    source: UiRuntimeServiceInspectionSource,
    reason: UiFocusRestorationFailureInspectionReason,
    cost: UiRuntimeServiceInspectionCost,
}

impl UiFocusMovedInspectionSummary {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        previous_mounted_instance: Option<u64>,
        current_mounted_instance: Option<u64>,
        cause: UiFocusMoveInspectionCause,
        outcome: UiFocusMoveInspectionOutcome,
        participants_visited: u32,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            previous_mounted_instance,
            current_mounted_instance,
            cause,
            outcome,
            participants_visited,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn previous_mounted_instance(self) -> Option<u64> {
        self.previous_mounted_instance
    }
    pub const fn current_mounted_instance(self) -> Option<u64> {
        self.current_mounted_instance
    }
    pub const fn cause(self) -> UiFocusMoveInspectionCause {
        self.cause
    }
    pub const fn outcome(self) -> UiFocusMoveInspectionOutcome {
        self.outcome
    }
    pub const fn participants_visited(self) -> u32 {
        self.participants_visited
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}

impl UiFocusRestorationFailedInspectionSummary {
    pub const fn new(
        source: UiRuntimeServiceInspectionSource,
        reason: UiFocusRestorationFailureInspectionReason,
        cost: UiRuntimeServiceInspectionCost,
    ) -> Self {
        Self {
            source,
            reason,
            cost,
        }
    }

    pub const fn source(self) -> UiRuntimeServiceInspectionSource {
        self.source
    }
    pub const fn reason(self) -> UiFocusRestorationFailureInspectionReason {
        self.reason
    }
    pub const fn cost(self) -> UiRuntimeServiceInspectionCost {
        self.cost
    }
}
