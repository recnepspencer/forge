use super::{admit_current, component_fixture};

use crate::budget::{
    RuntimeWorldBranchBudgetInstallation, RuntimeWorldBudgetInstallation, RuntimeWorldBudgets,
    RuntimeWorldCustodyBudgetInstallation, RuntimeWorldHistoryBudgetInstallation,
    RuntimeWorldObservationBudgetInstallation, RuntimeWorldPublicationBudgetInstallation,
    RuntimeWorldRecoveryBudgetInstallation, RuntimeWorldRetentionBudgetInstallation,
};
use crate::lifecycle::owner::RuntimeWorldOwnerConstructionContract;
use crate::retention::{ComponentBasisDependencyClass, RuntimeWorldRetentionOwner};

#[test]
fn repeated_exact_component_retention_shares_one_binding_and_counts_dependencies() {
    let fixture = component_fixture();
    let budgets = RuntimeWorldBudgets::install(RuntimeWorldBudgetInstallation {
        branches: RuntimeWorldBranchBudgetInstallation {
            live_product_branches: 1,
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
            unique_exact_component_pins: 2,
            in_flight_pin_acquisition_reservations: 4,
        },
        custody: RuntimeWorldCustodyBudgetInstallation {
            owner_created_component_custody_records: 1,
        },
    })
    .expect("retention limits are valid");
    let first_owner =
        RuntimeWorldOwnerConstructionContract::new().expect("first World owner construction");
    let retention = RuntimeWorldRetentionOwner::new(
        first_owner.owner_identity(),
        budgets.unique_exact_component_pins(),
        budgets.in_flight_pin_acquisition_reservations(),
    );
    let first_basis = admit_current(
        first_owner.issuer(),
        &fixture.relational_port,
        &fixture.signal_port,
        &fixture.correspondence_port,
        fixture.relational.clone(),
        fixture.signal.clone(),
        fixture.correspondence.clone(),
    )
    .expect("the retention basis is owner-admitted");
    let first = retention
        .issue_observation(&first_basis)
        .expect("first observation obligation");

    let second_basis = admit_current(
        first_owner.issuer(),
        &fixture.relational_port,
        &fixture.signal_port,
        &fixture.correspondence_port,
        fixture.relational.clone(),
        fixture.signal.clone(),
        fixture.correspondence.clone(),
    )
    .expect("the repeated basis is owner-admitted");
    let second = retention
        .issue_observation(&second_basis)
        .expect("second observation obligation");

    let foreign_owner =
        RuntimeWorldOwnerConstructionContract::new().expect("foreign World owner construction");
    let foreign_basis = admit_current(
        foreign_owner.issuer(),
        &fixture.relational_port,
        &fixture.signal_port,
        &fixture.correspondence_port,
        fixture.relational.clone(),
        fixture.signal.clone(),
        fixture.correspondence.clone(),
    )
    .expect("the foreign basis is independently owner-admitted");
    assert!(matches!(
        retention.issue_observation(&foreign_basis),
        Err(crate::retention::RetentionObligationDenial::ForeignOwner { .. })
    ));

    assert_eq!(
        first.relational().binding_identity(),
        second.relational().binding_identity()
    );
    assert_eq!(
        first
            .relational()
            .dependency_count(ComponentBasisDependencyClass::AdmittedObservation),
        2
    );
    assert_eq!(retention.active_component_obligation_count(), 4);
    drop(first);
    assert_eq!(retention.active_component_obligation_count(), 2);
    drop(second);
    assert_eq!(retention.active_component_obligation_count(), 0);
}
