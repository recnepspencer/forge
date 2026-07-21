use crate::runtime::{
    WorthUiRuntimeArtifactComparisonCounters, WorthUiRuntimeArtifactComparisonOutcome,
    WorthUiRuntimeEquivalenceBasis,
};
use crate::source::{WorthUiArtifactDigest, WorthUiArtifactEquivalence};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiRuntimeArtifactComparison {
    runtime_basis: WorthUiRuntimeEquivalenceBasis,
    active_artifact_digest: WorthUiArtifactDigest,
    candidate_artifact_digest: WorthUiArtifactDigest,
    outcome: WorthUiRuntimeArtifactComparisonOutcome,
    artifact_equivalence: WorthUiArtifactEquivalence,
    counters: WorthUiRuntimeArtifactComparisonCounters,
}

impl WorthUiRuntimeArtifactComparison {
    pub(crate) fn new(
        runtime_basis: WorthUiRuntimeEquivalenceBasis,
        active_artifact_digest: WorthUiArtifactDigest,
        candidate_artifact_digest: WorthUiArtifactDigest,
        artifact_equivalence: WorthUiArtifactEquivalence,
        counters: WorthUiRuntimeArtifactComparisonCounters,
    ) -> Self {
        let outcome = if artifact_equivalence.is_equivalent() {
            WorthUiRuntimeArtifactComparisonOutcome::EquivalentNoOp
        } else {
            WorthUiRuntimeArtifactComparisonOutcome::MeaningfullyDifferent
        };
        Self {
            runtime_basis,
            active_artifact_digest,
            candidate_artifact_digest,
            outcome,
            artifact_equivalence,
            counters,
        }
    }

    #[cfg(test)]
    pub fn runtime_basis(&self) -> WorthUiRuntimeEquivalenceBasis {
        self.runtime_basis
    }

    pub fn active_artifact_digest(&self) -> u64 {
        self.active_artifact_digest.raw()
    }

    pub fn candidate_artifact_digest(&self) -> u64 {
        self.candidate_artifact_digest.raw()
    }

    pub fn outcome(&self) -> WorthUiRuntimeArtifactComparisonOutcome {
        self.outcome
    }

    pub fn counters(&self) -> WorthUiRuntimeArtifactComparisonCounters {
        self.counters
    }

    pub(crate) fn artifact_equivalence(&self) -> &WorthUiArtifactEquivalence {
        &self.artifact_equivalence
    }
}
