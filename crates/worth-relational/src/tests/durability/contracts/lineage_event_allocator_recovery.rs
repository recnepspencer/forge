use super::*;

#[test]
fn tail_lineage_event_allocator_exhaustion_denies_before_replay() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "checkpoint-before-tail-overflow");
    runtime
        .durability_authority()
        .checkpoint()
        .expect("checkpoint");
    let replay_invalid = create_entity_outcome(&mut runtime, "tail-before-overflow");
    let allocator_exhausted = create_entity_outcome(&mut runtime, "tail-event-overflow");
    let mut plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let invalid_envelope = plan
        .tail_log
        .iter_mut()
        .find(|envelope| envelope.commit.commit_id == replay_invalid.commit.commit_id)
        .expect("earlier tail envelope");
    let checkpoint = invalid_envelope
        .branch_cell_checkpoint
        .as_mut()
        .expect("tail envelope branch-cell checkpoint");
    checkpoint.truth_version = crate::branch::RelationalBranchVersion::new(
        checkpoint.truth_version.as_u64().saturating_add(1),
    );
    let event = plan
        .tail_log
        .iter_mut()
        .find(|envelope| envelope.commit.commit_id == allocator_exhausted.commit.commit_id)
        .expect("later tail envelope")
        .published_lineage_mut_for_test()
        .lineage_events_mut()
        .first_mut()
        .expect("later tail lineage event");
    event.event_id = u64::MAX - 1;
    let exhausted_envelope = plan
        .tail_log
        .iter_mut()
        .find(|envelope| envelope.commit.commit_id == allocator_exhausted.commit.commit_id)
        .expect("later tail envelope");
    exhausted_envelope.rebuild_lineage_basis_for_test();

    let mut recovered = persisted_runtime_with_test_schema();
    let error = recovered
        .durability_authority()
        .recover(plan)
        .expect_err("tail allocator exhaustion must deny");

    assert_eq!(error.class, RecoveryFailureClass::CorruptSegment);
    assert!(error.detail.contains("tail lineage event id exhausted"));
    assert_eq!(recovered.history().immutable_commit_count(), 0);
}

#[test]
fn failed_durable_append_lineage_id_gap_recovers_exact_tail_and_continues() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "lineage-gap-checkpoint");
    runtime.durability_authority().checkpoint().unwrap();
    let abandoned_lineage_id = runtime.lineage.next_lineage_id;
    let abandoned_event_id = runtime.lineage.next_event_id;
    let published_node_count = runtime.lineage.nodes.len();
    let published_event_count = runtime.lineage.events().count();
    let published_head = runtime
        .history()
        .branch_head(&BranchId("main".to_owned()))
        .expect("checkpoint head")
        .clone();

    runtime.durability.fail_next_append = true;
    let mut failed = test_owner_begin_transaction_for_main(&mut runtime);
    failed.push_batch(batch_create("lineage-gap-abandoned"));
    let error = failed
        .commit(&mut runtime)
        .expect_err("injected durable append failure must abandon the reserved lineage id");
    assert!(format!("{error:?}").contains("test-injected durable append failure"));
    assert_eq!(runtime.lineage.next_lineage_id, abandoned_lineage_id + 1);
    assert_eq!(runtime.lineage.next_event_id, abandoned_event_id + 1);
    assert_eq!(runtime.lineage.nodes.len(), published_node_count);
    assert_eq!(runtime.lineage.events().count(), published_event_count);
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_owned()))
            .expect("failed append cannot move branch head"),
        &published_head
    );

    let successful = create_entity_outcome(&mut runtime, "lineage-gap-tail");
    let successful_entity = changed_entities(&successful)[0];
    let successful_lineage_id = runtime
        .lineage_access()
        .for_record(successful_entity)
        .expect("successful tail lineage")
        .lineage_id();
    let successful_event_id = runtime
        .history()
        .commit_envelope(successful.commit.commit_id)
        .expect("successful tail envelope")
        .lineage_events()[0]
        .event_id();
    assert!(successful_lineage_id.0 > abandoned_lineage_id);
    assert!(successful_event_id > abandoned_event_id);

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(plan)
        .expect("canonical tail create must recover across an abandoned lineage-id gap");
    assert_eq!(
        recovered
            .lineage_access()
            .for_record(successful_entity)
            .expect("recovered tail lineage")
            .lineage_id(),
        successful_lineage_id
    );
    assert_eq!(
        recovered
            .history()
            .commit_envelope(successful.commit.commit_id)
            .expect("recovered successful envelope")
            .lineage_events()[0]
            .event_id(),
        successful_event_id
    );

    let post_recovery = create_entity_outcome(&mut recovered, "lineage-gap-post-recovery");
    let post_recovery_lineage_id = recovered
        .lineage_access()
        .for_record(changed_entities(&post_recovery)[0])
        .expect("post-recovery lineage")
        .lineage_id();
    assert_eq!(post_recovery_lineage_id.0, successful_lineage_id.0 + 1);
    assert_eq!(
        recovered
            .history()
            .commit_envelope(post_recovery.commit.commit_id)
            .expect("post-recovery envelope")
            .lineage_events()[0]
            .event_id(),
        successful_event_id + 1
    );
}

