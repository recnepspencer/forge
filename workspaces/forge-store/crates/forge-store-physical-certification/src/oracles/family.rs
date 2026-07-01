use crate::{
    CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, NoJsonAuthorityOracle, NoPrivateMutationOracle,
    ObservedPhysicalTrace, OracleFamilyKind, PhysicalSimulationPlan, TranscriptReplayOracle,
};

use super::{
    BlockedReclaimUntilReleaseOracle, NoMixedRootOracle, OldReaderSeesOldRootOracle, OracleDenial,
    OracleVerdictBasis, PhysicalProofOracleKind, PhysicalProofOracleVerdict,
    PostSwapReaderSeesNewRootOracle,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReusablePhysicalOracleFamily {
    kind: OracleFamilyKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalOracleJudgment<O> {
    family: ReusablePhysicalOracleFamily,
    oracle: O,
}

pub trait PhysicalProofOracle: sealed::CertificationOwnedOracle {
    fn oracle_kind(&self) -> PhysicalProofOracleKind;
    fn family_kind(&self) -> OracleFamilyKind;
    fn judge_basis(
        &self,
        basis: OracleVerdictBasis,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial>;
}

impl ReusablePhysicalOracleFamily {
    pub const fn s5_readiness_shape() -> Self {
        Self {
            kind: OracleFamilyKind::S5ReadinessShape,
        }
    }

    pub const fn transcript_replay_evidence() -> Self {
        Self {
            kind: OracleFamilyKind::TranscriptReplayEvidence,
        }
    }

    pub const fn s4_recovery_dogfood() -> Self {
        Self {
            kind: OracleFamilyKind::S4RecoveryDogfood,
        }
    }

    pub const fn forbidden_shortcut_rejection() -> Self {
        Self {
            kind: OracleFamilyKind::ForbiddenShortcutRejection,
        }
    }

    pub const fn kind(&self) -> OracleFamilyKind {
        self.kind
    }

    pub const fn oracle<O: PhysicalProofOracle>(self, oracle: O) -> PhysicalOracleJudgment<O> {
        PhysicalOracleJudgment {
            family: self,
            oracle,
        }
    }
}

mod sealed {
    pub trait CertificationOwnedOracle {}
}

use sealed::CertificationOwnedOracle;

impl CertificationOwnedOracle for NoMixedRootOracle {}
impl CertificationOwnedOracle for OldReaderSeesOldRootOracle {}
impl CertificationOwnedOracle for PostSwapReaderSeesNewRootOracle {}
impl CertificationOwnedOracle for BlockedReclaimUntilReleaseOracle {}
impl CertificationOwnedOracle for CrashRecoversOldOrNewNeverMixedOracle {}
impl CertificationOwnedOracle for NoPrivateMutationOracle {}
impl CertificationOwnedOracle for NoJsonAuthorityOracle {}
impl CertificationOwnedOracle for CounterContractOracle {}
impl CertificationOwnedOracle for TranscriptReplayOracle {}
impl CertificationOwnedOracle for IndependentVerifierAgreementOracle {}

impl<O: PhysicalProofOracle> PhysicalOracleJudgment<O> {
    pub fn judge(
        &self,
        plan: &PhysicalSimulationPlan,
        trace: &ObservedPhysicalTrace,
    ) -> Result<PhysicalProofOracleVerdict, OracleDenial> {
        if !plan.oracle_families().contains(self.family.kind) {
            return Err(OracleDenial::OracleFamilyNotRequired {
                family: self.family.kind,
            });
        }
        if self.oracle.family_kind() != self.family.kind {
            return Err(OracleDenial::OracleFamilyMismatch {
                family: self.family.kind,
                oracle: self.oracle.oracle_kind(),
            });
        }
        let basis = OracleVerdictBasis::from_plan_and_trace(plan, trace)?;
        self.oracle.judge_basis(basis)
    }
}
