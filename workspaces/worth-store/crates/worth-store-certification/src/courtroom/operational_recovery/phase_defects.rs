use std::collections::{BTreeMap, BTreeSet};

use super::S10Phase;
use sha2::{Digest, Sha256};

mod localizers;
pub use localizers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10PhaseDefectSourceKind {
    StructuralCompileGate,
    ControlSelection,
    IndependentInspection,
    RuntimeArtifactOmission,
    AuditOmission,
    PhysicalHarness,
    FormalMutation,
    CounterOmission,
    CloseoutJoinOmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10PhaseDefectDenial {
    PhaseNotInvoked,
    PreflightMismatch,
    ControlSelectionNotBound,
    ControlSelectionNotLocalizable,
    MutantDidNotFailIndependentOracle,
    MutantDeliveredNoFault,
    MutantScenarioMismatch,
    MutantReusedCleanTranscript,
    DefectiveTraceReusedCleanTrace,
    ObservationDefectAmbiguous,
    RuntimeArtifactDefectAmbiguous,
    RuntimeArtifactDefectMissing,
    ProductionAuditMismatch,
    AuditEvidenceEmpty,
    AuditOmissionWasAccepted,
    CounterEvidenceEmpty,
    CounterOmissionWasAccepted,
    CloseoutJoinOmissionWasAccepted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S10PhaseDefectLocalization {
    phase: S10Phase,
    source_kind: S10PhaseDefectSourceKind,
    scenario_identity: [u8; 32],
    phase_artifact_identity: [u8; 32],
    rejection_identity: [u8; 32],
    failed_check_count: u64,
    localization_identity: [u8; 32],
}

impl S10PhaseDefectLocalization {
    pub const fn phase(self) -> S10Phase {
        self.phase
    }
    pub const fn source_kind(self) -> S10PhaseDefectSourceKind {
        self.source_kind
    }
    pub const fn scenario_identity(self) -> [u8; 32] {
        self.scenario_identity
    }
    pub const fn phase_artifact_identity(self) -> [u8; 32] {
        self.phase_artifact_identity
    }
    pub const fn rejection_identity(self) -> [u8; 32] {
        self.rejection_identity
    }
    pub const fn failed_check_count(self) -> u64 {
        self.failed_check_count
    }
    pub const fn localization_identity(self) -> [u8; 32] {
        self.localization_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S10PhaseDefectSuiteDenial {
    DuplicatePhase(S10Phase),
    MissingPhase(S10Phase),
    ReusedRejectionEvidence,
    ForeignScenarioEvidence,
}

#[derive(Debug, Clone)]
pub struct S10PhaseDefectSuite {
    localizations: BTreeMap<S10Phase, S10PhaseDefectLocalization>,
    suite_identity: [u8; 32],
}

impl S10PhaseDefectSuite {
    pub fn join(
        localizations: impl IntoIterator<Item = S10PhaseDefectLocalization>,
    ) -> Result<Self, S10PhaseDefectSuiteDenial> {
        let mut by_phase = BTreeMap::new();
        let mut rejection_evidence = BTreeSet::new();
        for localization in localizations {
            if by_phase.insert(localization.phase, localization).is_some() {
                return Err(S10PhaseDefectSuiteDenial::DuplicatePhase(
                    localization.phase,
                ));
            }
            if !rejection_evidence.insert(localization.rejection_identity) {
                return Err(S10PhaseDefectSuiteDenial::ReusedRejectionEvidence);
            }
        }
        for phase in S10Phase::all() {
            if !by_phase.contains_key(&phase) {
                return Err(S10PhaseDefectSuiteDenial::MissingPhase(phase));
            }
        }
        let mut digest = Sha256::new();
        digest.update(b"worth-store-s10-phase-defect-suite-v2");
        for localization in by_phase.values() {
            digest.update(localization.localization_identity);
        }
        Ok(Self {
            localizations: by_phase,
            suite_identity: digest.finalize().into(),
        })
    }

    pub const fn suite_identity(&self) -> [u8; 32] {
        self.suite_identity
    }
    pub fn localizations(&self) -> impl Iterator<Item = &S10PhaseDefectLocalization> {
        self.localizations.values()
    }

    pub(super) fn require_scenario_membership(
        &self,
        scenario_identities: &BTreeSet<[u8; 32]>,
    ) -> Result<(), S10PhaseDefectSuiteDenial> {
        if self
            .localizations
            .values()
            .any(|localization| !scenario_identities.contains(&localization.scenario_identity))
        {
            return Err(S10PhaseDefectSuiteDenial::ForeignScenarioEvidence);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_requires_one_distinct_typed_rejection_per_phase() {
        let missing = S10PhaseDefectSuite::join((1_u8..19).map(localization)).unwrap_err();
        assert_eq!(
            missing,
            S10PhaseDefectSuiteDenial::MissingPhase(S10Phase(19))
        );

        let complete = S10PhaseDefectSuite::join((1_u8..=19).map(localization)).unwrap();
        assert_eq!(complete.localizations().count(), 19);
        assert_ne!(complete.suite_identity(), [0; 32]);
    }

    #[test]
    fn suite_rejects_reusing_one_rejection_as_multiple_phase_defects() {
        let mut localizations = (1_u8..=19).map(localization).collect::<Vec<_>>();
        localizations[1].rejection_identity = localizations[0].rejection_identity;
        assert_eq!(
            S10PhaseDefectSuite::join(localizations).unwrap_err(),
            S10PhaseDefectSuiteDenial::ReusedRejectionEvidence
        );
    }

    fn localization(phase: u8) -> S10PhaseDefectLocalization {
        S10PhaseDefectLocalization {
            phase: S10Phase(phase),
            source_kind: S10PhaseDefectSourceKind::RuntimeArtifactOmission,
            scenario_identity: [phase; 32],
            phase_artifact_identity: [phase.saturating_add(1); 32],
            rejection_identity: [phase.saturating_add(2); 32],
            failed_check_count: 1,
            localization_identity: [phase.saturating_add(3); 32],
        }
    }
}
