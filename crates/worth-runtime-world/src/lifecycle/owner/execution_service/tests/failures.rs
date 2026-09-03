use super::*;

use crate::publication::{
    CompositeLateCancellationPosture, CompositePublicationCostCounters,
    RuntimeWorldPublicationOutcome,
};

#[test]
fn relational_fork_destination_reservation_conflicts_and_releases_on_drop() {
    let (fixture, _owner, _expected) = setup();
    let held = fixture
        .reserve_relational_fork_target("relational-held-destination")
        .expect("the first Relational reservation is owner-issued");
    assert!(matches!(
        fixture.reserve_relational_fork_target("relational-held-destination"),
        Err(worth_relational::facade::branch::RelationalForkDenial::DuplicateTarget)
    ));
    drop(held);
    let released = fixture
        .reserve_relational_fork_target("relational-held-destination")
        .expect("dropping the owner reservation releases the exact target");
    drop(released);
}

#[test]
fn duplicate_signal_destination_denies_before_effect_and_retries_after_drop() {
    let (fixture, owner, expected) = setup();
    let held_name = validate_signal_branch_name("held-destination").expect("valid name");
    let held = owner
        .state
        .signal
        .mutation_port()
        .reserve_fork_exact(held_name, expected.basis().signal_basis())
        .expect("first owner-issued reservation holds the destination");
    let first_plan = plan(
        &fixture,
        &owner,
        &expected,
        "held-destination-first",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkExact,
        ),
        CompositeComponentIntent::signal_only(),
        Some("held-destination"),
    );
    let first = execute_without_signal(&owner, reserve(&owner, first_plan));
    assert!(matches!(
        first,
        OwnerExecutionOutcome::NoEffect(no_effect)
            if no_effect.cause() == crate::publication::NoEffectCause::PreEffectFailure
    ));
    drop(held);

    let retry_plan = plan(
        &fixture,
        &owner,
        &expected,
        "held-destination-retry",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ForkExact,
        ),
        CompositeComponentIntent::signal_only(),
        Some("held-destination"),
    );
    assert!(matches!(
        execute_without_signal(&owner, reserve(&owner, retry_plan)),
        OwnerExecutionOutcome::Settled(_)
    ));
}

#[test]
fn cancellation_before_effect_and_after_signal_effect_have_distinct_outcomes() {
    let (fixture, owner, expected) = setup();
    let cancel_before_plan = plan(
        &fixture,
        &owner,
        &expected,
        "cancel-before-effect",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
        None,
    );
    let reservation_cancellation = RuntimeWorldCancellationSource::new();
    let attempt = RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        cancel_before_plan,
        &reservation_cancellation.token(),
        None,
    )
    .expect("reservation completes before cancellation");
    reservation_cancellation.cancel();
    let before_token = reservation_cancellation.token();
    let before = RuntimeWorldOwnerExecutionService::execute(
        owner.as_ref(),
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &before_token,
    );
    assert!(matches!(
        before,
        OwnerExecutionOutcome::NoEffect(no_effect)
            if no_effect.cause() == crate::publication::NoEffectCause::CancelledBeforeEffect
    ));

    let cancel_after_plan = plan(
        &fixture,
        &owner,
        &expected,
        "cancel-after-effect",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
        None,
    );
    let attempt = reserve(&owner, cancel_after_plan);
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let runtime_token = runtime_cancellation.token();
    let signal_cancellation = SignalOwnerCancellationSource::new();
    let signal_token = signal_cancellation.token();
    let mut context = ();
    let after = RuntimeWorldOwnerExecutionService::execute(
        owner.as_ref(),
        attempt,
        CompositeExecutionBorrow::signal(&mut context, &signal_token, |_| {
            runtime_cancellation.cancel();
            Ok(())
        }),
        &runtime_token,
    );
    let retained = match after {
        OwnerExecutionOutcome::ProductUnpublished(retained) => retained,
        other => panic!("post-effect cancellation must retain effects: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    assert_eq!(retained.owner_effect_count(), 1);
    drop(retained);
}

#[test]
fn missing_signal_sibling_after_relational_movement_retains_exact_progress() {
    let (fixture, owner, expected) = setup();
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "missing-signal-sibling",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_and_signal(RelationalTransactionIntent::ordinary()),
        None,
    );
    let outcome = execute_without_signal(&owner, reserve(&owner, plan));
    let retained = match outcome {
        OwnerExecutionOutcome::ProductUnpublished(retained) => retained,
        other => panic!("missing Signal sibling must retain Relational work: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::SiblingOwnerDenied
    );
    assert_eq!(
        retained.progress().relational_posture(),
        RelationalAttemptProgressPosture::Settled
    );
    assert_eq!(
        retained.progress().signal_posture(),
        SignalAttemptProgressPosture::Untouched
    );
    assert_eq!(retained.owner_effect_count(), 1);
    drop(retained);
}

#[test]
fn stale_product_head_is_denied_before_the_first_owner_effect() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let competing_ready = ready_relational_fork_competitor(
        &fixture,
        owner.as_ref(),
        &expected,
        "stale-before-effect-competitor",
    );
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "stale-before-effect",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
        None,
    );
    let attempt = reserve(&owner, plan);
    let winner = publish_ready_competing_head(owner.as_ref(), competing_ready, &expected);
    let outcome = execute_without_signal(&owner, attempt);
    let no_effect = match outcome {
        OwnerExecutionOutcome::NoEffect(no_effect) => no_effect,
        other => panic!("stale product head must deny before effect: {other:?}"),
    };
    assert_eq!(
        no_effect.cause(),
        crate::publication::NoEffectCause::StaleExpectedProductHead
    );
    assert_eq!(
        no_effect.observed_head().unwrap().selected_commit(),
        winner.selected_commit()
    );
    assert_eq!(owner.recovery_record_count(), 0);
}

#[test]
fn lost_product_cas_retains_the_settled_owner_occurrence_and_observed_winner() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let competing_ready = ready_relational_fork_competitor(
        &fixture,
        owner.as_ref(),
        &expected,
        "lost-product-cas-competitor",
    );
    let plan = plan(
        &fixture,
        &owner,
        &expected,
        "lost-product-cas",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        None,
    );
    let settlement = settled(execute_without_signal(&owner, reserve(&owner, plan)));
    let successor = settlement.successor_basis().cloned().unwrap();
    let ready = settlement
        .ready(successor.clone())
        .expect("settled owner work is ready for product publication");
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let winner = publish_ready_competing_head(owner.as_ref(), competing_ready, &expected);
    let outcome = ready.publish(
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("lost product CAS must retain owner work: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::ProductPublicationLost
    );
    assert_eq!(
        retained.last_observed_head().unwrap().selected_commit(),
        winner.selected_commit()
    );
    assert_eq!(retained.successor_basis(), Some(&successor));
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(
        cell.atomic_snapshot().selected_commit(),
        winner.selected_commit()
    );
    assert_eq!(owner.recovery_record_count(), 1);
    drop(retained);
}
