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

    assert!(
        subscriber
            .patches
            .iter()
            .flat_map(|patch| patch.records.iter())
            .all(|record| !patch_detail_contains(record, "abandoned"))
    );

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

fn patch_detail_contains(
    record: &crate::facade::publication::PatchRecord,
    needle: &str,
) -> bool {
    match &record.detail {
        PatchDetail::StructuredJson(value) => value.to_string().contains(needle),
        PatchDetail::Payload(payload) => payload
            .as_json()
            .map(|value| value.to_string().contains(needle))
            .unwrap_or(false),
        PatchDetail::DenseBitset(_) => false,
    }
}
