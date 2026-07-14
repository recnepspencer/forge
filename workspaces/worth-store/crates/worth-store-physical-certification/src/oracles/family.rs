use crate::{
    BlobByteEqualityOracle, BlobChunkOrderingOracle, BlobConstantMemoryOracle,
    BlobDigestChecksumDistinctionOracle, BlobHeavyCleanupOracle, BlobHeavyPatternLaneOracle,
    BlobHeavyQualificationEvidenceOracle, BlobNoCrossScopeDedupeOracle, BlobNoSidecarPathOracle,
    BlobReachabilityOracle, CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, IoPressureSimulationOracle, NoJsonAuthorityOracle,
    NoPrivateMutationOracle, ObservedPhysicalTrace, OracleFamilyKind,
    PhysicalIsolationInterleavingOracle, PhysicalSimulationPlan, TranscriptReplayOracle,
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
    pub const fn physical_isolation_readiness_shape() -> Self {
        Self {
            kind: OracleFamilyKind::PhysicalIsolationReadinessShape,
        }
    }

    pub const fn physical_isolation_interleaving() -> Self {
        Self {
            kind: OracleFamilyKind::PhysicalIsolationInterleaving,
        }
    }

    pub const fn transcript_replay_evidence() -> Self {
        Self {
            kind: OracleFamilyKind::TranscriptReplayEvidence,
        }
    }

    pub const fn recovery_dogfood() -> Self {
        Self {
            kind: OracleFamilyKind::RecoveryDogfood,
        }
    }

    pub const fn forbidden_shortcut_rejection() -> Self {
        Self {
            kind: OracleFamilyKind::ForbiddenShortcutRejection,
        }
    }

    pub const fn io_pressure_simulation() -> Self {
        Self {
            kind: OracleFamilyKind::IoPressureSimulation,
        }
    }

    pub const fn blob_harness_evidence() -> Self {
        Self {
            kind: OracleFamilyKind::BlobHarnessEvidence,
        }
    }

    pub const fn blob_heavy_qualification() -> Self {
        Self {
            kind: OracleFamilyKind::BlobHeavyQualification,
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
impl CertificationOwnedOracle for PhysicalIsolationInterleavingOracle {}
impl CertificationOwnedOracle for IoPressureSimulationOracle {}
impl CertificationOwnedOracle for BlobByteEqualityOracle {}
impl CertificationOwnedOracle for BlobChunkOrderingOracle {}
impl CertificationOwnedOracle for BlobDigestChecksumDistinctionOracle {}
impl CertificationOwnedOracle for BlobNoSidecarPathOracle {}
impl CertificationOwnedOracle for BlobNoCrossScopeDedupeOracle {}
impl CertificationOwnedOracle for BlobConstantMemoryOracle {}
impl CertificationOwnedOracle for BlobReachabilityOracle {}
impl CertificationOwnedOracle for BlobHeavyQualificationEvidenceOracle {}
impl CertificationOwnedOracle for BlobHeavyCleanupOracle {}
impl CertificationOwnedOracle for BlobHeavyPatternLaneOracle {}

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
