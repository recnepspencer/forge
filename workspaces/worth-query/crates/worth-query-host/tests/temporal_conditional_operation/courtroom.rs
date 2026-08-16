use worth_query_host::facade::primary_graph;

use super::courtroom_support::{assert_authoritative_value, observe, raw_observe, wake_evidence};
use super::schema::{IntentEffectField, IntentLifecycleField};
use super::world::CourtroomWorld;

pub fn future_temporal_operation_waits_until_due() {
    let mut world = CourtroomWorld::publish("ready");
    world.clock_control.push(1, 4);
    let future = observe(&mut world);
    assert_eq!(
        future.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&future)
    );
    assert_eq!(world.contacts.snapshot(), (0, 0, 0, 0));

    world.clock_control.push(2, 5);
    let due = observe(&mut world);
    assert_eq!(
        due.committed_operation_count(),
        1,
        "{}",
        wake_evidence(&due)
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "payload".to_string(),
    );
}

pub fn unrelated_rows_do_not_expand_conditional_observation_work() {
    let mut world = CourtroomWorld::publish_with_unrelated_rows("ready", 2_048);
    let before = world.application.inspect_conditional_runtime();
    world.clock_control.push(1, 4);

    let receipt = observe(&mut world);

    assert_eq!(receipt.due_wake_count(), 0);
    assert_eq!(receipt.authoritative_commit_count(), 0);
    assert!(!receipt.authoritative_work_remaining());
    assert_eq!(world.contacts.snapshot(), (0, 0, 0, 0));
    assert_eq!(world.application.inspect_conditional_runtime(), before);
}

pub fn host_installs_and_executes_due_operation() {
    let mut world = CourtroomWorld::publish("ready");
    let receipt = observe(&mut world);
    assert_eq!(
        receipt.committed_operation_count(),
        1,
        "{}",
        wake_evidence(&receipt)
    );
    assert_eq!(receipt.retained_due_wake_count(), 0);
    let [provenance] = receipt.execution_provenance() else {
        panic!("one committed operation must expose one typed lineage")
    };
    assert_eq!(provenance.intent_identity(), "intent-1");
    assert_eq!(provenance.intent_revision(), 1);
    assert_eq!(
        provenance.signal_decision(),
        Some(primary_graph::WorthQueryConditionalSignalDecision::Eligible)
    );
    assert!(provenance.application_attempt_ordinal().is_some());
    assert_eq!(
        provenance.terminal(),
        primary_graph::WorthQueryConditionalExecutionTerminal::Committed
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

pub fn temporal_identity_work_is_cold_or_fresh_admission_only() {
    let mut world = CourtroomWorld::publish("ready");
    let binding = world.clock.binding_canonical_work();
    assert_eq!(binding.basis_preparations(), 1);
    assert_eq!(binding.digest_derivations(), 1);
    assert_eq!(binding.canonical_entries(), 8);
    assert_eq!(binding.digest_text_materializations(), 1);

    let installation = world
        .application
        .inspect_conditional_runtime()
        .installation_canonical_work();
    assert_eq!(installation.basis_preparations(), 2);
    assert_eq!(installation.digest_derivations(), 2);
    assert_eq!(installation.canonical_entries(), 14);
    assert_eq!(installation.digest_text_materializations(), 2);

    let receipt = observe(&mut world);
    let [provenance] = receipt.execution_provenance() else {
        panic!("one committed operation must expose one typed lineage")
    };
    let phases = provenance.canonical_work();
    assert_eq!(phases.admission().basis_preparations(), 2);
    assert_eq!(phases.admission().digest_derivations(), 2);
    assert_eq!(phases.admission().canonical_entries(), 5);
    assert_eq!(phases.admission().digest_text_materializations(), 0);
    for work in [
        phases.installation(),
        phases.execution(),
        phases.provider_commit(),
        phases.projection(),
        phases.live_delivery(),
        phases.retry_resolution(),
        phases.recovery_inspection(),
        phases.publication(),
    ] {
        assert_eq!(work.basis_preparations(), 0);
        assert_eq!(work.digest_derivations(), 0);
        assert_eq!(work.digest_text_materializations(), 0);
    }
}

pub fn cancellation_after_publication_retires_stale_wake() {
    let mut world = CourtroomWorld::publish("ready");
    world.amend_intent(2, "cancelled", "ready");
    let receipt = observe(&mut world);
    assert_eq!(
        receipt.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&receipt)
    );
    assert_eq!(
        receipt.failed_operation_count(),
        0,
        "{}",
        wake_evidence(&receipt)
    );
    assert_eq!(
        receipt.retained_due_wake_count(),
        0,
        "{}",
        wake_evidence(&receipt)
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "pending".to_string(),
    );
    assert_authoritative_value(
        &world,
        IntentLifecycleField::reference(),
        "cancelled".to_string(),
    );
    assert_eq!(world.contacts.snapshot(), (0, 0, 0, 0));
}

pub fn active_successor_revision_replaces_predecessor_wake() {
    let mut world = CourtroomWorld::publish("ready");
    world.supersede_intent(2, 8, "active", "successor-payload", "ready");
    world.clock_control.push(1, 10);
    let receipt = observe(&mut world);
    assert_eq!(
        receipt.committed_operation_count(),
        1,
        "{}",
        wake_evidence(&receipt)
    );
    assert_eq!(
        receipt.retained_due_wake_count(),
        0,
        "{}",
        wake_evidence(&receipt)
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "successor-payload".to_string(),
    );
    assert_authoritative_value(&world, super::schema::IntentRevisionField::reference(), 3);
    assert_eq!(world.contacts.snapshot(), (1, 1, 1, 1));
}

pub fn suppressed_wake_is_reconsidered_after_truth_change() {
    let mut world = CourtroomWorld::publish("blocked");
    let suppressed = observe(&mut world);
    assert_eq!(
        suppressed.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&suppressed)
    );
    assert_eq!(
        suppressed.retained_suppressed_wake_count(),
        1,
        "{}",
        wake_evidence(&suppressed)
    );
    let [provenance] = suppressed.execution_provenance() else {
        panic!("the suppressed wake must retain its typed lineage")
    };
    assert_eq!(
        provenance.signal_decision(),
        Some(primary_graph::WorthQueryConditionalSignalDecision::Suppressed)
    );
    assert_eq!(
        provenance.terminal(),
        primary_graph::WorthQueryConditionalExecutionTerminal::SuppressedRetained
    );
    world.amend_intent(1, "active", "ready");
    let reconsidered = observe(&mut world);
    assert_eq!(
        reconsidered.committed_operation_count(),
        1,
        "{}",
        wake_evidence(&reconsidered)
    );
    assert_eq!(
        reconsidered.retained_due_wake_count(),
        0,
        "{}",
        wake_evidence(&reconsidered)
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "payload".to_string(),
    );
}

