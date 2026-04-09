use super::*;

pub(super) enum HarnessTarget {
    CommittedRoute {
        commit_identity: String,
    },
    Stream(super::stream::StreamHarnessTarget),
    Source(super::source::SourceHarnessTarget),
    Merge(super::merge::MergeHarnessTarget),
    Structural(super::structural::StructuralHarnessTarget),
    HistoricalCommit {
        branch_identity: TruthBranchIdentity,
        commit_identity: TruthCommitIdentity,
    },
    BranchHead {
        branch_identity: TruthBranchIdentity,
    },
    BranchCommit {
        branch_identity: TruthBranchIdentity,
        commit_identity: TruthCommitIdentity,
    },
}

pub(super) enum HarnessExecution {
    Route {
        result: crate::facade::BridgeRouteResult,
        continuity_summary: Option<(
            crate::facade::BridgeContinuityArtifact,
            crate::facade::BridgeCanonicalContinuityRecord,
        )>,
    },
    Historical {
        artifact: crate::facade::LoweredHistoricalEvaluationArtifact,
        record: crate::facade::BridgeCanonicalHistoricalEvaluationRecord,
        explanation: BridgeHistoricalEvaluationExplanation,
    },
    Stream(super::stream::StreamHarnessExecution),
    Source(super::source::SourceHarnessExecution),
    Merge(super::merge::MergeHarnessExecution),
    Structural(super::structural::StructuralHarnessExecution),
}

impl HarnessExecution {
    pub(super) fn summary_json(&self) -> serde_json::Value {
        match self {
            Self::Route {
                result,
                continuity_summary,
            } => json!({
                "route_identity": result.result_summary().route_identity().as_str(),
                "invalidation_identity": result.result_summary().invalidation_identity().as_str(),
                "subscription_slice_identity": result.result_summary().subscription_slice_identity().as_str(),
                "snapshot_identity": result.result_summary().snapshot_identity().as_str(),
                "subscription_slice_count": result.result_summary().subscription_slice_count(),
                "delivered_target_count": result.result_summary().delivered_target_count(),
                "continuity_identity": continuity_summary
                    .as_ref()
                    .map(|(artifact, _)| artifact.continuity_identity().as_str()),
                "continuity_resolution_digest": continuity_summary
                    .as_ref()
                    .map(|(artifact, _)| artifact.continuity_resolution_digest()),
            }),
            Self::Historical {
                artifact,
                record,
                explanation,
            } => json!({
                "historical_artifact_identity": artifact.artifact_identity().as_str(),
                "historical_record_identity": record.record_identity().as_str(),
                "declaration_identity": record.declaration().declaration_identity().as_str(),
                "snapshot_identity": explanation.snapshot_identity().as_str(),
                "branch_identity": explanation.branch_identity().as_str(),
                "commit_identity": explanation.commit_identity().as_str(),
                "materialization_path": format!("{:?}", explanation.materialization_path()),
                "selector_width": record.counters().selector_width(),
                "branch_width": record.counters().branch_width(),
            }),
            Self::Stream(execution) => execution.summary_json(),
            Self::Source(execution) => execution.summary_json(),
            Self::Merge(execution) => execution.summary_json(),
            Self::Structural(execution) => execution.summary_json(),
        }
    }

