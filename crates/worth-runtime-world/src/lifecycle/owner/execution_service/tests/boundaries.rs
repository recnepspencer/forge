use super::*;

#[cfg(feature = "test-operation-control")]
fn assert_retained_fork_only(
    retained: &crate::recovery::ProductUnpublishedOwnerEffects,
    cell: &crate::branch::ProductBranchReferenceCell,
) {
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(
        retained.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    assert!(matches!(
        retained.component_results().signal_publication_identity(),
        Some(crate::history::CompositeSignalPublicationIdentity::Forked(
            _
        ))
    ));
    if let Some(observed_head) = retained.last_observed_head() {
        assert_eq!(
            observed_head.selected_commit(),
            cell.atomic_snapshot().selected_commit()
        );
    }
}

#[test]
fn execution_rechecks_deadline_after_reservation_before_first_owner_effect() {
    let clock = MutableClock::new(0);
    let (fixture, owner, expected) =
        setup_with_clock(RuntimeWorldClock::from_source(clock.clone()));
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "deadline-after-reservation",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
        None,
    );
    let attempt = reserve_with_deadline(
        &owner,
        plan,
        Some(crate::lifecycle::RuntimeWorldInstant::from_ticks(5)),
    )
    .expect("the future deadline admits the complete attempt reservation");
    clock.set(5);

    let outcome = execute_without_signal(&owner, attempt);
    assert!(matches!(
        outcome,
        OwnerExecutionOutcome::NoEffect(no_effect)
            if no_effect.cause() == crate::publication::NoEffectCause::DeadlineBeforeEffect
    ));
    assert_eq!(owner.recovery_record_count(), 0);
}

#[cfg(feature = "test-operation-control")]
fn run_paused_fork_and_advance<F>(
    fixture: &RealReferenceFixture,
    owner: &Arc<TestOwner>,
    attempt: crate::publication::ReservedCompositePublicationAttempt,
    on_reached: F,
) -> OwnerExecutionOutcome
where
    F: FnOnce(&RuntimeWorldCancellationSource),
{
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    use worth_signal::facade::branch::{
        SignalOwnerCancellationSource, SignalOwnerOperationBoundary,
    };

    let control = fixture.signal_operation_control();
    let pause = control.arm_pause_once(SignalOwnerOperationBoundary::OutcomeConstruction);
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let runtime_token = runtime_cancellation.token();
    let signal_cancellation = SignalOwnerCancellationSource::new();
    let signal_token = signal_cancellation.token();
    let owner_for_thread = Arc::clone(owner);
    let (started, started_receiver) = mpsc::channel();
    let join = thread::spawn(move || {
        let mut context = ();
        started
            .send(())
            .expect("execution thread is still observed");
        RuntimeWorldOwnerExecutionService::execute(
            owner_for_thread.as_ref(),
            attempt,
            CompositeExecutionBorrow::signal(&mut context, &signal_token, |_| Ok(())),
            &runtime_token,
        )
    });
    started_receiver
        .recv_timeout(Duration::from_secs(2))
        .expect("execution thread starts before the owner boundary");
    assert!(pause.wait_until_reached(Duration::from_secs(2)));
    on_reached(&runtime_cancellation);
    pause.release();
    join.join().expect("real owner execution does not panic")
}

#[cfg(feature = "test-operation-control")]
#[test]
fn cancellation_after_fork_before_advance_retains_only_the_forked_owner_effect() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "cancel-between-fork-and-advance",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("cancel-between-fork-and-advance"),
    );
    let outcome =
        run_paused_fork_and_advance(&fixture, &owner, reserve(&owner, plan), |cancellation| {
            cancellation.cancel()
        });
    let retained = match outcome {
        OwnerExecutionOutcome::ProductUnpublished(retained) => retained,
        other => panic!("a cancelled second owner retains the fork: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    assert_retained_fork_only(&retained, &cell);
    let handle = retained.recovery_handle();
    assert!(owner.cleanup_recovery(retained));
    assert!(owner.inspect_recovery(&handle).is_none());
}

#[cfg(feature = "test-operation-control")]
#[test]
fn deadline_after_fork_before_advance_retains_only_the_forked_owner_effect() {
    let clock = MutableClock::new(0);
    let (fixture, owner, expected) =
        setup_with_clock(RuntimeWorldClock::from_source(clock.clone()));
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "deadline-between-fork-and-advance",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("deadline-between-fork-and-advance"),
    );
    let attempt = reserve_with_deadline(
        &owner,
        plan,
        Some(crate::lifecycle::RuntimeWorldInstant::from_ticks(5)),
    )
    .expect("the future deadline admits the complete attempt reservation");
    let outcome = run_paused_fork_and_advance(&fixture, &owner, attempt, |_| clock.set(5));
    let retained = match outcome {
        OwnerExecutionOutcome::ProductUnpublished(retained) => retained,
        other => panic!("an expired second-owner deadline retains the fork: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::DeadlineAfterEffect
    );
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    assert_retained_fork_only(&retained, &cell);
    let handle = retained.recovery_handle();
    assert!(owner.cleanup_recovery(retained));
    assert!(owner.inspect_recovery(&handle).is_none());
}

#[cfg(feature = "test-operation-control")]
#[test]
fn stale_product_head_after_fork_before_advance_retains_fork_and_winner_evidence() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let competing_ready = ready_relational_fork_competitor(
        &fixture,
        owner.as_ref(),
        &expected,
        "stale-between-fork-and-advance-competitor",
    );
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "stale-between-fork-and-advance",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
        ),
        CompositeComponentIntent::signal_only(),
        Some("stale-between-fork-and-advance"),
    );
    let owner_for_callback = Arc::clone(&owner);
    let expected_for_callback = expected.clone();
    let outcome = run_paused_fork_and_advance(&fixture, &owner, reserve(&owner, plan), move |_| {
        publish_ready_competing_head(
            owner_for_callback.as_ref(),
            competing_ready,
            &expected_for_callback,
        );
    });
    let retained = match outcome {
        OwnerExecutionOutcome::ProductUnpublished(retained) => retained,
        other => panic!("a stale second-owner head retains the fork: {other:?}"),
    };
    assert_eq!(retained.cause(), ProductUnpublishedCause::StaleProductHead);
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    assert!(retained.last_observed_head().is_some());
    assert_retained_fork_only(&retained, &cell);
    assert_eq!(owner.recovery_record_count(), 1);
    drop(retained);
}
