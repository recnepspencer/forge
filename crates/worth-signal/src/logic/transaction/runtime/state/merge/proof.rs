mod digest;
mod replay;
mod reports;

use serde::{Deserialize, Serialize};

use super::compatibility::SignalMergeCompatibilityWitness;
use super::scoped_proof::ScopedMergeProofPacket;
use super::strategy_witness::SignalMergeStrategyWitness;

pub const MERGE_PROOF_SCHEMA_VERSION: &str = "worth-signal-proof-v1";
pub const BRANCH_STATE_PROOF_BASIS_VERSION: &str = "worth-signal-branch-state-v3";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RuntimeProofReport {
    pub proof_schema_version: String,
    pub schema_registry_digest: String,
    pub merge_strategy_registry_digest: String,
    pub merge_base_strategy_registry_digest: String,
    pub aspect_merge_policy_registry_digest: String,
    pub conflict_isolation_registry_digest: String,
    pub conflict_policy_registry_digest: String,
    pub identity_matcher_registry_digest: String,
    pub source_only_policy_registry_digest: String,
    pub deletion_policy_registry_digest: String,
    pub registry_bundle_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergePlanProofReport {
    pub proof_schema_version: String,
    pub registry_bundle_digest: String,
    pub plan_digest: String,
    pub semantics_digest: String,
    pub lowered_strategy_bundle_digest: String,
    pub selected_strategy_digest: String,
    pub selected_merge_base_digest: String,
    pub selected_conflict_policy_digest: String,
    pub selected_conflict_isolation_digest: String,
    pub selected_identity_matcher_digest: String,
    pub selected_source_only_policy_digest: String,
    pub selected_deletion_policy_digest: String,
    #[serde(with = "crate::diagnostics::model::replay_strategy_witness_serde")]
    pub strategy_witness: SignalMergeStrategyWitness,
    pub scoped_merge_proof: ScopedMergeProofPacket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MergeResultProofReport {
    pub proof_schema_version: String,
    pub registry_bundle_digest: String,
    pub result_digest: String,
    pub semantics_digest: String,
    pub lowered_strategy_bundle_digest: String,
    pub lineage_digest: String,
    pub selected_strategy_digest: String,
    pub selected_merge_base_digest: String,
    pub selected_conflict_policy_digest: String,
    pub selected_conflict_isolation_digest: String,
    pub selected_identity_matcher_digest: String,
    pub selected_source_only_policy_digest: String,
    pub selected_deletion_policy_digest: String,
    #[serde(with = "crate::diagnostics::model::replay_strategy_witness_serde")]
    pub strategy_witness: SignalMergeStrategyWitness,
    pub compatibility_witness: SignalMergeCompatibilityWitness,
    pub scoped_merge_proof: ScopedMergeProofPacket,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchStateDenseGridProofBasis {
    pub family_id: String,
    pub width: u32,
    pub height: u32,
    pub key_count: usize,
    pub ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchStateProofBasis<TStore> {
    pub proof_schema_version: String,
    pub catalog_ids: Vec<String>,
    pub dense_grids: Vec<BranchStateDenseGridProofBasis>,
    pub store: TStore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BranchStateProofReport {
    pub proof_schema_version: String,
    pub branch_id: u64,
    pub branch_name: String,
    pub snapshot_id: Option<u64>,
    pub state_digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ReplayMismatchClass {
    LegacyMergeArtifactUnsupported,
    ProofSchemaVersionMismatch,
    MissingRegistryBundleDigest,
    RegistryBundleDigestMismatch,
    MissingLoweredStrategyBundleDigest,
    LoweredStrategyBundleDigestMismatch,
    MissingMergePlanDigest,
    MergePlanDigestMismatch,
    MissingMergeResultDigest,
    MergeResultDigestMismatch,
    MissingLineageDigest,
    LineageDigestMismatch,
    MissingStrategyWitness,
    StrategyWitnessMismatch,
    MissingCompatibilityWitness,
    CompatibilityWitnessMismatch,
    MissingScopedMergeProof,
    ScopedMergeProofMismatch,
    BranchStateDigestMismatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayParityProofReport {
    pub proof_schema_version: String,
    pub expected_branch_id: u64,
    pub expected_branch_name: String,
    pub expected_snapshot_id: Option<u64>,
    pub expected_state_digest: String,
    pub replayed_branch_id: u64,
    pub replayed_branch_name: String,
    pub replayed_snapshot_id: Option<u64>,
    pub replayed_state_digest: String,
    pub parity: bool,
    pub mismatch_classes: Vec<ReplayMismatchClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayArtifactProofInput {
    pub proof_schema_version: String,
    pub registry_bundle_digest: Option<String>,
    pub lowered_strategy_bundle_digest: Option<String>,
    pub merge_plan_digest: Option<String>,
    pub merge_result_digest: Option<String>,
    pub lineage_digest: Option<String>,
    #[serde(
        default,
        serialize_with = "crate::diagnostics::model::replay_strategy_witness_serde::serialize_option",
        deserialize_with = "crate::diagnostics::model::replay_strategy_witness_serde::deserialize_option"
    )]
    pub strategy_witness: Option<SignalMergeStrategyWitness>,
    pub compatibility_witness: Option<SignalMergeCompatibilityWitness>,
    pub scoped_merge_proof: Option<ScopedMergeProofPacket>,
    pub branch_state_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayArtifactProofReport {
    pub proof_schema_version: String,
    pub expected: ReplayArtifactProofInput,
    pub replayed: ReplayArtifactProofInput,
    pub parity: bool,
    pub mismatch_classes: Vec<ReplayMismatchClass>,
}

pub use digest::{
    branch_state_proof_report, canonical_digest, lowered_strategy_bundle_digest,
    merge_lineage_digest,
};
pub use replay::{replay_artifact_proof_report, replay_parity_proof_report};
pub use reports::{merge_plan_proof_report, merge_result_proof_report, runtime_proof_report};
