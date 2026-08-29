use crate::tests::support::*;

#[test]
fn direct_publication_settles_before_a_child_can_acknowledge() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "settlement-anchor");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint seals the baseline before the direct parent");

    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction
        .push_batch(batch_create("direct-parent"))
        .expect("test staging stays within configured resource budgets");
    let candidate = runtime
        .prepare_branch_transaction(transaction)
        .expect("direct parent prepares");
    let crate::mvcc::RelationalPublicationOutcome::Performed(performed) =
        runtime.publication_port().compare_and_publish(candidate)
    else {
        panic!("direct parent must perform");
    };
    let parent_id = performed.canonical_commit().commit.commit_id;
    let parent_position = performed.patch_position();

    assert!(runtime
        .history()
        .recent_commit_ids(None, 8)
        .contains(&parent_id));
    assert_eq!(
        runtime
            .publication()
            .read_patch_stream(PatchStreamRequest::default())
            .expect("direct parent is stream-visible")
            .latest_commit_id,
        Some(parent_id)
    );
    let subscriber = runtime
        .publication()
        .read_subscriber_stream(SubscriberResumeRequest::from_head(8))
        .expect("CDC sees the direct parent through canonical stream inventory");
    assert_eq!(subscriber.latest_commit_id, Some(parent_id));
    assert!(subscriber
        .patches
        .iter()
        .any(|patch| patch.position == parent_position));

    let fork_denial = runtime
        .fork()
        .expect_err("runtime fork is denied while direct publication is unsettled");
    assert_eq!(
        fork_denial,
        crate::runtime::RelationalRuntimeForkDenial::PerformedPublicationRequiresSettlement {
            commit_id: parent_id,
        }
    );
    let mut blocked_child = test_owner_begin_transaction_for_main(&mut runtime);
    blocked_child
        .push_batch(batch_create("blocked-before-settlement"))
        .expect("test staging stays within configured resource budgets");
    let child_denial = runtime
        .prepare_branch_transaction(blocked_child)
        .expect_err("a child cannot hide its parent's unfinished settlement");
    assert!(format!("{child_denial:?}").contains("requires explicit owner settlement"));
    let settled_parent = runtime
        .settle_performed_publication(performed)
        .expect("the direct owner explicitly settles its performed publication");
    release_test_commit_snapshot(&mut runtime, &settled_parent);

    let child = create_entity_outcome(&mut runtime, "ordinary-child");
    assert_eq!(child.commit.parents, vec![parent_id]);
    let durability = runtime.durability();
    let durable = durability.durable_log();
    let tail = &durable[durable.len().saturating_sub(2)..];
    assert_eq!(tail.len(), 2, "checkpoint tail contains parent then child");
    assert_eq!(tail[0].envelope().commit.commit_id, parent_id);
    assert_eq!(tail[0].position(), parent_position);
    assert_eq!(tail[1].envelope().commit.commit_id, child.commit.commit_id);
    assert!(tail[0].position() < tail[1].position());
    let child_position = tail[1].position();
    release_test_commit_snapshot(&mut runtime, &child);

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(plan)
        .expect("fresh runtime recovers the baseline checkpoint plus exact tail");
    assert_eq!(
        recovered
            .history()
            .immutable_commit_receipt(parent_id)
            .expect("settled direct parent recovers")
            .commit_id,
        parent_id
    );
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_owned()))
            .expect("ordinary child remains the recovered head")
            .commit_id,
        child.commit.commit_id
    );
    let continued = create_entity_outcome(&mut recovered, "post-recovery-child");
    let continued_position = recovered
        .history
        .canonical_stream_position(continued.commit.commit_id)
        .expect("post-recovery child receives a stream position");
    assert!(child_position < continued_position);
    release_test_commit_snapshot(&mut recovered, &continued);
}

