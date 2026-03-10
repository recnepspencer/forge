use crate::tests::support::*;

// CONTRACT: historical_lineage_resolution
// LANES: success, adversarial, recovery

#[test]
fn historical_lineage_resolution_follows_replace_events() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime.lineage_for_record(entity).unwrap().lineage_id;

    let mut txn = runtime.begin_transaction(TransactionOptions::default());
    txn.push_batch(
        WorkerIntentBatch::new("replace").push(TransactionIntent::ReplaceEntity {
            entity_id: entity,
            replacement: crate::transactions::data::EntitySpec {
                partition_id: PartitionId::main(),
                kind_id: KindId(1),
                client_key: InternedString::Raw("replacement".to_string()),
                payload: RecordPayload::StructuredJson(json!({"name":"replacement"})),
            },
        }),
    );
    let outcome = txn.commit().unwrap();
    let resolution =
        runtime.resolve_historical_lineage(&BranchId("main".to_string()), start_lineage);

    assert_eq!(resolution.start, start_lineage);
    assert_eq!(resolution.traversed_event_ids.len(), 1);
    assert_eq!(resolution.resolved.len(), 1);
    assert_ne!(resolution.resolved[0], start_lineage);
    assert_eq!(
        runtime
            .lineage_graph(&BranchId("main".to_string()))
            .events
            .iter()
            .filter(|event| event.commit.commit_id == outcome.commit.commit_id)
            .count(),
        2
    );
}

#[test]
fn historical_lineage_resolution_is_branch_local_under_divergent_replacements() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "source");
    let main_target = create_entity_outcome(&mut runtime, "main-target");
    let feature_target = create_entity_outcome(&mut runtime, "feature-target");
    let entity = changed_entities(&created)[0];
    let start_lineage = runtime.lineage_for_record(entity).unwrap().lineage_id;
    let main_target_lineage = runtime
        .lineage_for_record(changed_entities(&main_target)[0])
        .unwrap()
        .lineage_id;
    let feature_target_lineage = runtime
        .lineage_for_record(changed_entities(&feature_target)[0])
        .unwrap()
        .lineage_id;
    runtime
        .create_branch(
            BranchId("feature".to_string()),
            &BranchId("main".to_string()),
        )
        .unwrap();

    let main_candidate = runtime.record_correspondence_candidate(
        BranchId("main".to_string()),
        vec![start_lineage],
        vec![main_target_lineage],
        "main-branch-resolution",
    );
    runtime
        .promote_correspondence(main_candidate.candidate_id, main_target.commit.clone())
        .unwrap();
    let feature_candidate = runtime.record_correspondence_candidate(
        BranchId("feature".to_string()),
        vec![start_lineage],
        vec![feature_target_lineage],
        "feature-branch-resolution",
    );
    runtime
        .promote_correspondence(
            feature_candidate.candidate_id,
            feature_target.commit.clone(),
        )
        .unwrap();

    let main_resolution =
        runtime.resolve_historical_lineage(&BranchId("main".to_string()), start_lineage);
    let feature_resolution =
        runtime.resolve_historical_lineage(&BranchId("feature".to_string()), start_lineage);

    assert_ne!(main_resolution.resolved, feature_resolution.resolved);
}
