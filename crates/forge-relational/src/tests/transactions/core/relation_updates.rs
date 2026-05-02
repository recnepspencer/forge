use crate::tests::support::*;
use crate::transactions::data::ConflictClass;
use crate::{
    facade::transactions::{CreatedEntityRef, EntityReference, EntitySpec},
    payloads::data::RecordPayload,
};

#[test]
fn relation_endpoint_update_preserves_relation_identity_and_rewrites_endpoints() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let original_target = create_entity(&mut runtime, "original-target");
    let new_target = create_entity(&mut runtime, "new-target");
    let relation = create_relation(&mut runtime, source, original_target, "edge");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewire-relation").push(MutationIntent::Relation(
            RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                relation_id: relation,
                kind_id: KindId(2),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(new_target),
            }),
        )),
    );
    let outcome = txn.commit().expect("relation update should commit");
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .expect("updated snapshot should read");
    let updated = read
        .relations()
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("updated relation should remain visible");

    assert_eq!(updated.relation_id, relation);
    assert_eq!(updated.source, source);
    assert_eq!(updated.target, new_target);
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(source, outcome.version_id)
        .contains(&relation));
    assert!(!runtime
        .storage_access()
        .all_relations_for_entity(original_target, outcome.version_id)
        .contains(&relation));
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(new_target, outcome.version_id)
        .contains(&relation));
}

#[test]
fn relation_endpoint_update_rejects_duplicate_relation_identity() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let left = create_entity(&mut runtime, "left");
    let right = create_entity(&mut runtime, "right");
    let first = create_relation(&mut runtime, source, left, "edge-left");
    let _second = create_relation(&mut runtime, source, right, "edge-right");

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("duplicate-rewire").push(MutationIntent::Relation(
            RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                relation_id: first,
                kind_id: KindId(2),
                source: EntityReference::Existing(source),
                target: EntityReference::Existing(right),
            }),
        )),
    );
    let error = txn
        .commit()
        .expect_err("duplicate relation identity should deny");

    match error {
        TransactionCommitError::Conflict { error, .. } => {
            assert!(matches!(
                error.class,
                ConflictClass::DuplicateRelationIdentity { .. }
            ));
        }
        other => panic!("expected duplicate relation identity conflict, got {other:?}"),
    }
}

#[test]
fn relation_endpoint_update_accepts_same_batch_created_target() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let old_target = create_entity(&mut runtime, "old-target");
    let relation = create_relation(&mut runtime, source, old_target, "edge");
    let created_target = CreatedEntityRef {
        partition_id: PartitionId(1),
        kind_id: KindId(1),
        client_key: InternedString::Raw("same-batch-target".to_string()),
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewire-to-created")
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: created_target.partition_id,
                kind_id: created_target.kind_id,
                client_key: created_target.client_key.clone(),
                payload: RecordPayload::OpaqueBytes(Vec::new()),
            })))
            .push(MutationIntent::Relation(
                RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                    relation_id: relation,
                    kind_id: KindId(2),
                    source: EntityReference::Existing(source),
                    target: EntityReference::Created(created_target.clone()),
                }),
            )),
    );
    let outcome = txn
        .commit()
        .expect("relation update to same-batch created target should commit");
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .expect("updated snapshot should read");
    let updated = read
        .relations()
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("updated relation should remain visible");
    let created_target_id = updated.target;

    assert_eq!(updated.relation_id, relation);
    assert_eq!(updated.source, source);
    assert_eq!(updated.target, created_target_id);
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(created_target_id, outcome.version_id)
        .contains(&relation));
}

#[test]
fn relation_endpoint_update_to_same_batch_created_target_survives_old_target_retirement() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "source");
    let old_target = create_entity(&mut runtime, "old-target");
    let relation = create_relation(&mut runtime, source, old_target, "edge");
    let created_target = CreatedEntityRef {
        partition_id: PartitionId(1),
        kind_id: KindId(1),
        client_key: InternedString::Raw("rewired-target".to_string()),
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewire-and-retire-old-target")
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: created_target.partition_id,
                kind_id: created_target.kind_id,
                client_key: created_target.client_key.clone(),
                payload: RecordPayload::OpaqueBytes(Vec::new()),
            })))
            .push(MutationIntent::Relation(
                RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                    relation_id: relation,
                    kind_id: KindId(2),
                    source: EntityReference::Existing(source),
                    target: EntityReference::Created(created_target),
                }),
            ))
            .push(MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent {
                    entity_id: old_target,
                },
            ))),
    );
    let outcome = txn
        .commit()
        .expect("moved relation should survive retirement of its old endpoint");
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .expect("updated snapshot should read");
    let updated = read
        .relations()
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("moved relation should remain visible after old target retirement");
    let created_target_id = updated.target;

    assert_eq!(updated.source, source);
    assert_ne!(updated.target, old_target);
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(source, outcome.version_id)
        .contains(&relation));
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(created_target_id, outcome.version_id)
        .contains(&relation));
    assert!(!runtime
        .storage_access()
        .all_relations_for_entity(old_target, outcome.version_id)
        .contains(&relation));
    assert!(read
        .entities()
        .iter()
        .all(|record| record.entity_id != old_target));
}

#[test]
fn relation_endpoint_update_to_same_batch_created_source_survives_old_source_retirement() {
    let mut runtime = runtime_with_test_schema();
    let old_source = create_entity(&mut runtime, "old-source");
    let target = create_entity(&mut runtime, "target");
    let relation = create_relation(&mut runtime, old_source, target, "edge");
    let created_source = CreatedEntityRef {
        partition_id: PartitionId(1),
        kind_id: KindId(1),
        client_key: InternedString::Raw("rewired-source".to_string()),
    };

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("rewire-and-retire-old-source")
            .push(MutationIntent::Create(CreateIntent::Entity(EntitySpec {
                partition_id: created_source.partition_id,
                kind_id: created_source.kind_id,
                client_key: created_source.client_key.clone(),
                payload: RecordPayload::OpaqueBytes(Vec::new()),
            })))
            .push(MutationIntent::Relation(
                RelationMutationIntent::UpdateEndpoints(UpdateRelationEndpointsIntent {
                    relation_id: relation,
                    kind_id: KindId(2),
                    source: EntityReference::Created(created_source),
                    target: EntityReference::Existing(target),
                }),
            ))
            .push(MutationIntent::Entity(EntityMutationIntent::Delete(
                DeleteEntityIntent {
                    entity_id: old_source,
                },
            ))),
    );
    let outcome = txn
        .commit()
        .expect("moved relation should survive retirement of its old source");
    let read = runtime
        .read_truth()
        .read_snapshot(&outcome.snapshot)
        .expect("updated snapshot should read");
    let updated = read
        .relations()
        .iter()
        .find(|record| record.relation_id == relation)
        .expect("moved relation should remain visible after old source retirement");
    let created_source_id = updated.source;

    assert_eq!(updated.target, target);
    assert_ne!(updated.source, old_source);
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(created_source_id, outcome.version_id)
        .contains(&relation));
    assert!(runtime
        .storage_access()
        .all_relations_for_entity(target, outcome.version_id)
        .contains(&relation));
    assert!(!runtime
        .storage_access()
        .all_relations_for_entity(old_source, outcome.version_id)
        .contains(&relation));
    assert!(read
        .entities()
        .iter()
        .all(|record| record.entity_id != old_source));
}
