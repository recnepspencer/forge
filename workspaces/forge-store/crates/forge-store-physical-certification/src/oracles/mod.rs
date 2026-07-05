mod basis;
mod family;
mod forbidden_sources;
mod oracle_impls;
mod s5_physical_isolation;
mod s5_readiness;
mod verdict;

pub use basis::OracleVerdictBasis;
pub use family::{PhysicalOracleJudgment, PhysicalProofOracle, ReusablePhysicalOracleFamily};
pub use forbidden_sources::{
    expected_error_text_oracle_attempt, fixture_label_oracle_attempt, log_only_oracle_attempt,
    same_run_self_comparison_oracle_attempt, test_support_oracle_verdict_attempt,
};
pub use oracle_impls::{
    CounterContractOracle, CrashRecoversOldOrNewNeverMixedOracle,
    IndependentVerifierAgreementOracle, NoJsonAuthorityOracle, NoPrivateMutationOracle,
    S6IoPressureSimulationOracle, TranscriptReplayOracle,
};
pub use s5_physical_isolation::S5PhysicalIsolationInterleavingOracle;
pub use s5_readiness::{
    BlockedReclaimUntilReleaseOracle, NoMixedRootOracle, OldReaderSeesOldRootOracle,
    PostSwapReaderSeesNewRootOracle,
};
pub use verdict::{
    phase7_verdict_topology, OracleDenial, PhysicalOracleNonClaim, PhysicalOracleVerdictTopology,
    PhysicalOracleVerdictTopologyPosture, PhysicalProofOracleKind, PhysicalProofOracleVerdict,
    PhysicalProofOracleVerdictKind,
};
