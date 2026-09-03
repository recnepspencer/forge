//! Settlement and recovery-catalog custody tests only. These tests manually
//! supply owner-issued effects to prove custody, not production dispatch.

use std::sync::Arc;

use worth_relational::facade::mvcc::RelationalTransactionIntent;

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
    RuntimeWorldPreparationService,
};
use crate::publication::{
    CompositeAttemptProgress, CompositeComponentIntent, ProductBranchIntent,
    RelationalAttemptProgress, SignalAttemptProgress,
};
use crate::recovery::ProductUnpublishedCause;

type TestOwner = super::super::RuntimeWorldOwnerRoot<(), (), (), (), ()>;

struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> crate::lifecycle::RuntimeWorldInstant {
        crate::lifecycle::RuntimeWorldInstant::from_ticks(7)
    }
}

fn budgets(maximum_recovery_records: u64) -> RuntimeWorldBudgets {
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
            retained_product_unpublished_records: maximum_recovery_records,
            retained_partial_metadata_bytes: 1,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 6,
            in_flight_pin_acquisition_reservations: 4,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("recovery tests install positive owner budgets")
}

fn setup_with_recovery_limit(
    maximum_recovery_records: u64,
) -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    let mut fixture = reference_test_fixture::real_fixture(8, 8);
    let owner = Arc::new(
        TestOwner::new(fixture.owner_inputs(
            budgets(maximum_recovery_records),
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
    (fixture, owner, performed.product_branch().clone())
}

fn setup() -> (
    RealReferenceFixture,
    Arc<TestOwner>,
    ProductBranchObservation,
) {
    setup_with_recovery_limit(1)
}

fn relational_plan(
    owner: &TestOwner,
    expected: ProductBranchObservation,
) -> crate::publication::LoweredOwnerComponentPlan {
    RuntimeWorldPreparationService::prepare(
        owner,
        expected,
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("recovery-custody")
                .expect("valid recovery operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary()),
        ),
    )
    .expect("the owner prepares the exact relational recovery test head")
}

fn successor_basis(
    owner: &TestOwner,
    expected: &ProductBranchObservation,
    relational: worth_relational::facade::branch::AdmittedRelationalBranchBasis,
    signal: Option<worth_signal::facade::branch::AdmittedSignalBranchBasis>,
) -> crate::basis::AdmittedCompositeRuntimeWorldBasis {
    crate::basis::admit_current(
        &owner
            .state
            .identities
            .lock()
            .unwrap_or_else(|error| error.into_inner()),
        &owner.state.relational.basis_port(),
        &owner.state.signal.basis_port(),
        &owner.state.bridge,
        relational,
        signal.unwrap_or_else(|| expected.basis().signal_basis().clone()),
        expected.basis().correspondence_basis().clone(),
    )
    .expect("the real component owners admit the exact successor tuple")
}

#[test]
fn caller_loss_preserves_relational_settlement_custody_and_catalog_capacity() {
    let (fixture, owner, expected) = setup();
    let plan = relational_plan(&owner, expected.clone());
    let cancellation = RuntimeWorldCancellationSource::new();
    let mut attempt =
        RuntimeWorldPreparationService::reserve(owner.as_ref(), plan, &cancellation.token(), None)
            .expect("the owner reserves the complete recovery attempt");
    attempt.begin_owner_execution();

    let performed = fixture.perform_relational_owner_change();
    let successor_basis = successor_basis(&owner, &expected, performed.next_basis().clone(), None);
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::performed(performed),
        SignalAttemptProgress::untouched(),
    );
    let retained = attempt
        .settle(progress)
        .ready(successor_basis)
        .expect_err("unsettled Relational work enters retained recovery");

    assert_eq!(retained.cause(), ProductUnpublishedCause::SettlementPending);
    let handle = retained.recovery_handle();
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.recovery_handles(), vec![handle.clone()]);
    drop(retained);

    assert_eq!(owner.recovery_record_count(), 1);
    let inspected = owner
        .inspect_recovery(&handle)
        .expect("caller loss leaves the catalog record inspectable");
    assert_eq!(
        inspected.cause(),
        ProductUnpublishedCause::SettlementPending
    );
    assert!(!owner.cleanup_recovery_handle(&handle));
    drop(inspected);
    assert!(!owner.cleanup_recovery_handle(&handle));
    assert!(matches!(
        owner
            .state
            .recovery
            .reserve_product_unpublished(owner.owner_identity()),
        Err(crate::recovery::RecoveryCatalogDenial::CapacityExhausted { maximum: 1 })
    ));
}

#[cfg(feature = "test-operation-control")]
#[test]
fn metadata_ceiling_rejects_second_charge_and_cleanup_releases_the_first() {
    let (mut fixture, owner, expected) = setup_with_recovery_limit(2);
    let plan = RuntimeWorldPreparationService::prepare(
        owner.as_ref(),
        expected.clone(),
        ProductBranchIntent::new(
            ProductBranchCreationIntent::named("signal-recovery-custody")
                .expect("valid signal recovery operation name"),
            ProductBranchComponentPostures::new(
                ProductBranchComponentPosture::ReuseExact,
                ProductBranchComponentPosture::ReuseExact,
            ),
            CompositeComponentIntent::signal_only(),
        ),
    )
    .expect("the owner prepares the exact signal recovery test head");
    let cancellation = RuntimeWorldCancellationSource::new();
    let mut attempt =
        RuntimeWorldPreparationService::reserve(owner.as_ref(), plan, &cancellation.token(), None)
            .expect("the owner reserves the signal recovery attempt");
    attempt.begin_owner_execution();

    let advanced = fixture.perform_signal_owner_change();
    let successor_basis = successor_basis(
        &owner,
        &expected,
        expected.basis().relational_basis().clone(),
        Some(advanced.advanced_basis().clone()),
    );
    fixture.inject_signal_retention_panic();
    let progress = CompositeAttemptProgress::new(
        RelationalAttemptProgress::untouched(),
        SignalAttemptProgress::advanced(advanced),
    );
    let retained = attempt
        .settle(progress)
        .ready(successor_basis)
        .expect_err("post-effect retention denial enters catalogued recovery");
    let first_charge = retained.metadata_bytes();
    assert!(first_charge > 0);
    owner
        .state
        .recovery
        .set_metadata_ceiling_for_test(first_charge);
    let handle = retained.recovery_handle();
    drop(retained);
    assert_eq!(owner.recovery_record_count(), 1);
    assert_eq!(owner.state.recovery.metadata_bytes(), first_charge);
    assert!(matches!(
        owner
            .state
            .recovery
            .reserve_product_unpublished(owner.owner_identity()),
        Err(crate::recovery::RecoveryCatalogDenial::CapacityExhausted { .. })
    ));
    assert!(owner.cleanup_recovery_handle(&handle));
    assert_eq!(owner.recovery_record_count(), 0);
    assert_eq!(owner.state.recovery.metadata_bytes(), 0);
    let released_slot = owner
        .state
        .recovery
        .reserve_product_unpublished(owner.owner_identity())
        .expect("eligible cleanup releases the exact metadata charge");
    drop(released_slot);
    assert_eq!(owner.state.retention.reserved_unique_pin_capacity(), 0);
    assert_eq!(
        owner
            .state
            .retention
            .reserved_in_flight_acquisition_capacity(),
        0
    );
}
