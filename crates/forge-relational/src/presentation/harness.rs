use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use forge_harness::facade::{
    diagnostics_id, replay_id, run_id, snapshot_id, AdapterSupport, CaptureDepth,
    DiagnosticsHarnessAdapter, DiagnosticsLevel, ExecutionMode, ExecutionProfile, ExecutionRequest,
    HarnessAdapter, HarnessCapabilities, MutationBatch, ReplayHarnessAdapter, ReplayRecord,
    ReplayRequest, RunOutcome, RunRecord, RunStatus, SnapshotObservation, SnapshotPayload,
    SnapshotRecord, StructuredValue, TargetStatusRecord,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::data::config::{CascadeDeletePolicy, CrossContextPolicy};
use crate::data::payload::RecordPayload;
use crate::data::query::{QueryWorkPacket, ReadTarget};
use crate::data::schema::RelationPayloadClass;
use crate::data::symbols::InternedString;
use crate::data::transaction::{TransactionIntent, TransactionOptions, WorkerIntentBatch};
use crate::facade::{
    EntityId, EntityKindRegistration, KindId, PartitionId, RelationId, RelationKindRegistration,
    RelationalRuntime, RelationalRuntimeApi, RelationalSchemaRegistry, SchemaId, SchemaVersionId,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessExpectations {
    pub execution_mode: ExecutionMode,
    pub diagnostics_level: DiagnosticsLevel,
    pub capture_depth: CaptureDepth,
    pub serial_parallel_parity_required: bool,
}

impl Default for RelationalHarnessExpectations {
    fn default() -> Self {
        Self {
            execution_mode: ExecutionMode::Serial,
            diagnostics_level: DiagnosticsLevel::Forensic,
            capture_depth: CaptureDepth::Rich,
            serial_parallel_parity_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessPlan {
    pub adapter_name: String,
    pub expectations: RelationalHarnessExpectations,
    pub required_seeders: Vec<String>,
}

impl RelationalHarnessPlan {
    pub fn relational() -> Self {
        Self {
            adapter_name: "forge-relational".to_string(),
            expectations: RelationalHarnessExpectations::default(),
            required_seeders: vec![
                "branch-history".to_string(),
                "replay-parity".to_string(),
                "diff-cdc".to_string(),
                "serialized-authority".to_string(),
                "cross-order-intents".to_string(),
            ],
        }
    }
}

pub fn default_harness_expectations() -> RelationalHarnessExpectations {
    RelationalHarnessExpectations::default()
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalFixture {
    pub entities: Vec<FixtureEntity>,
    pub relations: Vec<FixtureRelation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureEntity {
    pub kind_id: KindId,
    pub client_key: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FixtureRelation {
    pub kind_id: KindId,
    pub client_key: String,
    pub source_slot: u64,
    pub target_slot: u64,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationalMutation {
    Batch(WorkerIntentBatch),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalHarnessAdapter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelationalHarnessError(pub String);

impl fmt::Display for RelationalHarnessError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for RelationalHarnessError {}

impl HarnessAdapter for RelationalHarnessAdapter {
    type Runtime = RelationalRuntime;
    type Fixture = RelationalFixture;
    type Mutation = RelationalMutation;
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
        let mut batch = WorkerIntentBatch::new("fixture");
        for entity in &fixture.fixture.entities {
            batch.intents.push(TransactionIntent::CreateEntity(
                crate::data::transaction::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: entity.kind_id,
                    client_key: InternedString::Raw(entity.client_key.clone()),
                    payload: RecordPayload::StructuredJson(entity.payload.clone()),
                },
            ));
        }
        txn.push_batch(batch);
        let outcome = txn.commit().map_err(commit_error_to_harness_error)?;
        let entity_ids = outcome
            .changed_records
            .iter()
            .filter_map(|record| match record {
                crate::data::transaction::RecordRef::Entity(entity_id) => Some(*entity_id),
                crate::data::transaction::RecordRef::Relation(_) => None,
            })
            .collect::<Vec<_>>();
        if !fixture.fixture.relations.is_empty() {
            let mut relation_txn = runtime.begin_transaction(TransactionOptions::default());
            let mut relation_batch = WorkerIntentBatch::new("fixture-relations");
            for relation in &fixture.fixture.relations {
                let source = entity_ids
                    .get(relation.source_slot as usize)
                    .copied()
                    .ok_or_else(|| {
                        RelationalHarnessError("fixture relation source is missing".to_string())
                    })?;
                let target = entity_ids
                    .get(relation.target_slot as usize)
                    .copied()
                    .ok_or_else(|| {
                        RelationalHarnessError("fixture relation target is missing".to_string())
                    })?;
                relation_batch
                    .intents
                    .push(TransactionIntent::CreateRelation(
                        crate::data::transaction::RelationSpec {
                            partition_id: PartitionId::main(),
                            kind_id: relation.kind_id,
                            client_key: InternedString::Raw(relation.client_key.clone()),
                            source,
                            target,
                            payload: Some(RecordPayload::StructuredJson(relation.payload.clone())),
                        },
                    ));
            }
            relation_txn.push_batch(relation_batch);
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
            match operation {
                RelationalMutation::Batch(worker_batch) => txn.push_batch(worker_batch.clone()),
            }
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
        let snapshot = runtime.snapshot();
        let read_view = runtime
            .read_snapshot(&snapshot)
            .ok_or_else(|| RelationalHarnessError("snapshot unavailable".to_string()))?;
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
                    serde_json::to_value(runtime.latest_patch()).unwrap_or_else(|_| json!(null)),
                ),
                (
                    "relational_replay".to_string(),
                    serde_json::to_value(runtime.latest_replay()).unwrap_or_else(|_| json!(null)),
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
        let mut clone = runtime.clone();
        let snapshot = clone.snapshot();
        let read_view = clone
            .read_snapshot(&snapshot)
            .ok_or_else(|| RelationalHarnessError("snapshot unavailable".to_string()))?;
        let observations = resolve_targets(request)
            .into_iter()
            .map(|target| {
                let payload = match parse_target(&target)? {
                    ReadTarget::Entity(entity_id) => {
                        read_view.get_entity(entity_id).map(|entity| {
                            SnapshotPayload::Structured(StructuredValue::Json(json!(entity)))
                        })
                    }
                    ReadTarget::Relation(relation_id) => {
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
            summary: serde_json::to_value(runtime.diagnostics()).unwrap_or_else(|_| json!({})),
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

fn resolve_targets(request: &ExecutionRequest<String>) -> Vec<String> {
    if request.targets.is_empty() {
        Vec::new()
    } else {
        request.targets.clone()
    }
}

fn parse_target(target: &str) -> Result<ReadTarget, RelationalHarnessError> {
    let mut parts = target.split(':');
    let kind = parts
        .next()
        .ok_or_else(|| RelationalHarnessError("missing target kind".to_string()))?;
    let remainder = parts.collect::<Vec<_>>();
    let (partition_id, slot, generation) = match remainder.as_slice() {
        [slot, generation] => (
            PartitionId::main(),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        [partition, slot, generation] => (
            PartitionId(
                partition
                    .parse::<u32>()
                    .map_err(|_| RelationalHarnessError("invalid target partition".to_string()))?,
            ),
            slot.parse::<u64>()
                .map_err(|_| RelationalHarnessError("invalid target slot".to_string()))?,
            generation
                .parse::<u32>()
                .map_err(|_| RelationalHarnessError("invalid target generation".to_string()))?,
        ),
        _ => {
            return Err(RelationalHarnessError(
                "target must be kind:slot:generation or kind:partition:slot:generation"
                    .to_string(),
            ))
        }
    };
    match kind {
        "entity" => Ok(ReadTarget::Entity(EntityId::new(
            partition_id,
            slot,
            generation,
        ))),
        "relation" => Ok(ReadTarget::Relation(RelationId::new(
            partition_id,
            slot,
            generation,
        ))),
        _ => Err(RelationalHarnessError("unknown target kind".to_string())),
    }
}

fn commit_error_to_harness_error(
    error: crate::data::transaction::TransactionCommitError,
) -> RelationalHarnessError {
    match error {
        crate::data::transaction::TransactionCommitError::Conflict(conflict) => {
            RelationalHarnessError(conflict.detail)
        }
        crate::data::transaction::TransactionCommitError::Publication(publication) => {
            RelationalHarnessError(publication.detail)
        }
    }
}

fn default_harness_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "fixture.entity".to_string(),
            schema_id: SchemaId("fixture".to_string()),
            schema_version_id: SchemaVersionId(1),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "fixture.relation".to_string(),
                schema_id: SchemaId("fixture".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            })
        })
        .expect("valid default harness schema registry")
}
