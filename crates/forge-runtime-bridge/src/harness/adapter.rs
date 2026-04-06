use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    diagnostics_id, replay_id, run_id, snapshot_id, AdapterSupport, CaptureDepth,
    DiagnosticsHarnessAdapter, DiagnosticsLevel, ExecutionMode, ExecutionProfile, ExecutionRequest,
    HarnessAdapter, HarnessCapabilities, MutationBatch, ObservationStatus, RecordSchemaVersion,
    ReplayHarnessAdapter, ReplayRecord, ReplayRequest, RunOutcome, RunRecord, RunStatus,
    ScenarioFixture, SnapshotObservation, SnapshotPayload, SnapshotRecord, StructuredValue,
    TargetStatusRecord,
};
use serde_json::json;

use crate::facade::{BridgeRouteRequest, RawCommittedPatchEnvelope, RuntimeBridgeBuilder};

use super::fixtures::{
    BridgeHarnessFixture, InMemoryRelationalBridgeSource, RecordingSignalBridgeSink, SnapshotFixture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BridgeHarnessMutation {
    PublishCommittedPatch(RawCommittedPatchEnvelope),
    PublishSnapshot(SnapshotFixture),
}

#[derive(Debug, Clone)]
pub struct BridgeHarnessSession {
    pub(crate) runtime: Option<crate::facade::RuntimeBridge>,
    pub(crate) source: InMemoryRelationalBridgeSource,
    pub(crate) sink: RecordingSignalBridgeSink,
}

impl Default for BridgeHarnessSession {
    fn default() -> Self {
        Self {
            runtime: None,
            source: InMemoryRelationalBridgeSource::default(),
            sink: RecordingSignalBridgeSink::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeHarnessError(String);

impl BridgeHarnessError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl std::fmt::Display for BridgeHarnessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for BridgeHarnessError {}

#[derive(Debug, Clone, Copy, Default)]
pub struct BridgeHarnessAdapter;

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

        let builder = RuntimeBridgeBuilder::new()
            .with_relational_source(runtime.source.clone())
            .with_signal_sink(runtime.sink.clone())
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
        let commit_identity = request
            .targets
            .first()
            .ok_or_else(|| BridgeHarnessError::new("bridge execution requires one commit target"))?;
        let route = runtime_bridge
            .plan_committed_patch(BridgeRouteRequest::for_commit(commit_identity.clone()))
            .map_err(|error| BridgeHarnessError::new(format!("bridge planning failed: {error}")))?;
        let result = runtime_bridge
            .deliver_invalidation(route)
            .map_err(|error| BridgeHarnessError::new(format!("bridge delivery failed: {error}")))?;
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);

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
            summary: json!({
                "route_identity": result.result_summary().route_identity().as_str(),
                "invalidation_identity": result.result_summary().invalidation_identity().as_str(),
                "subscription_slice_identity": result.result_summary().subscription_slice_identity().as_str(),
                "snapshot_identity": result.result_summary().snapshot_identity().as_str(),
                "subscription_slice_count": result.result_summary().subscription_slice_count(),
                "delivered_target_count": result.result_summary().delivered_target_count(),
            }),
            extensions: BTreeMap::from([
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
            ]),
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
        let last_route_record = runtime_bridge.diagnostics().last_route_record();
        let (status, detail, value) = match last_route_record {
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

impl DiagnosticsHarnessAdapter for BridgeHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        _fixture: &ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<forge_harness::facade::DiagnosticsRecord, Self::Error> {
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let run_id_value =
            forge_harness::facade::RunId::new(format!("bridge-diagnostics:{}", profile.name));
        Ok(forge_harness::facade::DiagnosticsRecord {
            schema_version: RecordSchemaVersion::V1,
            diagnostics_id: diagnostics_id(&run_id_value),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: json!({
                "tier": format!("{:?}", runtime_bridge.diagnostics().tier()),
                "record_count": runtime_bridge.diagnostics().route_records().len(),
                "route_records": runtime_bridge
                    .diagnostics()
                    .route_records()
                    .into_iter()
                    .map(route_record_json)
                    .collect::<Vec<_>>(),
            }),
            extensions: BTreeMap::new(),
        })
    }
}

impl ReplayHarnessAdapter for BridgeHarnessAdapter {
    fn capture_replay(
        &self,
        runtime: &Self::Runtime,
        fixture: &ScenarioFixture<Self::Fixture>,
        replay: &ReplayRequest<Self::TargetId>,
    ) -> Result<ReplayRecord<Self::TargetId>, Self::Error> {
        let runtime_bridge = runtime
            .runtime
            .as_ref()
            .ok_or_else(|| BridgeHarnessError::new("bridge runtime not loaded"))?;
        let route_record = runtime_bridge
            .diagnostics()
            .last_canonical_route_record()
            .ok_or_else(|| BridgeHarnessError::new("no canonical bridge route record available"))?;
        let replay_record = runtime_bridge
            .replay_canonical_record(&route_record)
            .map_err(|error| BridgeHarnessError::new(format!("bridge replay failed: {error}")))?;
        Ok(ReplayRecord {
            schema_version: RecordSchemaVersion::V1,
            replay_id: replay_id(&replay.source_run.run_id, &replay.name),
            source_run_id: replay.source_run.run_id.clone(),
            scenario_id: forge_harness::facade::scenario_id(&fixture.name),
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            replay_name: replay.name.clone(),
            requested_targets: replay.request.targets.clone(),
            summary: json!({
                "route_identity": replay_record.route_identity().as_str(),
                "invalidation_identity": replay_record.invalidation_identity().as_str(),
                "subscription_slice_identity": replay_record.subscription_slice_identity().as_str(),
                "source_commit": replay_record.source_commit().as_str(),
                "source_patch": replay_record.source_patch().as_str(),
                "source_snapshot": replay_record.source_snapshot().as_str(),
            }),
            attachments: Vec::new(),
        })
    }
}

fn route_record_json(record: crate::diagnostics::BridgeRouteRecord) -> serde_json::Value {
    json!({
        "route_identity": record.route_identity().as_str(),
        "invalidation_identity": record.invalidation_identity().as_str(),
        "source_commit": record.source_commit().as_str(),
        "source_patch": record.source_patch().as_str(),
        "source_snapshot": record.source_snapshot().as_str(),
        "source_digest": record.source_digest().as_str(),
        "subscription_slice_identity": record.subscription_slice_identity().as_str(),
        "entries": record.entries().iter().map(|entry| {
            json!({
                "entity_identity": entry.entity_identity(),
                "aspect_label": entry.aspect_label(),
                "surface_label": entry.surface_label(),
                "mapping_id": entry.mapping_id().as_str(),
                "signal_scope": entry.signal_scope(),
                "routing_mode": format!("{:?}", entry.routing_mode()),
                "fallback_class": entry.fallback_class().map(|class| format!("{class:?}")),
                "truth_surface_kind": format!("{:?}", entry.truth_surface_kind()),
                "fine_grained_match_status": format!("{:?}", entry.fine_grained_match_status()),
                "aspect_registration_id": entry.aspect_registration_id().map(|id| id.as_str()),
                "subscription_slice_kind": entry.subscription_slice_kind().map(|kind| format!("{kind:?}")),
                "slice_fallback_policy": entry.slice_fallback_policy().map(|policy| format!("{policy:?}")),
            })
        }).collect::<Vec<_>>(),
        "subscription_slices": record.subscription_slices().iter().map(|slice| {
            json!({
                "entity_identity": slice.entity_identity(),
                "aspect_label": slice.aspect_label(),
                "surface_label": slice.surface_label(),
                "slice_kind": format!("{:?}", slice.slice_kind()),
                "match_status": format!("{:?}", slice.match_status()),
            })
        }).collect::<Vec<_>>(),
        "invalidation_targets": record.invalidation_targets().iter().map(|target| {
            json!({
                "signal_scope": target.signal_scope(),
                "routing_mode": format!("{:?}", target.routing_mode()),
            })
        }).collect::<Vec<_>>(),
        "counters": {
            "patch_item_count": record.counters().patch_item_count(),
            "normalized_patch_item_count": record.counters().normalized_patch_item_count(),
            "truth_delta_surface_count": record.counters().truth_delta_surface_count(),
            "normalized_truth_delta_surface_count": record.counters().normalized_truth_delta_surface_count(),
            "planned_slice_match_count": record.counters().planned_slice_match_count(),
            "slice_fallback_count": record.counters().slice_fallback_count(),
            "slice_suppression_count": record.counters().slice_suppression_count(),
            "routing_entry_count": record.counters().routing_entry_count(),
            "invalidation_target_count": record.counters().invalidation_target_count(),
            "mapping_lookup_count": record.counters().mapping_lookup_count(),
            "mapping_fallback_count": record.counters().mapping_fallback_count(),
            "snapshot_read_count": record.counters().snapshot_read_count(),
            "snapshot_read_packet_count": record.counters().snapshot_read_packet_count(),
            "snapshot_identity_mismatch_count": record.counters().snapshot_identity_mismatch_count(),
            "route_replay_mismatch_count": record.counters().route_replay_mismatch_count(),
        }
    })
}
