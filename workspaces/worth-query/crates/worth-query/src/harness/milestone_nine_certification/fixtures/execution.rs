use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::harness::milestone_nine_certification::fixtures::base_policy;
use crate::harness::milestone_nine_certification::fixtures::canonical_query_with_secret_projection;
use crate::harness::milestone_nine_certification::fixtures::phase_two_mask_snapshot;
use crate::harness::milestone_nine_certification::fixtures::schema;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::harness::milestone_nine_certification::fixtures::tenant;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_narrowing::narrow_policy_query;
use crate::relationship_proof::RelationshipProofDescriptorSet;

pub(crate) fn phase_three_test_narrowed_artifact(
) -> crate::policy_narrowing::NarrowedPolicyQueryArtifact {
    let canonical = canonical_query_with_secret_projection();
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
    narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn phase_three_test_unmasked_artifact(
) -> crate::policy_narrowing::NarrowedPolicyQueryArtifact {
    let canonical = canonical_query_with_secret_projection();
    let policy = base_policy(false);
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
    narrow_policy_query(
        &canonical,
        admitted.clone(),
        phase_two_mask_snapshot(
            &admitted,
            crate::authorized_projection::PolicyAspectMask::allow_all(),
        ),
        crate::authorized_projection::PolicyInfluenceSet::none(),
        RelationshipProofDescriptorSet::none(),
    )
    .unwrap()
}

pub(in crate::harness::milestone_nine_certification) fn native_authorized_projection_fields(
    narrowed: &crate::policy_narrowing::NarrowedPolicyQueryArtifact,
) -> Vec<crate::authorized_projection::AuthorizedProjectionFieldPath> {
    narrowed
        .authorized_projection()
        .visible_field_paths()
        .to_vec()
}

pub(in crate::harness::milestone_nine_certification) fn authorized_projection_field(
    aspect: &str,
    field: &str,
) -> crate::authorized_projection::AuthorizedProjectionFieldPath {
    crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
        worth_foundational::facade::AspectKey::new(aspect.to_string())
            .expect("certification aspect key"),
        worth_foundational::facade::FieldKey::new(field.to_string())
            .expect("certification field key"),
    )
}

pub(in crate::harness::milestone_nine_certification) fn policy_placeholder_request(
    fields: impl IntoIterator<Item = (&'static str, &'static str)>,
) -> crate::policy_delivery::PolicyPlaceholderMaskingRequest {
    let fields = fields
        .into_iter()
        .map(|(aspect, field)| {
            crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
                worth_foundational::facade::AspectKey::new(aspect)
                    .expect("placeholder aspect key should admit"),
                worth_foundational::facade::FieldKey::new(field)
                    .expect("placeholder field key should admit"),
            )
        })
        .collect();
    crate::policy_delivery::PolicyPlaceholderMaskingRequest::from_authorized_field_paths(fields)
}

pub(in crate::harness::milestone_nine_certification) fn phase_three_bundle(
    row_label: &str,
    plan_digest: impl Into<String>,
    seam_digest: impl Into<String>,
    delivery_digest: impl Into<String>,
) -> MilestoneNineCertificationBundle {
    let narrowed = phase_three_test_narrowed_artifact();
    let support_profile =
        crate::policy_execution_seam::runtime_backed_policy_execution_seam_support_profile();
    MilestoneNineCertificationBundle {
        canonical_query_digest: narrowed.canonical_query_digest().to_string(),
        policy_digest: narrowed.policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("row:{row_label}"),
            format!("narrowed:{}", narrowed.digest()),
            format!("shape:{}", narrowed.narrowed_result_shape_digest()),
        ]),
        tenant_truth_basis_digest: narrowed.tenant_truth_basis_digest().to_string(),
        tenant_schema_basis_digest: narrowed.tenant_schema_basis_digest().to_string(),
        branch_access_digest: narrowed.branch_access_digest().to_string(),
        schema_variant_digest: "phase3-schema-variant-bound-in-phase1".to_string(),
        execution_mode: row_label.to_string(),
        admission_disposition: "phase3-policy-aware-lowered".to_string(),
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
        policy_plan_digest: plan_digest.into(),
        policy_execution_seam_digest: seam_digest.into(),
        delivery_digest: delivery_digest.into(),
        employee_fixture_digest: "phase3-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase3-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase3-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase3-delivery-width-bound".to_string(),
        composition_policy_parity_digest: "phase3-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase3-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase3-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: narrowed
            .validation_report()
            .counter_snapshot_digest()
            .to_string(),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}
