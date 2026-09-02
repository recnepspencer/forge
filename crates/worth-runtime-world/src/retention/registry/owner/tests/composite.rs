use crate::retention::component_obligation::ObservationRetentionObligation;
use crate::retention::unique_component_pin::ExactComponentPinRequest;
use crate::retention::ComponentBasisDependencyClass;

use super::fixture::{ordinary_commit, real_fixture, root_commit};

fn consume_observation(observation: ObservationRetentionObligation) {
    drop(observation);
}

#[test]
fn all_composite_entry_points_consume_one_owner_issued_pair() {
    let mut fixture = real_fixture(4, 4);
    let root = root_commit(&mut fixture);
    let before_relational = fixture.relational_runtime.branch_basis_cost_counters();
    let before_signal = fixture
        .signal_port
        .owner_service_cost_snapshot()
        .expect("real Signal owner remains available");

    let observation = fixture
        .owner
        .issue_observation(&root)
        .expect("observation acquires both exact dependencies");
    let publication = fixture
        .owner
        .issue_publication(&fixture.basis)
        .expect("publication acquires both exact dependencies");
    let retained_partial = fixture
        .owner
        .issue_retained_partial(&fixture.basis)
        .expect("retained partial acquires both exact dependencies");

    assert!(observation.matches_captured_head(&root));
    assert!(publication.matches_basis(&fixture.basis));
    assert!(retained_partial.matches_basis(&fixture.basis));
    assert_eq!(
        observation.relational().dependency(),
        ComponentBasisDependencyClass::AdmittedObservation
    );
    assert_eq!(
        observation.signal().dependency(),
        ComponentBasisDependencyClass::AdmittedObservation
    );
    assert_eq!(
        publication.relational().dependency(),
        ComponentBasisDependencyClass::ActivePublicationAttempt
    );
    assert_eq!(
        retained_partial.signal().dependency(),
        ComponentBasisDependencyClass::ProductUnpublishedOwnerEffects
    );
    assert_eq!(fixture.owner.unique_pin_count(), 2);
    assert_eq!(fixture.owner.active_component_obligation_count(), 6);
    assert_eq!(fixture.owner.in_flight_acquisition_count(), 0);

    let costs = fixture.owner.cost_snapshot();
    assert_eq!(costs.batch_admitted(), 3);
    assert_eq!(costs.flights_started(), 2);
    assert_eq!(costs.single_flight_joins(), 0);
    assert_eq!(costs.relational_contacts(), 1);
    assert_eq!(costs.signal_contacts(), 1);
    assert_eq!(costs.relational_successes(), 1);
    assert_eq!(costs.signal_successes(), 1);
    assert_eq!(
        fixture
            .relational_runtime
            .branch_basis_cost_counters()
            .external_retention_acquires,
        before_relational.external_retention_acquires + 1
    );
    assert_eq!(
        fixture
            .signal_port
            .owner_service_cost_snapshot()
            .expect("real Signal owner remains available")
            .retention_registry_contacts(),
        before_signal.retention_registry_contacts() + 1
    );

    drop(retained_partial);
    drop(publication);
    consume_observation(observation);

    let after_relational = fixture.relational_runtime.branch_basis_cost_counters();
    let after_signal = fixture
        .signal_port
        .owner_service_cost_snapshot()
        .expect("real Signal owner remains available");
    assert_eq!(fixture.owner.active_component_obligation_count(), 0);
    assert_eq!(fixture.owner.unique_pin_count(), 2);
    assert_eq!(fixture.owner.cost_snapshot().owner_drop_releases(), 2);
    assert_eq!(
        after_relational.external_retention_releases,
        before_relational.external_retention_releases + 1
    );
    assert_eq!(
        after_relational.retained_basis_registry_entries,
        before_relational.retained_basis_registry_entries,
        "component final population returns to its independent baseline"
    );
    assert_eq!(
        after_signal.retention_registry_contacts(),
        before_signal.retention_registry_contacts() + 1
    );
    assert_eq!(fixture.owner.reclaim(2).reclaimed(), 2);
}

#[test]
fn observation_is_affine_to_exact_commit_and_basis() {
    let mut fixture = real_fixture(4, 4);
    let root = root_commit(&mut fixture);
    let ordinary = ordinary_commit(&mut fixture, &root);
    let observation = fixture
        .owner
        .issue_observation(&root)
        .expect("real owner issues the observation pair");

    assert!(observation.matches_captured_head(&root));
    assert!(
        !observation.matches_captured_head(&ordinary),
        "equal basis with a different commit occurrence is not the captured head"
    );
    assert_eq!(
        observation.relational().key(),
        &ExactComponentPinRequest::relational(
            &fixture.basis,
            ComponentBasisDependencyClass::AdmittedObservation,
        )
        .key()
    );
    assert_eq!(
        observation.signal().key(),
        &ExactComponentPinRequest::signal(
            &fixture.basis,
            ComponentBasisDependencyClass::AdmittedObservation,
        )
        .key()
    );
    assert_eq!(
        observation.relational().owner_identity(),
        fixture.owner_identity
    );
    assert_eq!(
        observation.signal().owner_identity(),
        fixture.owner_identity
    );

    let mut foreign_fixture = real_fixture(4, 4);
    let foreign_root = root_commit(&mut foreign_fixture);
    assert_ne!(foreign_root.identity(), root.identity());
    assert!(
        !observation.matches_captured_head(&foreign_root),
        "a foreign basis and commit cannot validate the captured head"
    );

    consume_observation(observation);
    assert_eq!(fixture.owner.active_component_obligation_count(), 0);
    assert_eq!(fixture.owner.unique_pin_count(), 2);
    assert_eq!(fixture.owner.reclaim(2).reclaimed(), 2);
}
