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
    RuntimeWorldCloseDenial, RuntimeWorldObservationService, RuntimeWorldOwnerExecutionService,
    RuntimeWorldPreparationService, RuntimeWorldProductPublicationService,
};
use crate::publication::{
    CompositeComponentIntent, CompositeExecutionBorrow, CompositeLateCancellationPosture,
    CompositePublicationCostCounters, OwnerExecutionOutcome, ProductBranchIntent,
    RuntimeWorldPublicationOutcome,
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
                crate::recovery::ProductUnpublishedOwnerEffects::metadata_charge_hint()
                    .saturating_mul(2) as u64,
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

fn setup_with_relational_source() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let (fixture, owner, expected) = setup();
    let ready = ready_relational(&fixture, &owner, expected.clone());
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

fn ready_relational(
    fixture: &RealReferenceFixture,
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
    let reservation_cancellation = RuntimeWorldCancellationSource::new();
    let attempt = RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &reservation_cancellation.token(),
        None,
    )
    .expect("publication capacity is reserved before owner effects");
    let execution_cancellation = RuntimeWorldCancellationSource::new();
    let outcome = RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &execution_cancellation.token(),
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
        .expect("the exact successor retention is available")
}

fn ready_relational_fork_competitor(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: &ProductBranchObservation,
) -> crate::publication::CompositePublicationReady {
    let input = fixture.relational_fork_input("publication-service-competitor", None);
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
        .with_relational_fork_input(input),
    )
    .expect("the canonical competing fork plan is prepared");
    let reservation_cancellation = RuntimeWorldCancellationSource::new();
    let attempt = RuntimeWorldPreparationService::reserve(
        owner,
        plan,
        &reservation_cancellation.token(),
        None,
    )
    .expect("the canonical competing attempt reserves publication capacity");
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let outcome = RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &runtime_cancellation.token(),
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

#[path = "publication_service_tests/outcomes.rs"]
mod outcomes;
