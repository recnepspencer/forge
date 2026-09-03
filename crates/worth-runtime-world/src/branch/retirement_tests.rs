use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchObservation, RuntimeWorldBootstrapOutcome, RuntimeWorldBranchAdmissionDenial,
    RuntimeWorldBranchRetirementDenial,
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
    RuntimeWorldObservationService, RuntimeWorldOwnerRoot,
};
use crate::publication::{CompositeComponentIntent, CompositeExecutionBorrow, ProductBranchIntent};
use worth_relational::facade::mvcc::RelationalTransactionIntent;

struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> RuntimeWorldInstant {
        RuntimeWorldInstant::from_ticks(7)
    }
}

type TestOwner = RuntimeWorldOwnerRoot<(), (), (), (), ()>;

fn budgets(live_branches: u64) -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: live_branches,
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
    .expect("branch test budgets")
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

fn fork_intent(name: &str) -> ProductBranchIntent {
    ProductBranchIntent::new(
        ProductBranchCreationIntent::named(name).expect("valid product branch name"),
        ProductBranchComponentPostures::new(
            ProductBranchComponentPosture::ForkExact,
            ProductBranchComponentPosture::ReuseExact,
        ),
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
    )
}

fn setup(
    live_branches: u64,
) -> (
    crate::branch::reference_test_fixture::RealReferenceFixture,
    TestOwner,
    ProductBranchObservation,
) {
    let mut fixture = crate::branch::reference_test_fixture::real_fixture(8, 8);
    let owner = TestOwner::new(fixture.owner_inputs(
        budgets(live_branches),
        RuntimeWorldClock::from_source(FixedClock),
    ))
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
fn reuse_selects_the_exact_commit_with_fresh_branch_and_lifecycle_identities() {
    let (_fixture, owner, root) = setup(3);
    let before = owner.state.retention.cost_snapshot();
    let child = create_reused_branch(&owner, &root, reuse_intent("child"));
    let after = owner.state.retention.cost_snapshot();

    assert_ne!(child.branch_identity(), root.branch_identity());
    assert_ne!(child.lifecycle_incarnation(), root.lifecycle_incarnation());
    assert_eq!(child.reference_generation().get(), 0);
    assert_eq!(child.selected_commit(), root.selected_commit());
    assert_eq!(child.basis(), root.basis());
    assert_eq!(owner.state.history.len(), 1);
    assert_eq!(owner.state.branches.branch_count(), 2);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(
        before.owner_acquisition_contacts(),
        after.owner_acquisition_contacts()
    );
    assert_eq!(before.relational_contacts(), after.relational_contacts());
    assert_eq!(before.signal_contacts(), after.signal_contacts());
}

#[test]
fn observation_issues_the_current_exact_branch_image_and_retirement_denies_new_reads() {
    let (_fixture, owner, root) = setup(3);
    let child = create_reused_branch(&owner, &root, reuse_intent("child"));
    let observed =
        RuntimeWorldObservationService::observe_product_branch(&owner, child.branch_identity())
            .expect("live child observation");
    assert_eq!(observed, child);

    RuntimeWorldBranchService::retire_product_branch(&owner, child.branch_identity().clone())
        .expect("product retirement removes only the product reference");
    assert!(matches!(
        RuntimeWorldObservationService::observe_product_branch(&owner, child.branch_identity()),
        Err(RuntimeWorldBranchAdmissionDenial::RetiredBranch)
    ));
    assert!(
        RuntimeWorldObservationService::observe_product_branch(&owner, root.branch_identity())
            .is_ok()
    );
}

#[test]
fn retirement_releases_product_capacity_without_releasing_live_observation_custody() {
    let (_fixture, owner, root) = setup(3);
    let child = create_reused_branch(&owner, &root, reuse_intent("reusable"));
    let child_id = child.branch_identity().clone();
    let child_lifecycle = child.lifecycle_incarnation();
    let before_retirement = owner.state.retention.active_component_obligation_count();
    let relational_lifecycle = owner
        .state
        .relational
        .lifecycle_port()
        .owner_lifecycle_observation();
    let signal_lifecycle = owner
        .state
        .signal
        .lifecycle_port()
        .owner_lifecycle_observation();

    RuntimeWorldBranchService::retire_product_branch(&owner, child_id.clone())
        .expect("retire child");
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(
        owner.state.retention.active_component_obligation_count(),
        before_retirement - 2
    );
    assert_eq!(
        owner
            .state
            .relational
            .lifecycle_port()
            .owner_lifecycle_observation(),
        relational_lifecycle
    );
    assert_eq!(
        owner
            .state
            .signal
            .lifecycle_port()
            .owner_lifecycle_observation(),
        signal_lifecycle
    );
    assert!(matches!(
        RuntimeWorldBranchService::retire_product_branch(&owner, child_id.clone()),
        Err(RuntimeWorldBranchRetirementDenial::AlreadyRetired)
    ));

    drop(child);
    assert_eq!(owner.state.retention.active_component_obligation_count(), 4);
    let replacement = create_reused_branch(&owner, &root, reuse_intent("reusable"));
    assert_ne!(replacement.branch_identity(), root.branch_identity());
    assert_ne!(replacement.branch_identity(), &child_id);
    assert_ne!(replacement.lifecycle_incarnation(), child_lifecycle);
    RuntimeWorldBranchService::retire_product_branch(&owner, replacement.branch_identity().clone())
        .expect("replacement retirement");
    drop(replacement);
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.retention.active_component_obligation_count(), 4);
}

