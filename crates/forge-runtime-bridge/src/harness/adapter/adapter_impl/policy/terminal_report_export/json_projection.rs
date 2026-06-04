use std::collections::BTreeMap;

use serde_json::json;

use super::super::certification_digest_basis::empty_provenance_report_digest;
use super::super::matrices::{
    AdmittedPolicyMatrixRow, PolicyCertificationMatrix, PolicyCertificationRow,
    PolicyRejectionMatrixRow, RequestPolicyMatrix, RequestPolicyMatrixRow, RoutePolicyMatrix,
    RoutePolicyMatrixRow,
};
use super::super::PolicyHarnessExecution;

pub(in crate::harness::adapter::adapter_impl) fn execution_summary_json(
    execution: &PolicyHarnessExecution,
) -> serde_json::Value {
    match execution {
        PolicyHarnessExecution::Provenance {
            policy_digest,
            policy_matrix,
            policy_provenance_report,
            request_policy_matrix,
            route_policy_matrix,
            routing_digest,
            replay_digest,
            diagnostics_digest,
            counter_snapshot,
        } => json!({
            "policy_digest": policy_digest,
            "policy_matrix": policy_matrix_json(policy_matrix),
            "policy_provenance_report": provenance_report_json(policy_provenance_report),
            "request_policy_matrix": request_policy_matrix_json(request_policy_matrix),
            "route_policy_matrix": route_policy_matrix_json(route_policy_matrix),
            "routing_digest": routing_digest,
            "replay_digest": replay_digest,
            "diagnostics_digest": diagnostics_digest,
            "failure_digest": serde_json::Value::Null,
            "counter_snapshot": counter_snapshot_json(counter_snapshot),
        }),
        PolicyHarnessExecution::Rejection {
            policy_matrix,
            failure_digest,
            diagnostics_digest,
            counter_snapshot,
        } => json!({
            "policy_digest": serde_json::Value::Null,
            "policy_matrix": policy_matrix_json(policy_matrix),
            "policy_provenance_report": empty_provenance_report_json(),
            "request_policy_matrix": request_policy_matrix_json(&RequestPolicyMatrix::empty()),
            "routing_digest": serde_json::Value::Null,
            "replay_digest": serde_json::Value::Null,
            "failure_digest": failure_digest,
            "diagnostics_digest": diagnostics_digest,
            "counter_snapshot": counter_snapshot_json(counter_snapshot),
        }),
        PolicyHarnessExecution::AmbientLeak {
            policy_digest,
            policy_matrix,
            policy_provenance_report,
            request_policy_matrix,
            replay_digest,
            diagnostics_digest,
            counter_snapshot,
        } => json!({
            "policy_digest": policy_digest,
            "policy_matrix": policy_matrix_json(policy_matrix),
            "policy_provenance_report": provenance_report_json(policy_provenance_report),
            "request_policy_matrix": request_policy_matrix_json(request_policy_matrix),
            "routing_digest": serde_json::Value::Null,
            "replay_digest": replay_digest,
            "diagnostics_digest": diagnostics_digest,
            "counter_snapshot": counter_snapshot_json(counter_snapshot),
            "failure_digest": serde_json::Value::Null,
        }),
    }
}

