use super::super::*;

use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::history::CompositeCommitParent;
use crate::lifecycle::{
    RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchCreationRequest,
    RuntimeWorldBranchService, RuntimeWorldCancellationSource, RuntimeWorldClock,
    RuntimeWorldObservationService, RuntimeWorldOwnerExecutionService,
    RuntimeWorldPreparationService, RuntimeWorldProductPublicationService,
};
use crate::publication::{
    CompositeComponentIntent, CompositeExecutionBorrow, CompositeLateCancellationPosture,
    CompositePublicationCostCounters, OwnerExecutionOutcome, ProductBranchIntent,
    RelationalForkPlanInput,
};
use crate::recovery::ProductUnpublishedCause;
use worth_relational::facade::mvcc::RelationalTransactionIntent;
use worth_relational::facade::transactions::WorkerIntentBatch;
use worth_signal::facade::branch::{validate_signal_branch_name, SignalOwnerCancellationSource};

type TestOwner = super::TestOwner;

fn fork_budgets(live_branches: u64) -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: live_branches,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 12,
            history_metadata_bytes: 4096,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 8,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 4,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 4,
            retained_partial_metadata_bytes:
                crate::recovery::ProductUnpublishedOwnerEffects::metadata_charge_hint()
                    .saturating_mul(4) as u64,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 12,
            in_flight_pin_acquisition_reservations: 12,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 4,
        },
    })
    .expect("fork creation budgets")
}

fn setup_with_relational_source(
    live_branches: u64,
) -> (
    crate::branch::reference_test_fixture::RealReferenceFixture,
    TestOwner,
    ProductBranchObservation,
) {
    let mut fixture = crate::branch::reference_test_fixture::real_fixture(12, 12);
    let owner = TestOwner::new(fixture.owner_inputs(
        fork_budgets(live_branches),
        RuntimeWorldClock::from_source(super::FixedClock),
    ))
    .expect("managed branch owner");
    let initial = bootstrap_root(&owner, &mut fixture);
    seed_relational_source(&owner, &mut fixture, initial);
    let source = current_root_observation(&owner);
    (fixture, owner, source)
}

fn bootstrap_root(
    owner: &TestOwner,
    fixture: &mut crate::branch::reference_test_fixture::RealReferenceFixture,
) -> ProductBranchObservation {
    match owner.bootstrap_root(fixture.bootstrap_intent()) {
        crate::branch::RuntimeWorldBootstrapOutcome::Performed(performed) => {
            performed.product_branch().clone()
        }
        crate::branch::RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!(
                "branch bootstrap unexpectedly denied: {:?}",
                no_effect.cause()
            )
        }
    }
}

fn seed_relational_source(
    owner: &TestOwner,
    fixture: &mut crate::branch::reference_test_fixture::RealReferenceFixture,
    initial: ProductBranchObservation,
) {
    let seed_intent = ProductBranchIntent::new(
        ProductBranchCreationIntent::named("branch-fork-source-seed")
            .expect("valid seed operation name"),
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
    )
    .with_prepared_relational_candidate(
        fixture.prepare_relational_owner_candidate("branch-fork-source-seed"),
    );
    let plan = RuntimeWorldPreparationService::prepare(owner, initial.clone(), seed_intent)
        .expect("the source seed plan is admitted");
    let cancellation_source = RuntimeWorldCancellationSource::new();
    let cancellation = cancellation_source.token();
    let attempt = RuntimeWorldPreparationService::reserve(owner, plan, &cancellation, None)
        .expect("the source seed reserves its bounded publication resources");
    let settlement = match RuntimeWorldOwnerExecutionService::execute(
        owner,
        attempt,
        CompositeExecutionBorrow::without_signal(),
        &cancellation,
    ) {
        OwnerExecutionOutcome::Settled(settlement) => settlement,
        other => panic!("source seed must settle: {other:?}"),
    };
    let successor = settlement
        .successor_basis()
        .cloned()
        .expect("source seed successor basis");
    let ready = settlement
        .ready(successor)
        .expect("source seed owner result is publication-ready");
    let cell = owner
        .state
        .branches
        .root_cell()
        .expect("bootstrapped root cell");
    match RuntimeWorldProductPublicationService::publish(
        owner,
        ready,
        &cell,
        CompositeLateCancellationPosture::NotRequested,
        CompositePublicationCostCounters::default(),
    ) {
        crate::publication::RuntimeWorldPublicationOutcome::Performed(_) => {}
        other => panic!("source seed must publish: {other:?}"),
    }
    drop(initial);
}