#[test]
fn retired_identity_high_water_classifies_evicted_history_as_already_retired() {
    let (_fixture, owner, root) = setup(2);
    let first = create_reused_branch(&owner, &root, reuse_intent("first"));
    let first_id = first.branch_identity().clone();
    RuntimeWorldBranchService::retire_product_branch(&owner, first_id.clone())
        .expect("first retirement");
    drop(first);

    let second = create_reused_branch(&owner, &root, reuse_intent("second"));
    let second_id = second.branch_identity().clone();
    RuntimeWorldBranchService::retire_product_branch(&owner, second_id).expect("second retirement");
    drop(second);

    let third = create_reused_branch(&owner, &root, reuse_intent("third"));
    let third_id = third.branch_identity().clone();
    RuntimeWorldBranchService::retire_product_branch(&owner, third_id).expect("third retirement");
    drop(third);

    assert!(matches!(
        RuntimeWorldBranchService::retire_product_branch(&owner, first_id),
        Err(RuntimeWorldBranchRetirementDenial::AlreadyRetired)
    ));
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
}

#[test]
fn branch_capacity_denies_before_identity_or_retention_work_for_fork_input() {
    let (_fixture, owner, root) = setup(1);
    let before = owner.state.retention.cost_snapshot();
    assert!(matches!(
        RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(
                root.clone(),
                reuse_intent("full"),
                CompositeExecutionBorrow::without_signal(),
            ),
        ),
        Err(RuntimeWorldBranchAdmissionDenial::CapacityExhausted)
    ));
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.retention.cost_snapshot(), before);

    let relational_lifecycle = owner
        .state
        .relational
        .lifecycle_port()
        .owner_lifecycle_observation();
    let signal_lifecycle = owner
        .state
        .signal
        .lifecycle_port()
        .owner_lifecycle_observation();
    assert!(matches!(
        RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(
                root.clone(),
                fork_intent("capacity-fork"),
                CompositeExecutionBorrow::without_signal(),
            ),
        ),
        Err(RuntimeWorldBranchAdmissionDenial::CapacityExhausted)
    ));
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.retention.cost_snapshot(), before);
    assert_eq!(
        owner
            .state
            .relational
            .lifecycle_port()
            .owner_lifecycle_observation(),
        relational_lifecycle
    );
    assert_eq!(
        owner
            .state
            .signal
            .lifecycle_port()
            .owner_lifecycle_observation(),
        signal_lifecycle
    );
}

#[test]
fn foreign_basis_is_rejected_before_branch_reservation() {
    let (_fixture, owner, _root) = setup(3);
    let mut foreign_fixture = crate::branch::reference_test_fixture::real_fixture(8, 8);
    let foreign_owner = TestOwner::new(
        foreign_fixture.owner_inputs(budgets(3), RuntimeWorldClock::from_source(FixedClock)),
    )
    .expect("foreign managed owner");
    let foreign_root = match foreign_owner.bootstrap_root(foreign_fixture.bootstrap_intent()) {
        RuntimeWorldBootstrapOutcome::Performed(performed) => performed.product_branch().clone(),
        RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!(
                "foreign bootstrap unexpectedly denied: {:?}",
                no_effect.cause()
            )
        }
    };
    assert!(matches!(
        RuntimeWorldBranchService::create_product_branch(
            &owner,
            RuntimeWorldBranchCreationRequest::new(
                foreign_root.clone(),
                reuse_intent("foreign"),
                CompositeExecutionBorrow::without_signal(),
            ),
        ),
        Err(RuntimeWorldBranchAdmissionDenial::ForeignOwner)
    ));
    assert!(matches!(
        RuntimeWorldBranchService::retire_product_branch(
            &owner,
            foreign_root.branch_identity().clone(),
        ),
        Err(RuntimeWorldBranchRetirementDenial::OwnerUnavailable)
    ));
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
}

#[cfg(test)]
#[path = "retirement_tests/fork_creation.rs"]
mod fork_creation;
