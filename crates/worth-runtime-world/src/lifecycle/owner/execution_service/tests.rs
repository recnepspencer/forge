use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;
use worth_relational::facade::transactions::WorkerIntentBatch;
use worth_signal::facade::branch::{validate_signal_branch_name, SignalOwnerCancellationSource};

use crate::branch::reference_test_fixture::{self, RealReferenceFixture};
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation, RuntimeWorldBootstrapOutcome,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::lifecycle::{
    RuntimeWorldCancellationSource, RuntimeWorldClock, RuntimeWorldClockSource,
    RuntimeWorldObservationService, RuntimeWorldOwnerExecutionService,
    RuntimeWorldPreparationService, RuntimeWorldProductPublicationService,
};
use crate::publication::{
    CompositeComponentIntent, CompositeExecutionBorrow, CompositeLateCancellationPosture,
    CompositePublicationCostCounters, LoweredOwnerComponentPlan, OwnerExecutionOutcome,
    OwnerExecutionSettlement, ProductBranchIntent, RelationalAttemptProgressPosture,
    RelationalForkPlanInput, SignalAttemptProgressPosture,
};
use crate::recovery::ProductUnpublishedCause;

#[path = "tests/boundaries.rs"]
mod boundaries;
#[path = "tests/failures.rs"]
mod failures;
#[path = "tests/planning.rs"]
mod planning;

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

#[derive(Clone, Copy)]
struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> crate::lifecycle::RuntimeWorldInstant {
        crate::lifecycle::RuntimeWorldInstant::from_ticks(0)
    }
}

#[derive(Clone)]
struct MutableClock {
    ticks: Arc<AtomicU64>,
}

impl MutableClock {
    fn new(ticks: u64) -> Self {
        Self {
            ticks: Arc::new(AtomicU64::new(ticks)),
        }
    }

    fn set(&self, ticks: u64) {
        self.ticks.store(ticks, Ordering::Release);
    }
}

impl RuntimeWorldClockSource for MutableClock {
    fn now(&self) -> crate::lifecycle::RuntimeWorldInstant {
        crate::lifecycle::RuntimeWorldInstant::from_ticks(self.ticks.load(Ordering::Acquire))
    }
}

fn budgets(publication_attempts: u64) -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 12,
            history_metadata_bytes: 4096,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 4,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: publication_attempts,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 4,
            retained_partial_metadata_bytes:
                crate::recovery::ProductUnpublishedOwnerEffects::metadata_charge_hint()
                    .saturating_mul(publication_attempts as usize) as u64,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 12,
            in_flight_pin_acquisition_reservations: 12,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 4,
        },
    })
    .expect("focused execution budgets are positive")
}

fn setup() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    setup_with_clock(RuntimeWorldClock::from_source(FixedClock))
}

fn setup_with_relational_source() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let (fixture, owner, expected) = setup();
    let warmup = plan(
        &fixture,
        &owner,
        &expected,
        "relational-fork-source-seed",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        None,
    );
    let warmup = settled(execute_without_signal(&owner, reserve(&owner, warmup)));
    let successor = warmup
        .successor_basis()
        .cloned()
        .expect("the canonical seed carries its successor basis");
    let ready = warmup
        .ready(successor)
        .expect("the canonical seed forms a product publication");
    let cell = owner.state.branches.root_cell().expect("bootstrapped cell");
    match RuntimeWorldProductPublicationService::publish(
        owner.as_ref(),
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    ) {
        crate::publication::RuntimeWorldPublicationOutcome::Performed(_) => {}
        other => panic!("the canonical seed publishes its product head: {other:?}"),
    }
    let expected = RuntimeWorldObservationService::observe_product_branch(
        owner.as_ref(),
        &expected.branch_identity().clone(),
    )
    .expect("the owner re-observes the published source basis");
    (fixture, owner, expected)
}

fn setup_with_clock(
    clock: RuntimeWorldClock,
) -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let mut fixture = reference_test_fixture::real_fixture(12, 12);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(budgets(4), clock))
            .expect("managed owner construction"),
    );
    let performed = match owner.bootstrap_root(fixture.bootstrap_intent()) {
        RuntimeWorldBootstrapOutcome::Performed(performed) => performed,
        RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!("bootstrap unexpectedly denied: {:?}", no_effect.cause())
        }
    };
    (fixture, owner, performed.product_branch().clone())
}

