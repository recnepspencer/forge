use crate::correspondence::CorrespondenceEvidenceResolved;
use crate::historical::HistoricalMaterializationPathMetadata;
use crate::runtime::WorthQueryReadResult;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonChange {
    Unchanged,
    Changed,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryComparisonJourneyCounters {
    basis_pair_validation_count: usize,
    current_execution_attempt_count: usize,
    historical_execution_attempt_count: usize,
    correspondence_resolution_attempt_count: usize,
}

impl WorthQueryComparisonJourneyCounters {
    pub fn basis_pair_validation_count(&self) -> usize {
        self.basis_pair_validation_count
    }
    pub fn current_execution_attempt_count(&self) -> usize {
        self.current_execution_attempt_count
    }
    pub fn historical_execution_attempt_count(&self) -> usize {
        self.historical_execution_attempt_count
    }
    pub fn correspondence_resolution_attempt_count(&self) -> usize {
        self.correspondence_resolution_attempt_count
    }
    pub(crate) fn validate_pair() -> Self {
        Self {
            basis_pair_validation_count: 1,
            ..Self::default()
        }
    }
    pub(crate) fn execute_current(mut self) -> Self {
        self.current_execution_attempt_count = 1;
        self
    }
    pub(crate) fn execute_historical(mut self) -> Self {
        self.historical_execution_attempt_count = 1;
        self
    }
    pub(crate) fn resolve_correspondence(mut self) -> Self {
        self.correspondence_resolution_attempt_count = 1;
        self
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonNextAction {
    ReviseDeclaration,
    RefreshBasisPair,
    ResolveAuthority,
    NarrowCandidates,
    RetryRuntime,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonStopSource {
    StaleBasisPair,
    CurrentExecution,
    HistoricalExecution,
    CorrespondenceDenied,
}

#[derive(Debug)]
pub struct WorthQueryComparisonStop {
    source: WorthQueryComparisonStopSource,
    next_action: WorthQueryComparisonNextAction,
    reason: String,
    counters: WorthQueryComparisonJourneyCounters,
}

impl WorthQueryComparisonStop {
    pub fn source(&self) -> WorthQueryComparisonStopSource {
        self.source
    }
    pub fn next_action(&self) -> WorthQueryComparisonNextAction {
        self.next_action
    }
    pub fn reason(&self) -> &str {
        &self.reason
    }
    pub fn journey_counters(&self) -> &WorthQueryComparisonJourneyCounters {
        &self.counters
    }
    pub(crate) fn new(
        source: WorthQueryComparisonStopSource,
        next_action: WorthQueryComparisonNextAction,
        reason: impl Into<String>,
        counters: WorthQueryComparisonJourneyCounters,
    ) -> Self {
        Self {
            source,
            next_action,
            reason: reason.into(),
            counters,
        }
    }
}

#[derive(Debug)]
pub struct WorthQueryComparisonCompletion {
    left: WorthQueryReadResult,
    right: WorthQueryReadResult,
    right_materialization: HistoricalMaterializationPathMetadata,
    change: WorthQueryComparisonChange,
    counters: WorthQueryComparisonJourneyCounters,
}

impl WorthQueryComparisonCompletion {
    pub fn left(&self) -> &WorthQueryReadResult {
        &self.left
    }
    pub fn right(&self) -> &WorthQueryReadResult {
        &self.right
    }
    pub fn right_materialization(&self) -> &HistoricalMaterializationPathMetadata {
        &self.right_materialization
    }
    pub fn change(&self) -> WorthQueryComparisonChange {
        self.change
    }
    pub fn journey_counters(&self) -> &WorthQueryComparisonJourneyCounters {
        &self.counters
    }
    pub(crate) fn new(
        left: WorthQueryReadResult,
        right: WorthQueryReadResult,
        right_materialization: HistoricalMaterializationPathMetadata,
        change: WorthQueryComparisonChange,
        counters: WorthQueryComparisonJourneyCounters,
    ) -> Self {
        Self {
            left,
            right,
            right_materialization,
            change,
            counters,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryComparisonCorrespondencePosture {
    AuthoritativeContinuity,
    Advisory,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryComparisonCorrespondence {
    correspondence: CorrespondenceEvidenceResolved,
    posture: WorthQueryComparisonCorrespondencePosture,
    counters: WorthQueryComparisonJourneyCounters,
}

impl WorthQueryComparisonCorrespondence {
    pub fn correspondence(&self) -> &CorrespondenceEvidenceResolved {
        &self.correspondence
    }
    pub fn posture(&self) -> WorthQueryComparisonCorrespondencePosture {
        self.posture
    }
    pub fn journey_counters(&self) -> &WorthQueryComparisonJourneyCounters {
        &self.counters
    }
    pub(crate) fn new(
        correspondence: CorrespondenceEvidenceResolved,
        posture: WorthQueryComparisonCorrespondencePosture,
        counters: WorthQueryComparisonJourneyCounters,
    ) -> Self {
        Self {
            correspondence,
            posture,
            counters,
        }
    }
}

#[derive(Debug)]
pub enum WorthQueryComparisonOutcome {
    Completed(WorthQueryComparisonCompletion),
    Correspondence(WorthQueryComparisonCorrespondence),
    Stopped(WorthQueryComparisonStop),
}

impl WorthQueryComparisonOutcome {
    pub fn completed(&self) -> Option<&WorthQueryComparisonCompletion> {
        match self {
            Self::Completed(value) => Some(value),
            _ => None,
        }
    }
    pub fn correspondence(&self) -> Option<&WorthQueryComparisonCorrespondence> {
        match self {
            Self::Correspondence(value) => Some(value),
            _ => None,
        }
    }
    pub fn stop(&self) -> Option<&WorthQueryComparisonStop> {
        match self {
            Self::Stopped(value) => Some(value),
            _ => None,
        }
    }
}
