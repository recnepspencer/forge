use worth_query_host::facade::primary_graph;

use super::courtroom_support::{assert_authoritative_value, observe};
use super::schema::{IntentEffectField, IntentLifecycleField};
use super::world::CourtroomWorld;

pub fn temporal_wake_repairs_durable_settlement_before_exact_retirement() {
    let mut world = CourtroomWorld::publish("ready");
    let integration = world
        .application
        .granular_invalidation_installation()
        .retain_primary_graph_integration_handle();
    let commits_before =
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count());
    integration
        .execute_mutation_with_index_refresh(|runtime| {
            runtime.fail_next_durable_append_for_test();
            Ok::<(), ()>(())
        })
        .expect("fault installation changes no primary index")
        .expect("real durable-append fault must install");

    let deferred = observe(&mut world);
    assert_eq!(deferred.committed_operation_count(), 0);
    assert_eq!(deferred.already_committed_operation_count(), 0);
    assert_eq!(deferred.retained_due_wake_count(), 1);
    assert_eq!(deferred.retained_deferred_wake_count(), 1);
    let [provenance] = deferred.execution_provenance() else {
        panic!("one deferred operation must expose one typed lineage")
    };
    assert_eq!(
        provenance.terminal(),
        primary_graph::WorthQueryConditionalExecutionTerminal::DeferredRetained
    );
    assert_eq!(
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count()),
        commits_before + 1,
        "the performed effect must commit before settlement is deferred"
    );

    let repaired = observe(&mut world);
    assert_eq!(repaired.retained_due_wake_count(), 0);
    assert_eq!(repaired.committed_operation_count(), 0);
    assert_eq!(repaired.already_committed_operation_count(), 1);
    assert_eq!(
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count()),
        commits_before + 1,
        "repair and exact idempotency resolution must not duplicate the effect"
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "payload".to_string(),
    );
    assert_authoritative_value(
        &world,
        IntentLifecycleField::reference(),
        "completed".to_string(),
    );
}

pub fn temporal_wake_retries_query_publication_after_settlement_repair() {
    let mut world = CourtroomWorld::publish("ready");
    let integration = world
        .application
        .granular_invalidation_installation()
        .retain_primary_graph_integration_handle();
    let truth_before = world
        .application
        .primary_truth_snapshot_for_test()
        .expect("published application must bind its initial Bridge truth head");
    let commits_before =
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count());
    integration
        .execute_mutation_with_index_refresh(|runtime| {
            runtime.fail_next_durable_append_for_test();
            Ok::<(), ()>(())
        })
        .expect("fault installation changes no primary index")
        .expect("real durable-append fault must install");
    world.application.fail_next_index_publication_for_test();

    let deferred = observe(&mut world);
    assert_eq!(deferred.committed_operation_count(), 0);
    assert_eq!(deferred.retained_deferred_wake_count(), 1);
    assert_eq!(world.contacts.snapshot(), (1, 1, 1, 1));
    assert_eq!(
        world.application.primary_truth_snapshot_for_test(),
        Some(truth_before.clone()),
        "the failed Query publication must leave the Bridge head unchanged"
    );
    assert_eq!(
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count()),
        commits_before + 1,
        "the combined boundary faults still perform one authoritative effect"
    );

    let repaired = observe(&mut world);
    assert_eq!(
        world.contacts.snapshot(),
        (1, 1, 1, 1),
        "idempotent publication retry must not rerun application behavior"
    );
    assert_eq!(
        repaired.retained_due_wake_count(),
        0,
        "retry provenance: {:?}",
        repaired.execution_provenance()
    );
    assert_eq!(repaired.committed_operation_count(), 0);
    assert_eq!(repaired.already_committed_operation_count(), 1);
    let [provenance] = repaired.execution_provenance() else {
        panic!("one publication retry must expose one typed lineage")
    };
    assert!(provenance.application_attempt_ordinal().is_some());
    assert_eq!(
        provenance.canonical_work().admission().digest_derivations(),
        2
    );
    let truth_after = world
        .application
        .primary_truth_snapshot_for_test()
        .expect("publication recovery must bind the performed Bridge truth head");
    assert_ne!(truth_after, truth_before);
    assert_eq!(
        integration.with_runtime(|runtime| runtime.history().immutable_commit_count()),
        commits_before + 1,
        "the idempotent Query publication retry must not duplicate the effect"
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "payload".to_string(),
    );
    assert_authoritative_value(
        &world,
        IntentLifecycleField::reference(),
        "completed".to_string(),
    );
    let settled = observe(&mut world);
    assert_eq!(settled.retained_due_wake_count(), 0);
    assert_eq!(
        world.application.primary_truth_snapshot_for_test(),
        Some(truth_after),
        "completed recovery must not publish a second Bridge truth head"
    );
}
