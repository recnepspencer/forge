//! Installation under the source head's guard.
//!
//! Exact reuse and fork finalization both recheck the source head before they
//! charge anything, and a publication can land between that recheck and the
//! registry installation. The installation therefore happens under the source
//! cell's own read guard, so the recheck and the install are one step: a
//! source that moved in the window is refused at the install, named the way
//! the pre-effect recheck names it, and nothing is installed from a head that
//! is no longer current. The window is reached through the seam that holds
//! one creation just before it takes the guard.
//!
//! A source branch that holds no occurrence at all is `RetiredBranch` on
//! every creation route, the same answer observation gives for that branch.

use std::sync::{mpsc, Arc};
use std::time::Duration;

use super::fork_creation::{
    current_root_observation, fork_intent, relational_fork, seed_relational_source,
    setup_with_relational_source, signal_fork,
};
use super::{create_reused_branch, owner_lifecycles, reuse_intent, TestOwner};
use crate::branch::{
    ProductBranchCreationIntent, ProductBranchObservation, RuntimeWorldBranchAdmissionDenial,
};
use crate::lifecycle::{
    RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchCreationRequest,
    RuntimeWorldBranchService, RuntimeWorldObservationService,
};
use crate::publication::{
    RelationalAttemptProgressPosture, RuntimeWorldCancellationSource, SignalAttemptProgressPosture,
};
use crate::recovery::{ProductUnpublishedCause, ProductUnpublishedOwnerEffects};

const SOURCE_GUARD_TEST_TIMEOUT: Duration = Duration::from_secs(5);

/// Run one creation, hold it just before its source-guarded installation, let
/// the caller act on the world while it is held, then release it and return
/// where it landed.
fn creation_interrupted_before_install(
    owner: &Arc<TestOwner>,
    source: ProductBranchObservation,
    intent: ProductBranchCreationIntent,
    while_held: impl FnOnce(&TestOwner),
) -> Result<RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchAdmissionDenial> {
    let (reached_tx, reached_rx) = mpsc::sync_channel(1);
    let rehearsal = owner.rehearse_source_guarded_install(reached_tx);
    let (finished_tx, finished_rx) = mpsc::sync_channel(1);
    let worker_owner = Arc::clone(owner);
    let worker = std::thread::spawn(move || {
        let cancellation = RuntimeWorldCancellationSource::new();
        let outcome = RuntimeWorldBranchService::create_product_branch(
            worker_owner.as_ref(),
            RuntimeWorldBranchCreationRequest::new(source, intent, &cancellation.token()),
        );
        finished_tx
            .send(outcome)
            .expect("the guard proof still owns its completion receiver");
    });

    let paused = reached_rx
        .recv_timeout(SOURCE_GUARD_TEST_TIMEOUT)
        .expect("the creation reaches its source-guarded installation");
    assert_eq!(paused, owner.owner_identity());
    while_held(owner.as_ref());

    drop(rehearsal);
    let outcome = finished_rx
        .recv_timeout(SOURCE_GUARD_TEST_TIMEOUT)
        .expect("the creation finishes once its installation is released");
    worker.join().expect("the creation worker does not panic");
    outcome
}

#[test]
fn exact_reuse_whose_source_moves_before_install_is_refused_under_the_guard() {
    let (mut fixture, owner, source) = setup_with_relational_source(3);
    let owner = Arc::new(owner);
    let lifecycles_before = owner_lifecycles(&owner);
    let history_before = owner.state.history.len();
    let held_source = source.clone();
    let mut pins_after_move = None;
    let denial = creation_interrupted_before_install(
        &owner,
        source.clone(),
        reuse_intent("reuse-under-moved-source"),
        |paused_owner| {
            assert_eq!(
                paused_owner.state.branches.reserved_branch_count(),
                1,
                "the reuse holds its destination reservation at the guard"
            );
            assert_eq!(
                paused_owner.state.operation.active(),
                1,
                "the reuse holds an operation reservation, so close cannot drain under it"
            );
            // A different operation wins the product head this reuse was
            // admitted against, after its pre-charge recheck passed.
            seed_relational_source(paused_owner, &mut fixture, held_source);
            pins_after_move = Some(paused_owner.state.retention.unique_pin_count());
        },
    )
    .expect_err("a source that moved before the install is refused at the install");

    assert_eq!(denial, RuntimeWorldBranchAdmissionDenial::StaleSourceHead);
    assert_eq!(owner_lifecycles(&owner), lifecycles_before);
    assert_eq!(
        owner.state.branches.branch_count(),
        1,
        "no child is installed from a head that is no longer current"
    );
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(
        owner.state.history.len(),
        history_before + 1,
        "the winning publication is the only history the window added"
    );
    assert_eq!(
        Some(owner.state.retention.unique_pin_count()),
        pins_after_move,
        "the refused child's head and observation pins are released"
    );
    drop(source);
    drop(fixture);
}

