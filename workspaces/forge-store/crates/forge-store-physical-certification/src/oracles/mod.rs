mod basis;
mod blob_harness;
mod family;
mod forbidden_sources;
mod oracle_impls;
mod physical_isolation;
mod verdict;

pub use basis::OracleVerdictBasis;
pub use blob_harness::{
    BlobByteEqualityOracle, BlobChunkOrderingOracle, BlobConstantMemoryOracle,
    BlobDigestChecksumDistinctionOracle, BlobHeavyCleanupOracle, BlobHeavyPatternLaneOracle,
    BlobHeavyQualificationEvidenceOracle, BlobNoCrossScopeDedupeOracle, BlobNoSidecarPathOracle,
    BlobReachabilityOracle,
};
pub use family::{PhysicalOracleJudgment, PhysicalProofOracle, ReusablePhysicalOracleFamily};
pub use forbidden_sources::{
    expected_error_text_oracle_attempt, fixture_label_oracle_attempt, log_only_oracle_attempt,
    same_run_self_comparison_oracle_attempt, test_support_oracle_verdict_attempt,
};
pub use oracle_impls::{
    CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, IoPressureSimulationOracle, NoJsonAuthorityOracle,
    NoPrivateMutationOracle, TranscriptReplayOracle,
};
pub use physical_isolation::{
    BlockedReclaimUntilReleaseOracle, NoMixedRootOracle, OldReaderSeesOldRootOracle,
    PhysicalIsolationInterleavingOracle, PostSwapReaderSeesNewRootOracle,
};
pub use verdict::{
    phase7_verdict_topology, OracleDenial, PhysicalOracleNonClaim, PhysicalOracleVerdictTopology,
    PhysicalOracleVerdictTopologyPosture, PhysicalProofOracleKind, PhysicalProofOracleVerdict,
    PhysicalProofOracleVerdictKind,
};
