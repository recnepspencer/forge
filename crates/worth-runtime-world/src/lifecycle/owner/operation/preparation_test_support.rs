use std::sync::Arc;

use crate::branch::reference_test_fixture;
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
use crate::lifecycle::{RuntimeWorldClock, RuntimeWorldClockSource, RuntimeWorldInstant};
use crate::publication::{CompositeComponentIntent, ProductBranchIntent};

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
            retained_partial_metadata_bytes:
                crate::recovery::ProductUnpublishedOwnerEffects::metadata_charge_hint()
                    .saturating_mul(publication_attempts as usize) as u64,
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