#[test]
fn failed_durable_append_returns_an_idempotent_owner_repair_capability() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "repair-anchor");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("repair baseline checkpoint");

    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction
        .push_batch(batch_create("performed-before-append-fault"))
        .expect("test staging stays within configured resource budgets");
    let candidate = runtime
        .prepare_branch_transaction(transaction)
        .expect("fault candidate prepares");
    let crate::mvcc::RelationalPublicationOutcome::Performed(performed) =
        runtime.publication_port().compare_and_publish(candidate)
    else {
        panic!("fault candidate performs");
    };
    let commit_id = performed.canonical_commit().commit.commit_id;
    let position = performed.patch_position();
    runtime.durability.arm_append_failure();
    let error = runtime
        .settle_performed_publication(performed)
        .expect_err("injected append fault defers settlement after performance");
    let settlement = error
        .deferred_settlement()
        .expect("the owner receives exact settlement repair authority");
    assert_eq!(settlement.commit().commit_id, commit_id);
    assert_eq!(settlement.patch_position(), position);
    assert_eq!(
        runtime
            .durability_authority()
            .checkpoint()
            .expect_err("unsettled performed publication blocks checkpoint")
            .class,
        crate::durability::data::RecoveryFailureClass::PerformedPublicationRequiresSettlement
    );

    let repaired = runtime
        .repair_deferred_publication_settlement(settlement)
        .expect("owner retries the exact missing durable append");
    assert_eq!(repaired.commit_id, commit_id);
    let repaired_again = runtime
        .repair_deferred_publication_settlement(settlement)
        .expect("repeating an already successful repair is harmless");
    assert_eq!(repaired_again, repaired);
    assert_eq!(
        runtime
            .durability()
            .durable_log()
            .iter()
            .filter(|entry| entry.envelope().commit.commit_id == commit_id)
            .count(),
        1
    );
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint succeeds after repair");
    let child = create_entity_outcome(&mut runtime, "child-after-repair");
    assert_eq!(child.commit.parents, vec![commit_id]);

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(plan)
        .expect("fresh runtime recovers repaired parent and child");
    assert_eq!(
        recovered
            .history()
            .branch_head(&BranchId("main".to_owned()))
            .expect("repaired lineage remains current")
            .commit_id,
        child.commit.commit_id
    );
    release_test_commit_snapshot(&mut runtime, &child);
}

#[test]
fn runtime_recovers_settlement_after_external_capability_is_dropped() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity(&mut runtime, "runtime-recovery-anchor");
    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction
        .push_batch(batch_create("runtime-owned-deferred-settlement"))
        .unwrap();
    let candidate = runtime.prepare_branch_transaction(transaction).unwrap();
    let crate::mvcc::RelationalPublicationOutcome::Performed(performed) =
        runtime.publication_port().compare_and_publish(candidate)
    else {
        panic!("runtime recovery candidate performs before its injected append fault");
    };
    let commit_id = performed.canonical_commit().commit.commit_id;
    runtime.durability.arm_append_failure();
    let error = runtime
        .settle_performed_publication(performed)
        .expect_err("durable append fault returns deferred settlement");
    assert_eq!(runtime.publication_binding().pending_settlement_count(), 1);
    assert_eq!(runtime.visibility.published_snapshot_handle_count(), 1);
    drop(error);
    assert_eq!(runtime.publication_binding().pending_settlement_count(), 1);
    assert_eq!(runtime.visibility.published_snapshot_handle_count(), 1);

    let mut blocked = test_owner_begin_transaction_for_main(&mut runtime);
    blocked
        .push_batch(batch_create("blocked-unsettled-child"))
        .unwrap();
    assert!(runtime
        .prepare_branch_transaction(blocked)
        .unwrap_err()
        .detail()
        .contains("requires explicit owner settlement"));

    let repaired = runtime
        .repair_pending_publication_settlement(commit_id)
        .expect("runtime-owned recovery survives loss of the external capability");
    assert_eq!(repaired.commit_id, commit_id);
    assert_eq!(runtime.publication_binding().pending_settlement_count(), 0);
    assert_eq!(runtime.visibility.published_snapshot_handle_count(), 0);
    let child = create_entity_outcome(&mut runtime, "child-after-runtime-recovery");
    assert_eq!(child.commit.parents, vec![commit_id]);
    release_test_commit_snapshot(&mut runtime, &child);
}
