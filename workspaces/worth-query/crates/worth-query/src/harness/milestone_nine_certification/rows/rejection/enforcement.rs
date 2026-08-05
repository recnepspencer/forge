use crate::harness::milestone_nine_certification::bundles::MilestoneNineRejectionRow;
use crate::harness::milestone_nine_certification::classifications::MilestoneNinePerturbationClass;
use crate::harness::milestone_nine_certification::fixtures::admitted_bundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::phase_three_test_narrowed_artifact;
use crate::harness::milestone_nine_certification::fixtures::phase_two_mask_snapshot;
use crate::harness::milestone_nine_certification::fixtures::policy_execution_seam_rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::policy_narrowing_rejection_bundle;
use crate::harness::milestone_nine_certification::fixtures::policy_placeholder_request;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;

pub(super) fn rejection_enforcement_rows() -> Vec<MilestoneNineRejectionRow> {
    let control = admitted_bundle(PolicyExecutionModeRequest::CurrentRead, false);
    let policy = base_policy(false);
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
    let phase_three_narrowed = phase_three_test_narrowed_artifact();
    let placeholder_masking = crate::policy_delivery::deny_policy_placeholder_masking(
        &phase_three_narrowed,
        policy_placeholder_request([("secret", "salary")]),
    )
    .unwrap_err();
    let masked_aggregation_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_aggregation_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_cursor_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_cursor_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let masked_view_membership_influence = narrow_policy_query(
        &phase_two_canonical,
        phase_two_admitted.clone(),
        phase_two_mask_snapshot(
            &phase_two_admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none().with_view_membership_field(
            crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap(),
        ),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap_err();
    let per_row_allocation = crate::policy_execution_seam::deny_policy_per_row_allocation_claim();
    let cross_tenant_fanout = crate::policy_execution_seam::deny_policy_cross_tenant_fanout_claim();
    let saved_query_bypass = crate::policy_execution_seam::deny_saved_query_policy_bypass_claim();
    let unsupported_workflow_composition =
        crate::policy_execution_seam::deny_unsupported_policy_workflow_composition_claim();
    vec![
        MilestoneNineRejectionRow {
            row_name: "masked-placeholder-shape-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedPlaceholderShapeForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(placeholder_masking),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-aggregation-without-witness-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedAggregationWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_aggregation_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-cursor-without-witness-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::MaskedCursorWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_cursor_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "masked-view-membership-without-witness-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::MaskedViewMembershipWithoutWitnessForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_narrowing_rejection_bundle(masked_view_membership_influence),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "policy-per-row-allocation-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::PolicyPerRowAllocationForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(per_row_allocation),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "policy-cross-tenant-fanout-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::PolicyCrossTenantFanoutForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(cross_tenant_fanout),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "saved-query-policy-bypass-forbidden",
            perturbation_class: MilestoneNinePerturbationClass::SavedQueryPolicyBypassForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(saved_query_bypass),
            parity_lane: control.clone(),
        },
        MilestoneNineRejectionRow {
            row_name: "unsupported-policy-workflow-composition-forbidden",
            perturbation_class:
                MilestoneNinePerturbationClass::UnsupportedPolicyWorkflowCompositionForbidden,
            control_lane: control.clone(),
            hostile_lane: policy_execution_seam_rejection_bundle(unsupported_workflow_composition),
            parity_lane: control,
        },
    ]
}
