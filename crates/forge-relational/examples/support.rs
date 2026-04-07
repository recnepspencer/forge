#![allow(dead_code)]

use forge_relational::facade::{
    config::{CascadeDeletePolicy, CrossContextPolicy},
    history::BranchId,
    identity::{EntityId, KindId, PartitionId, RelationId},
    payloads::RecordPayload,
    schema::{
        EntityKindRegistration, KindAspectDeclarations, RelationIntegrityDeclarations,
        RelationKindRegistration, RelationPayloadClass, RelationalSchemaRegistry, SchemaId,
        SchemaVersionId,
    },
    symbols::InternedString,
    transactions::{
        CreateIntent, DeleteEntityIntent, EntityMutationIntent, EntitySpec, MutationIntent,
        RelationSpec, TransactionOptions, UpdateEntityIntent, WorkerIntentBatch,
    },
};
use serde_json::json;

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

fn main() {
    // This example is a shared helper module for other examples.
}

pub fn create_entity(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    name: &str,
) -> (
    forge_relational::facade::transactions::CommitResult,
    EntityId,
) {
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
    let entity_id = changed_entity(&outcome).expect("created entity id");
    (outcome, entity_id)
}

pub fn update_entity(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    entity_id: EntityId,
    name: &str,
) -> forge_relational::facade::transactions::CommitResult {
    update_entity_on_branch(runtime, entity_id, name, None)
}

pub fn update_entity_on_branch(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    entity_id: EntityId,
    name: &str,
    target_branch: Option<BranchId>,
) -> forge_relational::facade::transactions::CommitResult {
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
    tx.commit().expect("update commit")
}

pub fn delete_entity(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    entity_id: EntityId,
) -> forge_relational::facade::transactions::CommitResult {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new("delete-entity").push(MutationIntent::Entity(
            EntityMutationIntent::Delete(DeleteEntityIntent { entity_id }),
        )),
    );
    tx.commit().expect("delete entity commit")
}

pub fn create_relation(
    runtime: &mut forge_relational::facade::runtime::RelationalRuntime,
    source: EntityId,
    target: EntityId,
    label: &str,
) -> (
    forge_relational::facade::transactions::CommitResult,
    RelationId,
) {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(
        WorkerIntentBatch::new(format!("rel-{label}")).push(MutationIntent::Create(
            CreateIntent::Relation(RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw(label.to_string()),
                source,
                target,
                payload: Some(RecordPayload::StructuredJson(json!({ "label": label }))),
            }),
        )),
    );
    let outcome = tx.commit().expect("relation commit");
    let relation_id = changed_relation(&outcome).expect("created relation id");
    (outcome, relation_id)
}

pub fn changed_entity(
    outcome: &forge_relational::facade::transactions::CommitResult,
) -> Option<EntityId> {
    outcome
        .changed_records
        .iter()
        .find_map(|record| match record {
            forge_relational::facade::transactions::RecordRef::Entity(entity_id) => {
                Some(*entity_id)
            }
            forge_relational::facade::transactions::RecordRef::Relation(_) => None,
        })
}

pub fn changed_relation(
    outcome: &forge_relational::facade::transactions::CommitResult,
) -> Option<RelationId> {
    outcome
        .changed_records
        .iter()
        .find_map(|record| match record {
            forge_relational::facade::transactions::RecordRef::Relation(relation_id) => {
                Some(*relation_id)
            }
            forge_relational::facade::transactions::RecordRef::Entity(_) => None,
        })
}
