use std::collections::{BTreeMap, BTreeSet};

use forge_harness::facade::{
    diagnostics_id, replay_id, run_id, snapshot_id, AdapterSupport, CaptureDepth,
    DiagnosticsHarnessAdapter, DiagnosticsLevel, ExecutionMode, ExecutionProfile, ExecutionRequest,
    HarnessAdapter, HarnessCapabilities, MutationBatch, ReplayHarnessAdapter, ReplayRecord,
    ReplayRequest, RunOutcome, RunRecord, RunStatus, SnapshotObservation, SnapshotPayload,
    SnapshotRecord, StructuredValue, TargetStatusRecord,
};
use serde_json::json;

use crate::query::data::QueryWorkPacket;
use crate::transactions::data::{RecordRef, TransactionOptions};
use crate::facade::{RelationalRuntime, RelationalRuntimeApi, WorkerIntentBatch};

use super::harness_data::{
    RelationalFixture, RelationalHarnessAdapter, RelationalHarnessError,
};
use super::harness_batches::{entity_fixture_batch, relation_fixture_batch};
use super::harness_targets::{
    commit_error_to_harness_error, default_harness_schema_registry, parse_target, resolve_targets,
};

impl HarnessAdapter for RelationalHarnessAdapter {
    type Runtime = RelationalRuntime;
    type Fixture = RelationalFixture;
    type Mutation = WorkerIntentBatch;
    type TargetId = String;
    type Error = RelationalHarnessError;

