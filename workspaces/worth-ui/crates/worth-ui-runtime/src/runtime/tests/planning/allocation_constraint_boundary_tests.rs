use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    admitted_allocation_neighborhood, admitted_measurement_basis, changed_allocation_neighborhood,
    changed_measurement_basis,
};

#[test]
fn planning_basis_preserves_constraint_set_identity() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let measurement_basis = admitted_measurement_basis("allocation.constraint.identity");
    let neighborhood = admitted_allocation_neighborhood("allocation.constraint.identity");
    let planning = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &measurement_basis,
        &neighborhood,
    );

    assert!(planning.is_admitted());
    assert_eq!(
        planning
            .basis()
            .allocation_constraint_set()
            .expect("admitted planning should preserve constraint set")
            .identity()
            .identity_digest(),
        planning
            .allocation_constraint_set()
            .expect("admitted planning should expose constraint set")
            .identity()
            .identity_digest()
    );
}

#[test]
fn changed_constraint_set_changes_planning_identity() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let plan_input = runtime
        .prepare_execution_plan_input(&pending)
        .expect("plan input prepares");
    let first_basis = admitted_measurement_basis("allocation.constraint.changed");
    let first_neighborhood = admitted_allocation_neighborhood("allocation.constraint.changed");
    let second_basis = changed_measurement_basis("allocation.constraint.changed");
    let second_neighborhood = changed_allocation_neighborhood("allocation.constraint.changed");
    let first = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input.clone(),
        &first_basis,
        &first_neighborhood,
    );
    let second = runtime.plan_allocation_for_lowered_input_for_test(
        plan_input,
        &second_basis,
        &second_neighborhood,
    );

    assert_ne!(
        first
            .allocation_constraint_set()
            .expect("admitted planning should expose first constraint set")
            .identity()
            .identity_digest(),
        second
            .allocation_constraint_set()
            .expect("admitted planning should expose second constraint set")
            .identity()
            .identity_digest()
    );
    assert_ne!(
        first.planning_identity_digest(),
        second.planning_identity_digest()
    );
}
