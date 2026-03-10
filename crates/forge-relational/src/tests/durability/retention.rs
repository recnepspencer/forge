use crate::tests::support::*;

// CONTRACT: retention_plan
// LANES: success, adversarial, recovery

#[test]
fn retention_plan_reports_snapshot_pinned_records_before_release() {
    let mut runtime = runtime_with_test_schema();
    let entity_created = create_entity_outcome(&mut runtime, "entity-pinned");
    let entity = changed_entities(&entity_created)[0];
    let deleted_entity = delete_entity(&mut runtime, entity);

    let relation_source = create_entity(&mut runtime, "relation-left");
    let relation_target = create_entity(&mut runtime, "relation-right");
    let relation_created =
        create_relation_outcome(&mut runtime, relation_source, relation_target, "r1");
    let relation = changed_relations(&relation_created)[0];
    let deleted_relation = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("delete-relation")
                .push(TransactionIntent::DeleteRelation { relation_id: relation }),
        );
        txn.commit().unwrap()
    };

    let plan = runtime.inspect_retention_plan();

    assert!(plan.active_snapshot_count >= 4);
    assert!(plan.snapshot_pinned_entities >= 1);
    assert!(plan.snapshot_pinned_relations >= 1);
    assert_eq!(plan.reclaimable_entities, 0);
    assert_eq!(plan.reclaimable_relations, 0);

    assert!(runtime.release_snapshot(&entity_created.snapshot));
    assert!(runtime.release_snapshot(&deleted_entity.snapshot));
    assert!(runtime.release_snapshot(&relation_created.snapshot));
    assert!(runtime.release_snapshot(&deleted_relation.snapshot));
}

#[test]
fn retention_plan_turns_deleted_records_reclaimable_after_snapshot_release() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "reclaimable");
    let entity = changed_entities(&created)[0];
    let deleted = delete_entity(&mut runtime, entity);

    assert!(runtime.release_snapshot(&created.snapshot));
    assert!(runtime.release_snapshot(&deleted.snapshot));

    let plan = runtime.inspect_retention_plan();
    let pass = runtime.run_retention_pass();

    assert_eq!(plan.active_snapshot_count, 0);
    assert_eq!(plan.snapshot_pinned_entities, 0);
    assert!(plan.reclaimable_entities >= 1);
    assert!(pass.entity_reclaimable >= 1);
}

#[test]
fn retention_plan_reports_branch_pinned_deleted_records_when_sibling_branch_lags() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity_outcome(&mut runtime, "source");
    let source_entity = changed_entities(&source)[0];
    let target = create_entity_outcome(&mut runtime, "target");
    let target_entity = changed_entities(&target)[0];
    let relation_created = create_relation_outcome(&mut runtime, source_entity, target_entity, "r1");
    runtime
        .create_branch(BranchId("feature".to_string()), &BranchId("main".to_string()))
        .unwrap();
    let deleted = delete_entity(&mut runtime, source_entity);

    assert!(runtime.release_snapshot(&source.snapshot));
    assert!(runtime.release_snapshot(&target.snapshot));
    assert!(runtime.release_snapshot(&relation_created.snapshot));
    assert!(runtime.release_snapshot(&deleted.snapshot));

    let plan = runtime.inspect_retention_plan();

    assert_eq!(plan.snapshot_pinned_entities, 0);
    assert_eq!(plan.snapshot_pinned_relations, 0);
    assert!(plan.branch_pinned_entities >= 1);
    assert!(plan.branch_pinned_relations >= 1);
    assert_eq!(plan.reclaimable_entities, 0);
    assert_eq!(plan.reclaimable_relations, 0);
}

#[test]
fn retention_plan_reports_explicit_replay_pins_until_released() {
    let mut runtime = runtime_with_test_schema();
    let created = create_entity_outcome(&mut runtime, "replay-pinned");
    let entity = changed_entities(&created)[0];
    let deleted = delete_entity(&mut runtime, entity);

    assert!(runtime.release_snapshot(&created.snapshot));
    assert!(runtime.release_snapshot(&deleted.snapshot));
    assert!(runtime.retain_version_for_replay(created.version_id));

    let pinned = runtime.inspect_retention_plan();
    assert!(pinned.replay_pinned_entities >= 1);
    assert_eq!(pinned.reclaimable_entities, 0);

    assert!(runtime.release_version_replay_retention(created.version_id));
    let released = runtime.inspect_retention_plan();
    assert_eq!(released.replay_pinned_entities, 0);
    assert!(released.reclaimable_entities >= 1);
}

#[test]
fn retention_plan_reports_explicit_replay_pins_for_deleted_relations_until_released() {
    let mut runtime = runtime_with_test_schema();
    let source = create_entity(&mut runtime, "replay-left");
    let target = create_entity(&mut runtime, "replay-right");
    let created = create_relation_outcome(&mut runtime, source, target, "replay-r1");
    let relation = changed_relations(&created)[0];
    let deleted = {
        let mut txn = runtime.begin_transaction(TransactionOptions::default());
        txn.push_batch(
            WorkerIntentBatch::new("delete-relation")
                .push(TransactionIntent::DeleteRelation { relation_id: relation }),
        );
        txn.commit().unwrap()
    };

    assert!(runtime.release_snapshot(&created.snapshot));
    assert!(runtime.release_snapshot(&deleted.snapshot));
    assert!(runtime.retain_version_for_replay(created.version_id));

    let pinned = runtime.inspect_retention_plan();
    assert!(pinned.replay_pinned_relations >= 1);
    assert_eq!(pinned.reclaimable_relations, 0);

    assert!(runtime.release_version_replay_retention(created.version_id));
    let released = runtime.inspect_retention_plan();
    assert_eq!(released.replay_pinned_relations, 0);
    assert!(released.reclaimable_relations >= 1);
}
