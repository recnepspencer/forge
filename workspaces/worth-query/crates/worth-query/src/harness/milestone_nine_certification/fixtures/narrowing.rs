use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;

pub(in crate::harness::milestone_nine_certification) fn phase_two_bundle(
    canonical: crate::canonicalization::CanonicalQueryBundle,
    mask: crate::authorized_projection::PolicyAspectMask,
    descriptors: RelationshipProofDescriptorSet,
) -> MilestoneNineCertificationBundle {
    let policy = base_policy(true);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let admitted = admit_policy_tenant_context(
        canonical.query(),
        policy,
        tenant(),
        branch,
        schema(),
        PolicyExecutionModeRequest::CurrentRead,
    )
    .unwrap();
    let mask = crate::authorized_projection::PolicyMaskSnapshot::synthetic_authority(
        admitted.bundle().policy_digest(),
        mask,
    );
    let narrowed = narrow_policy_query(
        &canonical,
        admitted.clone(),
        mask,
        crate::authorized_projection::PolicyInfluenceSet::none(),
        descriptors,
    )
    .unwrap();
    let support_profile =
        crate::policy_narrowing::runtime_backed_policy_narrowing_support_profile();

    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: admitted.bundle().policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
            format!(
                "authorized_projection:{}",
                narrowed.authorized_projection().identity().as_str()
            ),
        ]),
        tenant_truth_basis_digest: admitted.bundle().tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: admitted.bundle().tenant_schema_basis_digest().to_string(),
        branch_access_digest: admitted.bundle().branch_access_digest().to_string(),
        schema_variant_digest: admitted.bundle().schema_variant_digest().to_string(),
        execution_mode: admitted.bundle().execution_mode().as_str().to_string(),
        admission_disposition: admitted
            .bundle()
            .admission_disposition()
            .as_str()
            .to_string(),
        policy_cost_posture: narrowed.cost_posture().as_str().to_string(),
        policy_work_budget_digest: narrowed.work_budget().digest_part(),
        authorized_projection_digest: narrowed
            .authorized_projection()
            .identity()
            .as_str()
            .to_string(),
        narrowed_result_shape_digest: narrowed.narrowed_result_shape_digest().to_string(),
        relationship_proof_digest: narrowed
            .relationship_proof()
            .identity()
            .as_str()
            .to_string(),
        validation_report_digest: narrowed.validation_report().digest().to_string(),
        policy_plan_digest: "phase2-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "phase2-policy-seam-deferred".to_string(),
        delivery_digest: "phase2-delivery-deferred".to_string(),
        employee_fixture_digest: "phase2-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase2-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase2-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase2-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase2-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase2-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase2-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: narrowed
            .validation_report()
            .counter_snapshot_digest()
            .to_string(),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}

pub(in crate::harness::milestone_nine_certification) fn phase_two_mask_snapshot(
    admitted: &crate::policy_basis::AdmittedPolicyTenantContext,
    mask: crate::authorized_projection::PolicyAspectMask,
) -> crate::authorized_projection::PolicyMaskSnapshot {
    crate::authorized_projection::PolicyMaskSnapshot::synthetic_authority(
        admitted.bundle().policy_digest(),
        mask,
    )
}

pub(in crate::harness::milestone_nine_certification) fn secret_salary_key(
) -> crate::authoring::AspectFieldKey {
    crate::authoring::AspectFieldKey::from_authoring_parts("secret", "salary").unwrap()
}
