use std::sync::Arc;

use crate::branch::reference_test_fixture;
use crate::branch::{
    ProductBranchComponentPosture, ProductBranchComponentPostures, ProductBranchCreationIntent,
    ProductBranchHeadProtection, ProductBranchObservation, ProductBranchReferenceSnapshot,
    RuntimeWorldBootstrapOutcome,
};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::history::CompositeRuntimeWorldCommit;
use crate::lifecycle::{RuntimeWorldClock, RuntimeWorldClockSource, RuntimeWorldInstant};
use crate::publication::{
    CompositeComponentIntent, CompositeOwnerExecutionResults, ProductBranchIntent,
};

pub(super) type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

#[derive(Clone, Copy)]
struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> RuntimeWorldInstant {
        RuntimeWorldInstant::from_ticks(0)
    }
}

fn budgets(publication_attempts: u64) -> RuntimeWorldBudgets {
    RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 8,
            history_metadata_bytes: 4096,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 8,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: publication_attempts,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 8,
            retained_partial_metadata_bytes: 4096,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 8,
            in_flight_pin_acquisition_reservations: 8,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 8,
        },
    })
    .expect("focused preparation budgets are nonzero")
}

pub(super) fn setup(publication_attempts: u64) -> (Arc<TestOwner>, ProductBranchObservation) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(
            budgets(publication_attempts),
            RuntimeWorldClock::from_source(FixedClock),
        ))
        .expect("managed owner construction"),
    );
    let performed = match owner.bootstrap_root(fixture.bootstrap_intent()) {
        RuntimeWorldBootstrapOutcome::Performed(performed) => performed,
        RuntimeWorldBootstrapOutcome::NoEffect(no_effect) => {
            panic!("bootstrap unexpectedly denied: {:?}", no_effect.cause())
        }
    };
    (owner, performed.product_branch().clone())
}

pub(super) fn intent(
    name: &str,
    relational: ProductBranchComponentPosture,
    signal: ProductBranchComponentPosture,
    component: CompositeComponentIntent,
) -> ProductBranchIntent {
    ProductBranchIntent::new(
        ProductBranchCreationIntent::named(name).expect("valid product operation name"),
        ProductBranchComponentPostures::new(relational, signal),
        component,
    )
}

pub(super) fn signal_intent(name: &str) -> ProductBranchIntent {
    intent(
        name,
        ProductBranchComponentPosture::ReuseExact,
        ProductBranchComponentPosture::ReuseExact,
        CompositeComponentIntent::signal_only(),
    )
}

pub(super) fn install_competing_head(owner: &TestOwner, expected: &ProductBranchObservation) {
    let commit = competing_commit(owner, expected);
    owner
        .state
        .history
        .append(Arc::clone(&commit))
        .expect("competitor commit installs");
    let snapshot = competing_snapshot(expected, Arc::clone(&commit));
    let protection = competing_protection(owner, snapshot, commit.as_ref());
    owner
        .state
        .branches
        .root_cell()
        .expect("bootstrapped root cell")
        .compare_and_publish(expected, protection)
        .expect("competitor wins the exact branch-cell CAS");
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
    Arc::new(
        CompositeRuntimeWorldCommit::from_ordinary_publication(
            commit_identity,
            expected.snapshot().commit(),
            expected.basis().clone(),
            attempt_identity,
            &CompositeOwnerExecutionResults::retained(),
            None,
        )
        .expect("same-basis competitor commit"),
    )
}

fn competing_snapshot(
    expected: &ProductBranchObservation,
    commit: Arc<CompositeRuntimeWorldCommit>,
) -> ProductBranchReferenceSnapshot {
    ProductBranchReferenceSnapshot::owner_issued(
        expected.owner_identity(),
        expected.branch_identity().clone(),
        expected.lifecycle_incarnation(),
        expected
            .reference_generation()
            .advance()
            .expect("competitor generation"),
        commit,
    )
    .expect("competitor snapshot is coherent")
}

fn competing_protection(
    owner: &TestOwner,
    snapshot: ProductBranchReferenceSnapshot,
    commit: &CompositeRuntimeWorldCommit,
) -> ProductBranchHeadProtection {
    let transfer = owner
        .state
        .retention
        .issue_publication(commit.basis())
        .expect("competitor publication retention")
        .into_product_head_transfer(commit.basis())
        .expect("competitor transfer matches its basis");
    let history = owner
        .state
        .history
        .protect_product_head(commit)
        .expect("competitor history protection");
    ProductBranchHeadProtection::owner_issued(snapshot, transfer, history)
        .expect("competitor protection is coherent")
}

pub(super) fn reservation_counts(owner: &TestOwner) -> (usize, usize, usize, usize, usize) {
    (
        owner.state.history.reserved_len(),
        owner.state.recovery.reserved_slots(),
        owner.state.retention.reserved_unique_pin_capacity(),
        owner
            .state
            .retention
            .reserved_in_flight_acquisition_capacity(),
        owner.state.publication_capacity.active(),
    )
}
