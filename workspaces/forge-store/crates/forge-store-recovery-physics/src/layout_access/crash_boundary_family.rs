use crate::{
    partial_publication::PartialPublicationClassification, PartialPublicationCounterSnapshot,
    PartialPublicationCrashEdge, PartialPublicationEvidence, PartialPublicationObservationSet,
    UnacknowledgedDurableWal, UnacknowledgedPublicationOutcome,
};

use super::{RecoveryLayoutAccessDenial, RecoveryLayoutAccessDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCrashBoundaryLayoutRule {
    _private: (),
}

impl AdmittedCrashBoundaryLayoutRule {
    pub(crate) const fn internal_phase22() -> Self {
        Self { _private: () }
    }

    #[cfg(feature = "phase22-layout-rule-construction")]
    #[doc(hidden)]
    pub const fn phase22() -> Self {
        Self::internal_phase22()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashBoundaryLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashBoundaryLayoutAdmission {
    _private: (),
}

impl CrashBoundaryLayoutFamilyHome {
    pub const fn s8() -> Self {
        Self
    }

    pub fn admit(
        self,
        _rule: &AdmittedCrashBoundaryLayoutRule,
    ) -> Result<CrashBoundaryLayoutAdmission, RecoveryLayoutAccessDenial> {
        Ok(CrashBoundaryLayoutAdmission { _private: () })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AdmittedCrashBoundaryLayoutFamily {
    _admission: CrashBoundaryLayoutAdmission,
}

impl AdmittedCrashBoundaryLayoutFamily {
    pub(crate) const fn new(admission: CrashBoundaryLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    pub fn admit_observations(
        &self,
        observations: PartialPublicationObservationSet,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        self.admit_classification(&PartialPublicationClassification::classify_observations(
            observations,
        ))
    }

    pub fn admit_evidence(
        &self,
        evidence: PartialPublicationEvidence,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        self.admit_classification(&PartialPublicationClassification::classify(evidence))
    }

    pub fn admit_crash_edge(
        &self,
        crash_edge: PartialPublicationCrashEdge,
    ) -> Result<CrashBoundaryLayoutReport, RecoveryLayoutAccessDenial> {
        self.admit_evidence(PartialPublicationEvidence::from_persisted_crash_edge(
            crash_edge,
        ))
    }

    pub fn admit_classification(
        &self,
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
            UnacknowledgedPublicationOutcome::NoUndoPostureSatisfied
            | UnacknowledgedPublicationOutcome::RollbackImageProtected
            | UnacknowledgedPublicationOutcome::UndoCapableRecoveryDeferred => Err(
                RecoveryLayoutAccessDenial::new(
                    RecoveryLayoutAccessDenialKind::DerivedRollbackCannotStandInForCrashBoundaryAuthority,
                ),
            ),
            _ => Ok(CrashBoundaryLayoutReport::from_classification(classification)),
        }
    }

    pub fn reject_derived_rollback_outcome(
        &self,
        _outcome: UnacknowledgedPublicationOutcome,
    ) -> Result<(), RecoveryLayoutAccessDenial> {
        Err(RecoveryLayoutAccessDenial::new(
            RecoveryLayoutAccessDenialKind::DerivedRollbackCannotStandInForCrashBoundaryAuthority,
        ))
    }
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
