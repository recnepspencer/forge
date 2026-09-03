use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation, ProductBranchReferenceSnapshot,
};
use crate::lifecycle::{
    RuntimeWorldObservationService, RuntimeWorldOwnerExecutionService,
    RuntimeWorldPreparationService, RuntimeWorldProductPublicationService,
};
use crate::publication::{
    CompositeAttemptProgress, CompositeComponentIntent, CompositeExecutionBorrow,
    CompositeLateCancellationPosture, CompositePublicationCostCounters, OwnerExecutionOutcome,
    ProductBranchIntent, RelationalAttemptProgress, RuntimeWorldPublicationOutcome,
    SignalAttemptProgress,
};
use crate::recovery::{ProductUnpublishedCause, ProductUnpublishedRetentionPosture};

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

fn setup() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(
            super::bootstrap_budgets(),
            crate::lifecycle::RuntimeWorldClock::from_source(super::FixedClock),
        ))
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

fn setup_with_relational_source() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let (fixture, owner, expected) = setup();
    let ready = ready_relational_publication(&fixture, &owner, expected.clone());
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    match RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::zero(),
    ) {
        RuntimeWorldPublicationOutcome::Performed(_) => {}
        other => panic!("the canonical seed publishes its product head: {other:?}"),
    }
    let expected = RuntimeWorldObservationService::observe_product_branch(
        owner.as_ref(),
        &expected.branch_identity().clone(),
    )
    .expect("the owner re-observes the published source basis");
    (fixture, owner, expected)
}

fn relational_plan(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: ProductBranchObservation,
) -> crate::publication::LoweredOwnerComponentPlan {
    crate::lifecycle::RuntimeWorldPreparationService::prepare(
        owner,
        expected,
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("publication").expect("valid operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        )
        .with_prepared_relational_candidate(
            fixture.prepare_relational_owner_candidate("publication"),
        ),
    )
    .expect("the current product head admits Relational preparation")
}

fn signal_plan(
    owner: &TestOwner,
    expected: ProductBranchObservation,
) -> crate::publication::LoweredOwnerComponentPlan {
    crate::lifecycle::RuntimeWorldPreparationService::prepare(
        owner,
        expected,
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("signal-publication").expect("valid operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::signal_only(),
        ),
    )
    .expect("the current product head admits Signal preparation")
}

fn ready_relational_publication(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: ProductBranchObservation,
) -> crate::publication::CompositePublicationReady {
    let plan = relational_plan(fixture, owner, expected.clone());
    let reservation_cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let attempt = RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &reservation_cancellation.token(),
        None,
    )
    .expect("the owner reserves complete publication capacity");
    let cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let outcome = RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &cancellation.token(),
    );
    let settlement = match outcome {
        OwnerExecutionOutcome::Settled(settlement) => settlement,
        other => panic!("the production owner execution must settle: {other:?}"),
    };
    let successor = settlement
        .successor_basis()
        .cloned()
        .expect("production owner execution returns its successor basis");
    settlement
        .ready(successor)
        .expect("real successor retention binds after owner execution")
}

fn ready_relational_fork_competitor(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: &ProductBranchObservation,
) -> crate::publication::CompositePublicationReady {
    let plan = RuntimeWorldPreparationService::prepare(
        owner,
        expected.clone(),
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("canonical-relational-fork-competitor")
                .expect("valid competing operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ForkExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        )
        .with_relational_fork_input(fixture.relational_fork_input("publication-competitor", None)),
    )
    .expect("the canonical competing fork plan is prepared");
    let reservation_cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let attempt = RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &reservation_cancellation.token(),
        None,
    )
    .expect("the canonical competing attempt reserves publication capacity");
    let cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let outcome = RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &cancellation.token(),
    );
    let settlement = match outcome {
        OwnerExecutionOutcome::Settled(settlement) => settlement,
        other => panic!("the canonical competing owner execution must settle: {other:?}"),
    };
    let successor = settlement
        .successor_basis()
        .cloned()
        .expect("the competing owner execution returns its successor basis");
    let ready = settlement
        .ready(successor)
        .expect("the competing owner execution forms a ready publication");
    ready
}

fn publish_ready_competing_head(
    owner: &TestOwner,
    ready: crate::publication::CompositePublicationReady,
    expected: &ProductBranchObservation,
) -> ProductBranchObservation {
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    match RuntimeWorldProductPublicationService::publish(
        owner,
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::zero(),
    ) {
        RuntimeWorldPublicationOutcome::Performed(_) => {}
        other => panic!("the canonical competing publication must perform: {other:?}"),
    }
    RuntimeWorldObservationService::observe_product_branch(
        owner,
        &expected.branch_identity().clone(),
    )
    .expect("the owner observes the canonical competing product head")
}

#[test]
fn complete_ready_publish_path_derives_and_installs_the_successor_snapshot() {
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational_publication(&fixture, &owner, expected.clone());
    assert_eq!(owner.state.operation.active(), 1);

    let outcome = ready.publish(
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::zero(),
    );
    let performed = match outcome {
        RuntimeWorldPublicationOutcome::Performed(performed) => performed,
        other => panic!("the uncontended ready publication must perform: {other:?}"),
    };

    assert_eq!(performed.old_product_head().snapshot(), expected.snapshot());
    assert_eq!(performed.new_product_head(), &cell.atomic_snapshot());
    assert_eq!(
        performed.new_product_head().selected_commit(),
        performed.commit().identity()
    );
    assert_eq!(owner.state.operation.active(), 0);
}

