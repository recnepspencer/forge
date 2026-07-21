use super::activation_staging_test_support::activation_staging_inputs;
use super::allocation_planning_test_support::{
    allocation_planning, allocation_planning_with_operator,
};

#[test]
fn planning_basis_preserves_constraint_set_identity() {
    let inputs = activation_staging_inputs();
    let (runtime, pending) = inputs.into_runtime_and_pending();
    let planning = allocation_planning(&runtime, &pending, "allocation.constraint.identity");

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
    let first = allocation_planning(&runtime, &pending, "allocation.constraint.changed");
    let second = allocation_planning_with_operator(
        &runtime,
        &pending,
        "allocation.constraint.changed",
        "operator:grid",
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
