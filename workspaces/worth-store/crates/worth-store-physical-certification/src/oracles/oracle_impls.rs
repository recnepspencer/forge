use worth_store_io_scheduler::LatencyEnvelopeAssessmentStatus;

use crate::{
    IndependentVerifierObservationKind, OracleFamilyKind, RecoveryOutcomeKind,
    ShortcutRejectionObservationKind,
};

use super::{
    OracleDenial, OracleVerdictBasis, PhysicalOracleNonClaim, PhysicalProofOracle,
    PhysicalProofOracleKind, PhysicalProofOracleVerdict,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashRecoversOldOrNewNeverMixedOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoPrivateMutationOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoJsonAuthorityOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CounterContractOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TranscriptReplayOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IndependentVerifierAgreementOracle;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IoPressureSimulationOracle;

impl PhysicalProofOracle for CrashRecoversOldOrNewNeverMixedOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::CrashRecoversOldOrNewNeverMixed
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::RecoveryDogfood
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let outcome = basis
            .recovery_outcome()
            .map(|observation| observation.kind())
            .ok_or(OracleDenial::MissingRecoveryOutcomeObservation)?;

        match outcome {
            RecoveryOutcomeKind::RecoveredOldRoot | RecoveryOutcomeKind::RecoveredNewRoot => {
                Ok(PhysicalProofOracleVerdict::satisfied(
                    self.family_kind(),
                    self.oracle_kind(),
                    basis,
                    [],
                ))
            }
            RecoveryOutcomeKind::MixedRoot => Ok(PhysicalProofOracleVerdict::failed(
                self.family_kind(),
                self.oracle_kind(),
                basis,
                [],
            )),
        }
    }
}

impl PhysicalProofOracle for NoPrivateMutationOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::NoPrivateMutation
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::ForbiddenShortcutRejection
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_shortcut_rejection(
            &basis,
            ShortcutRejectionObservationKind::PrivateMutationDenied,
        )?;
        Ok(PhysicalProofOracleVerdict::satisfied(
            self.family_kind(),
            self.oracle_kind(),
            basis,
            [],
        ))
    }
}

impl PhysicalProofOracle for NoJsonAuthorityOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::NoJsonAuthority
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::ForbiddenShortcutRejection
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        require_shortcut_rejection(
            &basis,
            ShortcutRejectionObservationKind::JsonAuthorityDenied,
        )?;
        Ok(PhysicalProofOracleVerdict::satisfied(
            self.family_kind(),
            self.oracle_kind(),
            basis,
            [],
        ))
    }
}

impl PhysicalProofOracle for CounterContractOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::CounterContract
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::PhysicalIsolationReadinessShape
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        Ok(PhysicalProofOracleVerdict::satisfied(
            self.family_kind(),
            self.oracle_kind(),
            basis,
            [PhysicalOracleNonClaim::PhysicalIsolationCorrectness],
        ))
    }
}

impl PhysicalProofOracle for TranscriptReplayOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::TranscriptReplay
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::TranscriptReplayEvidence
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        Ok(PhysicalProofOracleVerdict::satisfied(
            self.family_kind(),
            self.oracle_kind(),
            basis,
            [],
        ))
    }
}

impl PhysicalProofOracle for IndependentVerifierAgreementOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::IndependentVerifierAgreement
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::PhysicalIsolationReadinessShape
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        let observation_kind = basis
            .independent_verifier()
            .map(|observation| observation.kind())
            .ok_or(OracleDenial::MissingIndependentVerifierObservation)?;

        match observation_kind {
            IndependentVerifierObservationKind::Agreement => {
                Ok(PhysicalProofOracleVerdict::satisfied(
                    self.family_kind(),
                    self.oracle_kind(),
                    basis,
                    [PhysicalOracleNonClaim::PhysicalIsolationCorrectness],
                ))
            }
            IndependentVerifierObservationKind::Disagreement => {
                Ok(PhysicalProofOracleVerdict::failed(
                    self.family_kind(),
                    self.oracle_kind(),
                    basis,
                    [PhysicalOracleNonClaim::PhysicalIsolationCorrectness],
                ))
            }
        }
    }
}

impl PhysicalProofOracle for IoPressureSimulationOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind {
        PhysicalProofOracleKind::IoPressureSimulation
    }

    fn family_kind(&self) -> OracleFamilyKind {
        OracleFamilyKind::IoPressureSimulation
    }

    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        if basis.scenario_family() != crate::PhysicalSimulationScenarioFamily::IoPressureHarness {
            return Err(OracleDenial::PlanTraceIdentityMismatch);
        }
        let observation = basis
            .io_pressure()
            .ok_or(OracleDenial::MissingIoPressureObservation)?;
        if !observation.attribution_complete() {
            return Err(OracleDenial::IncompleteIoPressureAttribution);
        }
        if observation.envelope_status() != LatencyEnvelopeAssessmentStatus::Held {
            return Ok(PhysicalProofOracleVerdict::failed(
                self.family_kind(),
                self.oracle_kind(),
                basis,
                [],
            ));
        }
        Ok(PhysicalProofOracleVerdict::satisfied(
            self.family_kind(),
            self.oracle_kind(),
            basis,
            [],
        ))
    }
}

fn require_shortcut_rejection(
    basis: &OracleVerdictBasis,
    required: ShortcutRejectionObservationKind,
) -> Result<(), OracleDenial> {
    if basis.has_shortcut_rejection(required) {
        Ok(())
    } else {
        Err(OracleDenial::MissingRequiredShortcutRejectionObservation { required })
    }
}
