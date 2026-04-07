use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    run_id, snapshot_id, AdapterSupport, CaptureDepth, DiagnosticsLevel, ExecutionMode,
    ExecutionProfile, ExecutionRequest, HarnessAdapter, HarnessCapabilities, MutationBatch,
    ObservationStatus, RecordSchemaVersion, RunOutcome, RunRecord, RunStatus, ScenarioFixture,
    SnapshotObservation, SnapshotPayload, SnapshotRecord, StructuredValue, TargetStatusRecord,
};
use serde_json::json;

use crate::facade::{
    BridgeDeliveryIntent, BridgeHistoricalEvaluationExplanation, BridgeMappingContext,
    BridgeReplayMode, BridgeRouteRequest, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
    RuntimeBridgeBuilder, SnapshotReadPacket, TruthBranchIdentity, TruthCommitIdentity,
};

use super::super::fixtures::BridgeHarnessFixture;
use super::support::route_record_json;
use super::types::{BridgeHarnessAdapter, BridgeHarnessError, BridgeHarnessMutation, BridgeHarnessSession};

impl HarnessAdapter for BridgeHarnessAdapter {
    type Runtime = BridgeHarnessSession;
    type Fixture = BridgeHarnessFixture;
    type Mutation = BridgeHarnessMutation;
    type TargetId = String;
    type Error = BridgeHarnessError;