fn plan(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: &ProductBranchObservation,
    operation_name: &str,
    postures: ProductBranchComponentPostures,
    component_intent: CompositeComponentIntent,
    signal_name: Option<&str>,
) -> LoweredOwnerComponentPlan {
    let changes_relational = component_intent.changes_relational();
    let mut intent = ProductBranchIntent::new(
        ProductBranchCreationIntent::named(operation_name).expect("valid operation name"),
        postures,
        component_intent,
    );
    if changes_relational {
        intent = intent.with_prepared_relational_candidate(
            fixture.prepare_relational_owner_candidate(operation_name),
        );
    }
    if let Some(signal_name) = signal_name {
        intent = intent.with_signal_fork_name(
            validate_signal_branch_name(signal_name).expect("valid Signal branch name"),
        );
    }
    RuntimeWorldPreparationService::prepare(owner, expected.clone(), intent)
        .expect("the exact observed head admits the requested owner plan")
}

fn plan_with_relational_fork(
    owner: &TestOwner,
    expected: &ProductBranchObservation,
    operation_name: &str,
    postures: ProductBranchComponentPostures,
    component_intent: CompositeComponentIntent,
    fork_input: RelationalForkPlanInput,
    signal_name: Option<&str>,
) -> LoweredOwnerComponentPlan {
    let mut intent = ProductBranchIntent::new(
        ProductBranchCreationIntent::named(operation_name).expect("valid operation name"),
        postures,
        component_intent,
    )
    .with_relational_fork_input(fork_input);
    if let Some(signal_name) = signal_name {
        intent = intent.with_signal_fork_name(
            validate_signal_branch_name(signal_name).expect("valid Signal branch name"),
        );
    }
    RuntimeWorldPreparationService::prepare(owner, expected.clone(), intent)
        .expect("the exact observed head admits the requested owner plan")
}

fn ready_relational_fork_competitor(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
    expected: &ProductBranchObservation,
    target: &str,
) -> crate::publication::CompositePublicationReady {
    let input = fixture.relational_fork_input(target, None);
    let plan = plan_with_relational_fork(
        owner,
        expected,
        "canonical-relational-fork-competitor",
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ForkExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        input,
        None,
    );
    let settlement = settled(execute_without_signal(owner, reserve(owner, plan)));
    let successor = settlement
        .successor_basis()
        .cloned()
        .expect("the canonical fork competitor returns its successor basis");
    settlement
        .ready(successor)
        .expect("the canonical fork competitor forms a ready publication")
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
        CompositePublicationCostCounters::default(),
    ) {
        crate::publication::RuntimeWorldPublicationOutcome::Performed(_) => {}
        other => panic!("the canonical competing publication must perform: {other:?}"),
    }
    RuntimeWorldObservationService::observe_product_branch(
        owner,
        &expected.branch_identity().clone(),
    )
    .expect("the owner observes the canonical competing product head")
}

fn reserve(
    owner: &TestOwner,
    plan: LoweredOwnerComponentPlan,
) -> crate::publication::ReservedCompositePublicationAttempt {
    reserve_with_deadline(owner, plan, None).expect("the complete attempt reservation admits")
}

fn reserve_with_deadline(
    owner: &TestOwner,
    plan: LoweredOwnerComponentPlan,
    deadline: Option<crate::lifecycle::RuntimeWorldInstant>,
) -> Result<
    crate::publication::ReservedCompositePublicationAttempt,
    crate::publication::NoEffectCompositePublication,
> {
    let cancellation = RuntimeWorldCancellationSource::new();
    RuntimeWorldPreparationService::reserve(owner, plan, &cancellation.token(), deadline)
}

fn execute_without_signal(
    owner: &TestOwner,
    attempt: crate::publication::ReservedCompositePublicationAttempt,
) -> OwnerExecutionOutcome {
    let cancellation = RuntimeWorldCancellationSource::new();
    RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &cancellation.token(),
    )
}

fn execute_with_empty_signal(
    owner: &TestOwner,
    attempt: crate::publication::ReservedCompositePublicationAttempt,
) -> OwnerExecutionOutcome {
    let runtime_cancellation = RuntimeWorldCancellationSource::new();
    let runtime_token = runtime_cancellation.token();
    let signal_cancellation = SignalOwnerCancellationSource::new();
    let signal_token = signal_cancellation.token();
    let mut context = ();
    RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::signal(&mut context, &signal_token, |_| Ok(())),
        &runtime_token,
    )
}

fn settled(outcome: OwnerExecutionOutcome) -> OwnerExecutionSettlement {
    match outcome {
        OwnerExecutionOutcome::Settled(settlement) => settlement,
        other => panic!("owner execution must settle: {other:?}"),
    }
}
