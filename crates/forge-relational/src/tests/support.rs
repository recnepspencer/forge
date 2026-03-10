pub(super) use forge_harness::facade::{
    DiagnosticsHarnessAdapter, ExecutionProfile, ExecutionRequest, HarnessAdapter, MutationBatch,
    ReplayHarnessAdapter, ReplayRequest, ScenarioPlan,
};
pub(super) use serde_json::json;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

pub(super) use crate::config::data::{CascadeDeletePolicy, CrossContextPolicy};
pub(super) use crate::config::data::{DurableLogPolicy, DurableLogRetentionMode};
pub(super) use crate::facade::{
    BranchCreateError, BranchId, CommitOutcome, DiagnosticCode, DiagnosticsArtifactKind,
    DiagnosticsScope, DurabilityMode, DurableStoreLayout, EntityKindRegistration, EntityReadRecord,
    InvariantCatalog, InvariantClass, InvariantExecutionPoint, InvariantRule, KindId, PartitionId,
    PatchStreamPosition, PatchStreamRequest, PublicationStage, PublicationStatus, QueryWorkPacket,
    ReadTarget, RelationId, RelationKindRegistration, RelationalHarnessAdapter, RelationalMutation,
    RelationalRuntime, RelationalRuntimeApi, RelationalRuntimeProfile, RelationalSchemaRegistry,
    ReplayMismatchClass, SchemaId, SchemaVersionId, StorageLayoutConfig, TransactionCommitError,
    TransactionIntent, TransactionOptions, VisibilityCachePolicy, WorkerIntentBatch,
};
pub(super) use crate::payloads::data::RecordPayload;
pub(super) use crate::publication::data::diff::{PatchCompatibilityClass, PatchDetail};
pub(super) use crate::schema::data::RelationPayloadClass;
pub(super) use crate::symbols::data::{InternedString, SymbolPolicy};

pub(super) fn test_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "test.entity".to_string(),
            schema_id: SchemaId("test".to_string()),
            schema_version_id: SchemaVersionId(1),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "test.relation".to_string(),
                schema_id: SchemaId("test".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
            })
        })
        .unwrap()
}

pub(super) fn runtime_with_test_schema() -> RelationalRuntime {
    runtime_with_test_schema_profile(RelationalRuntimeProfile::CertificationCore)
}

pub(super) fn runtime_with_test_schema_profile(
    profile: RelationalRuntimeProfile,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(profile)
        .schema_registry(test_schema_registry())
        .build()
}

pub(super) fn persisted_runtime_with_test_schema() -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .durability_mode(DurabilityMode::PersistedSegmentedLocalFs)
        .durable_store_layout(DurableStoreLayout {
            root_path: unique_test_store_path("forge-relational-persisted"),
            segment_commit_capacity: 2,
        })
        .build()
}

pub(super) fn unique_test_store_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("{prefix}-{nanos}"));
    let _ = fs::remove_dir_all(&path);
    path
}

pub(super) fn runtime_with_test_schema_and_chunks(
    entity_chunk_size: usize,
    relation_chunk_size: usize,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .profile(RelationalRuntimeProfile::CertificationCore)
        .schema_registry(test_schema_registry())
        .storage_layout(StorageLayoutConfig {
            entity_chunk_size,
            relation_chunk_size,
            scan_packet_size: 64,
        })
        .build()
}

pub(super) fn runtime_with_test_schema_and_invariants(
    invariant_catalog: InvariantCatalog,
) -> RelationalRuntime {
    RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .invariant_catalog(invariant_catalog)
        .build()
}

pub(super) fn batch_create(name: &str) -> WorkerIntentBatch {
    WorkerIntentBatch::new(format!("batch-{name}")).push(TransactionIntent::CreateEntity(
        crate::transactions::data::EntitySpec {
            partition_id: PartitionId::main(),
            kind_id: KindId(1),
            client_key: InternedString::Raw(name.to_string()),
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        },
    ))
}

pub(super) fn create_entity(
    runtime: &mut RelationalRuntime,
    name: &str,
) -> crate::facade::EntityId {
    changed_entities(&create_entity_outcome(runtime, name))[0]
}

