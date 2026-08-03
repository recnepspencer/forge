use crate::{
    partial_publication::PartialPublicationClassification, PartialPublicationCounterSnapshot,
    PartialPublicationCrashEdge, PartialPublicationEvidence, PartialPublicationObservationSet,
    UnacknowledgedDurableWal, UnacknowledgedPublicationOutcome,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

impl CrashBoundaryLayoutReport {
    pub fn admit_observations(
        observations: PartialPublicationObservationSet,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        Self::admit_classification(&PartialPublicationClassification::classify_observations(
            observations,
        ))
    }

    pub fn admit_evidence(
        evidence: PartialPublicationEvidence,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        Self::admit_classification(&PartialPublicationClassification::classify(evidence))
    }

    pub fn admit_crash_edge(
        crash_edge: PartialPublicationCrashEdge,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        Self::admit_evidence(PartialPublicationEvidence::from_persisted_crash_edge(
            crash_edge,
        ))
    }

    pub fn admit_classification(
        classification: &PartialPublicationClassification,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        match classification.outcome() {
            UnacknowledgedPublicationOutcome::RejectedNonAuthoritativePromotion => Err(
                RecoveryLayoutAccessDenial::new(
                    RecoveryLayoutAccessDenialKind::BackendResidueCannotStandInForCrashBoundaryAuthority,
                ),
            ),
            UnacknowledgedPublicationOutcome::CheckpointCutoverAmbiguous
            | UnacknowledgedPublicationOutcome::Ambiguous => Err(
                RecoveryLayoutAccessDenial::new(
                    RecoveryLayoutAccessDenialKind::AmbiguousResidueCannotStandInForCrashBoundaryAuthority,
                ),
            ),
            _ => Ok(CrashBoundaryLayoutReport::from_classification(classification)),
        }
    }
}

pub(crate) fn admit_partial_publication_classification(
    classification: &PartialPublicationClassification,
) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
    CrashBoundaryLayoutReport::admit_classification(classification)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrashBoundaryLayoutReport {
    outcome: UnacknowledgedPublicationOutcome,
    classification_digest: String,
    counters: PartialPublicationCounterSnapshot,
    replayable: bool,
    replayable_durable_wal: Option<UnacknowledgedDurableWal>,
    ambiguous: bool,
    observed_crash_edges: u64,
}

impl CrashBoundaryLayoutReport {
    fn from_classification(classification: &PartialPublicationClassification) -> Self {
        Self {
            outcome: classification.outcome(),
            classification_digest: classification.classification_digest().to_owned(),
            counters: classification.counters(),
            replayable: classification
                .recovered_or_rejected()
                .is_replayable_without_promoting_acknowledgment(),
            replayable_durable_wal: classification
                .recovered_or_rejected()
                .replayable_durable_wal()
                .cloned(),
            ambiguous: matches!(
                classification.outcome(),
                UnacknowledgedPublicationOutcome::CheckpointCutoverAmbiguous
                    | UnacknowledgedPublicationOutcome::Ambiguous
            ),
            observed_crash_edges: classification.counters().observed_crash_edges() as u64,
        }
    }

    pub const fn outcome(&self) -> UnacknowledgedPublicationOutcome {
        self.outcome
    }

    pub fn classification_digest(&self) -> &str {
        &self.classification_digest
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.counters
    }

    pub const fn replayable(&self) -> bool {
        self.replayable
    }

    pub fn replayable_durable_wal(&self) -> Option<&UnacknowledgedDurableWal> {
        self.replayable_durable_wal.as_ref()
    }

    pub const fn ambiguous(&self) -> bool {
        self.ambiguous
    }

    pub const fn observed_crash_edges(&self) -> u64 {
        self.observed_crash_edges
    }
}
