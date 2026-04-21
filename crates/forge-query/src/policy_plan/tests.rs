use crate::authoring::{
    AspectFieldSelector, AuthoredResultShapeField, GuidedAuthoringPath, RawAuthoredQuery,
    RawAuthoredResultShape, RootEntityKey,
};
use crate::authorized_projection::{PolicyAspectMask, PolicyInfluenceSet, PolicyMaskSnapshot};
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyEpoch, PolicyExecutionModeRequest,
    PolicyRuleSnapshot,
};
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;
use crate::tenant_basis::{SchemaVariantSnapshot, TenantBasisEpoch, TenantBindingSnapshot};

use super::{
    deny_raw_diff_scrub, lower_policy_aware_branch_plan, lower_policy_aware_current_plan,
    lower_policy_aware_diff_plan, lower_policy_aware_historical_plan, PolicyAwareDiffBasisPair,
    PolicyAwareHistoricalBasis, PolicyAwareReadBasis,
};

fn narrowed() -> crate::policy_narrowing::NarrowedPolicyQueryArtifact {
    let query = RawAuthoredQuery::detail_builder(RootEntityKey::new("user").unwrap())
        .project(AspectFieldSelector::new("identity", "id").unwrap())
        .project(AspectFieldSelector::new("profile", "display_name").unwrap())
        .project(AspectFieldSelector::new("secret", "salary").unwrap())
        .build()
        .unwrap();
    let result_shape = RawAuthoredResultShape::detail_builder()
        .field(AuthoredResultShapeField::new("identity", "id", "id").unwrap())
        .field(AuthoredResultShapeField::new("profile", "display_name", "display_name").unwrap())
        .build()
        .unwrap();
    let canonical = GuidedAuthoringPath::canonicalize_detail(query, result_shape).unwrap();
    let policy = PolicyRuleSnapshot::synthetic_authority_with_posture(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        true,
        true,
        false,
    );
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        TenantBindingSnapshot::synthetic_direct(
            "tenant-a",
            "branch-a",
            "schema-a",
            TenantBasisEpoch::Synthetic(3),
        ),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible"),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let mask = PolicyMaskSnapshot::synthetic_authority(
        admitted.bundle().policy_digest(),
        PolicyAspectMask::allow_all().with_masked("secret", "salary"),
    );
    narrow_policy_query(
        &canonical,
        admitted,
        mask,
        PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap()
}

#[test]
fn current_branch_history_and_diff_lower_from_narrowed_artifact() {
    let artifact = narrowed();
    let current = lower_policy_aware_current_plan(&artifact);
    let branch = lower_policy_aware_branch_plan(
        &artifact,
        PolicyAwareReadBasis::admitted_branch(artifact.branch_access_digest(), "basis-branch-a"),
    )
    .unwrap();
    let historical = lower_policy_aware_historical_plan(
        &artifact,
        PolicyAwareHistoricalBasis::runtime_backed("basis-historical-a"),
    )
    .unwrap();
    let diff = lower_policy_aware_diff_plan(
        &artifact,
        PolicyAwareDiffBasisPair::runtime_backed("basis-left", "basis-right"),
    )
    .unwrap();

    assert_eq!(
        current.core().seam().authorized_projection_digest(),
        branch.core().seam().authorized_projection_digest()
    );
    assert_eq!(
        historical.core().seam().relationship_proof_digest(),
        current.core().seam().relationship_proof_digest()
    );
    assert_eq!(diff.scrub_disposition().as_str(), "authorized_delta_only");
}

#[test]
fn branch_mismatch_and_raw_diff_scrub_deny_before_truth_touch() {
    let artifact = narrowed();
    let branch_error = lower_policy_aware_branch_plan(
        &artifact,
        PolicyAwareReadBasis::admitted_branch("wrong-branch-digest", "basis-branch-a"),
    )
    .expect_err("branch mismatch must deny before execution");
    let scrub_error = deny_raw_diff_scrub();

    assert_eq!(branch_error.counters().raw_plan_bypass_denial_count(), 1);
    assert_eq!(scrub_error.counters().raw_diff_scrub_denial_count(), 1);
}

#[test]
fn store_backed_historical_and_diff_are_deferred() {
    let artifact = narrowed();
    let historical = lower_policy_aware_historical_plan(
        &artifact,
        PolicyAwareHistoricalBasis::store_backed_deferred("store-basis"),
    )
    .expect_err("store-backed historical parity is deferred");
    let diff = lower_policy_aware_diff_plan(
        &artifact,
        PolicyAwareDiffBasisPair::store_backed_deferred("left", "right"),
    )
    .expect_err("store-backed diff parity is deferred");

    assert_eq!(historical.counters().store_backed_deferred_count(), 1);
    assert_eq!(diff.counters().store_backed_deferred_count(), 1);
}