pub(in crate::harness::adapter::adapter_impl) fn execution_extensions_json(
    execution: &PolicyHarnessExecution,
    _runtime_bridge: &crate::facade::RuntimeBridge,
) -> BTreeMap<String, serde_json::Value> {
    match execution {
        PolicyHarnessExecution::Provenance {
            policy_digest,
            policy_matrix,
            policy_provenance_report,
            request_policy_matrix,
            route_policy_matrix,
            routing_digest,
            replay_digest,
            diagnostics_digest,
            counter_snapshot,
        } => BTreeMap::from([(
            "bridge_policy_certification_bundle".to_string(),
            json!({
                "policy_digest": policy_digest,
                "policy_matrix": policy_matrix_json(policy_matrix),
                "policy_provenance_report": provenance_report_json(policy_provenance_report),
                "request_policy_matrix": request_policy_matrix_json(request_policy_matrix),
                "route_policy_matrix": route_policy_matrix_json(route_policy_matrix),
                "routing_digest": routing_digest,
                "replay_digest": replay_digest,
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
            }),
        )]),
        PolicyHarnessExecution::Rejection {
            policy_matrix,
            failure_digest,
            diagnostics_digest,
            counter_snapshot,
        } => BTreeMap::from([(
            "bridge_policy_certification_bundle".to_string(),
            json!({
                "policy_digest": serde_json::Value::Null,
                "policy_matrix": policy_matrix_json(policy_matrix),
                "policy_provenance_report": empty_provenance_report_json(),
                "request_policy_matrix": request_policy_matrix_json(&RequestPolicyMatrix::empty()),
                "routing_digest": serde_json::Value::Null,
                "replay_digest": serde_json::Value::Null,
                "failure_digest": failure_digest,
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
            }),
        )]),
        PolicyHarnessExecution::AmbientLeak {
            policy_digest,
            policy_matrix,
            policy_provenance_report,
            request_policy_matrix,
            replay_digest,
            diagnostics_digest,
            counter_snapshot,
        } => BTreeMap::from([(
            "bridge_policy_certification_bundle".to_string(),
            json!({
                "policy_digest": policy_digest,
                "policy_matrix": policy_matrix_json(policy_matrix),
                "policy_provenance_report": provenance_report_json(policy_provenance_report),
                "request_policy_matrix": request_policy_matrix_json(request_policy_matrix),
                "routing_digest": serde_json::Value::Null,
                "replay_digest": replay_digest,
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": counter_snapshot_json(counter_snapshot),
            }),
        )]),
    }
}

pub(in crate::harness::adapter::adapter_impl) fn policy_matrix_json(
    matrix: &PolicyCertificationMatrix,
) -> serde_json::Value {
    json!({
        "rows": matrix
            .rows()
            .iter()
            .map(policy_matrix_row_json)
            .collect::<Vec<_>>(),
    })
}

fn policy_matrix_row_json(row: &PolicyCertificationRow) -> serde_json::Value {
    match row {
        PolicyCertificationRow::Admitted(row) => admitted_policy_row_json(row),
        PolicyCertificationRow::Rejection(row) => rejection_policy_row_json(row),
    }
}

fn admitted_policy_row_json(row: &AdmittedPolicyMatrixRow) -> serde_json::Value {
    json!({
        "label": row.label(),
        "declaration_identity": row.declaration_identity().as_str(),
        "request_kind": format!("{:?}", row.request_kind()),
        "execution_class": format!("{:?}", row.execution_class()),
        "diagnostics_tier": format!("{:?}", row.diagnostics_tier()),
        "route_artifacts": row.route_artifacts(),
        "replay_artifacts": row.replay_artifacts(),
        "policy_digest": row.policy_digest(),
        "lowered_policy_digest": row.lowered_policy_digest(),
        "provenance_digest": row.provenance_digest(),
        "replay_digest": row.replay_digest(),
    })
}

