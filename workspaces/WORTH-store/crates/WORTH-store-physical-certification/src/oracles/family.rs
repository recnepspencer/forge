use crate::{
    BlobByteEqualityOracle, BlobChunkOrderingOracle, BlobConstantMemoryOracle,
    BlobHeavyCleanupOracle, BlobHeavyPatternLaneOracle, BlobHeavyQualificationEvidenceOracle,
    BlobDigestChecksumDistinctionOracle, BlobNoCrossScopeDedupeOracle, BlobNoSidecarPathOracle,
    BlobReachabilityOracle, CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, NoJsonAuthorityOracle, NoPrivateMutationOracle,
    ObservedPhysicalTrace, OracleFamilyKind, PhysicalSimulationPlan,
    S5PhysicalIsolationInterleavingOracle, S6IoPressureSimulationOracle, TranscriptReplayOracle,
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

    pub const fn s5_physical_isolation_interleaving() -> Self {
        Self {
            kind: OracleFamilyKind::S5PhysicalIsolationInterleaving,
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

    pub const fn s6_io_pressure_simulation() -> Self {
        Self {
            kind: OracleFamilyKind::S6IoPressureSimulation,
        }
    }

    pub const fn s7_blob_harness_evidence() -> Self {
        Self {
            kind: OracleFamilyKind::S7BlobHarnessEvidence,
        }
    }

    pub const fn s7_blob_heavy_qualification() -> Self {
        Self {
            kind: OracleFamilyKind::S7BlobHeavyQualification,
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
impl CertificationOwnedOracle for S5PhysicalIsolationInterleavingOracle {}
impl CertificationOwnedOracle for S6IoPressureSimulationOracle {}
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
