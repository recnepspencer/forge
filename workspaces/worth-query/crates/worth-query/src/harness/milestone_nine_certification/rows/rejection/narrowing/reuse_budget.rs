use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionBundle;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNineFailureClass;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::phase_two_mask_snapshot;
use crate::harness::milestone_nine_certification::fixtures::policy_narrowing_rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyCostPosture, PolicyEpoch,
    PolicyExecutionModeRequest, PolicyRuleSnapshot, PolicyWorkBudget,
};
use crate::policy_narrowing::{
    classify_saved_policy_narrowing_reuse, narrow_policy_query, SavedPolicyNarrowingReuseDescriptor,
};
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};

pub(super) fn reuse_budget_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let canonical = canonical_query();
    let policy = base_policy(false);
    let unknown_cost_policy = PolicyRuleSnapshot::synthetic_authority_with_budget(
        "runtime-policy",
        "rules-v1",
        PolicyEpoch::Synthetic(7),
        true,
        PolicyCostPosture::UnknownCost,
        Some(PolicyWorkBudget::bounded(1, 1, 1)),
    );
    let unknown_cost = admit_policy_tenant_context(
        canonical.query(),
        unknown_cost_policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &unknown_cost_policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap_err();
    let phase_two_canonical = canonical_query_with_secret_projection();
    let phase_two_admitted = admit_policy_tenant_context(
        phase_two_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let unbounded_recursion = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::unbounded_recursive_walk_for_test("manager")],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let saved_narrowing_drift =
        classify_saved_policy_narrowing_reuse(&SavedPolicyNarrowingReuseDescriptor::new(
            "saved-a",
            "narrowed-a",
            "policy-a",
            "tenant-truth-a",
            "tenant-schema-a",
            "projection-a",
            "proof-a",
            "policy-b",
            "tenant-truth-a",
            "tenant-schema-a",
            "projection-a",
            "proof-a",
        ));
    vec![
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-drift-renarrowing-required",
            perturbation_class:
                MilestoneNinePerturbationClass::SavedQueryPolicyDriftRenarrowingRequired,
            control_lane: control.clone(),
            hostile_lane: MilestoneNineRejectionBundle {
                failure_class: MilestoneNineFailureClass::SavedQueryPolicyTenantDrift,
                failure_digest: digest_parts(&[saved_narrowing_drift.as_str().to_string()]),
                counter_snapshot_digest: digest_parts(&[format!(
                    "saved_narrowing_reuse:{}",
                    saved_narrowing_drift.as_str()
                )]),
            },
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unknown-narrowing-cost-denied-before-truth",
            perturbation_class: MilestoneNinePerturbationClass::UnknownNarrowingCostDenied,
            control_lane: control.clone(),
            hostile_lane: rejection_bundle(unknown_cost),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "phase-two-no-truth-touch",
            perturbation_class: MilestoneNinePerturbationClass::PhaseTwoNoTruthTouch,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(unbounded_recursion),
            parity_lane: control.clone(),
        },
    ]
}