pub fn precondition_panic_isolated_and_retry_succeeds() {
    let mut world = CourtroomWorld::publish("ready");
    world.preconditions_panic.set(true);
    let failed = observe(&mut world);
    assert_eq!(
        failed.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&failed)
    );
    assert_eq!(
        failed.failed_operation_count(),
        1,
        "{}",
        wake_evidence(&failed)
    );
    world.preconditions_panic.set(false);
    let retried = observe(&mut world);
    assert_eq!(
        retried.committed_operation_count(),
        1,
        "{}",
        wake_evidence(&retried)
    );
    assert_authoritative_value(
        &world,
        IntentEffectField::reference(),
        "payload".to_string(),
    );
}

pub fn predicate_panic_does_not_corrupt_runtime_owners() {
    let mut world = CourtroomWorld::publish("ready");
    world.predicate_panic.set(true);
    let failed = observe(&mut world);
    assert_eq!(
        failed.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&failed)
    );
    assert_eq!(
        failed.retained_failed_wake_count(),
        1,
        "{}",
        wake_evidence(&failed)
    );
    let [provenance] = failed.execution_provenance() else {
        panic!("the failed wake must retain its typed lineage")
    };
    assert_eq!(provenance.signal_decision(), None);
    assert_eq!(
        provenance.terminal(),
        primary_graph::WorthQueryConditionalExecutionTerminal::Failed
    );
    world.predicate_panic.set(false);
    let next = observe(&mut world);
    assert_eq!(
        next.committed_operation_count(),
        0,
        "{}",
        wake_evidence(&next)
    );
    assert_eq!(
        next.retained_failed_wake_count(),
        1,
        "{}",
        wake_evidence(&next)
    );
}

pub fn duplicate_reordered_and_foreign_clocks_fail_closed() {
    let mut world = CourtroomWorld::publish("ready");
    let _ = observe(&mut world);
    world.clock_control.push(1, 10);
    assert!(matches!(
        raw_observe(&mut world),
        primary_graph::WorthQueryConditionalClockObservationOutcome::Duplicate(_)
    ));
    world.clock_control.push(2, 9);
    assert!(matches!(
        raw_observe(&mut world),
        primary_graph::WorthQueryConditionalClockObservationOutcome::Reordered
    ));
    world.clock_control.push(0, 10);
    assert!(matches!(
        raw_observe(&mut world),
        primary_graph::WorthQueryConditionalClockObservationOutcome::Stale
    ));
    let foreign = CourtroomWorld::publish("ready");
    let denial = world
        .application
        .conditional_clock(&foreign.clock)
        .err()
        .expect("foreign clock handle must fail closed");
    assert_eq!(
        denial.kind(),
        primary_graph::WorthQueryConditionalClockObservationDenialKind::ForeignRuntime
    );
}

pub fn provider_replacement_requires_fresh_runtime_publication() {
    let incumbent = CourtroomWorld::publish("ready");
    let mut replacement = CourtroomWorld::publish_replacement("ready");
    let denial = replacement
        .application
        .conditional_clock(&incumbent.clock)
        .err()
        .expect("replacement runtime must reject incumbent provider clock affinity");
    assert_eq!(
        denial.kind(),
        primary_graph::WorthQueryConditionalClockObservationDenialKind::ForeignRuntime
    );
    drop(incumbent);

    let receipt = observe(&mut replacement);
    assert_eq!(receipt.committed_operation_count(), 1);
    assert_eq!(replacement.contacts.snapshot(), (1, 1, 1, 1));
}
