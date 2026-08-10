use serde::Serialize;
use sha2::{Digest, Sha256};

use super::super::plan::{
    LoweredAspectMergeDecisionPlan, LoweredAspectMergePolicyPlan, LoweredConflictIsolationPlan,
    LoweredDeletionPolicyPlan, LoweredMergeBasePlan,
};
use super::super::result::MergedArtifactRecord;
use super::super::semantics::SelectedMergeSemanticsBundle;
use super::super::{BranchMergePlan, BranchMergeResult};

use super::{BranchStateProofReport, MERGE_PROOF_SCHEMA_VERSION};

#[derive(Debug, Clone, Serialize)]
pub(super) struct CanonicalRegistryBundleDigestBasis<'a> {
    pub(super) proof_schema_version: &'static str,
    pub(super) schema_registry_digest: &'a str,
    pub(super) merge_strategy_registry_digest: &'a str,
    pub(super) merge_base_strategy_registry_digest: &'a str,
    pub(super) aspect_merge_policy_registry_digest: &'a str,
    pub(super) conflict_isolation_registry_digest: &'a str,
    pub(super) conflict_policy_registry_digest: &'a str,
    pub(super) identity_matcher_registry_digest: &'a str,
    pub(super) source_only_policy_registry_digest: &'a str,
    pub(super) deletion_policy_registry_digest: &'a str,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CanonicalMergePlanDigestBasis<'a> {
    pub(super) proof_schema_version: &'static str,
    pub(super) plan: &'a BranchMergePlan,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CanonicalMergeResultDigestBasis<'a> {
    pub(super) proof_schema_version: &'static str,
    pub(super) result: &'a BranchMergeResult,
}

#[derive(Debug, Clone, Serialize)]
pub(super) struct CanonicalMergeLineageDigestBasis<'a> {
    proof_schema_version: &'static str,
    source_branch: u64,
    target_branch: u64,
    source_snapshot_id: Option<u64>,
    target_snapshot_id_before: Option<u64>,
    target_snapshot_id_after: Option<u64>,
    records: &'a [MergedArtifactRecord],
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalLoweredStrategyBundleDigestBasis<'a> {
    proof_schema_version: &'static str,
    selected_semantics: &'a SelectedMergeSemanticsBundle,
    merge_base: Option<&'a LoweredMergeBasePlan>,
    deletion_policy: &'a LoweredDeletionPolicyPlan,
    conflict_isolation: &'a LoweredConflictIsolationPlan,
    aspect_policies: &'a LoweredAspectMergePolicyPlan,
    aspect_decisions: &'a LoweredAspectMergeDecisionPlan,
}

pub fn canonical_digest<T: Serialize>(value: &T) -> String {
    let bytes = serde_json::to_vec(value).expect("merge proof serialization");
    let digest = Sha256::digest(bytes);
    digest
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>()
}

pub fn lowered_strategy_bundle_digest(
    selected_semantics: &SelectedMergeSemanticsBundle,
    merge_base: Option<&LoweredMergeBasePlan>,
    deletion_policy: &LoweredDeletionPolicyPlan,
    conflict_isolation: &LoweredConflictIsolationPlan,
    aspect_policies: &LoweredAspectMergePolicyPlan,
    aspect_decisions: &LoweredAspectMergeDecisionPlan,
) -> String {
    canonical_digest(&CanonicalLoweredStrategyBundleDigestBasis {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION,
        selected_semantics,
        merge_base,
        deletion_policy,
        conflict_isolation,
        aspect_policies,
        aspect_decisions,
    })
}

pub fn merge_lineage_digest(result: &BranchMergeResult) -> String {
    canonical_digest(&CanonicalMergeLineageDigestBasis {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION,
        source_branch: result.source_branch.0,
        target_branch: result.target_branch.0,
        source_snapshot_id: result.source_snapshot_id.map(|id| id.0),
        target_snapshot_id_before: result.target_snapshot_id_before.map(|id| id.0),
        target_snapshot_id_after: result.target_snapshot_id_after.map(|id| id.0),
        records: &result.records,
    })
}

pub fn branch_state_proof_report<T: Serialize>(
    branch_id: u64,
    branch_name: impl Into<String>,
    snapshot_id: Option<u64>,
    basis_version: &str,
    state_basis: &T,
) -> BranchStateProofReport {
    BranchStateProofReport {
        proof_schema_version: format!("{MERGE_PROOF_SCHEMA_VERSION}:{basis_version}"),
        branch_id,
        branch_name: branch_name.into(),
        snapshot_id,
        state_digest: canonical_digest(state_basis),
    }
}
