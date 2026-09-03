use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchName, ProductBranchObservation, RuntimeWorldBootstrapOutcome,
    RuntimeWorldBranchAdmissionDenial,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::lifecycle::{
    RuntimeWorldBranchCreationOutcome, RuntimeWorldBranchCreationRequest,
    RuntimeWorldBranchService, RuntimeWorldClock, RuntimeWorldClockSource, RuntimeWorldInstant,
};
use crate::publication::{CompositeComponentIntent, CompositeExecutionBorrow, ProductBranchIntent};

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> RuntimeWorldInstant {
        RuntimeWorldInstant::from_ticks(7)
    }
}

fn budgets() -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 3,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 8,
            history_metadata_bytes: 4096,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 8,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 4,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 2,
            retained_partial_metadata_bytes: 4096,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 8,
            in_flight_pin_acquisition_reservations: 8,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 2,
        },
    })
    .expect("branch service contract budgets")
}

fn reuse_intent(name: &str) -> ProductBranchIntent {
    ProductBranchIntent::new(
        ProductBranchCreationIntent::named(name).expect("valid product branch name"),
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ReuseExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::signal_only(),
    )
}

fn setup() -> (
    crate::branch::reference_test_fixture::RealReferenceFixture,
    TestOwner,
    ProductBranchObservation,
) {
    let mut fixture = crate::branch::reference_test_fixture::real_fixture(8, 8);
    let owner =
        TestOwner::new(fixture.owner_inputs(budgets(), RuntimeWorldClock::from_source(FixedClock)))
            .expect("managed branch owner");
    let root = match owner.bootstrap_root(fixture.bootstrap_intent()) {
        RuntimeWorldBootstrapOutcome::Performed(performed) => performed.product_branch().clone(),
        RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!(
                "branch bootstrap unexpectedly denied: {:?}",
                no_effect.cause()
            )
        }
    };
    (fixture, owner, root)
}

fn create_reused_branch(
    owner: &TestOwner,
    source: &ProductBranchObservation,
    intent: ProductBranchIntent,
) -> ProductBranchObservation {
    match RuntimeWorldBranchService::create_product_branch(
        owner,
        RuntimeWorldBranchCreationRequest::new(
            source.clone(),
            intent,
            CompositeExecutionBorrow::without_signal(),
        ),
    )
    .expect("exact reuse is admitted")
    {
        RuntimeWorldBranchCreationOutcome::Performed(observation) => observation,
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(_) => {
            panic!("exact reuse did not produce a branch observation")
        }
    }
}

#[test]
fn installed_duplicate_name_maps_to_the_exact_branch_denial() {
    let (_fixture, owner, root) = setup();
    create_reused_branch(&owner, &root, reuse_intent("installed-duplicate"));
    let before = owner.state.branches.branch_count();

    assert!(matches!(
        RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(
                root.clone(),
                reuse_intent("installed-duplicate"),
                CompositeExecutionBorrow::without_signal(),
            ),
        ),
        Err(RuntimeWorldBranchAdmissionDenial::DuplicateName)
    ));
    assert_eq!(owner.state.branches.branch_count(), before);
}

#[test]
fn held_duplicate_name_maps_to_the_same_denial_and_drop_releases_it() {
    let (_fixture, owner, root) = setup();
    let held = owner
        .state
        .branches
        .reserve_branch(
            owner.owner_identity(),
            ProductBranchName::try_new("held-duplicate").expect("valid held name"),
        )
        .expect("the registry holds the named slot");

    assert!(matches!(
        RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(
                root.clone(),
                reuse_intent("held-duplicate"),
                CompositeExecutionBorrow::without_signal(),
            ),
        ),
        Err(RuntimeWorldBranchAdmissionDenial::DuplicateName)
    ));
    drop(held);

    create_reused_branch(&owner, &root, reuse_intent("held-duplicate"));
}
