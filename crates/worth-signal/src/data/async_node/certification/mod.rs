use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod compile_time;
pub mod matrix;
pub mod performance;
pub mod run;

pub use compile_time::{
    async_node_compile_time_boundary_proof, AsyncNodeCompileTimeBoundaryProof,
    REQUIRED_ASYNC_NODE_COMPILE_TIME_FIXTURES,
};
pub use matrix::{
    async_node_milestone_d_scenario_matrix, AsyncNodeMilestoneDScenarioEvidenceKind,
    AsyncNodeMilestoneDScenarioInputs, AsyncNodeMilestoneDScenarioMatrix,
    AsyncNodeMilestoneDScenarioMatrixSummary, AsyncNodeMilestoneDScenarioRow,
};
pub use performance::{
    async_node_milestone_d_performance_closeout, AsyncNodeMilestoneDPerformanceCloseout,
    AsyncNodeMilestoneDPerformanceCloseoutRow, AsyncNodeMilestoneDPerformanceCloseoutSummary,
};
pub use run::{
    async_node_milestone_d_certification_run, AsyncNodeMilestoneDCertificationRun,
    AsyncNodeMilestoneDCertificationRunSummary,
};

pub const ASYNC_NODE_MILESTONE_D_SCENARIO_MATRIX_SCHEMA_VERSION: &str =
    "worth-signal-async-node-milestone-d-scenario-matrix-v1";
pub const ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION: &str =
    "worth-signal-async-node-milestone-d-performance-closeout-v1";
pub const ASYNC_NODE_MILESTONE_D_CERTIFICATION_RUN_SCHEMA_VERSION: &str =
    "worth-signal-async-node-milestone-d-certification-run-v1";
pub const ASYNC_NODE_COMPILE_TIME_BOUNDARY_PROOF_SCHEMA_VERSION: &str =
    "worth-signal-async-node-compile-time-boundary-proof-v1";

pub const REQUIRED_ASYNC_NODE_MILESTONE_D_SCENARIOS: [AsyncNodeMilestoneDScenarioId; 8] = [
    AsyncNodeMilestoneDScenarioId::AsyncCapabilityAttachmentEquivalence,
    AsyncNodeMilestoneDScenarioId::ConditionGatedAsyncAdmissionParity,
    AsyncNodeMilestoneDScenarioId::AspectScopedAsyncCapability,
    AsyncNodeMilestoneDScenarioId::PreviousValueAndTemporalAsyncCapabilityParity,
    AsyncNodeMilestoneDScenarioId::InteriorAsyncNodeGateEquivalence,
    AsyncNodeMilestoneDScenarioId::HierarchicalAsyncCapabilityReplayAndCancellation,
    AsyncNodeMilestoneDScenarioId::LegacyResourceAliasCompatibility,
    AsyncNodeMilestoneDScenarioId::AsyncCapabilityCompileTimeBoundary,
];

pub const REQUIRED_ASYNC_NODE_MILESTONE_D_PERFORMANCE_CLAIMS:
    [AsyncNodeMilestoneDPerformanceClaimId; 6] = [
    AsyncNodeMilestoneDPerformanceClaimId::AttachmentEquivalenceBounded,
    AsyncNodeMilestoneDPerformanceClaimId::ConditionAdmissionBoundaryBounded,
    AsyncNodeMilestoneDPerformanceClaimId::AspectScopedBreadthBounded,
    AsyncNodeMilestoneDPerformanceClaimId::InteriorGateCoordinationBounded,
    AsyncNodeMilestoneDPerformanceClaimId::HierarchyReplayRestoreBounded,
    AsyncNodeMilestoneDPerformanceClaimId::LegacyAliasCompatibilityBounded,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeMilestoneDScenarioId {
    AsyncCapabilityAttachmentEquivalence,
    ConditionGatedAsyncAdmissionParity,
    AspectScopedAsyncCapability,
    PreviousValueAndTemporalAsyncCapabilityParity,
    InteriorAsyncNodeGateEquivalence,
    HierarchicalAsyncCapabilityReplayAndCancellation,
    LegacyResourceAliasCompatibility,
    AsyncCapabilityCompileTimeBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AsyncNodeMilestoneDPerformanceClaimId {
    AttachmentEquivalenceBounded,
    ConditionAdmissionBoundaryBounded,
    AspectScopedBreadthBounded,
    InteriorGateCoordinationBounded,
    HierarchyReplayRestoreBounded,
    LegacyAliasCompatibilityBounded,
}

pub(crate) fn canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("async-node certification digest serialization");
    let digest = Sha256::digest(bytes);
    format!("{digest:x}")
}
