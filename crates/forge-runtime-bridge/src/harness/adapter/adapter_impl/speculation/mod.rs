use super::*;

mod churn;
mod discard;
mod promotion;
mod shared;

pub(super) enum SpeculationHarnessTarget {
    DiscardCertification,
    PromotionCertification,
    ChurnCertification,
}

pub(super) enum SpeculationHarnessExecution {
    Discard {
        execution_record: crate::facade::BridgePreviewExecutionRecord,
        discard_record: crate::facade::BridgePreviewDiscardRecord,
        routing_digest: Option<String>,
    },
    Promotion {
        promoted_execution_record: crate::facade::BridgePreviewExecutionRecord,
        promotion_record: crate::facade::BridgePreviewPromotionRecord,
        promoted_replay_bundle: crate::facade::BridgePreviewReplayBundle,
        discarded_execution_record: crate::facade::BridgePreviewExecutionRecord,
        discarded_record: crate::facade::BridgePreviewDiscardRecord,
        discarded_replay_bundle: crate::facade::BridgePreviewReplayBundle,
        routing_digest: Option<String>,
        diagnostics_digest: String,
    },
    Churn {
        lifecycle_digests: Vec<String>,
        branch_isolation_matrix: serde_json::Value,
        resource_bound_report: serde_json::Value,
        counter_snapshot: serde_json::Value,
    },
}

