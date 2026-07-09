use super::super::churn_certification::{
    SpeculationBranchIsolationMatrix, SpeculationBranchIsolationRow, SpeculationChurnCertification,
    SpeculationChurnCounterSnapshot, SpeculationResourceBoundReport,
};
use super::super::shared;
use serde_json::json;

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    certification: &SpeculationChurnCertification,
) -> serde_json::Value {
    json!({
        "preview_lifecycle_digest": shared::preview_lifecycle_digest(certification.preview_replay_bundle_set()),
        "resource_bound_report": resource_bound_report_json(certification.resource_bound_report()),
        "branch_isolation_matrix": branch_isolation_matrix_json(certification.branch_isolation_matrix()),
        "counter_snapshot": counter_snapshot_json(certification.counter_snapshot()),
    })
}

fn branch_isolation_matrix_json(matrix: &SpeculationBranchIsolationMatrix) -> serde_json::Value {
    json!({
        "rows": matrix
            .rows()
            .iter()
            .map(branch_isolation_row_json)
            .collect::<Vec<_>>(),
        "baseline_authoritative_route_digest": matrix.baseline_authoritative_route_digest(),
        "final_authoritative_route_digest": matrix.final_authoritative_route_digest(),
    })
}

fn branch_isolation_row_json(row: &SpeculationBranchIsolationRow) -> serde_json::Value {
    json!({
        "preview_session_identity": row.preview_session_identity(),
        "truth_branch_identity": row.truth_branch_identity(),
        "execution_record_identity": row.execution_record_identity(),
        "discard_record_identity": row.discard_record_identity(),
        "lifecycle_outcome": format!("{:?}", row.lifecycle_outcome()),
        "authoritative_route_digest_after_discard": row.authoritative_route_digest_after_discard(),
    })
}

fn resource_bound_report_json(report: &SpeculationResourceBoundReport) -> serde_json::Value {
    json!({
        "retained_preview_execution_record_count": report.retained_preview_execution_record_count(),
        "retained_preview_discard_record_count": report.retained_preview_discard_record_count(),
        "retained_preview_promotion_record_count": report.retained_preview_promotion_record_count(),
        "max_preview_artifact_count": report.max_preview_artifact_count(),
        "max_replay_bundle_width": report.max_replay_bundle_width(),
        "authoritative_route_observation_count": report.authoritative_route_observation_count(),
    })
}

fn counter_snapshot_json(snapshot: &SpeculationChurnCounterSnapshot) -> serde_json::Value {
    json!({
        "preview_session_count_touched": snapshot.preview_session_count_touched(),
        "max_preview_artifact_count": snapshot.max_preview_artifact_count(),
        "max_replay_bundle_width": snapshot.max_replay_bundle_width(),
        "retained_preview_execution_record_count": snapshot.retained_preview_execution_record_count(),
        "retained_preview_discard_record_count": snapshot.retained_preview_discard_record_count(),
        "retained_preview_promotion_record_count": snapshot.retained_preview_promotion_record_count(),
        "authoritative_route_observation_count": snapshot.authoritative_route_observation_count(),
    })
}
