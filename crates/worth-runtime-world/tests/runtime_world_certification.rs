use worth_relational::facade::mvcc::RelationalTransactionIntent;
use worth_runtime_world::facade::{
    CompositeComponentIntent, CompositeExecutionBorrow, ProductBranchComponentPosture,
    ProductBranchComponentPostures, ProductBranchName, RuntimeWorldBudgetDenial,
    RuntimeWorldBudgets, RuntimeWorldClock, RuntimeWorldClockSource, RuntimeWorldInstant,
    RuntimeWorldPublicationPhase,
};

#[test]
fn installed_budgets_are_nonzero_and_cover_every_runtime_world_population() {
    let budgets = RuntimeWorldBudgets::try_new(1, 2, 3, 4, 5, 6, 7, 8, 9, 10)
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

    let denial = RuntimeWorldBudgets::try_new(0, 1, 1, 1, 1, 1, 1, 1, 1, 1)
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
fn publication_progression_has_no_untyped_skip_or_boolean_outcome() {
    let phases = [
        RuntimeWorldPublicationPhase::ProductBranchIntent,
        RuntimeWorldPublicationPhase::ResolvedExpectedProductHead,
        RuntimeWorldPublicationPhase::AdmittedCompositeRuntimeWorldBasis,
        RuntimeWorldPublicationPhase::LoweredOwnerComponentPlan,
        RuntimeWorldPublicationPhase::ReservedCompositePublicationAttempt,
        RuntimeWorldPublicationPhase::OwnerExecutionSettlement,
        RuntimeWorldPublicationPhase::CompositePublicationReady,
        RuntimeWorldPublicationPhase::RuntimeWorldPublicationOutcome,
    ];
    assert_eq!(phases.len(), 8);
}

#[test]
fn signal_execution_borrow_is_scoped_to_the_owner_call() {
    let mut context = 7_u32;
    {
        let borrow = CompositeExecutionBorrow::signal(&mut context, ());
        match borrow {
            CompositeExecutionBorrow::Signal {
                context: borrowed,
                mutation: (),
            } => *borrowed += 1,
            CompositeExecutionBorrow::WithoutSignal => panic!("signal borrow was required"),
        }
    }
    assert_eq!(context, 8);
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
}
