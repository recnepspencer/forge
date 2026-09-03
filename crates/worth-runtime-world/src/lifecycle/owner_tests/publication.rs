use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceCell,
    ProductBranchReferenceSnapshot,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::publication::{
    CompositeAttemptProgress, CompositeComponentIntent, CompositeLateCancellationPosture,
    CompositeOwnerExecutionResults, CompositePublicationCostCounters, ProductBranchIntent,
    RelationalAttemptProgress, RuntimeWorldPublicationOutcome, SignalAttemptProgress,
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
    let cancellation = crate::lifecycle::RuntimeWorldCancellationSource::new();
    let mut attempt = crate::lifecycle::RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &cancellation.token(),
        None,
    )
    .expect("the owner reserves complete publication capacity");
    attempt.begin_owner_execution();

    let performed = fixture.perform_relational_owner_change();
    let commit_identity = performed.commit_identity();
    let successor_relational = performed.next_basis().clone();
    let successor_for_progress = successor_relational.clone();
    let result = owner
        .state
        .relational
        .settlement_port()
        .settle_performed_publication(performed)
        .expect("the canonical Relational settlement completes");
    let successor_basis = crate::basis::admit_current(
        &owner
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        &owner.state.relational.basis_port(),
        &owner.state.signal.basis_port(),
        &owner.state.bridge,
        successor_relational,
        expected.basis().signal_basis().clone(),
        expected.basis().correspondence_basis().clone(),
    )
    .expect("the component owners admit the exact successor tuple");
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::settled(commit_identity, successor_for_progress, result),
        SignalAttemptProgress::untouched(),
    );
    attempt
        .settle(progress)
        .ready(successor_basis)
        .expect("real successor retention binds after owner execution")
}

fn install_competing_head(
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
    let results = CompositeOwnerExecutionResults::retained();
    let commit = Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            commit_identity,
            expected.snapshot().commit(),
            expected.basis().clone(),
            attempt_identity,
            &results,
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
    let (fixture, owner, expected) = setup();
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    let ready = ready_relational_publication(&fixture, &owner, expected.clone());
    let loser_basis = ready.successor_basis().clone();
    let winner = install_competing_head(&owner, &cell, &expected);

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
        winner.identity()
    );
    assert_eq!(retained.successor_basis(), Some(&loser_basis));
    assert_ne!(retained.successor_commit(), winner.identity());
    let retained_commit = owner
        .state
        .history
        .lookup(retained.successor_commit())
        .expect("the losing occurrence remains installed in History");
    assert_eq!(retained_commit.basis(), &loser_basis);
    assert_eq!(owner.state.history.len(), 3);
    assert_eq!(retained.owner_effect_count(), 1);
    assert_eq!(
        retained.retention_posture(),
        ProductUnpublishedRetentionPosture::RetainedExact
    );
    assert_eq!(
        retained.component_results().relational_posture(),
        crate::history::CompositeComponentChangePosture::Published
    );
    assert_eq!(cell.atomic_snapshot().selected_commit(), winner.identity());
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
