use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::harness::milestone_nine_certification::fixtures::canonical_query;
use crate::harness::milestone_nine_certification::fixtures::secret_salary_key;
use crate::policy_basis::admit_policy_tenant_context;
use crate::policy_basis::runtime_backed_policy_tenant_admission_support_profile;
use crate::policy_basis::BranchAccessGrant;
use crate::policy_basis::PolicyEpoch;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_basis::PolicyRuleSnapshot;
use crate::tenant_basis::SchemaVariantSnapshot;
use crate::tenant_basis::TenantBasisEpoch;
use crate::tenant_basis::TenantBindingSnapshot;

pub(in crate::harness::milestone_nine_certification) fn base_policy(
    narrowed: bool,
) -> PolicyRuleSnapshot {
    if narrowed {
        PolicyRuleSnapshot::synthetic_authority_with_projection(
            "runtime-policy",
            "rules-v1",
            PolicyEpoch::Synthetic(7),
            crate::authorized_projection::PolicyAspectMask::allow_all()
                .with_masked(secret_salary_key()),
        )
    } else {
        PolicyRuleSnapshot::synthetic_authority(
            "runtime-policy",
            "rules-v1",
            PolicyEpoch::Synthetic(7),
        )
    }
}

pub(in crate::harness::milestone_nine_certification) fn tenant() -> TenantBindingSnapshot {
    TenantBindingSnapshot::synthetic_direct(
        "tenant-a",
        "branch-a",
        "schema-a",
        TenantBasisEpoch::Synthetic(3),
    )
}

pub(in crate::harness::milestone_nine_certification) fn schema() -> SchemaVariantSnapshot {
    SchemaVariantSnapshot::synthetic_authority("tenant-a", "schema-a", "compatible")
}

pub(in crate::harness::milestone_nine_certification) fn admitted_bundle(
    mode: PolicyExecutionModeRequest,
    narrowed: bool,
) -> MilestoneNineCertificationBundle {
    let canonical = canonical_query();
    let policy = base_policy(narrowed);
    let branch = BranchAccessGrant::synthetic_granted("branch-a", &policy);
    let schema = schema();
    let admitted =
        admit_policy_tenant_context(canonical.query(), policy, tenant(), branch, schema, mode)
            .unwrap();
    let support_profile = runtime_backed_policy_tenant_admission_support_profile();

    MilestoneNineCertificationBundle {
        canonical_query_digest: admitted.bundle().canonical_query_digest().to_string(),
        policy_digest: admitted.bundle().policy_digest().to_string(),
        result_digest: digest_parts(&[
            format!("query:{}", admitted.bundle().canonical_query_digest()),
            format!("policy:{}", admitted.bundle().policy_digest()),
            format!("mode:{}", admitted.bundle().execution_mode().as_str()),
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
        policy_cost_posture: admitted.bundle().policy_cost_posture().as_str().to_string(),
        policy_work_budget_digest: admitted.bundle().policy_work_budget().digest_part(),
        authorized_projection_digest: "phase1-authorized-projection-deferred".to_string(),
        narrowed_result_shape_digest: "phase1-narrowed-result-shape-deferred".to_string(),
        relationship_proof_digest: "phase1-relationship-proof-deferred".to_string(),
        validation_report_digest: "phase1-validation-report-deferred".to_string(),
        policy_plan_digest: "phase1-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "phase1-policy-seam-deferred".to_string(),
        delivery_digest: "phase1-delivery-deferred".to_string(),
        employee_fixture_digest: "phase1-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "phase1-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "phase1-live-drift-deferred".to_string(),
        delivery_width_class_digest: "phase1-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "phase1-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "phase1-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "phase1-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[
            format!(
                "policy:{}",
                admitted
                    .bundle()
                    .counters()
                    .policy()
                    .policy_basis_admitted_count()
            ),
            format!(
                "tenant:{}",
                admitted
                    .bundle()
                    .counters()
                    .tenant()
                    .direct_tenant_binding_admitted_count()
            ),
            format!(
                "bundle:{}",
                admitted.bundle().counters().admission_bundle_count()
            ),
        ]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}
