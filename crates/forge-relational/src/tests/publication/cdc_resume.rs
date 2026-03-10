use crate::tests::support::*;

// CONTRACT: patch_stream
// LANES: success, failure, adversarial, determinism

#[test]
fn patch_stream_resume_batches_commits_without_duplication() {
    let mut runtime = runtime_with_test_schema();
    let _first = create_entity_outcome(&mut runtime, "a");
    let _second = create_entity_outcome(&mut runtime, "b");
    let _third = create_entity_outcome(&mut runtime, "c");

    let first_batch = runtime
        .read_patch_stream(PatchStreamRequest {
            after_position: None,
            max_commits: 2,
        })
        .unwrap();
    let resumed = runtime
        .read_patch_stream(PatchStreamRequest {
            after_position: first_batch.next_position,
            max_commits: 2,
        })
        .unwrap();

    assert_eq!(first_batch.patches.len(), 2);
    assert_eq!(first_batch.next_position, Some(PatchStreamPosition(2)));
    assert_eq!(first_batch.latest_position, Some(PatchStreamPosition(3)));
    assert_eq!(resumed.patches.len(), 1);
    assert_eq!(resumed.resumed_after, Some(PatchStreamPosition(2)));
    assert_eq!(resumed.patches[0].position, PatchStreamPosition(3));
}

#[test]
fn patch_stream_rejects_unknown_resume_position() {
    let mut runtime = runtime_with_test_schema();
    let _ = create_entity_outcome(&mut runtime, "anchor");

    let error = runtime
        .read_patch_stream(PatchStreamRequest {
            after_position: Some(PatchStreamPosition(99)),
            max_commits: 1,
        })
        .unwrap_err();

    assert_eq!(
        error.class,
        crate::facade::PatchStreamReadErrorClass::UnknownResumePosition
    );
}

#[test]
fn patch_stream_records_aspects_for_entity_and_relation_payloads() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let target = create_entity_outcome(&mut runtime, "target");
    let source = changed_entities(&source)[0];
    let target = changed_entities(&target)[0];

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("mixed-aspects")
            .push(TransactionIntent::UpdateEntity {
                entity_id: source,
                payload: RecordPayload::StructuredJson(
                    json!({"name":"source-2","status":"hot","risk":"elevated"}),
                ),
            })
            .push(TransactionIntent::CreateRelation(
                crate::transactions::data::RelationSpec {
                    partition_id: PartitionId::main(),
                    kind_id: KindId(2),
                    client_key: InternedString::Raw("weighted".to_string()),
                    source,
                    target,
                    payload: Some(RecordPayload::StructuredJson(
                        json!({"label":"weighted","weight":7}),
                    )),
                },
            )),
    );
    let outcome = txn.commit().unwrap();
    let relation = changed_relations(&outcome)[0];
    let latest_patch = runtime.latest_patch().unwrap();
    let entity_patch = latest_patch
        .records
        .iter()
        .find(|record| record.entity_id == Some(source))
        .unwrap();
    let relation_patch = latest_patch
        .records
        .iter()
        .find(|record| record.relation_id == Some(relation))
        .unwrap();

    assert_eq!(entity_patch.aspects.len(), 3);
    assert_eq!(relation_patch.aspects.len(), 2);
    assert_eq!(
        runtime
            .entity_aspects_at_version(source, outcome.version_id)
            .unwrap()
            .len(),
        3
    );
    assert_eq!(
        runtime
            .relation_aspects_at_version(relation, outcome.version_id)
            .unwrap()
            .len(),
        2
    );
    assert!(runtime.entity_aspect_versions(source).unwrap().len() >= 3);
    assert!(runtime.relation_aspect_versions(relation).unwrap().len() >= 2);
}