    pub(super) fn extensions_json(
        &self,
        runtime_bridge: &crate::facade::RuntimeBridge,
    ) -> BTreeMap<String, serde_json::Value> {
        match self {
            Self::Route {
                result,
                continuity_summary,
            } => BTreeMap::from([
                (
                    "bridge_delivery_receipt".to_string(),
                    json!({
                        "delivered_target_count": result.receipt().delivered_target_count(),
                        "snapshot_identity": result.receipt().snapshot_identity().as_str(),
                    }),
                ),
                (
                    "bridge_route_record".to_string(),
                    runtime_bridge
                        .diagnostics()
                        .last_route_record()
                        .map(route_record_json)
                        .unwrap_or_else(|| json!(null)),
                ),
                (
                    "bridge_continuity_record".to_string(),
                    continuity_summary
                        .as_ref()
                        .map(|(artifact, canonical)| json!({
                            "route_identity": canonical.route_identity().as_str(),
                            "continuity_request_digest": canonical.continuity_request_digest(),
                            "continuity_resolution_digest": canonical.continuity_resolution_digest(),
                            "continuity_artifact_identity": canonical.continuity_artifact_identity().as_str(),
                            "source_snapshot": canonical.source_snapshot().as_str(),
                            "source_branch": canonical.source_branch().as_str(),
                            "outcome_classes": artifact
                                .continuity_outcomes()
                                .iter()
                                .map(|entry| format!("{:?}", entry.outcome_class()))
                                .collect::<Vec<_>>(),
                        }))
                        .unwrap_or_else(|| json!(null)),
                ),
            ]),
            Self::Historical {
                artifact,
                record,
                explanation,
            } => BTreeMap::from([(
                "bridge_historical_evaluation_record".to_string(),
                json!({
                    "artifact_identity": artifact.artifact_identity().as_str(),
                    "artifact_digest": artifact.digest(),
                    "record_identity": record.record_identity().as_str(),
                    "decision_log_identity": record.decision_log().decision_log_identity().as_str(),
                    "declaration_identity": record.declaration().declaration_identity().as_str(),
                    "snapshot_identity": explanation.snapshot_identity().as_str(),
                    "branch_identity": explanation.branch_identity().as_str(),
                    "commit_identity": explanation.commit_identity().as_str(),
                    "materialization_path": format!("{:?}", explanation.materialization_path()),
                    "counters": {
                        "truth_view_selector_count": record.counters().truth_view_selector_count(),
                        "historical_truth_view_count": record.counters().historical_truth_view_count(),
                        "branch_truth_view_count": record.counters().branch_truth_view_count(),
                        "planned_truth_view_packet_count": record.counters().planned_truth_view_packet_count(),
                        "resolved_truth_view_policy_count": record.counters().resolved_truth_view_policy_count(),
                        "materialized_truth_view_count": record.counters().materialized_truth_view_count(),
                        "truth_view_unavailable_count": record.counters().truth_view_unavailable_count(),
                        "truth_view_branch_mismatch_count": record.counters().truth_view_branch_mismatch_count(),
                        "truth_view_snapshot_mismatch_count": record.counters().truth_view_snapshot_mismatch_count(),
                        "historical_replay_mismatch_count": record.counters().historical_replay_mismatch_count(),
                        "branch_local_evaluation_count": record.counters().branch_local_evaluation_count(),
                        "truth_view_decision_log_count": record.counters().truth_view_decision_log_count(),
                        "selector_width": record.counters().selector_width(),
                        "branch_width": record.counters().branch_width(),
                        "direct_snapshot_materialization_count": record.counters().direct_snapshot_materialization_count(),
                        "commit_envelope_materialization_count": record.counters().commit_envelope_materialization_count(),
                        "branch_head_materialization_count": record.counters().branch_head_materialization_count(),
                    },
                }),
            )]),
            Self::Stream(execution) => execution.extensions_json(runtime_bridge),
            Self::Source(execution) => execution.extensions_json(runtime_bridge),
            Self::Merge(execution) => execution.extensions_json(runtime_bridge),
            Self::Structural(execution) => execution.extensions_json(runtime_bridge),
        }
    }
}

pub(super) fn execute_historical_request(
    runtime_bridge: &crate::facade::RuntimeBridge,
    declaration: HistoricalEvaluationDeclaration,
) -> Result<HarnessExecution, BridgeHarnessError> {
    let planned = runtime_bridge
        .plan_truth_view_packet(declaration, SnapshotReadPacket::new(vec![]))
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge historical planning failed: {error}"))
        })?;
    let observation = runtime_bridge
        .materialize_truth_view_observation(planned)
        .map_err(|error| {
            BridgeHarnessError::new(format!("bridge historical materialization failed: {error}"))
        })?;
    let artifact = runtime_bridge.lower_historical_evaluation_artifact(&observation);
    let record = runtime_bridge.canonicalize_historical_evaluation_record(&observation);
    let explanation = runtime_bridge
        .diagnostics()
        .explain_historical_evaluation_record(&record);

    Ok(HarnessExecution::Historical {
        artifact,
        record,
        explanation,
    })
}

pub(super) fn parse_harness_target(target: &str) -> Result<HarnessTarget, BridgeHarnessError> {
    if let Some(stream_target) = super::stream::parse_stream_harness_target(target) {
        return stream_target.map(HarnessTarget::Stream);
    }
    if let Some(source_target) = super::source::parse_source_harness_target(target) {
        return source_target.map(HarnessTarget::Source);
    }
    if let Some(merge_target) = super::merge::parse_merge_harness_target(target) {
        return merge_target.map(HarnessTarget::Merge);
    }
    if let Some(structural_target) = super::structural::parse_structural_harness_target(target) {
        return structural_target.map(HarnessTarget::Structural);
    }

    if let Some(rest) = target.strip_prefix("history-commit:") {
        let mut parts = rest.splitn(2, ':');
        let branch = parts
            .next()
            .ok_or_else(|| BridgeHarnessError::new("history-commit target requires branch"))?;
        let commit = parts
            .next()
            .ok_or_else(|| BridgeHarnessError::new("history-commit target requires commit"))?;
        return Ok(HarnessTarget::HistoricalCommit {
            branch_identity: TruthBranchIdentity::new(branch),
            commit_identity: TruthCommitIdentity::new(commit),
        });
    }
    if let Some(rest) = target.strip_prefix("branch-head:") {
        return Ok(HarnessTarget::BranchHead {
            branch_identity: TruthBranchIdentity::new(rest),
        });
    }
    if let Some(rest) = target.strip_prefix("branch-commit:") {
        let mut parts = rest.splitn(2, ':');
        let branch = parts
            .next()
            .ok_or_else(|| BridgeHarnessError::new("branch-commit target requires branch"))?;
        let commit = parts
            .next()
            .ok_or_else(|| BridgeHarnessError::new("branch-commit target requires commit"))?;
        return Ok(HarnessTarget::BranchCommit {
            branch_identity: TruthBranchIdentity::new(branch),
            commit_identity: TruthCommitIdentity::new(commit),
        });
    }

    Ok(HarnessTarget::CommittedRoute {
        commit_identity: target.to_string(),
    })
}
