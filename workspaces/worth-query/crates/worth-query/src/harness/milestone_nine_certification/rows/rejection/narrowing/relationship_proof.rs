use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
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
use crate::relationship_proof::{
    RelationshipProofBudget, RelationshipProofDescriptor, RelationshipProofDescriptorSet,
};

pub(super) fn relationship_proof_rows() -> Vec<MilestoneNineRejectionRow> {
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
    let template_hidden_influence = narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_template_predicate_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let host_callback = narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::host_callback_forbidden(
                "authz",
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let query_conflict = narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::new(
            vec![RelationshipProofDescriptor::query_shape_mismatch_for_test(
                "different-query-digest",
            )],
            RelationshipProofBudget::bounded(1, 1),
        ),
    )
    .unwrap_err();
    let unbounded_recursion = narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
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
    vec![
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-host-callback-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofHostCallbackForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(host_callback),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-unbounded-recursion-denied",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofUnboundedRecursionDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(unbounded_recursion.clone()),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "relationship-proof-query-conflict-denied",
            perturbation_class:
                MilestoneNinePerturbationClass::RelationshipProofQueryConflictDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(query_conflict),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "template-hidden-influence-denied",
            perturbation_class: MilestoneNinePerturbationClass::TemplateHiddenInfluenceDenied,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(template_hidden_influence),
            parity_lane: control.clone(),
        },
    ]
}
