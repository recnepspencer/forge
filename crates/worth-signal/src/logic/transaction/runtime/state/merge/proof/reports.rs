use super::super::{BranchMergePlan, BranchMergeResult};

use super::digest::{
    canonical_digest, merge_lineage_digest, CanonicalMergePlanDigestBasis,
    CanonicalMergeResultDigestBasis, CanonicalRegistryBundleDigestBasis,
};
use super::{
    MergePlanProofReport, MergeResultProofReport, RuntimeProofReport, MERGE_PROOF_SCHEMA_VERSION,
};

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
