use super::*;

use crate::branch::{
    ProductBranchHeadProtection, ProductBranchReferenceCell, ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::publication::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    CompositePublicationCostCounters, RuntimeWorldPublicationOutcome,
};

pub(super) fn install_competing_head(
    owner: &TestOwner,
    cell: &ProductBranchReferenceCell,
    expected: &ProductBranchObservation,
) -> Arc<CompositeRuntimeWorldCommit> {
    let (commit_identity, attempt_identity) = {
        let mut identities = owner
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner());
        (
            identities
                .composite_commit()
                .expect("competitor commit identity"),
            identities
                .publication_attempt()
                .expect("competitor attempt identity"),
        )
    };
    let commit = Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            commit_identity,
            expected.snapshot().commit(),
            expected.basis().clone(),
            attempt_identity,
            &CompositeOwnerExecutionResults::retained(),
            None,
        )
        .expect("same-basis competitor commit"),
    );
    owner
        .state
        .history
        .append(Arc::clone(&commit))
        .expect("competitor commit installs");
    let snapshot = ProductBranchReferenceSnapshot::owner_issued(
        expected.owner_identity(),
        expected.branch_identity().clone(),
        expected.lifecycle_incarnation(),
        expected
            .reference_generation()
            .advance()
            .expect("one competitor generation"),
        Arc::clone(&commit),
    )
    .expect("competitor snapshot belongs to the selected branch");
    let transfer = owner
        .state
        .retention
        .issue_publication(commit.basis())
        .expect("competitor acquires existing component pins")
        .into_product_head_transfer(commit.basis())
        .expect("competitor transfer matches its basis");
    let history = owner
        .state
        .history
        .protect_product_head(commit.as_ref())
        .expect("competitor history protection");
    let protection = ProductBranchHeadProtection::owner_issued(snapshot, transfer, history)
        .expect("competitor protection is coherent");
    cell.compare_and_publish(expected, protection)
        .expect("competitor wins the exact branch-cell CAS");
    commit
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
    let (fixture, owner, expected) = setup();
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
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let winner = install_competing_head(&owner, &cell, &expected);
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
        winner.identity()
    );
    assert_eq!(owner.recovery_record_count(), 0);
}

#[test]
fn lost_product_cas_retains_the_settled_owner_occurrence_and_observed_winner() {
    let (fixture, owner, expected) = setup();
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
    let winner = install_competing_head(&owner, &cell, &expected);
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
        winner.identity()
    );
    assert_eq!(retained.successor_basis(), Some(&successor));
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(cell.atomic_snapshot().selected_commit(), winner.identity());
    assert_eq!(owner.recovery_record_count(), 1);
    drop(retained);
}
