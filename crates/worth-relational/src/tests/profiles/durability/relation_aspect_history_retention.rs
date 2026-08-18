use crate::facade::config::{MvccConfig, RetentionBackend, SnapshotReleasePolicy};
use crate::tests::support::*;

#[test]
fn relation_aspect_history_remains_available_for_historical_reads_after_reclaim() {
    let mut runtime = RelationalRuntimeApi::builder()
        .schema_registry(test_schema_registry())
        .mvcc(MvccConfig {
            track_visibility_metadata: true,
            snapshot_release_policy: SnapshotReleasePolicy::ExplicitRelease,
            auto_reclaim_deleted_records: true,
            reclaim_batch_size: 32,
            retention_backend: RetentionBackend::PinTrackedRetention,
        })
        .build();
    let source_outcome = create_entity_outcome(&mut runtime, "source");
    let target_outcome = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source_outcome)[0];
    let target = changed_entities(&target_outcome)[0];
    let relation = create_aspect_bearing_relation(&mut runtime, source, target);

    let deleted = delete_relation(&mut runtime, relation);
    runtime
        .visibility_authority()
        .release_snapshot(&source_outcome.snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&target_outcome.snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&deleted.created_snapshot);
    runtime
        .visibility_authority()
        .release_snapshot(&deleted.deleted_snapshot);
    let _ = runtime.retention().run_pass();

    assert_eq!(runtime.relation_history_len_for_test(relation), 1);
    let historical = runtime.read_truth().read_version(deleted.created_version);
    let relation_record = historical
        .relations
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("retained relation aspect record");
    assert_eq!(
        read_relation_field(relation_record, field_key("label")),
        Some("r1".into())
    );
}

struct DeletedAspectRelationEvidence {
    created_version: crate::facade::identity::VersionId,
    created_snapshot: crate::snapshots::data::SnapshotHandle,
    deleted_snapshot: crate::snapshots::data::SnapshotHandle,
}

fn create_aspect_bearing_relation(
    mut runtime: &mut RelationalRuntime,
    source: crate::facade::identity::EntityId,
    target: crate::facade::identity::EntityId,
) -> crate::facade::identity::RelationId {
    let mut txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    txn.push_batch(
        WorkerIntentBatch::new("aspect-bearing-relation").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: crate::symbols::data::ClientKey::raw("r1"),
                source: crate::transactions::data::EntityReference::Existing(source),
                target: crate::transactions::data::EntityReference::Existing(target),
                fields: relation_label_field_patch("r1"),
            }),
        )),
    );
    let created = txn.commit().unwrap();
    let relation = changed_relations(&created)[0];
    assert_eq!(runtime.relation_history_len_for_test(relation), 1);
    relation
}

fn delete_relation(
    mut runtime: &mut RelationalRuntime,
    relation: crate::facade::identity::RelationId,
) -> DeletedAspectRelationEvidence {
    let created_version = runtime.history().latest_commit().unwrap().version_id;
    let created_snapshot = runtime.visibility_authority().snapshot();
    let mut delete_txn = crate::tests::support::test_owner_begin_transaction_for_main(&mut runtime);
    delete_txn.push_batch(WorkerIntentBatch::new("delete-relation").push(
        MutationIntent::Relation(RelationMutationIntent::Delete(DeleteRelationIntent {
            relation_id: relation,
        })),
    ));
    let deleted = delete_txn.commit().unwrap();
    DeletedAspectRelationEvidence {
        created_version,
        created_snapshot,
        deleted_snapshot: deleted.snapshot.clone(),
    }
}
