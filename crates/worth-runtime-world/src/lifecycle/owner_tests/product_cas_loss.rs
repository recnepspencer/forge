//! Proofs for the losing arm of the product CAS.
//!
//! A production ready attempt is admitted and executed against an observed
//! product head. A second owner-issued publication then moves that head. The
//! ready attempt reaches `publish` against a cell that has already moved and
//! must terminate as retained owner effects naming the winner it observed.
//!
//! The winner is issued from the owner's own identity issuer, history catalog
//! and retention registry rather than from a second `CompositePublicationReady`
//! because two composite attempts cannot coexist off one product head: the
//! second attempt's owner execution is denied `OwnerUnavailable` once the first
//! attempt's execution has advanced the relational component owner. See the
//! lane report for the exact mechanism.

use std::sync::Arc;

use crate::branch::reference_test_fixture::RealReferenceFixture;
use crate::branch::{
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::publication::{
    CompositeLateCancellationPosture, CompositeOwnerExecutionResults,
    RuntimeWorldPublicationOutcome,
};
use crate::recovery::{
    ProductUnpublishedCause, ProductUnpublishedNextAction, ProductUnpublishedOwnerEffects,
    ProductUnpublishedRetentionPosture,
};

use super::publication::{
    prepare_relational, ready_from_prepared, setup, setup_with_relational_source, TestOwner,
};

/// One resolved race: the winner's product head, and the retained record the
/// loser produced against it.
pub(super) struct ResolvedRace {
    pub(super) owner: Arc<TestOwner>,
    pub(super) expected: ProductBranchObservation,
    pub(super) winner_head: ProductBranchReferenceSnapshot,
    pub(super) retained: ProductUnpublishedOwnerEffects,
    pub(super) history_len_after_winner: usize,
}

/// Admit and execute one ready attempt, move the product head out from under
/// it with a second owner-issued publication, then send it into `publish`.
pub(super) fn resolve_one_race(late: CompositeLateCancellationPosture) -> ResolvedRace {
    race_from(setup(), late)
}

/// The same race, run against an owner that has already completed one ordinary
/// publication. The completed attempt released its exact component custody, so
/// the registry carries a zero-count entry no live holder still names.
pub(super) fn resolve_one_race_after_a_completed_publication(
    late: CompositeLateCancellationPosture,
) -> ResolvedRace {
    race_from(setup_with_relational_source(), late)
}

fn race_from(
    base: (
        RealReferenceFixture,
        Arc<TestOwner>,
        ProductBranchObservation,
    ),
    late: CompositeLateCancellationPosture,
) -> ResolvedRace {
    let (fixture, owner, expected) = base;
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let loser = ready_from_prepared(
        owner.as_ref(),
        prepare_relational(&fixture, owner.as_ref(), expected.clone(), "cas-loser"),
        "the losing attempt settles its owner work before the race",
    );

    let winner_head = publish_competing_head(owner.as_ref(), &expected, &cell);
    assert_eq!(&winner_head, &cell.atomic_snapshot());
    assert_ne!(
        winner_head.reference_generation(),
        expected.snapshot().reference_generation(),
        "the competing publication must actually move the product reference"
    );
    let history_len_after_winner = owner.state.history.len();

    let retained = match loser.publish(&cell, late) {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("the attempt that lost the product reference must retain: {other:?}"),
    };
    ResolvedRace {
        owner,
        expected,
        winner_head,
        retained,
        history_len_after_winner,
    }
}

/// Move the product reference with a second owner-issued publication, built
/// from the owner's own identity issuer, history catalog and retention
/// registry. It retains both component bases, so it is an ordinary successor
/// publication that changes only the product reference.
fn publish_competing_head(
    owner: &TestOwner,
    current: &ProductBranchObservation,
    cell: &ProductBranchReferenceCell,
) -> ProductBranchReferenceSnapshot {
    let commit = Arc::new(competing_commit(owner, current));
    owner
        .state
        .history
        .append(Arc::clone(&commit))
        .expect("the competing successor commit installs into owner history");
    let history = owner
        .state
        .history
        .protect_product_head(&commit)
        .expect("the installed competing commit admits product-head protection");
    let transfer = owner
        .state
        .retention
        .issue_publication(commit.basis())
        .expect("the owner issues real publication retention for the competing basis")
        .into_product_head_transfer(commit.basis())
        .expect("competing publication retention binds its exact basis");
    let snapshot = ProductBranchReferenceSnapshot::owner_issued(
        current.owner_identity(),
        current.branch_identity().clone(),
        current.lifecycle_incarnation(),
        current
            .reference_generation()
            .advance()
            .expect("the bootstrapped reference has generation capacity"),
        Arc::clone(&commit),
    )
    .expect("the competing snapshot shares the owner and branch lineage");
    let protection = ProductBranchHeadProtection::owner_issued(snapshot.clone(), transfer, history)
        .expect("competing component and history custody match its successor image");
    cell.compare_and_publish(current, protection)
        .expect("the competing publication holds the current product head");
    snapshot
}

fn competing_commit(
    owner: &TestOwner,
    current: &ProductBranchObservation,
) -> CompositeRuntimeWorldCommit {
    let mut issuer = owner
        .state
        .identities
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    let commit_identity = issuer
        .composite_commit()
        .expect("the owner issues a competing commit identity");
    let attempt_identity = issuer
        .publication_attempt()
        .expect("the owner issues a competing publication attempt identity");
    drop(issuer);
    CompositeRuntimeWorldCommit::from_ordinary_publication(
        commit_identity,
        current.snapshot().commit(),
        current.basis().clone(),
        attempt_identity,
        &CompositeOwnerExecutionResults::retained(),
        None,
    )
    .expect("a competing ordinary publication off the current product head")
}

/// SPEC-P4-016. The expected-observation comparison precedes materialization,
/// so the attempt that lost the product reference never takes product-head
/// authority: it advances no product reference generation, leaves the winner's
/// snapshot as the sole product head, and leaks no reserved capacity.
///
/// Its reserved history slot is released rather than spent; that is proved on
/// its own by `a_losing_cas_attempt_does_not_consume_its_reserved_history_slot`.
#[test]
fn a_losing_cas_attempt_takes_no_product_head_authority_and_advances_no_generation() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);
    let cell = race
        .owner
        .state
        .branches
        .root_cell()
        .expect("bootstrapped cell");

    assert_eq!(
        cell.atomic_snapshot(),
        race.winner_head,
        "the loser must not move the product reference"
    );
    assert_eq!(
        cell.atomic_snapshot().reference_generation(),
        race.winner_head.reference_generation(),
        "the loser must not consume a product reference generation"
    );
    assert_eq!(
        cell.atomic_snapshot().selected_commit(),
        race.winner_head.selected_commit(),
        "the product head reaches the winner's commit alone"
    );
    assert_eq!(
        race.owner.state.recovery.reserved_slots(),
        0,
        "the loser leaks no reserved recovery capacity"
    );
    assert_eq!(race.owner.state.operation.active(), 0);
    assert_eq!(race.owner.state.publication_capacity.active(), 0);

    let handle = race.retained.recovery_handle();
    drop(race.retained);
    assert!(race.owner.cleanup_recovery_handle(&handle));
    assert_eq!(race.owner.recovery_record_count(), 0);
    assert_eq!(
        cell.atomic_snapshot(),
        race.winner_head,
        "cleaning up the loser leaves the winner's product head untouched"
    );
}

