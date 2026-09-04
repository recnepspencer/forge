//! SPEC-P4-005. The Runtime World cancellation source reaches a Signal advance
//! that is already in flight, because the token the seam hands the Signal owner
//! is the one embedded in that source.

use std::thread;

use super::*;

/// One `RuntimeWorldCancellationSource::cancel()` issued from another thread
/// while the owner is inside `advance_exact` denies that advance with no
/// movement. The already settled Relational effect is retained exactly, and the
/// Signal owner is left untouched.
#[test]
fn runtime_world_cancel_reaches_an_in_flight_signal_advance() {
    let (fixture, owner, expected) = setup();
    let prepared = prepare_both_owners(&fixture, &owner, &expected, "cancel-in-flight-advance");
    let (rehearsal, reached) =
        arm_rehearsal(owner.as_ref(), ExecutionRehearsalBoundary::SignalAdvance);
    let source = RuntimeWorldCancellationSource::new();
    let token = source.token();
    let mut context = ();

    let outcome = thread::scope(|scope| {
        let source_ref = &source;
        let rehearsal_ref = &rehearsal;
        let canceller = scope.spawn(move || {
            let signal_token = awaited_signal_token(&reached);
            assert!(
                !signal_token.is_cancelled(),
                "the advance is in flight before the cancellation is requested"
            );
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
        1,
        "the owner entered the Signal advance exactly once"
    );
    let record = retained(outcome);
    assert_retains_only_the_relational_effect(
        &record,
        ProductUnpublishedCause::CancellationAfterEffect,
    );
    drop(record);
}

/// The Signal owner is handed the token embedded in this publication's Runtime
/// World source. An unrelated caller's source cannot reach the advance, and
/// this source's `cancel()` does.
#[test]
fn signal_advance_receives_the_embedded_token_not_a_caller_token() {
    let (_fixture, owner, expected) = setup();
    let prepared = prepare_signal(&owner, &expected, None);
    let (rehearsal, reached) =
        arm_rehearsal(owner.as_ref(), ExecutionRehearsalBoundary::SignalAdvance);
    let source = RuntimeWorldCancellationSource::new();
    let unrelated = RuntimeWorldCancellationSource::new();
    let token = source.token();
    let mut context = ();

    let outcome = thread::scope(|scope| {
        let source_ref = &source;
        let unrelated_ref = &unrelated;
        let rehearsal_ref = &rehearsal;
        let observer = scope.spawn(move || {
            let signal_token = awaited_signal_token(&reached);
            unrelated_ref.cancel();
            assert!(
                !signal_token.is_cancelled(),
                "an unrelated caller's source cannot cancel this advance"
            );
            source_ref.cancel();
            assert!(
                signal_token.is_cancelled(),
                "the advance received the Signal token embedded in this publication's source"
            );
            rehearsal_ref.release();
        });
        let outcome = RuntimeWorldOwnerExecutionService::execute_with_signal(
            owner.as_ref(),
            prepared,
            &mut context,
            &token,
            |_| Ok(()),
        );
        observer
            .join()
            .expect("the observing thread completes within the rehearsal budget");
        outcome
    });

    assert!(
        matches!(
            outcome,
            OwnerExecutionOutcome::NoEffect(no_effect)
                if no_effect.cause() == crate::publication::NoEffectCause::CancelledBeforeEffect
        ),
        "a Signal-only advance denied with no movement leaves no owner effect"
    );
}
