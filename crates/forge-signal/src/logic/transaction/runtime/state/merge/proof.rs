use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::compatibility::SignalMergeCompatibilityWitness;
use super::plan::{
    LoweredAspectMergeDecisionPlan, LoweredAspectMergePolicyPlan, LoweredConflictIsolationPlan,
    LoweredDeletionPolicyPlan, LoweredMergeBasePlan,
};
use super::result::MergedArtifactRecord;
use super::scoped_proof::ScopedMergeProofPacket;
use super::semantics::SelectedMergeSemanticsBundle;
use super::strategy_witness::SignalMergeStrategyWitness;
use super::{BranchMergePlan, BranchMergeResult};

pub const MERGE_PROOF_SCHEMA_VERSION: &str = "forge-signal-proof-v1";
pub const BRANCH_STATE_PROOF_BASIS_VERSION: &str = "forge-signal-branch-state-v3";

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

#[derive(Debug, Clone, Serialize)]
struct CanonicalRegistryBundleDigestBasis<'a> {
    proof_schema_version: &'static str,
    schema_registry_digest: &'a str,
    merge_strategy_registry_digest: &'a str,
    merge_base_strategy_registry_digest: &'a str,
    aspect_merge_policy_registry_digest: &'a str,
    conflict_isolation_registry_digest: &'a str,
    conflict_policy_registry_digest: &'a str,
    identity_matcher_registry_digest: &'a str,
    source_only_policy_registry_digest: &'a str,
    deletion_policy_registry_digest: &'a str,
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalMergePlanDigestBasis<'a> {
    proof_schema_version: &'static str,
    plan: &'a BranchMergePlan,
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalMergeResultDigestBasis<'a> {
    proof_schema_version: &'static str,
    result: &'a BranchMergeResult,
}

#[derive(Debug, Clone, Serialize)]
struct CanonicalMergeLineageDigestBasis<'a> {
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

fn replay_mismatch_classes(
    expected: &ReplayArtifactProofInput,
    replayed: &ReplayArtifactProofInput,
) -> Vec<ReplayMismatchClass> {
    let mut mismatch_classes = Vec::new();
    let compare_optional =
        |left: &Option<String>,
         right: &Option<String>,
         missing_class: ReplayMismatchClass,
         mismatch_class: ReplayMismatchClass,
         output: &mut Vec<ReplayMismatchClass>| match (left, right) {
            (Some(left), Some(right)) => {
                if left != right {
                    output.push(mismatch_class);
                }
            }
            (None, Some(_)) | (Some(_), None) => output.push(missing_class),
            (None, None) => {}
        };
    if !expected
        .proof_schema_version
        .starts_with(MERGE_PROOF_SCHEMA_VERSION)
        || !replayed
            .proof_schema_version
            .starts_with(MERGE_PROOF_SCHEMA_VERSION)
    {
        mismatch_classes.push(ReplayMismatchClass::LegacyMergeArtifactUnsupported);
    }
    if expected.proof_schema_version != replayed.proof_schema_version {
        mismatch_classes.push(ReplayMismatchClass::ProofSchemaVersionMismatch);
    }
    compare_optional(
        &expected.registry_bundle_digest,
        &replayed.registry_bundle_digest,
        ReplayMismatchClass::MissingRegistryBundleDigest,
        ReplayMismatchClass::RegistryBundleDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.lowered_strategy_bundle_digest,
        &replayed.lowered_strategy_bundle_digest,
        ReplayMismatchClass::MissingLoweredStrategyBundleDigest,
        ReplayMismatchClass::LoweredStrategyBundleDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.merge_plan_digest,
        &replayed.merge_plan_digest,
        ReplayMismatchClass::MissingMergePlanDigest,
        ReplayMismatchClass::MergePlanDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.merge_result_digest,
        &replayed.merge_result_digest,
        ReplayMismatchClass::MissingMergeResultDigest,
        ReplayMismatchClass::MergeResultDigestMismatch,
        &mut mismatch_classes,
    );
    compare_optional(
        &expected.lineage_digest,
        &replayed.lineage_digest,
        ReplayMismatchClass::MissingLineageDigest,
        ReplayMismatchClass::LineageDigestMismatch,
        &mut mismatch_classes,
    );
    match (&expected.strategy_witness, &replayed.strategy_witness) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::StrategyWitnessMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingStrategyWitness);
        }
        (None, None) => {}
    }
    match (&expected.scoped_merge_proof, &replayed.scoped_merge_proof) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::ScopedMergeProofMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingScopedMergeProof);
        }
        (None, None) => {}
    }
    match (
        &expected.compatibility_witness,
        &replayed.compatibility_witness,
    ) {
        (Some(left), Some(right)) => {
            if left != right {
                mismatch_classes.push(ReplayMismatchClass::CompatibilityWitnessMismatch);
            }
        }
        (None, Some(_)) | (Some(_), None) => {
            mismatch_classes.push(ReplayMismatchClass::MissingCompatibilityWitness);
        }
        (None, None) => {}
    }
    if expected.branch_state_digest != replayed.branch_state_digest {
        mismatch_classes.push(ReplayMismatchClass::BranchStateDigestMismatch);
    }
    mismatch_classes
}

