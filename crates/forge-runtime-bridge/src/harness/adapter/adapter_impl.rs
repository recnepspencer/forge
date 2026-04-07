use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    run_id, snapshot_id, AdapterSupport, CaptureDepth, DiagnosticsLevel, ExecutionMode,
    ExecutionProfile, ExecutionRequest, HarnessAdapter, HarnessCapabilities, MutationBatch,
    ObservationStatus, RecordSchemaVersion, RunOutcome, RunRecord, RunStatus, ScenarioFixture,
    SnapshotObservation, SnapshotPayload, SnapshotRecord, StructuredValue, TargetStatusRecord,
};
use serde_json::json;

use crate::facade::{BridgeRouteRequest, RuntimeBridgeBuilder};

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
