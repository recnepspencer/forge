use crate::tests::support::*;

#[test]
fn savepoint_abandoned_work_never_appears_in_subscriber_cdc() {
    let mut runtime = runtime_with_test_schema();
    let anchor = create_entity_outcome(&mut runtime, "anchor");
    let anchor_entity = changed_entities(&anchor)[0];
    let checkpoint = checkpoint_for_schema_version(anchor.patch_position(), SchemaVersionId(1));

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(batch_create("surviving"));
    let savepoint = txn.create_savepoint();
    txn.push_batch(batch_create("abandoned"));
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"abandoned-anchor"})),
            }),
        )),
    );
    let rollback = txn.rollback_to_savepoint(savepoint).unwrap();
    txn.push_batch(
        WorkerIntentBatch::new("survived-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor_entity,
                payload: RecordPayload::StructuredJson(json!({"name":"survived-anchor"})),
            }),
        )),
    );
    let outcome = txn.commit().unwrap();

    assert!(rollback.summary().has_discarded_entity_creation());

    let subscriber = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 8))
        .unwrap();

    assert!(subscriber
        .patches
        .iter()
        .flat_map(|patch| patch.records.iter())
        .all(|record| !patch_detail_contains(record, "abandoned")));

    let read = runtime
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let names = read
        .entities()
        .iter()
        .filter_map(|record| read_entity_name(record))
        .collect::<Vec<_>>();

    assert!(names.contains(&"surviving"));
    assert!(names.contains(&"survived-anchor"));
    assert!(!names.contains(&"abandoned"));
    assert!(!names.contains(&"abandoned-anchor"));
}

#[test]
fn nested_savepoint_abandoned_aspect_work_leaves_zero_patch_cdc_history_and_lineage_residue() {
    let mut runtime =
        runtime_with_declared_aspect_schema(CascadeDeletePolicy::CascadeDeleteRelations);
    let created = create_entity_outcome(&mut runtime, "anchor");
    let anchor = changed_entities(&created)[0];
    let target = create_entity(&mut runtime, "target");
    let start_lineage = runtime
        .lineage_access()
        .for_record(anchor)
        .unwrap()
        .lineage_id;
    let checkpoint = checkpoint_for_schema_version(
        runtime
            .publication_access()
            .latest_patch()
            .unwrap()
            .position,
        SchemaVersionId(1),
    );

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    let savepoint_a = txn.create_savepoint();
    txn.push_batch(batch_create("surviving-a"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-a-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-a-anchor"})),
            }),
        )),
    );

    let savepoint_b = txn.create_savepoint();
    txn.push_batch(batch_create("abandoned-entity"));
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-relation").push(MutationIntent::Create(
            CreateIntent::Relation(crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("abandoned-r".to_string()),
                source: anchor,
                target,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"abandoned-label"}),
                )),
            }),
        )),
    );
    txn.push_batch(
        WorkerIntentBatch::new("abandoned-replace").push(MutationIntent::Entity(
            EntityMutationIntent::Replace(ReplaceEntityIntent {
                entity_id: anchor,
                replacement: crate::transactions::data::EntitySpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(1),
                    client_key: InternedString::Raw("abandoned-replacement".to_string()),
                    payload: RecordPayload::StructuredJson(json!({"name":"abandoned-replacement"})),
                },
            }),
        )),
    );
    let rollback_b = txn.rollback_to_savepoint(savepoint_b).unwrap();

    txn.push_batch(batch_create("surviving-b"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-b-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-b-anchor"})),
            }),
        )),
    );
    let rollback_a = txn.rollback_to_savepoint(savepoint_a).unwrap();

    txn.push_batch(batch_create("surviving-final"));
    txn.push_batch(
        WorkerIntentBatch::new("surviving-final-update").push(MutationIntent::Entity(
            EntityMutationIntent::Update(UpdateEntityIntent {
                entity_id: anchor,
                payload: RecordPayload::StructuredJson(json!({"name":"surviving-final-anchor"})),
            }),
        )),
    );
    txn.push_batch(WorkerIntentBatch::new("surviving-final-relation").push(
        MutationIntent::Create(CreateIntent::Relation(
            crate::transactions::data::RelationSpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(2),
                client_key: InternedString::Raw("surviving-r".to_string()),
                source: anchor,
                target,
                payload: Some(RecordPayload::StructuredJson(
                    json!({"label":"surviving-label"}),
                )),
            },
        )),
    ));
    let outcome = txn.commit().unwrap();

    assert!(rollback_b.has_effects());
    assert!(rollback_a.has_effects());
    assert_patch_truth_invariants(&outcome);
    assert!(outcome
        .patch()
        .iter()
        .all(|record| !patch_detail_contains(record, "abandoned")));

    let subscriber = runtime
        .publication_access()
        .read_subscriber_stream(SubscriberResumeRequest::resume_after(checkpoint, 8))
        .unwrap();
    assert!(subscriber
        .patches
        .iter()
        .flat_map(|patch| patch.records.iter())
        .all(|record| !patch_detail_contains(record, "abandoned")));

    let direct_history =
        runtime
            .history_access()
            .entity_aspect_history(&BranchId("main".to_string()), anchor, None);
    let direct_traced = runtime.history_access().entity_aspect_history_with_trace(
        &BranchId("main".to_string()),
        anchor,
        None,
    );
    assert_eq!(direct_history.len(), 2);
    assert_eq!(direct_traced.aspect_history_digest().entry_count, 2);
    assert_direct_history_origin_invariants(&direct_history, RecordRef::Entity(anchor));

    let lineage_traced = runtime.lineage_access().entity_aspect_history_with_trace(
        &BranchId("main".to_string()),
        start_lineage,
        None,
    );
    let lineage_history = lineage_traced
        .history
        .as_ref()
        .expect("lineage aspect history");
    assert_eq!(lineage_history.traversed_event_ids.len(), 0);
    assert_eq!(lineage_history.entries.len(), 2);
    assert_lineage_history_origin_invariants(&lineage_history.entries, start_lineage);
    assert_eq!(
        lineage_traced
            .lineage_aspect_resolution_digest()
            .traversed_lineage_events,
        0
    );

    let read = runtime
        .visibility_reads()
        .read_snapshot(&outcome.snapshot)
        .unwrap();
    let entity_names = read
        .entities()
        .iter()
        .filter_map(|record| read_entity_name(record))
        .collect::<Vec<_>>();

    assert!(entity_names.contains(&"target"));
    assert!(entity_names.contains(&"surviving-final"));
    assert!(entity_names.contains(&"surviving-final-anchor"));
    assert!(!entity_names.iter().any(|name| name.contains("abandoned")));
    assert_eq!(read.relations().len(), 1);
}

fn patch_detail_contains(record: &crate::facade::publication::PatchRecord, needle: &str) -> bool {
    match &record.detail {
        PatchDetail::StructuredJson(value) => value.to_string().contains(needle),
        PatchDetail::Payload(payload) => payload
            .as_json()
            .map(|value| value.to_string().contains(needle))
            .unwrap_or(false),
        PatchDetail::DenseBitset(_) => false,
    }
}