impl SpeculationHarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Discard {
                execution_record,
                discard_record,
                routing_digest,
            } => json!({
                "speculative_resource_digest": shared::speculative_resource_digest(
                    execution_record.digest(),
                    Some(discard_record.digest()),
                    None,
                ),
                "discard_residue_report": shared::discard_residue_report_json(discard_record),
                "routing_digest": routing_digest,
                "counter_snapshot": shared::counter_snapshot_json(discard_record.counters()),
            }),
            Self::Promotion {
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
                "preview_vs_authoritative_matrix": shared::preview_vs_authoritative_matrix_json(
                    promotion_record,
                    discarded_record,
                    routing_digest.as_deref(),
                ),
                "replay_digest": shared::replay_digest(
                    promoted_replay_bundle.digest(),
                    discarded_replay_bundle.digest(),
                ),
                "diagnostics_digest": diagnostics_digest,
                "counter_snapshot": shared::counter_snapshot_json(promotion_record.counters()),
            }),
            Self::Churn {
                lifecycle_digests,
                branch_isolation_matrix,
                resource_bound_report,
                counter_snapshot,
            } => json!({
                "preview_lifecycle_digest": shared::preview_lifecycle_digest(lifecycle_digests),
                "resource_bound_report": resource_bound_report,
                "branch_isolation_matrix": branch_isolation_matrix,
                "counter_snapshot": counter_snapshot,
            }),
        }
    }

    pub(super) fn extensions_json(
        &self,
        runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Discard {
                execution_record,
                discard_record,
                routing_digest,
            } => BTreeMap::from([
                (
                    "bridge_speculation_certification_bundle".to_string(),
                    json!({
                        "speculative_resource_digest": shared::speculative_resource_digest(
                            execution_record.digest(),
                            Some(discard_record.digest()),
                            None,
                        ),
                        "discard_residue_report": shared::discard_residue_report_json(discard_record),
                        "routing_digest": routing_digest,
                        "counter_snapshot": shared::counter_snapshot_json(discard_record.counters()),
                    }),
                ),
                (
                    "bridge_speculation_record".to_string(),
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
                    }),
                ),
            ]),
            Self::Promotion {
                promoted_execution_record,
                promotion_record,
                promoted_replay_bundle,
                discarded_execution_record,
                discarded_record,
                discarded_replay_bundle,
                routing_digest,
                diagnostics_digest,
            } => BTreeMap::from([
                (
                    "bridge_speculation_certification_bundle".to_string(),
                    json!({
                        "speculative_commit_digest": shared::speculative_commit_digest(
                            promoted_execution_record.digest(),
                            promotion_record.digest(),
                            discarded_execution_record.digest(),
                            discarded_record.digest(),
                        ),
                        "preview_vs_authoritative_matrix": shared::preview_vs_authoritative_matrix_json(
                            promotion_record,
                            discarded_record,
                            routing_digest.as_deref(),
                        ),
                        "replay_digest": shared::replay_digest(
                            promoted_replay_bundle.digest(),
                            discarded_replay_bundle.digest(),
                        ),
                        "diagnostics_digest": diagnostics_digest,
                        "counter_snapshot": shared::counter_snapshot_json(promotion_record.counters()),
                    }),
                ),
                (
                    "bridge_speculation_record".to_string(),
                    json!({
                        "preview_execution_record_identity": promoted_execution_record.record_identity().as_str(),
                        "preview_promotion_record_identity": promotion_record.record_identity().as_str(),
                        "preview_session_identity": promotion_record.preview_session_identity(),
                        "discarded_preview_execution_record_identity": discarded_execution_record.record_identity().as_str(),
                        "discarded_preview_discard_record_identity": discarded_record.record_identity().as_str(),
                        "discarded_preview_session_identity": discarded_record.preview_session_identity(),
                        "promotion_explanation": {
                            "preview_promotion_record_identity": runtime_bridge
                                .diagnostics()
                                .explain_preview_promotion_record(promotion_record)
                                .preview_promotion_record_identity(),
                        },
                        "replay_explanation": {
                            "lifecycle_outcome": format!(
                                "{:?}",
                                runtime_bridge
                                    .diagnostics()
                                    .explain_preview_replay_bundle(promoted_replay_bundle)
                                    .lifecycle_outcome()
                            ),
                        },
                        "discard_explanation": {
                            "preview_discard_record_identity": runtime_bridge
                                .diagnostics()
                                .explain_preview_discard_record(discarded_record)
                                .preview_discard_record_identity(),
                        },
                        "discard_replay_explanation": {
                            "lifecycle_outcome": format!(
                                "{:?}",
                                runtime_bridge
                                    .diagnostics()
                                    .explain_preview_replay_bundle(discarded_replay_bundle)
                                    .lifecycle_outcome()
                            ),
                        },
                    }),
                ),
            ]),
            Self::Churn {
                lifecycle_digests,
                branch_isolation_matrix,
                resource_bound_report,
                counter_snapshot,
            } => BTreeMap::from([(
                "bridge_speculation_certification_bundle".to_string(),
                json!({
                    "preview_lifecycle_digest": shared::preview_lifecycle_digest(lifecycle_digests),
                    "resource_bound_report": resource_bound_report,
                    "branch_isolation_matrix": branch_isolation_matrix,
                    "counter_snapshot": counter_snapshot,
                }),
            )]),
        }
    }
}

pub(super) fn parse_speculation_harness_target(
    target: &str,
) -> Option<Result<SpeculationHarnessTarget, BridgeHarnessError>> {
    match target {
        "speculation-discard-certify" => Some(Ok(SpeculationHarnessTarget::DiscardCertification)),
        "speculation-promotion-certify" => {
            Some(Ok(SpeculationHarnessTarget::PromotionCertification))
        }
        "speculation-churn-certify" => Some(Ok(SpeculationHarnessTarget::ChurnCertification)),
        _ => None,
    }
}

pub(super) fn execute_speculation_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &crate::harness::fixtures::BridgeHarnessFixture,
    target: SpeculationHarnessTarget,
) -> Result<SpeculationHarnessExecution, BridgeHarnessError> {
    match target {
        SpeculationHarnessTarget::DiscardCertification => {
            discard::execute_discard_certification(runtime_bridge, fixture)
        }
        SpeculationHarnessTarget::PromotionCertification => {
            promotion::execute_promotion_certification(runtime_bridge, fixture)
        }
        SpeculationHarnessTarget::ChurnCertification => {
            churn::execute_churn_certification(runtime_bridge, fixture)
        }
    }
}
