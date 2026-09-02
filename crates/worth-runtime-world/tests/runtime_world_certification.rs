#[path = "runtime_world_certification/basis_history.rs"]
mod basis_history;
#[path = "runtime_world_certification/bridge.rs"]
mod bridge;
#[path = "runtime_world_certification/reference.rs"]
mod reference;
#[path = "runtime_world_certification/retention.rs"]
mod retention;

use worth_relational::facade::mvcc::RelationalTransactionIntent;
use worth_runtime_world::facade::{
    CompositeComponentIntent, CompositeExecutionBorrow, ProductBranchComponentPosture,
    ProductBranchComponentPostures, ProductBranchName, RuntimeWorldBranchBudgetInstallation,
    RuntimeWorldBudgetDenial, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCancellationSource, RuntimeWorldClock, RuntimeWorldClockSource,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldInstant, RuntimeWorldObservationBudgetInstallation,
    RuntimeWorldPublicationBudgetInstallation, RuntimeWorldRecoveryBudgetInstallation,
    RuntimeWorldRetentionBudgetInstallation,
};

#[test]
fn installed_budgets_are_nonzero_and_cover_every_runtime_world_population() {
    let budgets = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 2,
            history_metadata_bytes: 3,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 4,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 5,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 6,
            retained_partial_metadata_bytes: 7,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 8,
            in_flight_pin_acquisition_reservations: 9,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 10,
        },
    })
    .expect("all installed limits are nonzero");

    assert_eq!(budgets.live_product_branches().get(), 1);
    assert_eq!(budgets.retained_composite_commits().get(), 2);
    assert_eq!(budgets.history_metadata_bytes().get(), 3);
    assert_eq!(budgets.active_observations().get(), 4);
    assert_eq!(budgets.active_publication_attempts().get(), 5);
    assert_eq!(budgets.retained_product_unpublished_records().get(), 6);
    assert_eq!(budgets.retained_partial_metadata_bytes().get(), 7);
    assert_eq!(budgets.unique_exact_component_pins().get(), 8);
    assert_eq!(budgets.in_flight_pin_acquisition_reservations().get(), 9);
    assert_eq!(budgets.owner_created_component_custody_records().get(), 10);

    let denial = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 0,
        },
        history: RuntimeWorldHistoryBudgetInstallation {
            retained_composite_commits: 1,
            history_metadata_bytes: 1,
        },
        observations: RuntimeWorldObservationBudgetInstallation {
            active_observations: 1,
        },
        publication: RuntimeWorldPublicationBudgetInstallation {
            active_publication_attempts: 1,
        },
        recovery: RuntimeWorldRecoveryBudgetInstallation {
            retained_product_unpublished_records: 1,
            retained_partial_metadata_bytes: 1,
        },
        retention: RuntimeWorldRetentionBudgetInstallation {
            unique_exact_component_pins: 1,
            in_flight_pin_acquisition_reservations: 1,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect_err("zero capacity is not an installed bound");
    assert!(matches!(denial, RuntimeWorldBudgetDenial::ZeroLimit { .. }));
}

#[test]
fn branch_name_and_component_postures_are_explicit() {
    assert!(ProductBranchName::try_new("main").is_ok());
    assert!(ProductBranchName::try_new("  ").is_err());

    let postures = ProductBranchComponentPostures::new(
        ProductBranchComponentPosture::ForkExact,
        ProductBranchComponentPosture::ReuseExact,
    );
    assert_eq!(
        postures.relational(),
        ProductBranchComponentPosture::ForkExact
    );
    assert_eq!(postures.signal(), ProductBranchComponentPosture::ReuseExact);
}

#[test]
fn component_intent_carries_owner_meaning_without_ambient_currentness() {
    let relational =
        CompositeComponentIntent::relational_only(RelationalTransactionIntent::ordinary());
    assert!(relational.changes_relational());
    assert!(!relational.changes_signal());
    assert!(relational.relational_change().is_some());

    let signal = CompositeComponentIntent::signal_only();
    assert!(!signal.changes_relational());
    assert!(signal.changes_signal());
    assert!(signal.relational_change().is_none());
}

#[test]
fn signal_execution_borrow_is_scoped_to_the_owner_call() {
    let mut context = 7_u32;
    let cancellation = worth_signal::facade::branch::SignalOwnerCancellationSource::new();
    let signal_token = cancellation.token();
    {
        let borrow = CompositeExecutionBorrow::<(), (), (), u32, ()>::signal(
            &mut context,
            &signal_token,
            |_transaction| Ok(()),
        );
        match borrow {
            CompositeExecutionBorrow::Signal {
                context: borrowed,
                cancellation: borrowed_cancellation,
                ..
            } => {
                assert!(!borrowed_cancellation.is_cancelled());
                *borrowed += 1;
            }
            CompositeExecutionBorrow::WithoutSignal => panic!("signal borrow was required"),
        }
        assert!(!signal_token.is_cancelled());
    }
    assert_eq!(context, 8);
}

#[test]
fn cancellation_has_a_named_pre_effect_source_and_token() {
    let source = RuntimeWorldCancellationSource::new();
    let token = source.token();
    assert!(!token.is_cancelled());
    source.cancel();
    assert!(token.is_cancelled());
}

struct FixedClock;

impl RuntimeWorldClockSource for FixedClock {
    fn now(&self) -> RuntimeWorldInstant {
        RuntimeWorldInstant::from_ticks(42)
    }
}

#[test]
fn clock_is_explicit_and_only_reports_deadline_time() {
    let clock = RuntimeWorldClock::from_source(FixedClock);
    assert_eq!(clock.now().ticks(), 42);
}

#[test]
fn compile_failures_protect_the_public_contract() {
    let tests = trybuild::TestCases::new();
    tests.compile_fail("tests/ui/*.rs");
    tests.pass("tests/pass/*.rs");
}
