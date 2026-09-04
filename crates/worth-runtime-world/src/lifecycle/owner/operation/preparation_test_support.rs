use std::sync::Arc;

use crate::branch::reference_test_fixture;
use crate::branch::{
    ProductBranchCreationIntent, ProductBranchCreationPlans, RelationalBranchCreationPlan,
    SignalBranchCreationPlan,
};
use crate::branch::{ProductBranchObservation, RuntimeWorldBootstrapOutcome};
use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::lifecycle::{RuntimeWorldClock, RuntimeWorldClockSource, RuntimeWorldInstant};
use crate::publication::{
    CompositeComponentIntent, CompositePublicationIntent, LoweredOwnerComponentPlan,
    RelationalComponentPlan, ResolvedExpectedProductHead, SignalComponentPlan,
    SignalComponentPlanPosture, WithSignal,
};

pub(super) type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

#[derive(Clone, Copy)]
struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> RuntimeWorldInstant {
        RuntimeWorldInstant::from_ticks(0)
    }
}

fn budgets(publication_attempts: u64, custody_records: u64) -> RuntimeWorldBudgets {
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
            owner_created_component_custody_records: custody_records,
        },
    })
    .expect("focused preparation budgets are nonzero")
}

pub(super) fn setup(publication_attempts: u64) -> (Arc<TestOwner>, ProductBranchObservation) {
    setup_with_custody(publication_attempts, 8)
}

/// The same focused world with an explicit owner-created custody bound, so a
/// creation reservation can be driven to custody exhaustion without disturbing
/// any other capacity.
pub(super) fn setup_with_custody(
    publication_attempts: u64,
    custody_records: u64,
) -> (Arc<TestOwner>, ProductBranchObservation) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(
            budgets(publication_attempts, custody_records),
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

/// The focused Signal-only publication meaning: an explicit Signal
/// `AdvanceExact` with no Relational change.
pub(super) fn signal_intent() -> CompositePublicationIntent<WithSignal> {
    CompositePublicationIntent::with_signal(None)
}

/// A creation cell that asks both owners to fork, so preparation must charge
/// one custody slot per component.
pub(super) fn fork_both_intent(name: &str) -> ProductBranchCreationIntent {
    ProductBranchCreationIntent::from_source(
        name,
        ProductBranchCreationPlans::new(
            RelationalBranchCreationPlan::ForkExact {
                target: worth_relational::facade::history::BranchId(format!("{name}-relational")),
            },
            SignalBranchCreationPlan::ForkExact {
                target: worth_signal::facade::branch::validate_signal_branch_name(name)
                    .expect("focused Signal branch name validates"),
            },
        ),
    )
    .expect("focused product branch name validates")
}

/// Build a lowered publication plan directly, bypassing lowering, so a posture
/// that contradicts its component intent can be handed to the reservation
/// boundary. Only postures a lowering denial would have refused are
/// constructible here: neither leg carries an owner-issued candidate.
pub(super) fn retained_relational_plan(
    owner: &TestOwner,
    expected: &ProductBranchObservation,
    intent: CompositeComponentIntent,
    signal: SignalComponentPlanPosture,
) -> LoweredOwnerComponentPlan {
    let current = owner
        .current_product_head_snapshot(expected)
        .expect("the bootstrapped product branch still has a reference cell");
    let resolved =
        ResolvedExpectedProductHead::from_current(intent.clone(), expected.clone(), &current)
            .expect("the bootstrapped head is its own current image");
    let signal_basis = expected.basis().signal_basis().clone();
    let signal_plan = match signal {
        SignalComponentPlanPosture::RetainExact => SignalComponentPlan::retain_exact(signal_basis),
        SignalComponentPlanPosture::AdvanceExact => {
            SignalComponentPlan::advance_exact(signal_basis)
        }
    };
    LoweredOwnerComponentPlan::new(
        resolved,
        intent,
        RelationalComponentPlan::retain_exact(expected.basis().relational_basis().clone()),
        signal_plan,
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