fn current_root_observation(owner: &TestOwner) -> ProductBranchObservation {
    let cell = owner
        .state
        .branches
        .root_cell()
        .expect("bootstrapped root cell");
    RuntimeWorldObservationService::observe_product_branch(
        owner,
        &cell.atomic_snapshot().branch_identity().clone(),
    )
    .expect("the seeded root remains observable")
}

struct ForkIntentSpec<'a> {
    name: &'a str,
    relational_posture: ProductBranchComponentPosture,
    signal_posture: ProductBranchComponentPosture,
    relational_input: RelationalForkPlanInput,
    signal_name: &'a str,
}

fn fork_intent(spec: ForkIntentSpec<'_>) -> ProductBranchIntent {
    ProductBranchIntent::new(
        ProductBranchCreationIntent::named(spec.name).expect("valid product branch name"),
        ProductBranchComponentPostures::new(spec.relational_posture, spec.signal_posture),
        CompositeComponentIntent::relational_and_signal(RelationalTransactionIntent::ordinary()),
    )
    .with_relational_fork_input(spec.relational_input)
    .with_signal_fork_name(
        validate_signal_branch_name(spec.signal_name).expect("valid Signal name"),
    )
}

fn create_forked_branch(
    owner: &TestOwner,
    source: &ProductBranchObservation,
    intent: ProductBranchIntent,
    signal: CompositeExecutionBorrow<'_, (), (), (), (), ()>,
) -> ProductBranchObservation {
    match RuntimeWorldBranchService::create_product_branch(
        owner,
        RuntimeWorldBranchCreationRequest::new(source.clone(), intent, signal),
    )
    .expect("owner-issued fork reaches a performed branch outcome")
    {
        RuntimeWorldBranchCreationOutcome::Performed(observation) => observation,
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(_) => {
            panic!("fully admitted fork unexpectedly became unpublished")
        }
    }
}

fn assert_new_branch_observation(
    owner: &TestOwner,
    source: &ProductBranchObservation,
    child: &ProductBranchObservation,
    history_before: usize,
) {
    assert_ne!(child.branch_identity(), source.branch_identity());
    assert_ne!(
        child.lifecycle_incarnation(),
        source.lifecycle_incarnation()
    );
    assert_eq!(child.reference_generation().get(), 0);
    assert_ne!(child.selected_commit(), source.selected_commit());
    assert_ne!(child.basis(), source.basis());
    let observed =
        RuntimeWorldObservationService::observe_product_branch(owner, child.branch_identity())
            .expect("the new branch observation is live");
    assert_eq!(observed, *child);
    let source_after =
        RuntimeWorldObservationService::observe_product_branch(owner, source.branch_identity())
            .expect("the source remains live");
    assert_eq!(source_after, *source);
    let commit = owner
        .state
        .history
        .lookup(child.selected_commit())
        .expect("the destination occurrence is installed in history");
    match commit.parent() {
        CompositeCommitParent::Ordinary(parent) => {
            assert_eq!(parent.commit(), source.selected_commit())
        }
        CompositeCommitParent::Root => panic!("a fork destination must have an ordinary parent"),
    }
    assert_eq!(owner.state.history.len(), history_before + 1);
    assert_eq!(owner.state.branches.branch_count(), 2);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
}

fn create_partial_effects(
    owner: &TestOwner,
    source: &ProductBranchObservation,
    intent: ProductBranchIntent,
) -> crate::recovery::ProductUnpublishedOwnerEffects {
    let outcome = RuntimeWorldBranchService::create_product_branch(
        owner,
        RuntimeWorldBranchCreationRequest::new(
            source.clone(),
            intent,
            CompositeExecutionBorrow::without_signal(),
        ),
    )
    .expect("the later sibling denial is a product-unpublished outcome");
    match outcome {
        RuntimeWorldBranchCreationOutcome::Performed(_) => {
            panic!("Signal ForkAndAdvance without its borrow must deny")
        }
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(effects) => effects,
    }
}

