use super::*;

pub(in crate::harness::milestone_eight_certification) fn bundle_digest_parts(
    matrix: &MilestoneEightCertificationMatrix,
) -> Vec<String> {
    let mut parts = vec![format!("suite:{}", matrix.suite_name)];
    for row in &matrix.rows {
        parts.push(format!("row:{}", row.row_name));
        parts.push(format!(
            "control:{}:{}:{}:{}:{}:{}:{}:{}",
            row.control_lane.query_digest,
            row.control_lane.plan_digest,
            row.control_lane.result_shape_digest,
            row.control_lane.delivery_digest,
            row.control_lane.counter_snapshot_digest,
            row.control_lane.identity_consumption_digest,
            row.control_lane.inspector_identity_digest,
            row.control_lane.inspector_identity_classification,
        ));
        parts.push(format!(
            "hostile:{}:{}:{}:{}:{}:{}:{}:{}",
            row.hostile_lane.query_digest,
            row.hostile_lane.plan_digest,
            row.hostile_lane.result_shape_digest,
            row.hostile_lane.delivery_digest,
            row.hostile_lane.counter_snapshot_digest,
            row.hostile_lane.identity_consumption_digest,
            row.hostile_lane.inspector_identity_digest,
            row.hostile_lane.inspector_identity_classification,
        ));
        parts.push(format!(
            "parity:{}:{}:{}:{}:{}:{}:{}:{}",
            row.parity_lane.query_digest,
            row.parity_lane.plan_digest,
            row.parity_lane.result_shape_digest,
            row.parity_lane.delivery_digest,
            row.parity_lane.counter_snapshot_digest,
            row.parity_lane.identity_consumption_digest,
            row.parity_lane.inspector_identity_digest,
            row.parity_lane.inspector_identity_classification,
        ));
    }
    for row in &matrix.rejection_rows {
        parts.push(format!("rejection:{}", row.row_name));
        parts.push(format!(
            "hostile:{}:{}",
            match row.hostile_lane.failure_class {
                MilestoneEightFailureClass::UnsupportedScopeFamily => "unsupported_scope_family",
                MilestoneEightFailureClass::UnsupportedTemplateFamily =>
                    "unsupported_template_family",
                MilestoneEightFailureClass::SavedQuerySupportProfileDrift => {
                    "saved_query_support_profile_drift"
                }
                MilestoneEightFailureClass::DurableSavedQueryDeferredDebt => {
                    "durable_saved_query_deferred_debt"
                }
                MilestoneEightFailureClass::GroupedHiddenRefreshForbidden => {
                    "grouped_hidden_refresh_forbidden"
                }
            },
            row.hostile_lane.failure_digest
        ));
    }
    parts
}