#[test]
fn multi_event_reservation_exhaustion_denies_before_public_effects() {
    let mut runtime = persisted_runtime_with_test_schema();
    let baseline = create_entity_outcome(&mut runtime, "reservation-exhaustion-baseline");
    let baseline_head = baseline.commit.clone();
    let baseline_entities = runtime.storage_access().storage_stats().live_entities;
    let lineage_frontier = runtime.lineage.next_lineage_id;
    runtime.lineage.next_event_id = u64::MAX - 1;

    let mut transaction = test_owner_begin_transaction_for_main(&mut runtime);
    transaction.push_batch(batch_create("reservation-exhaustion-first"));
    transaction.push_batch(batch_create("reservation-exhaustion-second"));
    let error = transaction
        .commit(&mut runtime)
        .expect_err("two-event reservation must deny at the allocator boundary");

    assert!(format!("{error:?}").contains("lineage event id allocator exhausted"));
    assert_eq!(runtime.lineage.next_event_id, u64::MAX - 1);
    assert_eq!(runtime.lineage.next_lineage_id, lineage_frontier);
    assert_eq!(
        runtime
            .history()
            .branch_head(&BranchId("main".to_owned()))
            .expect("baseline head"),
        &baseline_head
    );
    assert_eq!(
        runtime.storage_access().storage_stats().live_entities,
        baseline_entities
    );
}

#[test]
fn branch_local_sparse_slot_create_publishes_and_recovers_exact_lineage() {
    let mut runtime = persisted_runtime_with_test_schema();
    create_entity_outcome(&mut runtime, "sparse-slot-shared-root");
    let feature = create_branch_from_main(&mut runtime, "sparse-slot-feature");
    create_entity_outcome(&mut runtime, "sparse-slot-main-divergence");
    let feature_create = create_entity_outcome_on_branch(
        &mut runtime,
        "sparse-slot-feature-create",
        feature.clone(),
    );
    let feature_entity = changed_entities(&feature_create)[0];
    assert!(feature_entity.local_slot.0 > 1);
    let lineage_id = runtime
        .lineage
        .nodes
        .values()
        .find(|node| node.entity_id() == feature_entity)
        .expect("sparse logical slot has lineage")
        .lineage_id();
    let event_targets = runtime
        .history()
        .commit_envelope(feature_create.commit.commit_id)
        .expect("feature create envelope")
        .lineage_events()[0]
        .targets()
        .to_vec();
    assert_eq!(event_targets, vec![lineage_id]);

    let plan = runtime.durability().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
    let mut recovered = persisted_runtime_with_test_schema();
    recovered
        .durability_authority()
        .recover(plan)
        .expect("sparse branch-local lineage recovers");
    assert_eq!(
        recovered
            .lineage
            .nodes
            .values()
            .find(|node| node.entity_id() == feature_entity)
            .expect("recovered sparse lineage")
            .lineage_id(),
        lineage_id
    );
}
