use super::preparation_test_support::{intent, reservation_counts, setup, signal_intent};
use crate::branch::ProductBranchComponentPosture;
use crate::lifecycle::{
    RuntimeWorldCancellationSource, RuntimeWorldInstant, RuntimeWorldPreparationService,
};
use crate::publication::{CompositeComponentIntent, NoEffectCause};

#[test]
fn healthy_signal_plan_is_exact_and_reservation_is_linear() {
    let (owner, expected) = setup(2);
    let plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected.clone(),
        signal_intent("healthy-signal"),
    )
    .expect("current head admits Signal preparation");
    assert_eq!(plan.expected().expected(), &expected);
    assert_eq!(
        plan.signal().expected().admission_identity(),
        expected.basis().signal_basis().admission_identity()
    );
    assert_eq!(
        plan.signal().posture(),
        crate::publication::SignalComponentPlanPosture::AdvanceExact
    );
    assert_eq!(
        plan.relational().posture(),
        crate::publication::RelationalComponentPlanPosture::RetainExact
    );

    let cancellation = RuntimeWorldCancellationSource::new();
    let attempt =
        RuntimeWorldPreparationService::reserve(owner.as_ref(), plan, &cancellation.token(), None)
            .expect("healthy plan reserves");
    assert_eq!(
        attempt.order(),
        crate::publication::CompositePublicationOrder::RelationalThenSignal
    );
    assert_eq!(reservation_counts(owner.as_ref()), (1, 1, 2, 2, 1));
    assert_eq!(attempt.expected_head(), &expected);
    drop(attempt);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn incompatible_posture_and_intent_is_rejected_before_reservation() {
    let (owner, expected) = setup(2);
    let denied = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        intent(
            "wrong-route",
            ProductBranchComponentPosture::ForkAndAdvance,
            ProductBranchComponentPosture::ReuseExact,
            CompositeComponentIntent::signal_only(),
        ),
    )
    .expect_err("Signal-only intent cannot request a Relational fork advance");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn relational_publication_without_owner_candidate_is_rejected_before_reservation() {
    let (owner, expected) = setup(2);
    let denied = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        intent(
            "missing-relational-candidate",
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
            CompositeComponentIntent::relational_only(
                worth_relational::facade::mvcc::RelationalTransactionIntent::ordinary(),
            ),
        ),
    )
    .expect_err("policy-only Relational intent cannot forge a prepared candidate");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn signal_fork_reserves_destination_before_owner_execution_capacity() {
    let (owner, expected) = setup(2);
    let plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        intent(
            "signal-fork-without-owner-name-reservation",
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
            CompositeComponentIntent::signal_only(),
        )
        .with_signal_fork_name(
            worth_signal::facade::branch::validate_signal_branch_name(
                "signal-fork-without-owner-name-reservation",
            )
            .expect("Signal fork name validates"),
        ),
    )
    .expect("syntax-only Signal fork lowering is pre-effect");
    assert_eq!(
        plan.signal().posture(),
        crate::publication::SignalComponentPlanPosture::ForkAndAdvance
    );
    let attempt = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        plan,
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect("owner-issued Signal destination reservation is part of preparation");
    assert_eq!(reservation_counts(owner.as_ref()), (1, 1, 2, 2, 1));
    drop(attempt);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

#[test]
fn signal_fork_without_explicit_validated_name_is_rejected_before_reservation() {
    let (owner, expected) = setup(2);
    let denied = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        intent(
            "signal-fork-without-component-name",
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkAndAdvance,
            CompositeComponentIntent::signal_only(),
        ),
    )
    .expect_err("a product name cannot stand in for a validated Signal name");
    assert_eq!(denied.cause(), NoEffectCause::PreEffectFailure);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));
}

// Serial-integration blocker (P4): restore stale-head coverage through the
// canonical owner-publication path after contract convergence; this lane
// cannot mint competing publication/CAS authority.

#[test]
fn cancellation_and_deadline_win_before_any_capacity_is_reserved() {
    let (owner, expected) = setup(2);
    let plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        signal_intent("cancelled"),
    )
    .expect("healthy plan");
    let cancellation = RuntimeWorldCancellationSource::new();
    cancellation.cancel();
    let denied =
        RuntimeWorldPreparationService::reserve(owner.as_ref(), plan, &cancellation.token(), None)
            .expect_err("cancelled pre-effect operation is no-effect");
    assert_eq!(denied.cause(), NoEffectCause::CancelledBeforeEffect);
    assert_eq!(reservation_counts(owner.as_ref()), (0, 0, 0, 0, 0));

    let (owner, expected) = setup(2);
    let plan =
        RuntimeWorldPreparationService::prepare(owner.as_ref(), expected, signal_intent("expired"))
            .expect("healthy plan");
    let cancellation = RuntimeWorldCancellationSource::new();
    let denied = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        plan,
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
    let first_plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected.clone(),
        signal_intent("capacity-first"),
    )
    .expect("first plan");
    let first = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        first_plan,
        &RuntimeWorldCancellationSource::new().token(),
        None,
    )
    .expect("one active attempt fits");
    let second_plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected.clone(),
        signal_intent("capacity-second"),
    )
    .expect("second plan remains pre-effect");
    let denied = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        second_plan,
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
    let plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected,
        signal_intent("identity-exhausted"),
    )
    .expect("identity exhaustion is discovered only at reservation");
    let denied = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        plan,
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
    let first_plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected.clone(),
        signal_intent("first"),
    )
    .expect("first plan");
    let second_plan =
        RuntimeWorldPreparationService::prepare(owner.as_ref(), expected, signal_intent("second"))
            .expect("second plan");
    let cancellation = RuntimeWorldCancellationSource::new();
    let first = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        first_plan,
        &cancellation.token(),
        None,
    )
    .expect("first attempt");
    let second = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        second_plan,
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
