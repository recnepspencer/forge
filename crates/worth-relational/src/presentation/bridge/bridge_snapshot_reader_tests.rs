use std::sync::{Arc, Mutex};

use worth_foundational::facade::{AspectKey, AspectValue, ScalarAspectType};
use worth_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, SnapshotReadContract, SnapshotReadSource,
};

use crate::config::data::CascadeDeletePolicy;
use crate::facade::identity::PartitionId;
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, WorkerIntentBatch,
};
use crate::tests::support::{
    changed_entities, create_entity_outcome, field_key, runtime_with_declared_aspect_schema,
    single_string_aspect_field_patch,
};

use super::RuntimeBridgeRelationalSource;

#[test]
fn runtime_bridge_snapshot_reader_prefers_retained_observation_over_later_commit_id_collision() {
    let runtime = Arc::new(Mutex::new(runtime_with_test_schema()));
    let source =
        RuntimeBridgeRelationalSource::for_shared_graph_role(Arc::clone(&runtime), "model")
            .expect("test graph role");
    let created = {
        let mut runtime = runtime.lock().expect("test runtime lock");
        create_entity_outcome(&mut runtime, "alice")
    };
    let active_entity_identity = active_entity_identity(&created);
    let branch_identity = runtime
        .lock()
        .expect("test runtime lock")
        .branch_identity(&created.commit.branch_id)
        .expect("committed branch identity");
    let (_, basis) = source
        .observe_branch_basis(&branch_identity)
        .expect("owner-admitted active basis");
    let lease = source
        .retain_branch_basis_for_bridge(&basis)
        .expect("retained active observation");
    let active_snapshot_identity = lease.snapshot_identity().clone();

    replace_entity_after_snapshot(&mut runtime.lock().expect("test runtime lock"), &created);

    let reader = source
        .open_snapshot(&active_snapshot_identity)
        .expect("active snapshot should remain bridge-readable after later commit id collision");
    let packet = worth_runtime_bridge::facade::SnapshotReadPacket::new(vec![
        worth_runtime_bridge::facade::SnapshotReadRequest::for_relational_record(
            active_entity_identity,
            SnapshotReadContract::scalar(aspect_key("name"), ScalarAspectType::String),
        ),
    ]);
    let result = reader
        .read_packet(&packet)
        .expect("bridge snapshot packet should read from active binding");

    assert_eq!(result.snapshot_identity(), &active_snapshot_identity);
    assert_eq!(result.records().len(), 1);
    assert_eq!(
        result.records()[0].scalar_aspect_value(),
        Some(&AspectValue::String("alice".into()))
    );
}

#[test]
fn runtime_bridge_snapshot_reader_requires_a_retained_branch_observation() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "managed");
    let branch_id = created.snapshot.branch_id.clone();
    let branch_identity = runtime
        .branch_identity(&branch_id)
        .expect("created branch identity is owner-issued");
    let entity_identity = active_entity_identity(&created);
    assert!(runtime
        .snapshots()
        .release_snapshot(&created.snapshot)
        .is_ok());
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("test graph role");
    let (_, basis) = source
        .observe_branch_basis(&branch_identity)
        .expect("Relational owner should admit its exact branch basis");
    let lease = source
        .retain_branch_basis_for_bridge(&basis)
        .expect("Bridge should retain the admitted observation");
    let identity = lease.snapshot_identity().clone();

    let reader = source
        .open_snapshot(&identity)
        .expect("retained observation should authorize Bridge snapshot access");
    let packet = worth_runtime_bridge::facade::SnapshotReadPacket::new(vec![
        worth_runtime_bridge::facade::SnapshotReadRequest::for_relational_record(
            entity_identity,
            SnapshotReadContract::scalar(aspect_key("name"), ScalarAspectType::String),
        ),
    ]);
    assert_eq!(
        reader
            .read_packet(&packet)
            .expect("observation packet should read")
            .records()
            .len(),
        1
    );

    assert!(lease.release().released());
    assert!(source.open_snapshot(&identity).is_err());
}

fn active_entity_identity(
    result: &crate::facade::transactions::CommitResult,
) -> RelationalBridgeRecordIdentityParts {
    let entity = changed_entities(result)[0];
    RelationalBridgeRecordIdentityParts::entity(
        entity.partition_id.0,
        entity.local_slot.0,
        entity.generation.0,
    )
}

fn replace_entity_after_snapshot(
    mut runtime: &mut crate::facade::runtime::RelationalRuntime,
    created: &crate::facade::transactions::CommitResult,
) {
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("update").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: changed_entities(created)[0],
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: crate::facade::identity::KindId(1),
                    client_key: crate::symbols::data::ClientKey::raw("alice"),
                    fields: single_string_aspect_field_patch(
                        crate::tests::support::aspect_key("name"),
                        field_key("name"),
                        "alice-updated",
                    ),
                },
            }),
        )),
    )
    .expect("test staging stays within configured resource budgets");
    txn.commit(&mut runtime)
        .expect("second commit should publish");
}

fn runtime_with_test_schema() -> crate::facade::runtime::RelationalRuntime {
    runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations)
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid test aspect key")
}
