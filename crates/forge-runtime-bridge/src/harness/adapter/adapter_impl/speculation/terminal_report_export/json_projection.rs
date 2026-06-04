use std::collections::BTreeMap;

use super::super::{shared, SpeculationHarnessExecution};
use super::churn_json_projection;
use serde_json::json;

pub(in crate::harness::adapter::adapter_impl) fn summary_json(
    execution: &SpeculationHarnessExecution,
) -> serde_json::Value {
    match execution {
        SpeculationHarnessExecution::Discard {
            execution_record,
            discard_record,
            routing_digest,
        } => json!({
            "speculative_resource_digest": shared::speculative_resource_digest(
                execution_record.digest(),
                Some(discard_record.digest()),
                None,
            ),
            "discard_residue_report": discard_residue_report_json(discard_record),
            "routing_digest": routing_digest,
            "counter_snapshot": counter_snapshot_json(discard_record.counters()),
        }),
        SpeculationHarnessExecution::Promotion {
            promoted_execution_record,
            promotion_record,
            promoted_replay_bundle,
            discarded_execution_record,
            discarded_record,
            discarded_replay_bundle,
            routing_digest,
            diagnostics_digest,
        } => json!({
            "speculative_commit_digest": shared::speculative_commit_digest(
                promoted_execution_record.digest(),
                promotion_record.digest(),
                discarded_execution_record.digest(),
                discarded_record.digest(),
            ),
            "preview_vs_authoritative_matrix": preview_vs_authoritative_matrix_json(
                promotion_record,
                discarded_record,
                routing_digest.as_deref(),
            ),
            "replay_digest": shared::replay_digest(
                promoted_replay_bundle,
                discarded_replay_bundle,
            ),
            "diagnostics_digest": diagnostics_digest,
            "counter_snapshot": counter_snapshot_json(promotion_record.counters()),
        }),
        SpeculationHarnessExecution::Churn { certification } => {
            churn_json_projection::summary_json(certification)
        }
    }
}

pub(in crate::harness::adapter::adapter_impl) fn extensions_json(
    execution: &SpeculationHarnessExecution,
    runtime_bridge: &crate::facade::RuntimeBridge,
) -> BTreeMap<String, serde_json::Value> {
    match execution {
        SpeculationHarnessExecution::Discard {
            execution_record,
            discard_record,
            ..
        } => BTreeMap::from([
            (
                "bridge_speculation_certification_bundle".to_string(),
                summary_json(execution),
            ),
            (
                "bridge_speculation_record".to_string(),
                discard_record_json(runtime_bridge, execution_record, discard_record),
            ),
        ]),
        SpeculationHarnessExecution::Promotion {
            promoted_execution_record,
            promotion_record,
            promoted_replay_bundle,
            discarded_execution_record,
            discarded_record,
            discarded_replay_bundle,
            ..
        } => BTreeMap::from([
            (
                "bridge_speculation_certification_bundle".to_string(),
                summary_json(execution),
            ),
            (
                "bridge_speculation_record".to_string(),
                promotion_record_json(PromotionTerminalRecordEvidence {
                    promoted_execution_record,
                    promotion_record,
                    promoted_replay_bundle,
                    discarded_execution_record,
                    discarded_record,
                    discarded_replay_bundle,
                    runtime_bridge,
                }),
            ),
        ]),
        SpeculationHarnessExecution::Churn { .. } => BTreeMap::from([(
            "bridge_speculation_certification_bundle".to_string(),
            summary_json(execution),
        )]),
    }
}

fn discard_record_json(
    runtime_bridge: &crate::facade::RuntimeBridge,
    execution_record: &crate::facade::BridgePreviewExecutionRecord,
    discard_record: &crate::facade::BridgePreviewDiscardRecord,
) -> serde_json::Value {
    json!({
        "preview_execution_record_identity": execution_record.record_identity().as_str(),
        "preview_discard_record_identity": discard_record.record_identity().as_str(),
        "preview_session_identity": discard_record.preview_session_identity(),
        "discard_explanation": {
            "preview_discard_record_identity": runtime_bridge
                .diagnostics()
                .explain_preview_discard_record(discard_record)
                .preview_discard_record_identity(),
        },
    })
}

struct PromotionTerminalRecordEvidence<'a> {
    promoted_execution_record: &'a crate::facade::BridgePreviewExecutionRecord,
    promotion_record: &'a crate::facade::BridgePreviewPromotionRecord,
    promoted_replay_bundle: &'a crate::facade::BridgePreviewReplayBundle,
    discarded_execution_record: &'a crate::facade::BridgePreviewExecutionRecord,
    discarded_record: &'a crate::facade::BridgePreviewDiscardRecord,
    discarded_replay_bundle: &'a crate::facade::BridgePreviewReplayBundle,
    runtime_bridge: &'a crate::facade::RuntimeBridge,
}

