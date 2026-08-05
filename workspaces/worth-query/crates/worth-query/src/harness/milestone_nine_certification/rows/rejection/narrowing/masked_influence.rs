use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_ordering;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_predicate;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::phase_two_mask_snapshot;
use crate::harness::milestone_nine_certification::fixtures::policy_narrowing_rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::{
    admit_policy_tenant_context, BranchAccessGrant, PolicyExecutionModeRequest,
};
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;

pub(super) fn masked_influence_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let policy = base_policy(false);
    let canonical = canonical_query_with_secret_projection();
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let predicate_canonical = canonical_query_with_secret_predicate();
    let predicate_admitted = admit_policy_tenant_context(
        predicate_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let masked_predicate = narrow_policy_query(
        &predicate_canonical,
        predicate_admitted.clone(),
        phase_two_mask_snapshot(
            &predicate_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let ordering_canonical = canonical_query_with_secret_ordering();
    let ordering_admitted = admit_policy_tenant_context(
        ordering_canonical.query(),
        policy.clone(),
        tenant(),
        BranchAccessGrant::synthetic_granted("branch-a", &policy),
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let masked_ordering = narrow_policy_query(
        &ordering_canonical,
        ordering_admitted.clone(),
        phase_two_mask_snapshot(
            &ordering_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_non_disclosing_use_only(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_grouping = narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_non_disclosing_use_only(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_grouping_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    vec![
        MilestoneNineRejectionRow {
            row_name: "masked-predicate-denies-before-narrowing",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedPredicateDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_predicate),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-ordering-denies-before-narrowing",
            perturbation_class: MilestoneNinePerturbationClass::MaskedOrderingDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_ordering),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-grouping-denies-before-narrowing",
            perturbation_class: MilestoneNinePerturbationClass::MaskedGroupingDeniedBeforeNarrowing,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_grouping),
            parity_lane: control.clone(),
        },
    ]
}
