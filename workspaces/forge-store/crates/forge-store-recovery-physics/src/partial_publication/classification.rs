use super::{
    AmbiguousPublicationReport, NoUndoPartialPublicationClassification,
    NonAuthoritativePublicationDenial, NonAuthoritativePublicationSource,
    PartialPublicationCounterSnapshot, PartialPublicationCrashEdge, PartialPublicationEvidence,
    PartialPublicationObservationAdmission, PartialPublicationObservationSet,
    RecoveredOrRejectedPartialPublication, RollbackImageRequiredPosture, TornPublicationDenial,
    UnacknowledgedPublicationOutcome,
};
use crate::partial_publication::evidence::PartialPublicationEvidenceKind;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartialPublicationClassification {
    outcome: UnacknowledgedPublicationOutcome,
    recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    counters: PartialPublicationCounterSnapshot,
    classification_digest: String,
    before_wal_append_operation_digest: Option<String>,
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

fn classify_persisted_crash_edge(
    edge: &PartialPublicationCrashEdge,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_observed_crash_edge();
    match edge {
        PartialPublicationCrashEdge::BeforeWalAppend { operation_digest } => {
            classification_with_before_wal_operation_digest(
                UnacknowledgedPublicationOutcome::NoWalAppendObserved,
                RecoveredOrRejectedPartialPublication::NoRecoveredWork { counters },
                counters,
                digest,
                operation_digest.clone(),
            )
        }
        PartialPublicationCrashEdge::AfterWalAppendBeforeDurability { .. } => classification(
            UnacknowledgedPublicationOutcome::WalAppendedButNotDurable,
            RecoveredOrRejectedPartialPublication::NoRecoveredWork { counters },
            counters,
            digest,
        ),
        PartialPublicationCrashEdge::AfterDurabilityBeforeAck { durable_wal } => {
            let counters = counters.with_replayable_unacknowledged_wal();
            classification(
                UnacknowledgedPublicationOutcome::DurableWalReplayable,
                RecoveredOrRejectedPartialPublication::ReplayableUnacknowledgedWal {
                    durable_wal: durable_wal.clone(),
                    counters,
                },
                counters,
                digest,
            )
        }
        PartialPublicationCrashEdge::AfterAckBeforePageFlush { .. } => classification(
            UnacknowledgedPublicationOutcome::AcknowledgedBeforePageFlush,
            RecoveredOrRejectedPartialPublication::AcknowledgedWorkAwaitingPageFlush { counters },
            counters,
            digest,
        ),
        PartialPublicationCrashEdge::DuringCheckpointCutover { checkpoint_digest } => {
            let counters = counters.with_ambiguous_outcome();
            let report = AmbiguousPublicationReport::insufficient_persisted_evidence(
                checkpoint_digest.clone(),
            );
            classification(
                UnacknowledgedPublicationOutcome::CheckpointCutoverAmbiguous,
                RecoveredOrRejectedPartialPublication::Ambiguous { report, counters },
                counters,
                digest,
            )
        }
    }
}

fn reject_non_authoritative_promotion(
    outcome: UnacknowledgedPublicationOutcome,
    source: NonAuthoritativePublicationSource,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    classification(
        outcome,
        RecoveredOrRejectedPartialPublication::RejectedNonAuthoritativePromotion {
            denial: NonAuthoritativePublicationDenial::new(source, digest),
            counters,
        },
        counters,
        digest,
    )
}

fn reject_torn_publication(
    denial: TornPublicationDenial,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_torn_publication_denial();
    classification(
        UnacknowledgedPublicationOutcome::TornPublicationRejected,
        RecoveredOrRejectedPartialPublication::RejectedTornPublication { denial, counters },
        counters,
        digest,
    )
}

fn reject_no_undo_hazard(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    match no_undo.posture() {
        RollbackImageRequiredPosture::RequiredButMissing => reject_missing_rollback_image(
            no_undo,
            PartialPublicationCounterSnapshot::default().with_no_undo_denial(),
            digest,
        ),
        RollbackImageRequiredPosture::DeferredToUndoCapableRecovery => {
            defer_to_undo_capable_recovery(no_undo, digest)
        }
        RollbackImageRequiredPosture::NotRequiredForAdmittedRedoOnlyMutation
        | RollbackImageRequiredPosture::ProtectedByRollbackImage => {
            accept_no_undo_posture(no_undo, digest)
        }
    }
}

fn reject_missing_rollback_image(
    no_undo: NoUndoPartialPublicationClassification,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    classification(
        UnacknowledgedPublicationOutcome::RejectedNoUndoHazard,
        RecoveredOrRejectedPartialPublication::RejectedNoUndoHazard {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}

fn defer_to_undo_capable_recovery(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_no_undo_posture();
    classification(
        UnacknowledgedPublicationOutcome::UndoCapableRecoveryDeferred,
        RecoveredOrRejectedPartialPublication::UndoCapableRecoveryDeferred {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}

fn accept_no_undo_posture(
    no_undo: NoUndoPartialPublicationClassification,
    digest: &str,
) -> PartialPublicationClassification {
    let counters = PartialPublicationCounterSnapshot::default().with_no_undo_posture();
    let outcome = match no_undo.posture() {
        RollbackImageRequiredPosture::NotRequiredForAdmittedRedoOnlyMutation => {
            UnacknowledgedPublicationOutcome::NoUndoPostureSatisfied
        }
        RollbackImageRequiredPosture::ProtectedByRollbackImage => {
            UnacknowledgedPublicationOutcome::RollbackImageProtected
        }
        RollbackImageRequiredPosture::RequiredButMissing
        | RollbackImageRequiredPosture::DeferredToUndoCapableRecovery => {
            unreachable!("callers route rejected and deferred no-undo postures first")
        }
    };
    classification(
        outcome,
        RecoveredOrRejectedPartialPublication::NoUndoPostureAccepted {
            classification: no_undo,
            counters,
        },
        counters,
        digest,
    )
}

fn classify_ambiguity(ambiguity_digest: &str) -> PartialPublicationClassification {
    let report = AmbiguousPublicationReport::insufficient_persisted_evidence(ambiguity_digest);
    let counters = report.counters();
    classification(
        UnacknowledgedPublicationOutcome::Ambiguous,
        RecoveredOrRejectedPartialPublication::Ambiguous { report, counters },
        counters,
        ambiguity_digest,
    )
}

fn classification(
    outcome: UnacknowledgedPublicationOutcome,
    recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
) -> PartialPublicationClassification {
    PartialPublicationClassification {
        outcome,
        recovered_or_rejected,
        counters,
        classification_digest: format!("{outcome:?}:{digest}"),
        before_wal_append_operation_digest: None,
    }
}

fn classification_with_before_wal_operation_digest(
    outcome: UnacknowledgedPublicationOutcome,
    recovered_or_rejected: RecoveredOrRejectedPartialPublication,
    counters: PartialPublicationCounterSnapshot,
    digest: &str,
    operation_digest: String,
) -> PartialPublicationClassification {
    PartialPublicationClassification {
        outcome,
        recovered_or_rejected,
        counters,
        classification_digest: format!("{outcome:?}:{digest}"),
        before_wal_append_operation_digest: Some(operation_digest),
    }
}
