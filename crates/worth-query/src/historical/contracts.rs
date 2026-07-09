use super::cost::{
    HistoricalPerformanceStatusMarker, ReplayTailReuseEligibility, RetainedStateReuseEligibility,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalReplaySpanBudget {
    max_replay_events: usize,
}

impl HistoricalReplaySpanBudget {
    pub fn max_replay_events(&self) -> usize {
        self.max_replay_events
    }

    pub fn bounded(max_replay_events: usize) -> Self {
        Self::new(max_replay_events)
    }

    pub(crate) fn new(max_replay_events: usize) -> Self {
        Self {
            max_replay_events: max_replay_events.max(1),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalReconstructionBudget {
    max_reconstruction_scope: usize,
}

impl HistoricalReconstructionBudget {
    pub fn max_reconstruction_scope(&self) -> usize {
        self.max_reconstruction_scope
    }

    pub fn bounded(max_reconstruction_scope: usize) -> Self {
        Self::new(max_reconstruction_scope)
    }

    pub(crate) fn new(max_reconstruction_scope: usize) -> Self {
        Self {
            max_reconstruction_scope: max_reconstruction_scope.max(1),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathReuseDescriptor {
    retained_state_reuse: RetainedStateReuseEligibility,
    replay_tail_reuse: ReplayTailReuseEligibility,
}

impl HistoricalPathReuseDescriptor {
    pub fn retained_state_reuse(&self) -> &RetainedStateReuseEligibility {
        &self.retained_state_reuse
    }

    pub fn replay_tail_reuse(&self) -> &ReplayTailReuseEligibility {
        &self.replay_tail_reuse
    }

    pub fn no_reuse() -> Self {
        Self::new(
            RetainedStateReuseEligibility::NotReusable,
            ReplayTailReuseEligibility::NotReusable,
        )
    }

    pub fn retained_reuse() -> Self {
        Self::new(
            RetainedStateReuseEligibility::Reusable,
            ReplayTailReuseEligibility::NotReusable,
        )
    }

    pub fn with_replay_tail_reuse() -> Self {
        Self::new(
            RetainedStateReuseEligibility::NotReusable,
            ReplayTailReuseEligibility::Reusable,
        )
    }

    pub(crate) fn new(
        retained_state_reuse: RetainedStateReuseEligibility,
        replay_tail_reuse: ReplayTailReuseEligibility,
    ) -> Self {
        Self {
            retained_state_reuse,
            replay_tail_reuse,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HistoricalPathComplexityContract {
    contract_name: &'static str,
    status_marker: HistoricalPerformanceStatusMarker,
}

impl HistoricalPathComplexityContract {
    pub fn contract_name(&self) -> &'static str {
        self.contract_name
    }

    pub fn status_marker(&self) -> &HistoricalPerformanceStatusMarker {
        &self.status_marker
    }

    pub(crate) fn retained_path() -> Self {
        Self {
            contract_name: "historical_retained_path",
            status_marker: HistoricalPerformanceStatusMarker::Verified,
        }
    }

    pub(crate) fn replay_path() -> Self {
        Self {
            contract_name: "historical_replay_path",
            status_marker: HistoricalPerformanceStatusMarker::Verified,
        }
    }

    pub(crate) fn reconstruction_path() -> Self {
        Self {
            contract_name: "historical_reconstruction_path",
            status_marker: HistoricalPerformanceStatusMarker::Debt,
        }
    }
}