#[test]
fn a_fork_whose_source_moves_before_install_retains_the_winner_instead_of_installing() {
    let (mut fixture, owner, source) = setup_with_relational_source(3);
    let owner = Arc::new(owner);
    let held_source = source.clone();
    let outcome = creation_interrupted_before_install(
        &owner,
        source.clone(),
        fork_intent(
            "branch-fork-under-moved-source",
            relational_fork("relational-branch-fork-under-moved-source"),
            signal_fork("signal-branch-fork-under-moved-source"),
        ),
        |paused_owner| {
            assert_eq!(
                paused_owner.state.custody.installed(),
                2,
                "both owner forks happened before the guard"
            );
            assert_eq!(
                paused_owner.state.branches.branch_count(),
                1,
                "the destination is not installed while held"
            );
            seed_relational_source(paused_owner, &mut fixture, held_source);
        },
    )
    .expect("a source displaced at the install is retained, not denied");

    let winner = current_root_observation(&owner);
    assert_ne!(winner.selected_commit(), source.selected_commit());
    let effects = match outcome {
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(effects) => effects,
        RuntimeWorldBranchCreationOutcome::Performed(_) => {
            panic!("a fork cannot install a child from a head that is no longer current")
        }
    };
    assert_retained_behind_winner(&effects, &winner);
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(
        owner.state.custody.installed(),
        2,
        "the component branches the fork really made stay in custody"
    );
    assert_eq!(owner.recovery_record_count(), 1);
    assert!(owner.cleanup_recovery(effects).is_some());
    assert_eq!(owner.recovery_record_count(), 0);
    drop(winner);
    drop(fixture);
}

/// The retained record of a fork that performed both owner forks and was then
/// refused at the install names the head that displaced it.
fn assert_retained_behind_winner(
    effects: &ProductUnpublishedOwnerEffects,
    winner: &ProductBranchObservation,
) {
    assert_eq!(effects.cause(), ProductUnpublishedCause::StaleProductHead);
    let observed = effects
        .last_observed_head()
        .expect("the record names the head that displaced the fork");
    assert_eq!(observed.commit().identity(), winner.selected_commit());
    assert_eq!(observed.branch(), winner.branch_identity());
    assert_eq!(
        effects.progress().relational_posture(),
        RelationalAttemptProgressPosture::Performed
    );
    assert_eq!(
        effects.progress().signal_posture(),
        SignalAttemptProgressPosture::Performed
    );
    assert_eq!(
        effects.live_obligation_count(),
        4,
        "the exact pin pair, the recovery slot, and the installed successor history"
    );
}

#[test]
fn creation_from_a_retired_source_is_named_retired_on_every_route() {
    let (_fixture, owner, root) = setup_with_relational_source(3);
    let child = create_reused_branch(&owner, &root, reuse_intent("retired-source"));
    let report =
        RuntimeWorldBranchService::retire_product_branch(&owner, child.branch_identity().clone())
            .expect("the child retires");
    assert!(report.owner_retirement_work().is_empty());
    assert_eq!(
        RuntimeWorldObservationService::observe_product_branch(&owner, child.branch_identity())
            .expect_err("a retired branch is not observable"),
        RuntimeWorldBranchAdmissionDenial::RetiredBranch
    );
    let lifecycles_before = owner_lifecycles(&owner);

    for intent in [
        reuse_intent("from-retired-reuse"),
        fork_intent(
            "branch-from-retired-fork",
            relational_fork("relational-branch-from-retired-fork"),
            signal_fork("signal-branch-from-retired-fork"),
        ),
    ] {
        let cancellation = RuntimeWorldCancellationSource::new();
        let denial = RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(child.clone(), intent, &cancellation.token()),
        )
        .expect_err("a retired source cannot be the source of a creation");
        assert_eq!(
            denial,
            RuntimeWorldBranchAdmissionDenial::RetiredBranch,
            "a source branch with no occurrence is retired, not stale, on every route"
        );
    }

    assert_eq!(owner_lifecycles(&owner), lifecycles_before);
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.custody.installed(), 0);
    assert_eq!(owner.recovery_record_count(), 0);
}
