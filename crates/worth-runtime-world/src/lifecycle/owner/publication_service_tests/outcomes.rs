use super::*;

#[test]
fn close_denies_reserved_attempt_and_drop_releases_all_attempt_capacity() {
    let (fixture, owner, expected) = setup();
    let ready = ready_relational(&fixture, &owner, &expected, "publication-service");
    assert_eq!(owner.state.operation.active(), 1);
    assert_eq!(owner.state.publication_capacity.active(), 1);
    assert_eq!(owner.state.history.reserved_len(), 1);
    assert_eq!(owner.state.recovery.reserved_slots(), 1);
    assert_eq!(owner.recovery_record_count(), 0);
    assert_eq!(
        owner
            .close()
            .expect_err("a live reserved attempt denies close"),
        RuntimeWorldCloseDenial::AlreadyClosing
    );
    assert_eq!(
        owner.lifecycle_observation(),
        crate::lifecycle::RuntimeWorldOwnerLifecycleObservation::Open
    );

    drop(ready);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(owner.state.publication_capacity.active(), 0);
    assert_eq!(owner.state.history.reserved_len(), 0);
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    assert_eq!(owner.recovery_record_count(), 0);
    let _report = owner
        .close()
        .expect("close succeeds after attempt teardown");
    assert_eq!(
        owner.lifecycle_observation(),
        crate::lifecycle::RuntimeWorldOwnerLifecycleObservation::Closed
    );
}

#[test]
fn service_dispatch_records_one_exact_final_publication() {
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&fixture, &owner, &expected, "publication-service");
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
    );
    let performed = match outcome {
        RuntimeWorldPublicationOutcome::Performed(performed) => performed,
        other => panic!("service publication must perform: {other:?}"),
    };
    let counters = performed.cost_counters();
    assert_eq!(counters.expected_head_rechecks(), 1);
    assert_eq!(counters.history_slots_installed(), 1);
    assert_eq!(counters.product_cell_touches(), 1);
    assert_eq!(counters.cas_attempts(), 1);
    assert_eq!(counters.cas_wins(), 1);
    assert_eq!(counters.cas_losses(), 0);
    assert_eq!(counters.cancellation_observations(), 0);
    assert_eq!(performed.old_product_head().snapshot(), expected.snapshot());
    assert_eq!(performed.new_product_head(), &cell.atomic_snapshot());
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn cancellation_after_owner_movement_retains_partial_until_explicit_cleanup() {
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&fixture, &owner, &expected, "publication-service");
    let before = cell.atomic_snapshot();
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement,
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("cancelled owner effects must be retained: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    assert_eq!(cell.atomic_snapshot(), before);
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.state.operation.active(), 0);
    assert!(owner.cleanup_recovery(retained));
    assert_eq!(owner.recovery_record_count(), 0);
    let _report = owner
        .close()
        .expect("close succeeds after recovery custody drops");
}

#[test]
fn cancellation_before_product_movement_does_not_cas_current_head() {
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&fixture, &owner, &expected, "publication-service");
    let before = cell.atomic_snapshot();
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement,
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("current-head cancellation must retain owner effects: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    assert_eq!(cell.atomic_snapshot(), before);
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    assert_eq!(owner.recovery_record_count(), 1);
    let handle = retained.recovery_handle();
    drop(retained);
    assert_eq!(
        owner
            .close()
            .expect_err("installed recovery custody denies close"),
        RuntimeWorldCloseDenial::InFlightCriticalSection,
        "installed recovery custody must be cleaned before close"
    );
    assert!(owner.cleanup_recovery_handle(&handle));
    let _report = owner
        .close()
        .expect("close succeeds after cancellation custody drops");
}

#[test]
fn cancellation_observed_after_movement_is_performed_with_evidence() {
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&fixture, &owner, &expected, "publication-service");
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::RequestedAfterProductMovement,
    );
    let performed = match outcome {
        RuntimeWorldPublicationOutcome::Performed(performed) => performed,
        other => panic!("post-movement cancellation must preserve publication: {other:?}"),
    };
    assert_eq!(
        performed.late_cancellation(),
        CompositeLateCancellationPosture::RequestedAfterProductMovement
    );
    assert_eq!(performed.cost_counters().cancellation_observations(), 1);
}
