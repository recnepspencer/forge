use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::lifecycle::{
    RuntimeWorldCancellationSource, RuntimeWorldClock, RuntimeWorldClockSource,
    RuntimeWorldCloseDenial,
};
use crate::publication::{
    CompositeAttemptProgress, CompositeComponentIntent, CompositeLateCancellationPosture,
    CompositeOwnerExecutionResults, CompositePublicationCostCounters, ProductBranchIntent,
    RelationalAttemptProgress, RuntimeWorldPublicationOutcome, SignalAttemptProgress,
};
use crate::recovery::ProductUnpublishedCause;

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> crate::lifecycle::RuntimeWorldInstant {
        crate::lifecycle::RuntimeWorldInstant::from_ticks(7)
    }
}

fn budgets() -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 4,
            history_metadata_bytes: 4096,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 1,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 2,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 2,
            retained_partial_metadata_bytes: 1,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 6,
            in_flight_pin_acquisition_reservations: 4,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("test budgets are positive")
}

fn setup() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(budgets(), RuntimeWorldClock::from_source(FixedClock)))
            .expect("managed owner construction"),
    );
    let performed = match owner.bootstrap_root(fixture.bootstrap_intent()) {
        crate::branch::RuntimeWorldBootstrapOutcome::Performed(performed) => performed,
        crate::branch::RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!("bootstrap unexpectedly denied: {:?}", no_effect.cause())
        }
    };
    (fixture, owner, performed.product_branch().clone())
}

fn ready_relational(
    fixture: &mut RealReferenceFixture,
    owner: &TestOwner,
    expected: ProductBranchObservation,
    cancelled: bool,
) -> crate::publication::CompositePublicationReady {
    let plan = crate::lifecycle::RuntimeWorldPreparationService::prepare(
        owner,
        expected.clone(),
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("publication-service")
                .expect("valid operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        ),
    )
    .expect("the observed head admits preparation");
    let cancellation = RuntimeWorldCancellationSource::new();
    let mut attempt = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &cancellation.token(),
        None,
    )
    .expect("publication capacity is reserved before owner effects");
    attempt.begin_owner_execution();
    let performed = fixture.perform_relational_owner_change();
    let successor_basis = crate::basis::admit_current(
        &owner
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        &owner.state.relational.basis_port(),
        &owner.state.signal.basis_port(),
        &owner.state.bridge,
        performed.next_basis().clone(),
        expected.basis().signal_basis().clone(),
        expected.basis().correspondence_basis().clone(),
    )
    .expect("owner-issued component results admit the successor basis");
    if cancelled {
        attempt.observe_cancellation();
    }
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::performed(performed),
        SignalAttemptProgress::untouched(),
    );
    attempt
        .settle(progress)
        .ready(successor_basis)
        .expect("the exact successor retention is available")
}

fn competing_commit(
    owner: &TestOwner,
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
    let results = CompositeOwnerExecutionResults::retained();
    Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            commit_identity,
            expected.snapshot().commit(),
            expected.basis().clone(),
            attempt_identity,
            &results,
            None,
        )
        .expect("same-basis competitor commit"),
    )
}