#[test]
fn fork_exact_creates_a_distinct_composite_branch_after_both_owner_forks() {
    let (fixture, owner, source) = setup_with_relational_source(3);
    let history_before = owner.state.history.len();
    let costs_before = owner.state.retention.cost_snapshot();
    let intent = fork_intent(ForkIntentSpec {
        name: "branch-fork-exact",
        relational_posture: ProductBranchComponentPosture::ForkExact,
        signal_posture: ProductBranchComponentPosture::ForkExact,
        relational_input: fixture.relational_fork_input("relational-branch-fork-exact", None),
        signal_name: "signal-branch-fork-exact",
    });
    let child = create_forked_branch(
        &owner,
        &source,
        intent,
        CompositeExecutionBorrow::without_signal(),
    );
    assert_new_branch_observation(&owner, &source, &child, history_before);
    assert_ne!(
        child.basis().relational_basis().identity(),
        source.basis().relational_basis().identity()
    );
    assert_ne!(
        child.basis().signal_basis().admission_identity(),
        source.basis().signal_basis().admission_identity()
    );
    let costs_after = owner.state.retention.cost_snapshot();
    assert!(costs_after.relational_contacts() > costs_before.relational_contacts());
    assert!(costs_after.signal_contacts() > costs_before.signal_contacts());
}

#[test]
fn fork_and_advance_creates_a_distinct_composite_branch_after_ordered_owner_work() {
    let (fixture, owner, source) = setup_with_relational_source(3);
    let history_before = owner.state.history.len();
    let costs_before = owner.state.retention.cost_snapshot();
    let intent = fork_intent(ForkIntentSpec {
        name: "branch-fork-and-advance",
        relational_posture: ProductBranchComponentPosture::ForkAndAdvance,
        signal_posture: ProductBranchComponentPosture::ForkAndAdvance,
        relational_input: fixture.relational_fork_input(
            "relational-branch-fork-and-advance",
            Some(WorkerIntentBatch::new("branch-fork-and-advance")),
        ),
        signal_name: "signal-branch-fork-and-advance",
    });
    let signal_cancellation = SignalOwnerCancellationSource::new();
    let signal_token = signal_cancellation.token();
    let mut context = ();
    let child = create_forked_branch(
        &owner,
        &source,
        intent,
        CompositeExecutionBorrow::signal(&mut context, &signal_token, |_| Ok(())),
    );
    assert_new_branch_observation(&owner, &source, &child, history_before);
    assert_ne!(
        child.basis().relational_basis().identity(),
        source.basis().relational_basis().identity()
    );
    assert_ne!(
        child.basis().signal_basis().admission_identity(),
        source.basis().signal_basis().admission_identity()
    );
    let costs_after = owner.state.retention.cost_snapshot();
    assert!(costs_after.relational_contacts() > costs_before.relational_contacts());
    assert!(costs_after.signal_contacts() > costs_before.signal_contacts());
}

#[test]
fn forked_relational_effect_is_retained_when_later_signal_sibling_denies() {
    let (fixture, owner, source) = setup_with_relational_source(3);
    let history_before = owner.state.history.len();
    let custody_before = owner.state.retention.active_component_obligation_count();
    let input = fixture.relational_fork_input("relational-branch-partial", None);
    let intent = fork_intent(ForkIntentSpec {
        name: "branch-fork-partial",
        relational_posture: ProductBranchComponentPosture::ForkExact,
        signal_posture: ProductBranchComponentPosture::ForkAndAdvance,
        relational_input: input,
        signal_name: "signal-branch-partial",
    });
    let effects = create_partial_effects(&owner, &source, intent);
    assert_eq!(effects.cause(), ProductUnpublishedCause::SiblingOwnerDenied);
    assert_eq!(effects.owner_effect_count(), 1);
    assert_eq!(
        effects.progress().relational_posture(),
        crate::publication::RelationalAttemptProgressPosture::Performed
    );
    assert_eq!(
        effects.progress().signal_posture(),
        crate::publication::SignalAttemptProgressPosture::Untouched
    );
    assert_ne!(effects.successor_commit(), source.selected_commit());
    assert_ne!(
        effects
            .successor_basis()
            .expect("retained fork successor basis")
            .relational_basis()
            .identity(),
        source.basis().relational_basis().identity()
    );
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.history.len(), history_before + 1);
    assert_eq!(owner.recovery_record_count(), 1);
    assert!(
        owner.state.retention.active_component_obligation_count() > custody_before,
        "partial owner effects retain bounded component custody"
    );
    let handle = effects.recovery_handle();
    assert!(owner.inspect_recovery(&handle).is_some());
    assert!(owner.cleanup_recovery(effects));
    assert_eq!(owner.recovery_record_count(), 0);
    assert_eq!(
        owner.state.retention.active_component_obligation_count(),
        custody_before
    );
    let source_after =
        RuntimeWorldObservationService::observe_product_branch(&owner, source.branch_identity())
            .expect("source remains observable after recovery cleanup");
    assert_eq!(source_after, source);
    assert_eq!(owner.state.branches.branch_count(), 1);
}
