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
            retained_partial_metadata_bytes:
                crate::recovery::ProductUnpublishedOwnerEffects::metadata_charge_hint() as u64,
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
        )
        .with_prepared_relational_candidate(
            fixture.prepare_relational_owner_candidate("publication-service"),
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
    .expect("owner-issued component results admit the successor basis");
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::settled(commit_identity, successor_for_progress, result),
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

#[path = "publication_service_tests/outcomes.rs"]
mod outcomes;