/// The retained record names the settled owner occurrence it produced and the
/// exact winner it observed, and derives its next actions from that progress.
#[test]
fn lost_product_cas_retains_the_settled_owner_occurrence_and_observed_winner() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);

    assert_eq!(
        race.retained.cause(),
        ProductUnpublishedCause::ProductPublicationLost
    );
    assert_eq!(
        race.retained.last_observed_head(),
        Some(&race.winner_head),
        "the retained record names the exact winner it observed"
    );
    assert_eq!(
        race.retained.expected_head().snapshot(),
        race.expected.snapshot(),
        "the retained record keeps the head the attempt admitted against"
    );
    assert_eq!(
        race.retained.progress().relational_posture(),
        crate::publication::RelationalAttemptProgressPosture::Settled,
        "the loser's own owner occurrence is settled, not rolled back"
    );
    assert_eq!(
        race.retained.next_actions(),
        crate::recovery::next_actions_for_progress(race.retained.progress(), race.retained.cause()),
        "next actions are derived from the attempt's progress, never hard-coded"
    );
    assert!(!race
        .retained
        .next_actions()
        .contains(&ProductUnpublishedNextAction::SettleOwnerEffects));
}

/// A caller that reports cancellation in the same window as the loss must not
/// have its loss reclassified as cancellation.
#[test]
fn cancelled_ready_loses_to_winner_and_retains_publication_loss() {
    let race = resolve_one_race(CompositeLateCancellationPosture::RequestedBeforeProductMovement);

    assert_eq!(
        race.retained.cause(),
        ProductUnpublishedCause::ProductPublicationLost,
        "an observed loss is never hidden as cancellation"
    );
    assert_eq!(race.retained.last_observed_head(), Some(&race.winner_head));
    assert_eq!(
        race.owner
            .state
            .branches
            .root_cell()
            .expect("bootstrapped cell")
            .atomic_snapshot(),
        race.winner_head
    );
}

