use forge_relational::facade::{
    config::{CascadeDeletePolicy, CrossContextPolicy},
    history::{BranchId, CommitId},
    identity::{EntityId, KindId, PartitionId},
    payloads::RecordPayload,
    replay::CanonicalCommitEnvelope,
    runtime::{RelationalRuntime, RelationalRuntimeBuilder},
    schema::{
        EntityKindRegistration, KindAspectDeclarations, RelationIntegrityDeclarations,
        RelationKindRegistration, RelationPayloadClass, RelationalSchemaRegistry, SchemaId,
        SchemaVersionId,
    },
    symbols::InternedString,
    transactions::{
        CreateIntent, EntityMutationIntent, EntitySpec, MutationIntent, TransactionOptions,
        UpdateEntityIntent, WorkerIntentBatch,
    },
};
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static TEST_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn runtime_with_demo_schema() -> RelationalRuntime {
    RelationalRuntimeBuilder::new()
        .schema_registry(demo_schema_registry())
        .build()
}

pub fn latest_envelope(runtime: &RelationalRuntime) -> CanonicalCommitEnvelope {
    let commit_id = runtime.history().latest_commit().unwrap().commit_id;
    runtime
        .replay()
        .canonical_commit_envelope(commit_id)
        .unwrap()
        .clone()
}

pub fn demo_schema_registry() -> RelationalSchemaRegistry {
    RelationalSchemaRegistry::new()
        .register_entity_kind(EntityKindRegistration {
            kind_id: KindId(1),
            kind_name: "demo.entity".to_string(),
            schema_id: SchemaId("demo".to_string()),
            schema_version_id: SchemaVersionId(1),
            aspect_declarations: KindAspectDeclarations::default(),
        })
        .and_then(|registry| {
            registry.register_relation_kind(RelationKindRegistration {
                kind_id: KindId(2),
                kind_name: "demo.relation".to_string(),
                schema_id: SchemaId("demo".to_string()),
                schema_version_id: SchemaVersionId(1),
                payload_class: RelationPayloadClass::PayloadBearingRelation,
                cross_context_policy: CrossContextPolicy::AllowExplicit,
                cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
                aspect_declarations: KindAspectDeclarations::default(),
                relation_integrity: RelationIntegrityDeclarations::default(),
            })
        })
        .expect("demo schema registry")
}

pub fn create_entity(runtime: &mut RelationalRuntime, name: &str) -> EntityId {
    create_entity_with_commit(runtime, name).0
}

pub fn create_entity_commit(runtime: &mut RelationalRuntime, name: &str) -> CommitId {
    create_entity_with_commit(runtime, name).1
}

fn create_entity_with_commit(runtime: &mut RelationalRuntime, name: &str) -> (EntityId, CommitId) {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new(format!("create-{name}")).push(MutationIntent::Create(
            CreateIntent::Entity(EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw(name.to_string()),
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    let outcome = tx.commit().expect("entity commit");
    (
        changed_entity(&outcome.changed_records).expect("created entity id"),
        outcome.commit.commit_id,
    )
}

pub fn update_entity_on_branch(
    runtime: &mut RelationalRuntime,
    entity_id: EntityId,
    name: &str,
    target_branch: Option<BranchId>,
) {
    let mut tx = runtime.begin_transaction(TransactionOptions {
        target_branch,
        ..TransactionOptions::default()
    });
    tx.push_batch(
        WorkerIntentBatch::new(format!("update-{name}")).push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id,
                payload: RecordPayload::StructuredJson(json!({ "name": name })),
            }),
        )),
    );
    tx.commit().expect("update commit");
}

fn changed_entity(
    changed_records: &[forge_relational::facade::transactions::RecordRef],
) -> Option<EntityId> {
    changed_records.iter().find_map(|record| match record {
        forge_relational::facade::transactions::RecordRef::Entity(entity_id) => Some(*entity_id),
        forge_relational::facade::transactions::RecordRef::Relation(_) => None,
    })
}

pub fn unique_test_store_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}.json"))
}

pub fn unique_test_sqlite_path(prefix: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let counter = TEST_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("{prefix}-{nanos}-{counter}.sqlite"))
}

pub fn corrupt_first_sqlite_wal_record_digest(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            UPDATE wal_records
            SET record_digest = 'corrupted-wal-digest'
            WHERE wal_sequence = (
                SELECT wal_sequence
                FROM wal_records
                ORDER BY wal_sequence
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite wal digest should be corrupted");
}

pub fn corrupt_first_sqlite_snapshot_image(path: &std::path::Path) {
    let connection = rusqlite::Connection::open(path).expect("sqlite store should open");
    connection
        .execute(
            "
            UPDATE snapshot_image_records
            SET image_payload = '{\"corrupted\":true}'
            WHERE snapshot_id = (
                SELECT snapshot_id
                FROM snapshot_image_records
                ORDER BY snapshot_id
                LIMIT 1
            )
            ",
            [],
        )
        .expect("sqlite snapshot image payload should be corrupted");
}
