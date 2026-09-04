use super::preparation_test_support::{reservation_counts, setup, signal_intent};
use crate::lifecycle::{RuntimeWorldInstant, RuntimeWorldPreparationService};
use crate::publication::{
    CompositePublicationIntent, NoEffectCause, RuntimeWorldCancellationSource,
};

#[test]
fn healthy_signal_preparation_is_exact_and_reservation_is_linear() {
    let (owner, expected) = setup(2);
    let cancellation = RuntimeWorldCancellationSource::new();
    let prepared = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected.clone(),
        signal_intent(),
        &cancellation.token(),
        None,
    )
    .expect("current head admits Signal preparation");
    let attempt = prepared.attempt();
    assert_eq!(attempt.expected_head(), &expected);
    assert_eq!(
        attempt.plan().signal().expected().admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    assert_eq!(
        attempt.plan().signal().posture(),
        crate::publication::SignalComponentPlanPosture::AdvanceExact
    );
    assert_eq!(
        attempt.plan().relational().posture(),
        crate::publication::RelationalComponentPlanPosture::RetainExact
    );
    assert_eq!(
        attempt.order(),
        crate::publication::CompositePublicationOrder::RelationalThenSignal
    );
    assert_eq!(reservation_counts(owner.as_ref()), (1, 1, 2, 2, 1));
    drop(prepared);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn relational_publication_without_owner_candidate_is_rejected_before_reservation() {
    let (owner, expected) = setup(2);
    let denied = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        CompositePublicationIntent::without_signal(
            worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
        ),
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect_err("policy-only Relational intent cannot forge a prepared candidate");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

// Serial-integration blocker (P4): restore stale-head coverage through the
// canonical owner-publication path after contract convergence; this lane
// cannot mint competing publication/CAS authority.

#[test]
fn cancellation_and_deadline_win_before_any_capacity_is_reserved() {
    let (owner, expected) = setup(2);
    let cancellation = RuntimeWorldCancellationSource::new();
    cancellation.cancel();
    let denied = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        signal_intent(),
        &cancellation.token(),
        None,
    )
    .expect_err("cancelled pre-effect operation is no-effect");
    assert_eq!(denied.cause(), NoEffectCause::CancelledBeforeEffect);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));

    let (owner, expected) = setup(2);
    let cancellation = RuntimeWorldCancellationSource::new();
    let denied = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        signal_intent(),
        &cancellation.token(),
        Some(RuntimeWorldInstant::from_ticks(0)),
    )
    .expect_err("expired deadline is no-effect");
    assert_eq!(denied.cause(), NoEffectCause::DeadlineBeforeEffect);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn capacity_and_identity_denials_have_no_partial_reservation() {
    let (owner, expected) = setup(1);
    let first = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected.clone(),
        signal_intent(),
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect("one active attempt fits");
    let denied = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        signal_intent(),
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect_err("active-attempt capacity is bounded");
    assert_eq!(denied.cause(), NoEffectCause::CapacityExhausted);
    assert_eq!(reservation_counts(owner.as_ref()), (1, 1, 2, 2, 1));
    drop(first);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));

    let (owner, expected) = setup(2);
    owner
        .state
        .identities
        .lock()
        .unwrap_or_else(|error| error.into_inner())
        .set_next_publication_attempt_for_test(u64::MAX);
    let denied = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        signal_intent(),
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect_err("publication-attempt identity exhaustion is pre-effect");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn two_same_head_attempts_keep_independent_local_phase_and_cleanup() {
    let (owner, expected) = setup(2);
    let cancellation = RuntimeWorldCancellationSource::new();
    let first = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected.clone(),
        signal_intent(),
        &cancellation.token(),
        None,
    )
    .expect("first attempt");
    let second = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected,
        signal_intent(),
        &cancellation.token(),
        None,
    )
    .expect("second attempt is not serialized behind first");
    assert_eq!(owner.state.operation.active(), 2);
    drop(first);
    assert_eq!(owner.state.operation.active(), 1);
    drop(second);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}