    fn adapter_name(&self) -> &'static str {
        "forge-relational"
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
            "relational_patch".to_string(),
            "relational_replay".to_string(),
            "relational_diagnostics".to_string(),
        ]);
        capabilities
    }

    fn create_runtime(&self) -> Result<Self::Runtime, Self::Error> {
        Ok(RelationalRuntimeApi::builder()
            .schema_registry(default_harness_schema_registry())
            .build())
    }

    fn load_fixture(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
    ) -> Result<(), Self::Error> {
        if fixture.fixture.entities.is_empty() && fixture.fixture.relations.is_empty() {
            return Ok(());
        }
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(entity_fixture_batch(&fixture.fixture.entities));
        let outcome = txn.commit().map_err(commit_error_to_harness_error)?;
        let entity_ids = outcome
            .changed_records
            .iter()
            .filter_map(|record| match record {
                crate::transactions::data::RecordRef::Entity(entity_id) => Some(entity_id.clone()),
                crate::transactions::data::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        if !fixture.fixture.relations.is_empty() {
            let mut relation_txn = runtime.begin_transaction(TransactionOptions::default());
            relation_txn.push_batch(relation_fixture_batch(&fixture.fixture.relations, &entity_ids)?);
            relation_txn
                .commit()
                .map_err(commit_error_to_harness_error)?;
        }
        Ok(())
    }

    fn apply_mutation_batch(
        &self,
        runtime: &mut Self::Runtime,
        batch: &MutationBatch<Self::Mutation>,
    ) -> Result<(), Self::Error> {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        for operation in &batch.operations {
            txn.push_batch(operation.clone());
        }
        txn.commit().map_err(commit_error_to_harness_error)?;
        Ok(())
    }

    fn execute(
        &self,
        runtime: &mut Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<RunRecord<Self::TargetId>, Self::Error> {
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
        let snapshot = runtime.snapshot_access().snapshot();
        let mut read_view = runtime.visibility_reads().read_version(snapshot.version_id);
        read_view.snapshot = snapshot.clone();
        let targets = resolve_targets(request);
        let packet = QueryWorkPacket::bulk(
            "execute",
            targets
                .iter()
                .map(|target| parse_target(target))
                .collect::<Result<Vec<_>, _>>()?,
        );
        let result = read_view.execute_packet(&packet);
        Ok(RunRecord {
            schema_version: forge_harness::facade::RecordSchemaVersion::V1,
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
            requested_targets: targets.clone(),
            target_statuses: targets
                .iter()
                .map(|target| TargetStatusRecord {
                    target: target.clone(),
                    status: forge_harness::facade::ObservationStatus::Validated,
                    detail: None,
                })
                .collect(),
            changed_targets: targets,
            attachments: Vec::new(),
            summary: json!({
                "snapshot_id": snapshot.snapshot_id.0,
                "entity_hits": result.entities.len(),
                "relation_hits": result.relations.len(),
            }),
            extensions: BTreeMap::from([
                (
                    "relational_patch".to_string(),
                    serde_json::to_value(runtime.publication_access().latest_patch())
                        .unwrap_or_else(|_| json!(null)),
                ),
                (
                    "relational_replay".to_string(),
                    serde_json::to_value(runtime.publication_access().latest_replay())
                        .unwrap_or_else(|_| json!(null)),
                ),
            ]),
        })
    }

    fn capture_snapshot(
        &self,
        runtime: &Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        request: &ExecutionRequest<Self::TargetId>,
        profile: &ExecutionProfile,
    ) -> Result<SnapshotRecord<Self::TargetId>, Self::Error> {
        let scenario_id_value = forge_harness::facade::scenario_id(&fixture.name);
        let run_id_value = run_id(&scenario_id_value, &profile.name, &request.name);
        let mut clone = runtime.fork();
        let snapshot = clone.snapshot_access().snapshot();
        let mut read_view = clone.visibility_reads().read_version(snapshot.version_id);
        read_view.snapshot = snapshot.clone();
        let observations = resolve_targets(request)
            .into_iter()
            .map(|target| {
                let payload = match parse_target(&target)? {
                    RecordRef::Entity(entity_id) => {
                        read_view.get_entity(entity_id).map(|entity| {
                            SnapshotPayload::Structured(StructuredValue::Json(json!(entity)))
                        })
                    }
                    RecordRef::Relation(relation_id) => {
                        read_view.get_relation(relation_id).map(|relation| {
                            SnapshotPayload::Structured(StructuredValue::Json(json!(relation)))
                        })
                    }
                };
                Ok(SnapshotObservation {
                    target,
                    status: forge_harness::facade::ObservationStatus::Clean,
                    detail: None,
                    value: payload,
                })
            })
            .collect::<Result<Vec<_>, RelationalHarnessError>>()?;
        Ok(SnapshotRecord {
            schema_version: forge_harness::facade::RecordSchemaVersion::V1,
            snapshot_id: snapshot_id(&run_id_value, "capture"),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            profile_name: profile.name.clone(),
            time_marker: profile.time_marker.clone(),
            observations,
            attachments: Vec::new(),
            extensions: BTreeMap::new(),
        })
    }
}

impl DiagnosticsHarnessAdapter for RelationalHarnessAdapter {
    fn capture_diagnostics(
        &self,
        runtime: &Self::Runtime,
        _fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        profile: &ExecutionProfile,
    ) -> Result<forge_harness::facade::DiagnosticsRecord, Self::Error> {
        let run_id_value =
            forge_harness::facade::RunId::new(format!("diagnostics:{}", profile.name));
        Ok(forge_harness::facade::DiagnosticsRecord {
            schema_version: forge_harness::facade::RecordSchemaVersion::V1,
            diagnostics_id: diagnostics_id(&run_id_value),
            run_id: run_id_value,
            adapter_name: self.adapter_name().to_string(),
            profile_name: profile.name.clone(),
            level: profile.diagnostics_level,
            time_marker: profile.time_marker.clone(),
            attachments: Vec::new(),
            summary: serde_json::to_value(runtime.publication_access().diagnostics())
                .unwrap_or_else(|_| json!({})),
            extensions: BTreeMap::new(),
        })
    }
}

impl ReplayHarnessAdapter for RelationalHarnessAdapter {
    fn capture_replay(
        &self,
        runtime: &Self::Runtime,
        fixture: &forge_harness::facade::ScenarioFixture<Self::Fixture>,
        replay: &ReplayRequest<Self::TargetId>,
    ) -> Result<ReplayRecord<Self::TargetId>, Self::Error> {
        let latest_replay = runtime
            .publication_access()
            .latest_replay()
            .cloned()
            .ok_or_else(|| RelationalHarnessError("no replay artifact available".to_string()))?;
        Ok(ReplayRecord {
            schema_version: forge_harness::facade::RecordSchemaVersion::V1,
            replay_id: replay_id(&replay.source_run.run_id, &replay.name),
            source_run_id: replay.source_run.run_id.clone(),
            scenario_id: forge_harness::facade::scenario_id(&fixture.name),
            adapter_name: self.adapter_name().to_string(),
            scenario_name: fixture.name.clone(),
            replay_name: replay.name.clone(),
            requested_targets: replay.request.targets.clone(),
            summary: serde_json::to_value(latest_replay).unwrap_or_else(|_| json!({})),
            attachments: Vec::new(),
        })
    }
}