fn promotion_record_json(evidence: PromotionTerminalRecordEvidence<'_>) -> serde_json::Value {
    json!({
        "preview_execution_record_identity": evidence.promoted_execution_record.record_identity().as_str(),
        "preview_promotion_record_identity": evidence.promotion_record.record_identity().as_str(),
        "preview_session_identity": evidence.promotion_record.preview_session_identity(),
        "discarded_preview_execution_record_identity": evidence.discarded_execution_record.record_identity().as_str(),
        "discarded_preview_discard_record_identity": evidence.discarded_record.record_identity().as_str(),
        "discarded_preview_session_identity": evidence.discarded_record.preview_session_identity(),
        "promotion_explanation": {
            "preview_promotion_record_identity": evidence.runtime_bridge
                .diagnostics()
                .explain_preview_promotion_record(evidence.promotion_record)
                .preview_promotion_record_identity(),
        },
        "replay_explanation": {
            "lifecycle_outcome": format!(
                "{:?}",
                evidence.runtime_bridge
                    .diagnostics()
                    .explain_preview_replay_bundle(evidence.promoted_replay_bundle)
                    .lifecycle_outcome()
            ),
        },
        "discard_explanation": {
            "preview_discard_record_identity": evidence.runtime_bridge
                .diagnostics()
                .explain_preview_discard_record(evidence.discarded_record)
                .preview_discard_record_identity(),
        },
        "discard_replay_explanation": {
            "lifecycle_outcome": format!(
                "{:?}",
                evidence.runtime_bridge
                    .diagnostics()
                    .explain_preview_replay_bundle(evidence.discarded_replay_bundle)
                    .lifecycle_outcome()
            ),
        },
    })
}

fn discard_residue_report_json(
    discard_record: &crate::facade::BridgePreviewDiscardRecord,
) -> serde_json::Value {
    json!({
        "digest": discard_record.residue_report().digest(),
        "authoritative_residue_count": discard_record.residue_report().authoritative_residue_count(),
        "destroyable_residue_count": discard_record.residue_report().destroyable_residue_count(),
        "retained_non_authoritative_count": discard_record.residue_report().retained_non_authoritative_count(),
        "classes": discard_record
            .residue_report()
            .residue_classes()
            .iter()
            .map(|class| format!("{class:?}"))
            .collect::<Vec<_>>(),
    })
}

fn preview_vs_authoritative_matrix_json(
    promotion_record: &crate::facade::BridgePreviewPromotionRecord,
    discarded_record: &crate::facade::BridgePreviewDiscardRecord,
    routing_digest: Option<&str>,
) -> serde_json::Value {
    json!({
        "promoted_preview": {
            "preview_session_identity": promotion_record.preview_session_identity(),
            "preview_execution_record_identity": promotion_record.preview_execution_record_identity().as_str(),
            "promotion_record_identity": promotion_record.record_identity().as_str(),
            "authoritative_commit_boundary_digest": promotion_record.authoritative_commit_boundary_digest(),
            "authoritative_artifact_digest": promotion_record.authoritative_artifact_digest(),
        },
        "discarded_preview": {
            "preview_session_identity": discarded_record.preview_session_identity(),
            "preview_execution_record_identity": discarded_record.preview_execution_record_identity().as_str(),
            "discard_record_identity": discarded_record.record_identity().as_str(),
            "discard_cleanup_outcome": format!("{:?}", discarded_record.cleanup_outcome()),
            "discard_residue_report_digest": discarded_record.residue_report().digest(),
        },
        "authoritative_route_digest": routing_digest,
    })
}

fn counter_snapshot_json(counters: &crate::facade::BridgeSpeculationCounters) -> serde_json::Value {
    json!({
        "preview_session_count_touched": counters.preview_session_count_touched(),
        "branch_binding_proof_width": counters.branch_binding_proof_width(),
        "admissibility_proof_width": counters.admissibility_proof_width(),
        "preview_artifact_count": counters.preview_artifact_count(),
        "discard_artifact_count": counters.discard_artifact_count(),
        "destroyed_artifact_count": counters.destroyed_artifact_count(),
        "retained_non_authoritative_artifact_count": counters.retained_non_authoritative_artifact_count(),
        "promotion_proof_checks": counters.promotion_proof_checks(),
        "replay_bundle_width": counters.replay_bundle_width(),
    })
}