pub fn replay_parity_proof_report(
    expected_branch_id: u64,
    expected_branch_name: impl Into<String>,
    expected_snapshot_id: Option<u64>,
    expected_state_digest: impl Into<String>,
    replayed_branch_id: u64,
    replayed_branch_name: impl Into<String>,
    replayed_snapshot_id: Option<u64>,
    replayed_state_digest: impl Into<String>,
) -> ReplayParityProofReport {
    let expected_state_digest = expected_state_digest.into();
    let replayed_state_digest = replayed_state_digest.into();
    let mismatch_classes = if expected_state_digest == replayed_state_digest {
        Vec::new()
    } else {
        vec![ReplayMismatchClass::BranchStateDigestMismatch]
    };
    ReplayParityProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        expected_branch_id,
        expected_branch_name: expected_branch_name.into(),
        expected_snapshot_id,
        expected_state_digest: expected_state_digest.clone(),
        replayed_branch_id,
        replayed_branch_name: replayed_branch_name.into(),
        replayed_snapshot_id,
        replayed_state_digest: replayed_state_digest.clone(),
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

pub fn replay_artifact_proof_report(
    expected: ReplayArtifactProofInput,
    replayed: ReplayArtifactProofInput,
) -> ReplayArtifactProofReport {
    let mismatch_classes = replay_mismatch_classes(&expected, &replayed);
    ReplayArtifactProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        expected,
        replayed,
        parity: mismatch_classes.is_empty(),
        mismatch_classes,
    }
}

pub fn runtime_proof_report(
    schema_registry_digest: &str,
    merge_strategy_registry_digest: &str,
    merge_base_strategy_registry_digest: &str,
    aspect_merge_policy_registry_digest: &str,
    conflict_isolation_registry_digest: &str,
    conflict_policy_registry_digest: &str,
    identity_matcher_registry_digest: &str,
    source_only_policy_registry_digest: &str,
    deletion_policy_registry_digest: &str,
) -> RuntimeProofReport {
    let registry_bundle_digest = canonical_digest(&CanonicalRegistryBundleDigestBasis {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION,
        schema_registry_digest,
        merge_strategy_registry_digest,
        merge_base_strategy_registry_digest,
        aspect_merge_policy_registry_digest,
        conflict_isolation_registry_digest,
        conflict_policy_registry_digest,
        identity_matcher_registry_digest,
        source_only_policy_registry_digest,
        deletion_policy_registry_digest,
    });
    RuntimeProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        schema_registry_digest: schema_registry_digest.to_owned(),
        merge_strategy_registry_digest: merge_strategy_registry_digest.to_owned(),
        merge_base_strategy_registry_digest: merge_base_strategy_registry_digest.to_owned(),
        aspect_merge_policy_registry_digest: aspect_merge_policy_registry_digest.to_owned(),
        conflict_isolation_registry_digest: conflict_isolation_registry_digest.to_owned(),
        conflict_policy_registry_digest: conflict_policy_registry_digest.to_owned(),
        identity_matcher_registry_digest: identity_matcher_registry_digest.to_owned(),
        source_only_policy_registry_digest: source_only_policy_registry_digest.to_owned(),
        deletion_policy_registry_digest: deletion_policy_registry_digest.to_owned(),
        registry_bundle_digest,
    }
}

pub fn merge_plan_proof_report(
    plan: &BranchMergePlan,
    registry_bundle_digest: &str,
) -> MergePlanProofReport {
    let semantics_digest = canonical_digest(plan.selected_semantics());
    let lowered_strategy_bundle_digest = plan.lowered_strategy_bundle_digest().to_owned();
    MergePlanProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: registry_bundle_digest.to_owned(),
        plan_digest: canonical_digest(&CanonicalMergePlanDigestBasis {
            proof_schema_version: MERGE_PROOF_SCHEMA_VERSION,
            plan,
        }),
        semantics_digest,
        lowered_strategy_bundle_digest,
        selected_strategy_digest: plan.selected_strategy_digest().to_owned(),
        selected_merge_base_digest: plan
            .lowered_merge_base()
            .map(|base| base.selected_merge_base_digest.clone())
            .expect("merge-base plan"),
        selected_conflict_policy_digest: plan.selected_conflict_policy_digest().to_owned(),
        selected_conflict_isolation_digest: plan.selected_conflict_isolation_digest().to_owned(),
        selected_identity_matcher_digest: plan.selected_identity_matcher_digest().to_owned(),
        selected_source_only_policy_digest: plan.selected_source_only_policy_digest().to_owned(),
        selected_deletion_policy_digest: plan.selected_deletion_policy_digest().to_owned(),
        strategy_witness: plan.strategy_witness().clone(),
        scoped_merge_proof: plan.scoped_merge_proof().clone(),
    }
}

pub fn merge_result_proof_report(result: &BranchMergeResult) -> MergeResultProofReport {
    let semantics_digest = canonical_digest(&result.selected_semantics);
    let lowered_strategy_bundle_digest = result.lowered_strategy_bundle_digest.clone();
    MergeResultProofReport {
        proof_schema_version: MERGE_PROOF_SCHEMA_VERSION.to_owned(),
        registry_bundle_digest: result.registry_bundle_digest.clone(),
        result_digest: canonical_digest(&CanonicalMergeResultDigestBasis {
            proof_schema_version: MERGE_PROOF_SCHEMA_VERSION,
            result,
        }),
        semantics_digest,
        lowered_strategy_bundle_digest,
        lineage_digest: merge_lineage_digest(result),
        selected_strategy_digest: result.selected_strategy_digest.clone(),
        selected_merge_base_digest: result.selected_merge_base_digest.clone(),
        selected_conflict_policy_digest: result.selected_conflict_policy_digest.clone(),
        selected_conflict_isolation_digest: result.selected_conflict_isolation_digest.clone(),
        selected_identity_matcher_digest: result.selected_identity_matcher_digest.clone(),
        selected_source_only_policy_digest: result.selected_source_only_policy_digest.clone(),
        selected_deletion_policy_digest: result.selected_deletion_policy_digest.clone(),
        strategy_witness: result.strategy_witness.clone(),
        compatibility_witness: result.compatibility_witness.clone(),
        scoped_merge_proof: result.scoped_merge_proof.clone(),
    }
}
