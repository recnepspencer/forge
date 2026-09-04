//! The gate between a settled Relational effect and the Signal advance. Every
//! denial here stops before the Signal owner is contacted and retains exactly
//! the Relational effect that was already performed.

use std::thread;

use super::*;

/// A cancellation observed after the Relational effect and before the Signal
/// advance retains only that Relational effect, and the Signal owner is never
/// contacted.
#[test]
fn cancellation_between_the_relational_effect_and_the_signal_advance_retains_only_the_relational_effect(
) {
    let (fixture, owner, expected) = setup();
    let prepared = prepare_both_owners(&fixture, &owner, &expected, "cancel-between-owners");
    let (rehearsal, reached) = arm_rehearsal(
        owner.as_ref(),
        ExecutionRehearsalBoundary::BetweenOwnerEffects,
    );
    let source = RuntimeWorldCancellationSource::new();
    let token = source.token();
    let mut context = ();

    let outcome = thread::scope(|scope| {
        let source_ref = &source;
        let rehearsal_ref = &rehearsal;
        let canceller = scope.spawn(move || {
            await_boundary(&reached);
            source_ref.cancel();
            rehearsal_ref.release();
        });
        let outcome = RuntimeWorldOwnerExecutionService::execute_with_signal(
            owner.as_ref(),
            prepared,
            &mut context,
            &token,
            |_| Ok(()),
        );
        canceller
            .join()
            .expect("the cancelling thread completes within the rehearsal budget");
        outcome
    });

    assert_eq!(
        rehearsal.signal_advance_entries(),
        0,
        "the Signal owner was never contacted"
    );
    let record = retained(outcome);
    assert_retains_only_the_relational_effect(
        &record,
        ProductUnpublishedCause::CancellationAfterEffect,
    );
    drop(record);
}

/// A deadline that expires after the Relational effect and before the Signal
/// advance retains only that Relational effect, and the Signal owner is never
/// contacted.
#[test]
fn deadline_between_the_relational_effect_and_the_signal_advance_retains_only_the_relational_effect(
) {
    let clock = MutableClock::new(0);
    let (fixture, owner, expected) =
        setup_with_clock(RuntimeWorldClock::from_source(clock.clone()));
    let cancellation = RuntimeWorldCancellationSource::new();
    let prepared = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected.clone(),
        CompositePublicationIntent::with_signal(Some(RelationalTransactionIntent::ordinary()))
            .with_prepared_relational_candidate(
                fixture.prepare_relational_owner_candidate("deadline-between-owners"),
            ),
        &cancellation.token(),
        Some(crate::lifecycle::RuntimeWorldInstant::from_ticks(5)),
    )
    .expect("the future deadline admits the complete attempt reservation");
    let (rehearsal, reached) = arm_rehearsal(
        owner.as_ref(),
        ExecutionRehearsalBoundary::BetweenOwnerEffects,
    );
    let mut context = ();

    let outcome = thread::scope(|scope| {
        let clock_ref = &clock;
        let rehearsal_ref = &rehearsal;
        let expirer = scope.spawn(move || {
            await_boundary(&reached);
            clock_ref.set(5);
            rehearsal_ref.release();
        });
        let outcome = RuntimeWorldOwnerExecutionService::execute_with_signal(
            owner.as_ref(),
            prepared,
            &mut context,
            &cancellation.token(),
            |_| Ok(()),
        );
        expirer
            .join()
            .expect("the deadline thread completes within the rehearsal budget");
        outcome
    });

    assert_eq!(
        rehearsal.signal_advance_entries(),
        0,
        "the Signal owner was never contacted"
    );
    let record = retained(outcome);
    assert_retains_only_the_relational_effect(
        &record,
        ProductUnpublishedCause::DeadlineAfterEffect,
    );
    drop(record);
}

/// The third arm of the same gate. A product head replaced by a competing
/// winner is reported as a stale head together with the winner the attempt now
/// observes, so a settled Relational effect stops before the Signal owner.
#[test]
fn the_pre_advance_gate_reports_a_stale_product_head_with_the_observed_winner() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let competing_ready = ready_relational_competitor(
        &fixture,
        owner.as_ref(),
        &expected,
        "stale-between-owners-competitor",
    );
    let cancellation = RuntimeWorldCancellationSource::new();
    assert!(
        owner
            .pre_advance_signal_gate(&expected, None, &cancellation.token())
            .is_ok(),
        "the gate admits the attempt while its exact head is current"
    );

    let winner = publish_ready_competing_head(owner.as_ref(), competing_ready, &expected);
    assert!(
        matches!(
            owner.pre_advance_signal_gate(&expected, None, &cancellation.token()),
            Err((
                ProductUnpublishedCause::StaleProductHead,
                crate::publication::NoEffectCause::StaleExpectedProductHead
            ))
        ),
        "a replaced product head stops the attempt before the Signal advance"
    );
    assert_eq!(
        owner
            .current_product_head_snapshot(&expected)
            .expect("the replaced cell reports the winner the attempt observes")
            .selected_commit(),
        winner.selected_commit(),
    );
}