fn rejection_policy_row_json(row: &PolicyRejectionMatrixRow) -> serde_json::Value {
    json!({
        "label": row.label(),
        "declaration_identity": row.declaration_identity().as_str(),
        "failure_kind": format!("{:?}", row.failure_kind()),
        "stage": format!("{:?}", row.stage()),
        "field_kind": format!("{:?}", row.field_kind()),
        "primary_source": format!("{:?}", row.primary_source()),
        "secondary_source": format!("{:?}", row.secondary_source()),
        "digest": row.digest(),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn request_policy_matrix_json(
    matrix: &RequestPolicyMatrix,
) -> serde_json::Value {
    let mut matrix_json = json!({
        "rows": matrix
            .rows()
            .iter()
            .map(request_policy_row_json)
            .collect::<Vec<_>>(),
    });
    if let Some(object) = matrix_json.as_object_mut() {
        if let Some(branch_local_resolution) = matrix.branch_local_resolution() {
            object.insert(
                "branch_local_resolution".to_string(),
                json!(truth_view_resolution_label(branch_local_resolution)),
            );
        }
        if let Some(historical_resolution) = matrix.historical_resolution() {
            object.insert(
                "historical_resolution".to_string(),
                json!(truth_view_resolution_label(historical_resolution)),
            );
        }
    }
    matrix_json
}

fn request_policy_row_json(row: &RequestPolicyMatrixRow) -> serde_json::Value {
    let mut row_json = provenance_row_json(row.provenance_row());
    if let Some(object) = row_json.as_object_mut() {
        object.insert(
            "route_planning_policy_digest".to_string(),
            json!(row.route_planning_policy_digest()),
        );
        object.insert(
            "semantic_route_planning_policy_digest".to_string(),
            json!(row.semantic_route_planning_policy_digest()),
        );
    }
    row_json
}

pub(in crate::harness::adapter::adapter_impl) fn route_policy_matrix_json(
    matrix: &RoutePolicyMatrix,
) -> serde_json::Value {
    json!({
        "rows": matrix
            .rows()
            .iter()
            .map(route_policy_row_json)
            .collect::<Vec<_>>(),
    })
}

fn route_policy_row_json(row: &RoutePolicyMatrixRow) -> serde_json::Value {
    json!({
        "label": row.label(),
        "route_planning_policy_digest": row.route_planning_policy_digest(),
        "semantic_route_planning_policy_digest": row.semantic_route_planning_policy_digest(),
        "lowered_policy_identity": row.lowered_policy_identity().as_str(),
        "execution_class": format!("{:?}", row.execution_class()),
        "diagnostics_tier": format!("{:?}", row.diagnostics_tier()),
        "route_artifacts": row.route_artifacts(),
        "replay_artifacts": row.replay_artifacts(),
    })
}

fn truth_view_resolution_label(
    resolution: &crate::facade::BridgeTruthViewPolicyResolution,
) -> &'static str {
    match resolution {
        crate::facade::BridgeTruthViewPolicyResolution::Admitted(_) => "Admitted",
        crate::facade::BridgeTruthViewPolicyResolution::Rejected(_) => "Rejected",
    }
}

pub(in crate::harness::adapter::adapter_impl) fn empty_provenance_report_json() -> serde_json::Value
{
    json!({
        "digest": empty_provenance_report_digest(),
        "rows": [],
    })
}

pub(in crate::harness::adapter::adapter_impl) fn provenance_report_json(
    report: &crate::facade::BridgePolicyProvenanceReport,
) -> serde_json::Value {
    json!({
        "digest": report.digest(),
        "rows": report.rows().iter().map(provenance_row_json).collect::<Vec<_>>(),
    })
}

fn provenance_row_json(row: &crate::facade::BridgePolicyProvenanceReportRow) -> serde_json::Value {
    json!({
        "label": row.label(),
        "request_kind": format!("{:?}", row.request_kind()),
        "execution_class": format!("{:?}", row.execution_class()),
        "diagnostics_tier": format!("{:?}", row.diagnostics_tier()),
        "route_artifacts": row.route_artifacts(),
        "replay_artifacts": row.replay_artifacts(),
        "policy_digest": row.policy_digest(),
        "semantic_policy_digest": row.semantic_policy_digest(),
        "lowered_policy_digest": row.lowered_policy_digest(),
        "provenance_digest": row.provenance_digest(),
        "replay_digest": row.replay_digest(),
        "provenance_entries": row.provenance_entries().iter().map(|entry| json!({
            "field_kind": format!("{:?}", entry.field_kind()),
            "declared_source": format!("{:?}", entry.declared_source()),
            "operative_source": format!("{:?}", entry.operative_source()),
            "resolution": format!("{:?}", entry.resolution()),
        })).collect::<Vec<_>>(),
    })
}

pub(in crate::harness::adapter::adapter_impl) fn counter_snapshot_json(
    counters: &crate::facade::BridgePolicyCounters,
) -> serde_json::Value {
    json!({
        "declaration_count": counters.declaration_count(),
        "declaration_width_count": counters.declaration_width_count(),
        "admitted_contract_count": counters.admitted_contract_count(),
        "admission_width_count": counters.admission_width_count(),
        "rejected_contract_count": counters.rejected_contract_count(),
        "provenance_entry_count": counters.provenance_entry_count(),
        "provenance_width_count": counters.provenance_width_count(),
        "narrowed_field_count": counters.narrowed_field_count(),
        "inherited_field_count": counters.inherited_field_count(),
        "override_count": counters.override_count(),
        "ignored_field_count": counters.ignored_field_count(),
        "replay_bundle_count": counters.replay_bundle_count(),
        "replay_mismatch_count": counters.replay_mismatch_count(),
        "ambient_policy_leak_count": counters.ambient_policy_leak_count(),
        "policy_request_count": counters.policy_request_count(),
        "truth_view_interleave_count": counters.truth_view_interleave_count(),
        "preview_equivalence_preserved_count": counters.preview_equivalence_preserved_count(),
        "policy_source_ambiguity_count": counters.policy_source_ambiguity_count(),
        "substantive_illegality_count": counters.substantive_illegality_count(),
        "authority_escape_count": counters.authority_escape_count(),
    })
}
