use std::sync::{mpsc, Arc};
use std::time::Duration;

use crate::lifecycle::{
    RuntimeWorldCloseDenial, RuntimeWorldOwnerLifecycleObservation, RuntimeWorldRecoveryService,
};
use crate::publication::{
    CompositeAttemptProgress, RelationalAttemptProgress, SignalAttemptProgress,
};
use crate::recovery::ProductUnpublishedNextAction;

use super::settlement_catalog_tests::{relational_plan, setup, successor_basis};

const RECOVERY_CLOSE_TEST_TIMEOUT: Duration = Duration::from_secs(2);

#[test]
fn installed_and_updating_recovery_block_close_until_cleanup() {
    let (fixture, owner, expected) = setup();
    let plan = relational_plan(&fixture, &owner, expected.clone());
    let cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let mut attempt = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        plan,
        &cancellation.token(),
        None,
    )
    .expect("the owner reserves the recovery concurrency attempt");
    attempt.begin_owner_execution();

    let performed = fixture.perform_relational_owner_change();
    let successor_basis = successor_basis(&owner, &expected, performed.next_basis().clone(), None);
    let retained = attempt
        .settle(CompositeAttemptProgress::new(
            RelationalAttemptProgress::performed(performed),
            SignalAttemptProgress::untouched(),
        ))
        .ready(successor_basis)
        .expect_err("unsettled Relational work enters retained recovery");
    let handle = retained.recovery_handle();
    drop(retained);

    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.recovery_handles(), vec![handle.clone()]);
    assert!(owner.inspect_recovery(&handle).is_some());
    assert_eq!(
        owner.close(),
        Err(RuntimeWorldCloseDenial::RecoveryInProgress),
        "installed recovery custody denies close before update begins"
    );
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Open
    );

    let (reached_tx, reached_rx) = mpsc::sync_channel(1);
    let pause = super::install_test_recovery_update_pause(reached_tx);
    let effects = owner
        .inspect_recovery(&handle)
        .expect("installed recovery supplies the continuation capability");
    let worker_owner = Arc::clone(&owner);
    let (finished_tx, finished_rx) = mpsc::sync_channel(1);
    let worker = std::thread::spawn(move || {
        let outcome = RuntimeWorldRecoveryService::continue_effects(worker_owner.as_ref(), effects);
        finished_tx
            .send(outcome)
            .expect("the recovery proof still owns its completion receiver");
    });

    reached_rx
        .recv_timeout(RECOVERY_CLOSE_TEST_TIMEOUT)
        .expect("recovery reaches the catalog-update boundary");
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.recovery_handles(), vec![handle.clone()]);
    assert!(
        owner.inspect_recovery(&handle).is_none(),
        "the updating record is absent from the installed map but remains reported by identity"
    );
    assert_eq!(owner.state.operation.active(), 1);
    assert_eq!(
        owner
            .state
            .operation
            .state
            .lock()
            .unwrap_or_else(|error| error.into_inner())
            .recovery_active,
        1
    );
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Open
    );
    assert_eq!(
        owner.close(),
        Err(RuntimeWorldCloseDenial::RecoveryInProgress)
    );
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Open,
        "close denial must not enter Closing while recovery updates"
    );

    drop(pause);
    let continuation = finished_rx
        .recv_timeout(RECOVERY_CLOSE_TEST_TIMEOUT)
        .expect("recovery settlement finishes after the update boundary is released")
        .expect("the real Relational settlement authority completes recovery");
    worker.join().expect("recovery worker does not panic");
    assert!(!continuation
        .actions()
        .contains(&ProductUnpublishedNextAction::SettleOwnerEffects));
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(owner.recovery_record_count(), 1);
    let inspected = owner
        .inspect_recovery(&handle)
        .expect("settled recovery remains installed until cleanup");
    assert_eq!(
        inspected.progress().relational_posture(),
        crate::publication::RelationalAttemptProgressPosture::Settled
    );
    drop(inspected);

    assert!(owner.cleanup_recovery_handle(&handle));
    assert_eq!(owner.recovery_record_count(), 0);
    owner
        .close()
        .expect("cleanup removes the final recovery close obligation");
    assert_eq!(
        owner.lifecycle_observation(),
        RuntimeWorldOwnerLifecycleObservation::Closed
    );
}