/// The loser keeps its exact commit results and its exact component custody:
/// the successor occurrence stays reachable and the pin pair stays held.
#[test]
fn complete_ready_publish_path_retains_commit_results_and_custody_on_observed_product_loss() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);

    assert_eq!(
        race.retained.retention_posture(),
        ProductUnpublishedRetentionPosture::RetainedExact,
        "a lost CAS keeps the exact component pins it already held"
    );
    assert_eq!(
        race.retained.live_obligation_count(),
        2,
        "a record that installed no successor keeps the recovery slot and the          component pins alone"
    );
    assert_eq!(race.retained.owner_effect_count(), 1);
    assert_eq!(
        race.retained.successor_commit(),
        None,
        "a pre-movement loser names no installed successor occurrence"
    );
    assert_ne!(
        race.retained
            .successor_basis()
            .expect("a lost CAS retains its successor basis"),
        race.expected.basis(),
        "the successor this attempt would have published is retained as basis          evidence: the owner occurrence it settled, not the head it admitted          against"
    );
    assert_eq!(race.owner.recovery_record_count(), 1);
}

/// The reserved history slot of an attempt that lost before the product
/// reference moved goes back to the catalog. Installing it would charge a
/// bounded slot for a commit no product head names and no caller can reach,
/// and the retained record needs no installed occurrence to stay honest: the
/// successor basis is the evidence a fresh attempt names it by.
#[test]
fn a_losing_cas_attempt_does_not_consume_its_reserved_history_slot() {
    let race = resolve_one_race(CompositeLateCancellationPosture::NotRequested);

    assert_eq!(
        race.owner.state.history.len(),
        race.history_len_after_winner,
        "the loser installs nothing into history"
    );
    assert_eq!(
        race.owner.state.history.reserved_len(),
        0,
        "the loser's history reservation is released, not held"
    );
    assert_eq!(
        race.retained.successor_commit(),
        None,
        "no installed successor occurrence is retained"
    );

    let handle = race.retained.recovery_handle();
    drop(race.retained);
    assert!(race.owner.cleanup_recovery_handle(&handle));
    assert_eq!(
        race.owner.state.history.len(),
        race.history_len_after_winner,
        "cleanup has no successor occurrence to remove and removes none"
    );
    assert_eq!(race.owner.state.history.reserved_len(), 0);
}
