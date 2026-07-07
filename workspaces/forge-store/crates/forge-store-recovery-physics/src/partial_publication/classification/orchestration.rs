use crate::partial_publication::evidence::PartialPublicationEvidenceKind;
use crate::partial_publication::{
    NonAuthoritativePublicationSource, PartialPublicationCounterSnapshot,
    PartialPublicationEvidence, PartialPublicationObservationAdmission,
    PartialPublicationObservationSet, RecoveredOrRejectedPartialPublication,
    UnacknowledgedPublicationOutcome,
};

use super::{
    ambiguity::classify_ambiguity, crash_edge::classify_persisted_crash_edge,
    no_undo_hazard::reject_no_undo_hazard, non_authoritative::reject_non_authoritative_promotion,
    torn_publication::reject_torn_publication,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationClassification {
    pub(super) outcome: UnacknowledgedPublicationOutcome,
    pub(super) recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    pub(super) counters: PartialPublicationCounterSnapshot,
    pub(super) classification_digest: String,
    pub(super) before_wal_append_operation_digest: Option<String>,
}

impl PartialPublicationClassification {
    pub fn classify_observations(observations: PartialPublicationObservationSet) -> Self {
        Self::classify(
            PartialPublicationObservationAdmission::admit_observations(observations)
                .into_evidence(),
        )
    }

    pub fn classify(evidence: PartialPublicationEvidence) -> Self {
        match evidence.kind() {
            PartialPublicationEvidenceKind::PersistedCrashEdge(edge) => {
                classify_persisted_crash_edge(edge, evidence.persisted_digest())
            }
            PartialPublicationEvidenceKind::BackendResidueOnly { .. } => {
                reject_non_authoritative_promotion(
                    UnacknowledgedPublicationOutcome::RejectedNonAuthoritativePromotion,
                    NonAuthoritativePublicationSource::BackendResidue,
                    PartialPublicationCounterSnapshot::default().with_rejected_residue_promotion(),
                    evidence.persisted_digest(),
                )
            }
            PartialPublicationEvidenceKind::LiveAcknowledgmentMemoryOnly => {
                reject_non_authoritative_promotion(
                    UnacknowledgedPublicationOutcome::RejectedNonAuthoritativePromotion,
                    NonAuthoritativePublicationSource::LiveAcknowledgmentMemory,
                    PartialPublicationCounterSnapshot::default().with_rejected_live_ack_promotion(),
                    evidence.persisted_digest(),
                )
            }
            PartialPublicationEvidenceKind::LogOnly => reject_non_authoritative_promotion(
                UnacknowledgedPublicationOutcome::RejectedNonAuthoritativePromotion,
                NonAuthoritativePublicationSource::LogOnly,
                PartialPublicationCounterSnapshot::default().with_rejected_log_only_promotion(),
                evidence.persisted_digest(),
            ),
            PartialPublicationEvidenceKind::TornPublication(denial) => {
                reject_torn_publication(denial.clone(), evidence.persisted_digest())
            }
            PartialPublicationEvidenceKind::NoUndoHazard(classification) => {
                reject_no_undo_hazard(classification.clone(), evidence.persisted_digest())
            }
            PartialPublicationEvidenceKind::InsufficientPersistedEvidence { ambiguity_digest } => {
                classify_ambiguity(ambiguity_digest)
            }
        }
    }

    pub const fn outcome(&self) -> UnacknowledgedPublicationOutcome {
        self.outcome
    }

    pub const fn recovered_or_rejected(&self) -> &RecoveredOrRejectedPartialPublication {
        &self.recovered_or_rejected
    }

    pub const fn counters(&self) -> PartialPublicationCounterSnapshot {
        self.counters
    }

    pub fn classification_digest(&self) -> &str {
        &self.classification_digest
    }

    pub fn before_wal_append_operation_digest(&self) -> Option<&str> {
        self.before_wal_append_operation_digest.as_deref()
    }

    pub fn recover_or_reject_without_live_ack_memory(
        self,
    ) -> RecoveredOrRejectedPartialPublication {
        self.recovered_or_rejected
    }
}