#[test]
fn complete_ready_publish_path_retains_commit_results_and_custody_on_cas_loss() {
    let (fixture, owner, expected) = setup_with_relational_source();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let competing_ready = ready_relational_fork_competitor(&fixture, &owner, &expected);
    let ready = ready_relational_publication(&fixture, &owner, expected.clone());
    let loser_basis = ready.successor_basis().clone();
    let winner = publish_ready_competing_head(&owner, competing_ready, &expected);

    let outcome = ready.publish(
        &cell,
        CompositeLateCancellationPosture::RequestedBeforeProductMovement,
        CompositePublicationCostCounters::zero(),
    );
    let retained = match outcome {
        RuntimeWorldPublicationOutcome::ProductUnpublished(retained) => retained,
        other => panic!("the stale ready publication must retain owner effects: {other:?}"),
    };

    assert_eq!(
        retained.cause(),
        ProductUnpublishedCause::ProductPublicationLost
    );
    assert_eq!(
        retained.last_observed_head().unwrap().selected_commit(),
        winner.selected_commit()
    );
    assert_eq!(retained.successor_basis(), Some(&loser_basis));
    assert_ne!(retained.successor_commit(), winner.selected_commit());
    let retained_commit = owner
        .state
        .history
        .lookup(retained.successor_commit())
        .expect("the losing occurrence remains installed in History");
    assert_eq!(retained_commit.basis(), &loser_basis);
    assert_eq!(owner.state.history.len(), 4);
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(
        retained.retention_posture(),
        ProductUnpublishedRetentionPosture::RetainedExact
    );
    assert_eq!(
        retained.component_results().relational_posture(),
        crate::history::CompositeComponentChangePosture::Published
    );
    assert_eq!(
        cell.atomic_snapshot().selected_commit(),
        winner.selected_commit()
    );
    assert_eq!(owner.state.operation.active(), 0);
}

#[cfg(feature = "test-operation-control")]
#[test]
fn post_effect_retention_denial_installs_recovery_and_preserves_retry_capacity() {
    let (mut fixture, owner, expected) = setup();
    let plan = signal_plan(&owner, expected.clone());
    let cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let mut attempt = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner.as_ref(),
        plan,
        &cancellation.token(),
        None,
    )
    .expect("the owner reserves complete publication capacity");
    attempt.begin_owner_execution();

    let advanced = fixture.perform_signal_owner_change();
    let successor_signal = advanced.advanced_basis().clone();
    let successor_basis = crate::basis::admit_current(
        &owner
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        &owner.state.relational.basis_port(),
        &owner.state.signal.basis_port(),
        &owner.state.bridge,
        expected.basis().relational_basis().clone(),
        successor_signal,
        expected.basis().correspondence_basis().clone(),
    )
    .expect("the component owners admit the exact Signal successor tuple");
    fixture.inject_signal_retention_panic();
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::untouched(),
        SignalAttemptProgress::advanced(advanced),
    );

    let retained = attempt
        .settle(progress)
        .ready(successor_basis)
        .expect_err("the injected post-effect retention denial becomes recovery");

    assert_eq!(retained.cause(), ProductUnpublishedCause::OwnerLost);
    assert_eq!(
        retained.retention_posture(),
        ProductUnpublishedRetentionPosture::ReacquisitionPending
    );
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(retained.live_obligation_count(), 3);
    assert_eq!(
        retained.component_results().signal_posture(),
        crate::history::CompositeComponentChangePosture::Published
    );
    assert_eq!(owner.state.history.len(), 2);
    let retained_commit = owner
        .state
        .history
        .lookup(retained.successor_commit())
        .expect("post-effect denial installs the exact successor occurrence");
    assert_eq!(retained_commit.basis(), retained.successor_basis().unwrap());
    assert_eq!(&cell_snapshot(&owner), expected.snapshot());
    let recovery_handle = retained.recovery_handle();
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.state.retention.reserved_unique_pin_capacity(), 2);
    assert_eq!(
        owner
            .state
            .retention
            .reserved_in_flight_acquisition_capacity(),
        2
    );
    assert_eq!(owner.state.operation.active(), 0);
    drop(retained);
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.state.retention.reserved_unique_pin_capacity(), 2);
    assert_eq!(
        owner
            .state
            .retention
            .reserved_in_flight_acquisition_capacity(),
        2
    );
    assert!(owner.cleanup_recovery_handle(&recovery_handle));
    assert_eq!(owner.recovery_record_count(), 0);
    assert_eq!(owner.state.recovery.reserved_slots(), 0);
    assert_eq!(owner.state.retention.reserved_unique_pin_capacity(), 0);
    assert_eq!(
        owner
            .state
            .retention
            .reserved_in_flight_acquisition_capacity(),
        0
    );
}
fn cell_snapshot(owner: &TestOwner) -> ProductBranchReferenceSnapshot {
    owner
        .state
        .branches
        .root_cell()
        .expect("bootstrapped cell")
        .atomic_snapshot()
}
