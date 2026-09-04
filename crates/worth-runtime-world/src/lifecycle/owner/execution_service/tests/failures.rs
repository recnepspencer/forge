use super::*;

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
fn cancellation_before_effect_and_after_signal_effect_have_distinct_outcomes() {
    let (_fixture, owner, expected) = setup();
    let reservation_cancellation = RuntimeWorldCancellationSource::new();
    let prepared = RuntimeWorldPreparationService::prepare_publication(
        owner.as_ref(),
        expected.clone(),
        CompositePublicationIntent::with_signal(None),
        &reservation_cancellation.token(),
        None,
    )
    .expect("reservation completes before cancellation");
    reservation_cancellation.cancel();
    let mut context = ();
    let before = RuntimeWorldOwnerExecutionService::execute_with_signal(
        owner.as_ref(),
        prepared,
        &mut context,
        &reservation_cancellation.token(),
        |_| Ok(()),
    );
    assert!(matches!(
        before,
        OwnerExecutionOutcome::NoEffect(no_effect)
            if no_effect.cause() == crate::publication::NoEffectCause::CancelledBeforeEffect
    ));

    let prepared = prepare_signal(&owner, &expected, None);
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let runtime_token = runtime_cancellation.token();
    let mut context = ();
    let after = RuntimeWorldOwnerExecutionService::execute_with_signal(
        owner.as_ref(),
        prepared,
        &mut context,
        &runtime_token,
        |_| {
            runtime_cancellation.cancel();
            Ok(())
        },
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
fn stale_product_head_is_denied_before_the_first_owner_effect() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let competing_ready = ready_relational_competitor(
        &fixture,
        owner.as_ref(),
        &expected,
        "stale-before-effect-competitor",
    );
    let prepared = prepare_signal(&owner, &expected, None);
    let winner = publish_ready_competing_head(owner.as_ref(), competing_ready, &expected);
    let outcome = execute_with_empty_signal(&owner, prepared);
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
