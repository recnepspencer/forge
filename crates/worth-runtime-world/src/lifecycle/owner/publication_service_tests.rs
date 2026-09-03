use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::lifecycle::{
    RuntimeWorldCancellationSource, RuntimeWorldClock, RuntimeWorldClockSource,
    RuntimeWorldCloseDenial,
};
use crate::publication::{
    CompositeAttemptProgress, CompositeComponentIntent, CompositeLateCancellationPosture,
    CompositePublicationCostCounters, ProductBranchIntent, RelationalAttemptProgress,
    RuntimeWorldPublicationOutcome, SignalAttemptProgress,
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
