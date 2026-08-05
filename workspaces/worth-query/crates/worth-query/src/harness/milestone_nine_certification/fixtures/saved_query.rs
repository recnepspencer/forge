use crate::harness::certification::digest_parts;
use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationBundle;
use crate::policy_basis::runtime_backed_policy_tenant_admission_support_profile;
use crate::policy_basis::PolicyAdmissionDisposition;
use crate::policy_basis::PolicyCostPosture;
use crate::policy_basis::PolicyExecutionModeRequest;
use crate::policy_basis::PolicyWorkBudget;
use crate::policy_basis::SavedQueryPolicyReuseDisposition;

pub(in crate::harness::milestone_nine_certification) fn saved_query_reuse_bundle(
    disposition: SavedQueryPolicyReuseDisposition,
) -> MilestoneNineCertificationBundle {
    let support_profile = runtime_backed_policy_tenant_admission_support_profile();
    MilestoneNineCertificationBundle {
        canonical_query_digest: "saved-query-policy-tenant-reuse".to_string(),
        policy_digest: format!("reuse:{}", disposition.as_str()),
        result_digest: digest_parts(&[
            "saved-query-policy-tenant-reuse".to_string(),
            format!("reuse:{}", disposition.as_str()),
        ]),
        tenant_truth_basis_digest: "reuse-tenant-truth".to_string(),
        tenant_schema_basis_digest: "reuse-tenant-schema".to_string(),
        branch_access_digest: "reuse-branch".to_string(),
        schema_variant_digest: "reuse-schema".to_string(),
        execution_mode: PolicyExecutionModeRequest::CurrentRead.as_str().to_string(),
        admission_disposition: PolicyAdmissionDisposition::AdmittedUnchanged
            .as_str()
            .to_string(),
        policy_cost_posture: PolicyCostPosture::ConstantProof.as_str().to_string(),
        policy_work_budget_digest: PolicyWorkBudget::bounded(1, 1, 1).digest_part(),
        authorized_projection_digest: "saved-query-authorized-projection-deferred".to_string(),
        narrowed_result_shape_digest: "saved-query-narrowed-result-shape-deferred".to_string(),
        relationship_proof_digest: "saved-query-relationship-proof-deferred".to_string(),
        validation_report_digest: "saved-query-validation-report-deferred".to_string(),
        policy_plan_digest: "saved-query-policy-plan-deferred".to_string(),
        policy_execution_seam_digest: "saved-query-policy-seam-deferred".to_string(),
        delivery_digest: "saved-query-delivery-deferred".to_string(),
        employee_fixture_digest: "saved-query-employee-fixture-deferred".to_string(),
        policy_scale_counter_slope_digest: "saved-query-policy-scale-deferred".to_string(),
        live_drift_evidence_digest: "saved-query-live-drift-deferred".to_string(),
        delivery_width_class_digest: "saved-query-delivery-width-deferred".to_string(),
        composition_policy_parity_digest: "saved-query-composition-parity-deferred".to_string(),
        view_shape_policy_parity_digest: "saved-query-view-shape-parity-deferred".to_string(),
        placeholder_denial_digest: "saved-query-placeholder-denial-deferred".to_string(),
        counter_snapshot_digest: digest_parts(&[format!("reuse:{}", disposition.as_str())]),
        support_profile_digest: support_profile.profile_digest().to_string(),
    }
}