fn install_competing_head(
    owner: &TestOwner,
    cell: &ProductBranchReferenceCell,
    expected: &ProductBranchObservation,
) -> Arc<CompositeRuntimeWorldCommit> {
    let commit = competing_commit(owner, expected);
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
fn close_denies_reserved_attempt_and_drop_releases_all_attempt_capacity() {
    let (mut fixture, owner, expected) = setup();
    let ready = ready_relational(&mut fixture, &owner, expected, false);
    assert_eq!(owner.state.operation.active(), 1);
    assert_eq!(owner.state.publication_capacity.active(), 1);
    assert_eq!(owner.state.history.reserved_len(), 1);
    assert_eq!(owner.state.recovery.reserved_slots(), 1);
    assert_eq!(owner.close(), Err(RuntimeWorldCloseDenial::AlreadyClosing));
    assert_eq!(
        owner.lifecycle_observation(),
        crate::lifecycle::RuntimeWorldOwnerLifecycleObservation::Open
    );

    drop(ready);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(owner.state.publication_capacity.active(), 0);
    assert_eq!(owner.state.history.reserved_len(), 0);
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    owner
        .close()
        .expect("close succeeds after attempt teardown");
    assert_eq!(
        owner.lifecycle_observation(),
        crate::lifecycle::RuntimeWorldOwnerLifecycleObservation::Closed
    );
}

#[test]
fn service_dispatch_records_one_exact_final_publication() {
    let (mut fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&mut fixture, &owner, expected.clone(), false);
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    );
    let performed = match outcome {
        RuntimeWorldPublicationOutcome::Performed(performed) => performed,
        other => panic!("service publication must perform: {other:?}"),
    };
    let counters = performed.cost_counters();
    assert_eq!(counters.expected_head_rechecks(), 1);
    assert_eq!(counters.history_slots_installed(), 1);
    assert_eq!(counters.product_cell_touches(), 1);
    assert_eq!(counters.cas_attempts(), 1);
    assert_eq!(counters.cas_wins(), 1);
    assert_eq!(counters.cas_losses(), 0);
    assert_eq!(counters.cancellation_observations(), 0);
    assert_eq!(performed.old_product_head().snapshot(), expected.snapshot());
    assert_eq!(performed.new_product_head(), &cell.atomic_snapshot());
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn cancellation_after_owner_movement_retains_partial_and_fences_close() {
    let (mut fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&mut fixture, &owner, expected.clone(), true);
    let before = cell.atomic_snapshot();
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("cancelled owner effects must be retained: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    assert_eq!(cell.atomic_snapshot(), before);
    assert_eq!(owner.state.recovery.reserved_slots(), 1);
    assert_eq!(owner.state.operation.active(), 0);
    assert_eq!(owner.close(), Err(RuntimeWorldCloseDenial::AlreadyClosing));
    drop(retained);
    owner
        .close()
        .expect("close succeeds after recovery custody drops");
}

#[test]
fn cancellation_before_product_movement_does_not_cas_current_head() {
    let (mut fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&mut fixture, &owner, expected.clone(), false);
    let before = cell.atomic_snapshot();
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement,
        CompositePublicationCostCounters::default(),
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("current-head cancellation must retain owner effects: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::CancellationAfterEffect
    );
    assert_eq!(cell.atomic_snapshot(), before);
    assert_eq!(owner.state.recovery.reserved_slots(), 1);
    drop(retained);
    owner
        .close()
        .expect("close succeeds after cancellation custody drops");
}

#[test]
fn cancelled_ready_loses_to_winner_and_retains_publication_loss() {
    let (mut fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&mut fixture, &owner, expected.clone(), true);
    let loser_basis = ready.successor_basis().clone();
    let winner = install_competing_head(&owner, &cell, &expected);
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("cancelled stale publication must retain loss: {other:?}"),
    };
    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::ProductPublicationLost
    );
    assert_eq!(
        retained.last_observed_head().unwrap().selected_commit(),
        winner.identity()
    );
    assert_eq!(retained.successor_basis(), Some(&loser_basis));
    assert_ne!(retained.successor_commit(), winner.identity());
    assert_eq!(cell.atomic_snapshot().selected_commit(), winner.identity());
    assert!(owner
        .state
        .history
        .lookup(retained.successor_commit())
        .is_some());
    assert_eq!(owner.state.recovery.reserved_slots(), 1);
    assert_eq!(owner.state.operation.active(), 0);
    drop(retained);
    owner
        .close()
        .expect("close succeeds after stale cancellation custody drops");
}

#[test]
fn cancellation_observed_after_movement_is_performed_with_evidence() {
    let (mut fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational(&mut fixture, &owner, expected, false);
    let outcome = crate::lifecycle::ports::RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::RequestedAfterProductMovement,
        CompositePublicationCostCounters::default(),
    );
    let performed = match outcome {
        RuntimeWorldPublicationOutcome::Performed(performed) => performed,
        other => panic!("post-movement cancellation must preserve publication: {other:?}"),
    };
    assert_eq!(
        performed.late_cancellation(),
        CompositeLateCancellationPosture::RequestedAfterProductMovement
    );
    assert_eq!(performed.cost_counters().cancellation_observations(), 1);
}