    fn adapter_name(&self) -> &'static str {
        "forge-runtime-bridge"
    }

    fn capabilities(&self) -> HarnessCapabilities {
        let mut capabilities = HarnessCapabilities::default();
        capabilities.execution_modes =
            BTreeSet::from([ExecutionMode::RuntimeDefault, ExecutionMode::Serial]);
        capabilities.diagnostics_levels = BTreeSet::from([
            DiagnosticsLevel::Operational,
            DiagnosticsLevel::Development,
            DiagnosticsLevel::Forensic,
        ]);
        capabilities.capture_depths = BTreeSet::from([
            CaptureDepth::Minimal,
            CaptureDepth::Standard,
            CaptureDepth::Rich,
        ]);
        capabilities.replay_support = AdapterSupport::Supported;
        capabilities.rich_record_kinds = BTreeSet::from([
            "bridge_route_record".to_string(),
            "bridge_replay_record".to_string(),
            "bridge_delivery_receipt".to_string(),
            "bridge_historical_evaluation_record".to_string(),
        ]);
        capabilities
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(BridgeHarnessSession::default())
    }

    fn prepare_runtime(
        &self,
        _runtime: &mut Self::Runtime,
        profile: &ExecutionProfile,
    ) -> Result<(), Self::Error> {
        match profile.execution_mode {
            ExecutionMode::RuntimeDefault | ExecutionMode::Serial => Ok(()),
            mode => Err(BridgeHarnessError::new(format!(
                "bridge harness does not support execution mode `{mode:?}`"
            ))),
        }
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        for patch in fixture.fixture.committed_patches() {
            runtime.source.insert_committed_patch(patch.clone());
        }
        for snapshot in fixture.fixture.snapshots() {
            runtime.source.insert_snapshot(snapshot.clone());
        }
        for (entity_identity, authority) in fixture.fixture.continuity_authorities() {
            runtime
                .source
                .insert_continuity_authority(entity_identity.clone(), authority.clone());
        }

        let builder = RuntimeBridgeBuilder::new()
            .with_relational_source(runtime.source.clone())
            .with_truth_branch_head_source(runtime.source.clone())
            .with_signal_sink(runtime.sink.clone())
            .with_continuity_lineage_source(runtime.source.clone())
            .with_policy(fixture.fixture.policy());
        let (first_mapping, remaining_mappings) = fixture
            .fixture
            .mappings()
            .split_first()
            .ok_or_else(|| BridgeHarnessError::new("bridge harness fixture requires at least one mapping"))?;
        let mut builder = builder.register_mapping(first_mapping.clone());
        for mapping in remaining_mappings {
            builder = builder.register_mapping(mapping.clone());
        }
        for aspect_mapping in fixture.fixture.aspect_mappings() {
            builder = builder.register_aspect_mapping(aspect_mapping.clone());
        }
        runtime.runtime = Some(builder.build().map_err(|error| {
            BridgeHarnessError::new(format!("failed to build bridge runtime: {error}"))
        })?);
        Ok(())
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        for operation in &batch.operations {
            match operation {
                BridgeHarnessMutation::PublishCommittedPatch(patch) => {
                    runtime.source.insert_committed_patch(patch.clone());
                }
                BridgeHarnessMutation::PublishSnapshot(snapshot) => {
                    runtime.source.insert_snapshot(snapshot.clone());
                }
            }
        }
        Ok(())
    }

    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error> {
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let target = request
            .targets
            .first()
            .ok_or_else(|| BridgeHarnessError::new("bridge execution requires one target"))?;
        let harness_target = parse_harness_target(target)?;
        let mapping_context = fixture
            .fixture
            .lineage_context()
            .cloned()
            .map(|lineage_context| BridgeMappingContext::default().with_lineage_context(lineage_context))
            .unwrap_or_default();
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);

        let execution = match harness_target {
            HarnessTarget::CommittedRoute { commit_identity } => {
                let route = runtime_bridge
                    .plan_committed_patch_with_mapping_context(
                        BridgeRouteRequest::for_commit(commit_identity.clone()),
                        mapping_context,
                    )
                    .map_err(|error| {
                        BridgeHarnessError::new(format!("bridge planning failed: {error}"))
                    })?;
                let result = runtime_bridge
                    .deliver_invalidation(route)
                    .map_err(|error| {
                        BridgeHarnessError::new(format!("bridge delivery failed: {error}"))
                    })?;
                let continuity_summary = runtime_bridge
                    .diagnostics()
                    .last_route_record()
                    .and_then(|route_record| {
                        let requests = runtime_bridge.plan_continuity_requests(&route_record).ok()?;
                        let packet = runtime_bridge.plan_historical_lineage_packet(&requests).ok()?;
                        let resolved = runtime_bridge.resolve_lineage_continuity(&packet).ok()?;
                        let artifact = runtime_bridge.lower_continuity_artifact(&resolved);
                        let canonical = runtime_bridge
                            .canonicalize_continuity_record(&route_record, &requests, &artifact);
                        Some((artifact, canonical))
                    });
                HarnessExecution::Route {
                    result,
                    continuity_summary,
                }
            }
            HarnessTarget::HistoricalCommit {
                branch_identity,
                commit_identity,
            } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::historical_commit(branch_identity, commit_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
            HarnessTarget::BranchHead { branch_identity } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::branch_head(branch_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
            HarnessTarget::BranchCommit {
                branch_identity,
                commit_identity,
            } => execute_historical_request(
                runtime_bridge,
                HistoricalEvaluationDeclaration::new(
                    BridgeTruthViewSelector::branch_commit(branch_identity, commit_identity),
                    BridgeReplayMode::Enabled,
                    fixture.fixture.policy().diagnostics_tier(),
                    BridgeDeliveryIntent::PrepareSignalEvaluation,
                ),
            )?,
        };

        Ok(RunRecord {
            schema_version: RecordSchemaVersion::V1,
            run_id: run_id_value,
            scenario_id: scenario_id_value,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            feed_batch: request.feed_batch.clone(),
            execution_mode: profile.execution_mode,
            diagnostics_level: profile.diagnostics_level,
            status: RunStatus::Succeeded,
            outcome: RunOutcome::Completed,
            budget_usage: None,
            requested_targets: request.targets.clone(),
            target_statuses: request
                .targets
                .iter()
                .map(|target| TargetStatusRecord {
                    target: target.clone(),
                    status: ObservationStatus::Validated,
                    detail: None,
                })
                .collect(),
            changed_targets: request.targets.clone(),
            attachments: Vec::new(),
            summary: execution.summary_json(),
            extensions: execution.extensions_json(runtime_bridge),
        })
    }

    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error> {
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let (status, detail, value) = if let Some(record) =
            runtime_bridge.diagnostics().last_historical_evaluation_record()
        {
            (
                ObservationStatus::Clean,
                Some("historical evaluation record captured".to_string()),
                Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                    "snapshot_identity": record.decision_log().snapshot_identity().as_str(),
                    "record_identity": record.record_identity().as_str(),
                })))),
            )
        } else {
            let last_route_record = runtime_bridge.diagnostics().last_route_record();
            match last_route_record {
                Some(record) => (
                    ObservationStatus::Clean,
                    Some(format!(
                        "{} snapshot records captured",
                        record.counters().snapshot_read_count()
                    )),
                    Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                        "snapshot_identity": record.source_snapshot().as_str(),
                        "read_count": record.counters().snapshot_read_count(),
                    })))),
                ),
                None => (
                    ObservationStatus::Clean,
                    Some("bridge delivery has not run yet".to_string()),
                    Some(SnapshotPayload::Structured(StructuredValue::Json(json!({
                        "snapshot_identity": null,
                        "read_count": 0,
                    })))),
                ),
            }
        };
        Ok(SnapshotRecord {
            schema_version: RecordSchemaVersion::V1,
            snapshot_id: snapshot_id(&run_id_value, "capture"),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            observations: vec![SnapshotObservation {
                target: request
                    .targets
                    .first()
                    .cloned()
                    .unwrap_or_else(|| "bridge".to_string()),
                status,
                detail,
                value,
            }],
            attachments: Vec::new(),
            extensions: BTreeMap::new(),
        })
    }
}

enum HarnessTarget {
    CommittedRoute {
        commit_identity: String,
    },
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

enum HarnessExecution {
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
}

impl HarnessExecution {
    fn summary_json(&self) -> serde_json::Value {
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
        }
    }

    fn extensions_json(
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
        }
    }
}

fn execute_historical_request(
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

fn parse_harness_target(target: &str) -> Result<HarnessTarget, BridgeHarnessError> {
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
