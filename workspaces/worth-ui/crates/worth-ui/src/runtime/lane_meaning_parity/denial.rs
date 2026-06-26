use crate::runtime::WorthUiLaneParityCounters;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLaneParityDenialReason {
    AmbiguousNodeReplacementPlan,
    NodePlanDigestMismatch,
    NarrowingDigestMismatch,
    QueryComparisonDigestMismatch,
    QueryRebindDigestMismatch,
    MissingLaneImpact,
    MissingLaneChangeTransition,
    SharedSemanticReferenceMismatch,
    QueryBindingChangedWithoutRebind,
    QueryRebindOutcomeMismatch,
    QueryRebindDenied,
    VisualSimilarityWithoutSemanticParity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLaneParityDenial {
    reason: WorthUiLaneParityDenialReason,
    active_artifact_digest: u64,
    candidate_artifact_digest: u64,
    counters: WorthUiLaneParityCounters,
}

impl WorthUiLaneParityDenial {
    pub(crate) fn new(
        reason: WorthUiLaneParityDenialReason,
        active_artifact_digest: u64,
        candidate_artifact_digest: u64,
        counters: WorthUiLaneParityCounters,
    ) -> Self {
        Self {
            reason,
            active_artifact_digest,
            candidate_artifact_digest,
            counters,
        }
    }

    pub fn reason(&self) -> WorthUiLaneParityDenialReason {
        self.reason
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest
    }

    pub fn counters(&self) -> WorthUiLaneParityCounters {
        self.counters
    }
}
