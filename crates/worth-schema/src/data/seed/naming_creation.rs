use forge_relational::facade::{
    identity::{EntityId, PartitionId},
    payloads::RecordPayload,
    runtime::{RelationalReadView, RelationalRuntime},
    symbols::InternedString,
    transactions::{
        CommitResult, CreateIntent, EntitySpec, MutationIntent, RelationSpec,
        TransactionCommitError, TransactionOptions, WorkerIntentBatch,
    },
};
use serde_json::json;

use crate::data::entities::{WorthEntityKind, WorthNamingEntityKind};
use crate::data::relations::{WorthNamingRelationKind, WorthRelationKind};
use crate::data::seed::labels::WorthMinimalTopologyLabels;
use crate::data::seed::lookup::find_seeded_entity;
use crate::data::seed::types::WorthMinimalTopologySeed;

pub fn create_persistent_names(
    runtime: &mut RelationalRuntime,
    topology: &WorthMinimalTopologySeed,
    labels: &WorthMinimalTopologyLabels,
) -> Result<CommitResult, TransactionCommitError> {
    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(build_name_entity_batch(labels));
    let name_commit = tx.commit()?;

    let name_read = runtime
        .read_truth()
        .read_snapshot(&name_commit.snapshot)
        .expect("persistent name snapshot should remain readable");
    let name_targets = collect_name_targets(&name_read, topology, labels);

    let mut tx = runtime.begin_transaction(TransactionOptions::default());
    tx.push_batch(build_name_binding_batch(&name_targets));
    tx.commit()
}

pub fn collect_persistent_name_ids(
    read_view: &RelationalReadView,
    labels: &WorthMinimalTopologyLabels,
) -> Vec<EntityId> {
    persistent_name_labels(labels)
        .into_iter()
        .map(|label| {
            find_seeded_entity(
                read_view,
                WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
                label.as_str(),
            )
        })
        .collect()
}

fn build_name_entity_batch(labels: &WorthMinimalTopologyLabels) -> WorkerIntentBatch {
    let mut batch = WorkerIntentBatch::new("worth-seed-persistent-name-entities");
    for label in persistent_name_labels(labels) {
        batch = batch.push(create_persistent_name_entity_intent(&label));
    }
    batch
}

fn build_name_binding_batch(name_targets: &[(EntityId, EntityId, String)]) -> WorkerIntentBatch {
    let mut batch = WorkerIntentBatch::new("worth-seed-persistent-name-bindings");
    for (name_id, target_id, label) in name_targets {
        batch = batch.push(create_persistent_name_binding_intent(
            *name_id,
            *target_id,
            label.as_str(),
        ));
    }
    batch
}

fn collect_name_targets(
    read_view: &RelationalReadView,
    topology: &WorthMinimalTopologySeed,
    labels: &WorthMinimalTopologyLabels,
) -> Vec<(EntityId, EntityId, String)> {
    let topology_targets = [
        (persistent_name_label(&labels.model), topology.model),
        (persistent_name_label(&labels.body), topology.body),
        (persistent_name_label(&labels.lump), topology.lump),
        (persistent_name_label(&labels.region), topology.region),
        (persistent_name_label(&labels.shell), topology.shell),
        (persistent_name_label(&labels.face), topology.face),
        (
            persistent_name_label(&labels.outer_loop),
            topology.outer_loop,
        ),
        (persistent_name_label(&labels.wire), topology.wire),
        (persistent_name_label(&labels.half_edge), topology.half_edge),
        (persistent_name_label(&labels.edge), topology.edge),
        (persistent_name_label(&labels.vertex), topology.vertex),
    ];

    topology_targets
        .into_iter()
        .map(|(name_label, target_id)| {
            let name_id = find_seeded_entity(
                read_view,
                WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName),
                &name_label,
            );
            (name_id, target_id, name_label)
        })
        .collect()
}

fn persistent_name_labels(labels: &WorthMinimalTopologyLabels) -> Vec<String> {
    vec![
        persistent_name_label(&labels.model),
        persistent_name_label(&labels.body),
        persistent_name_label(&labels.lump),
        persistent_name_label(&labels.region),
        persistent_name_label(&labels.shell),
        persistent_name_label(&labels.face),
        persistent_name_label(&labels.outer_loop),
        persistent_name_label(&labels.wire),
        persistent_name_label(&labels.half_edge),
        persistent_name_label(&labels.edge),
        persistent_name_label(&labels.vertex),
    ]
}

fn persistent_name_label(base_label: &str) -> String {
    format!("{base_label}.persistent_name")
}

fn create_persistent_name_entity_intent(label: &str) -> MutationIntent {
    MutationIntent::Create(CreateIntent::Entity(EntitySpec {
        partition_id: PartitionId::main(),
        kind_id: WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id(),
        client_key: InternedString::Raw(label.to_string()),
        payload: RecordPayload::StructuredJson(json!({ "label": label })),
    }))
}

fn create_persistent_name_binding_intent(
    name_id: EntityId,
    target_id: EntityId,
    label: &str,
) -> MutationIntent {
    MutationIntent::Create(CreateIntent::Relation(RelationSpec {
        partition_id: PartitionId::main(),
        kind_id: WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity)
            .kind_id(),
        client_key: InternedString::Raw(label.to_string()),
        source: name_id,
        target: target_id,
        payload: None,
    }))
}
