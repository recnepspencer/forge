use crate::harness::milestone_nine_certification::bundles::MilestoneNineCertificationMatrix;

pub(in crate::harness::milestone_nine_certification) fn bundle_digest_parts(
    matrix: &MilestoneNineCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.push(format!(
            "control:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            row.control_lane.canonical_query_digest,
            row.control_lane.policy_digest,
            row.control_lane.result_digest,
            row.control_lane.tenant_truth_basis_digest,
            row.control_lane.execution_mode,
            row.control_lane.admission_disposition,
            row.control_lane.policy_cost_posture,
            row.control_lane.policy_work_budget_digest,
            row.control_lane.authorized_projection_digest,
            row.control_lane.relationship_proof_digest,
            row.control_lane.validation_report_digest,
            row.control_lane.policy_plan_digest,
            row.control_lane.policy_execution_seam_digest,
            row.control_lane.delivery_digest,
            row.control_lane.employee_fixture_digest,
            row.control_lane.policy_scale_counter_slope_digest,
            row.control_lane.live_drift_evidence_digest,
            row.control_lane.delivery_width_class_digest,
            row.control_lane.composition_policy_parity_digest,
            row.control_lane.view_shape_policy_parity_digest,
            row.control_lane.placeholder_denial_digest,
        ));
        parts.push(format!(
            "hostile:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}:{}",
            row.hostile_lane.canonical_query_digest,
            row.hostile_lane.policy_digest,
            row.hostile_lane.result_digest,
            row.hostile_lane.tenant_truth_basis_digest,
            row.hostile_lane.execution_mode,
            row.hostile_lane.admission_disposition,
            row.hostile_lane.policy_cost_posture,
            row.hostile_lane.policy_work_budget_digest,
            row.hostile_lane.authorized_projection_digest,
            row.hostile_lane.relationship_proof_digest,
            row.hostile_lane.validation_report_digest,
            row.hostile_lane.policy_plan_digest,
            row.hostile_lane.policy_execution_seam_digest,
            row.hostile_lane.delivery_digest,
            row.hostile_lane.employee_fixture_digest,
            row.hostile_lane.policy_scale_counter_slope_digest,
            row.hostile_lane.live_drift_evidence_digest,
            row.hostile_lane.delivery_width_class_digest,
            row.hostile_lane.composition_policy_parity_digest,
            row.hostile_lane.view_shape_policy_parity_digest,
            row.hostile_lane.placeholder_denial_digest,
        ));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!(
            "rejection:{}:{}",
            row.row_name, row.hostile_lane.failure_digest
        ));
    }
    parts
}

pub(in crate::harness::milestone_nine_certification) fn coverage_digest_parts(
    matrix: &MilestoneNineCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    parts.extend(
        matrix
            .rows
            .iter()
            .map(|row| format!("row:{}", row.row_name)),
    );
    parts.extend(
        matrix
            .rejection_rows
            .iter()
            .map(|row| format!("rejection:{}", row.row_name)),
    );
    parts
}
