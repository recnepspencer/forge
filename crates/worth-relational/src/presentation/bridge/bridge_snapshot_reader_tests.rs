use std::sync::Arc;

use worth_foundational::facade::{AspectKey, AspectValue, ScalarAspectType};
use worth_runtime_bridge::facade::{
    RelationalBridgeRecordIdentityParts, SnapshotReadContract, SnapshotReadSource,
};

use crate::config::data::CascadeDeletePolicy;
use crate::facade::identity::PartitionId;
use crate::facade::transactions::{
    EntityMutationIntent, MutationIntent, ReplaceEntityIntent, TransactionOptions,
    WorkerIntentBatch,
};
use crate::tests::support::{
    changed_entities, create_entity_outcome, field_key, runtime_with_declared_aspect_schema,
    single_string_aspect_field_patch,
};

use super::{bridge_snapshot_identity_for_handle, RuntimeBridgeRelationalSource};

#[test]
fn runtime_bridge_snapshot_reader_prefers_active_snapshot_binding_over_later_commit_id_collision() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "alice");
    let active_snapshot = runtime.visibility_authority().snapshot();
    let active_snapshot_identity = bridge_snapshot_identity_for_handle(&active_snapshot);
    let active_entity_identity = active_entity_identity(&created);

    replace_entity_after_snapshot(&mut runtime, &created);

    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("test graph role");
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
fn runtime_bridge_snapshot_reader_requires_live_execution_basis_authority() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "managed");
    let version_id = created.version_id;
    let entity_identity = active_entity_identity(&created);
    assert!(runtime.snapshots().release_snapshot(&created.snapshot));
    let source = RuntimeBridgeRelationalSource::for_graph_role(Arc::new(runtime), "model")
        .expect("test graph role");
    let lease = source
        .admit_execution_basis(version_id)
        .expect("Relational source should admit its reconstructible version");
    let identity = bridge_snapshot_identity_for_handle(lease.snapshot_handle());

    let reader = source
        .open_snapshot(&identity)
        .expect("live execution basis should authorize Bridge snapshot access");
    let packet = worth_runtime_bridge::facade::SnapshotReadPacket::new(vec![
        worth_runtime_bridge::facade::SnapshotReadRequest::for_relational_record(
            entity_identity,
            SnapshotReadContract::scalar(aspect_key("name"), ScalarAspectType::String),
        ),
    ]);
    assert_eq!(
        reader
            .read_packet(&packet)
            .expect("execution basis packet should read")
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
    runtime: &mut crate::facade::runtime::RelationalRuntime,
    created: &crate::facade::transactions::CommitResult,
) {
    let mut txn = runtime.begin_transaction(TransactionOptions::default());
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
    );
    txn.commit().expect("second commit should publish");
}

fn runtime_with_test_schema() -> crate::facade::runtime::RelationalRuntime {
    runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations)
}

fn aspect_key(value: &str) -> AspectKey {
    AspectKey::new(value).expect("valid test aspect key")
}