pub(super) fn create_entity_in_partition(
    runtime: &mut RelationalRuntime,
    name: &str,
    partition_id: PartitionId,
) -> crate::facade::EntityId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(WorkerIntentBatch::new(format!("batch-{name}")).push(
        TransactionIntent::CreateEntity(crate::transactions::data::EntitySpec {
            partition_id,
            kind_id: KindId(1),
            client_key: InternedString::Raw(name.to_string()),
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        }),
    ));
    changed_entities(&txn.commit().unwrap())[0]
}

pub(super) fn create_entity_outcome(runtime: &mut RelationalRuntime, name: &str) -> CommitOutcome {
    create_entity_outcome_on_branch(runtime, name, BranchId("main".to_string()))
}

pub(super) fn create_entity_outcome_on_branch(
    runtime: &mut RelationalRuntime,
    name: &str,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(batch_create(name));
    txn.commit().unwrap()
}

pub(super) fn delete_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("delete").push(TransactionIntent::DeleteEntity { entity_id }),
    );
    txn.commit().unwrap()
}

pub(super) fn update_entity(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
    name: &str,
) -> CommitOutcome {
    update_entity_on_branch(runtime, entity_id, name, BranchId("main".to_string()))
}

pub(super) fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: crate::facade::EntityId,
    name: &str,
    branch_id: BranchId,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(branch_id),
        ..TransactionOptions::default()
    });
    txn.push_batch(
        WorkerIntentBatch::new("update").push(TransactionIntent::UpdateEntity {
            entity_id,
            payload: RecordPayload::StructuredJson(json!({ "name": name })),
        }),
    );
    txn.commit().unwrap()
}

pub(super) fn create_relation(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
) -> RelationId {
    create_relation_in_partition(runtime, source, target, client_key, PartitionId::main())
}

pub(super) fn create_relation_in_partition(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
    partition_id: PartitionId,
) -> RelationId {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(TransactionIntent::CreateRelation(
            crate::transactions::data::RelationSpec {
                partition_id,
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        )),
    );
    let outcome = txn.commit().unwrap();
    changed_relations(&outcome)[0]
}

pub(super) fn create_relation_outcome(
    runtime: &mut RelationalRuntime,
    source: crate::facade::EntityId,
    target: crate::facade::EntityId,
    client_key: &str,
) -> CommitOutcome {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("relation").push(TransactionIntent::CreateRelation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw(client_key.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({"label":"rel"}))),
            },
        )),
    );
    txn.commit().unwrap()
}

pub(super) fn changed_entities(outcome: &CommitOutcome) -> Vec<crate::facade::EntityId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Entity(entity_id) => Some(*entity_id),
            crate::facade::RecordRef::Relation(_) => None,
        })
        .collect()
}

pub(super) fn changed_relations(outcome: &CommitOutcome) -> Vec<RelationId> {
    outcome
        .changed_records
        .iter()
        .filter_map(|record| match record {
            crate::facade::RecordRef::Relation(relation_id) => Some(*relation_id),
            crate::facade::RecordRef::Entity(_) => None,
        })
        .collect()
}

pub(super) fn apply_batches(batches: Vec<WorkerIntentBatch>) -> RelationalRuntime {
    let mut runtime = runtime_with_test_schema();
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    for batch in batches {
        txn.push_batch(batch);
    }
    txn.commit().unwrap();
    runtime
}

pub(super) fn merge_commit_from_branches(
    runtime: &mut RelationalRuntime,
    target_branch: BranchId,
    merge_parent_branches: Vec<BranchId>,
) -> CommitOutcome {
    let txn = runtime.begin_transaction(TransactionOptions {
        target_branch: Some(target_branch),
        merge_parent_branches,
        ..TransactionOptions::default()
    });
    txn.commit().unwrap()
}

pub(super) fn read_entity_name(record: &EntityReadRecord) -> Option<&str> {
    record
        .payload
        .as_json()
        .and_then(|value| value.get("name"))
        .and_then(|value| value.as_str())
}
