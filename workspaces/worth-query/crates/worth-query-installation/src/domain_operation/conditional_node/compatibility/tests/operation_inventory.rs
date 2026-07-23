use super::fixture::{operation_definition, operation_node, workflow_node, TriggerA};
use crate::domain_operation::{
    compare_portable_operation_conditionals, WorthQueryComparatorRequirement,
    WorthQueryOperationConditionalComparisonOutcome, WorthQueryOperationConditionalDimension,
};

#[test]
fn operation_comparison_owns_operation_and_stage_inventory_order() {
    let alpha = operation_node::<TriggerA>(
        "alpha",
        WorthQueryComparatorRequirement::ExactCanonicalValue,
    );
    let zeta =
        operation_node::<TriggerA>("zeta", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let left = operation_definition(
        vec![zeta.clone(), alpha.clone()],
        workflow_node("stage-gate"),
    );
    let right = operation_definition(vec![alpha, zeta], workflow_node("stage-gate"));
    let WorthQueryOperationConditionalComparisonOutcome::Equivalent(equivalent) =
        compare_portable_operation_conditionals(&left, &right)
    else {
        panic!("owner canonical ordering must erase declaration order only");
    };

    assert_eq!(equivalent.node_count(), 3);
    assert!(equivalent.comparison_count() > 30);
}

#[test]
fn operation_comparison_denies_location_drift_before_node_meaning() {
    let operation =
        operation_node::<TriggerA>("gate", WorthQueryComparatorRequirement::ExactCanonicalValue);
    let left = operation_definition(vec![operation.clone()], workflow_node("stage-a"));
    let right = operation_definition(vec![operation], workflow_node("stage-b"));
    let WorthQueryOperationConditionalComparisonOutcome::Mismatched(mismatch) =
        compare_portable_operation_conditionals(&left, &right)
    else {
        panic!("moving equal node meaning to another owner location must mismatch");
    };
    assert_eq!(
        mismatch.dimension(),
        &WorthQueryOperationConditionalDimension::Location(1)
    );
}
