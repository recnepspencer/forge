use super::*;
use crate::domain_computation::WorthQueryExecutionResourceContract;
use crate::facade::{
    WorthQueryInstalledGraphObligation, WorthQueryInstalledGraphObligationEffectPosture as Effect,
    WorthQueryInstalledGraphObligationResourcePosture as Resource,
    WorthQueryInstalledGraphObligationSelectionBasis as Selection,
    WorthQueryInstalledGraphObligationTerminalRequirement as Terminal,
};

#[test]
fn installed_operation_obligations_retain_complete_row_semantics() {
    let index = installed_index();
    let schema = index
        .bind_application_schema(TestSchema::declaration().unwrap())
        .unwrap();
    let operation = schema
        .installed_operation(ApplicationOperationRef::<
            TestSchema,
            TestOperation,
            TestInput,
        >::from_schema_identifier("TestOperation"))
        .unwrap();
    let rows = operation.contracts().obligations().rows();

    let [read, authorization, touch, effect, invariant] = rows else {
        panic!("the installed operation must retain one row for every semantic kind");
    };
    assert_graph_read(read, operation.contracts().resources());
    assert_authorization(authorization, operation.contracts().resources());
    assert_touch(touch, operation.contracts().resources());
    assert_effect(effect, operation.contracts().resources());
    assert_invariant(invariant, operation.contracts().resources());
}

fn assert_graph_read(
    row: &WorthQueryInstalledGraphObligation,
    resources: &WorthQueryExecutionResourceContract,
) {
    assert!(matches!(
        row.selection_basis(),
        Selection::ApplicationOperationGraphRole(role) if role.role == "primary"
    ));
    assert_eq!(row.effect_posture(), Effect::Observational);
    assert_eq!(row.terminal_requirement(), Terminal::GraphReadProduct);
    assert_operation_resources(row, resources);
}

fn assert_authorization(
    row: &WorthQueryInstalledGraphObligation,
    resources: &WorthQueryExecutionResourceContract,
) {
    assert!(matches!(
        row.selection_basis(),
        Selection::AuthenticatedAccessContext
    ));
    assert_eq!(row.effect_posture(), Effect::Policy);
    assert_eq!(
        row.terminal_requirement(),
        Terminal::AuthorizationDecisionFact
    );
    assert_operation_resources(row, resources);
}

fn assert_touch(
    row: &WorthQueryInstalledGraphObligation,
    resources: &WorthQueryExecutionResourceContract,
) {
    assert!(matches!(row.selection_basis(), Selection::MutationTouch(_)));
    assert_eq!(row.effect_posture(), Effect::Observational);
    assert_eq!(row.terminal_requirement(), Terminal::TouchedScopeEvidence);
    assert_operation_resources(row, resources);
}

fn assert_effect(
    row: &WorthQueryInstalledGraphObligation,
    resources: &WorthQueryExecutionResourceContract,
) {
    assert!(matches!(row.selection_basis(), Selection::ProposedState));
    assert_eq!(row.effect_posture(), Effect::Mutating);
    assert_eq!(
        row.terminal_requirement(),
        Terminal::EffectApplicationReceipt
    );
    assert_eq!(
        row.effect_family(),
        Some(crate::facade::WorthQueryOperationEffectFamily::Mutation)
    );
    assert_operation_resources(row, resources);
}

fn assert_invariant(
    row: &WorthQueryInstalledGraphObligation,
    resources: &WorthQueryExecutionResourceContract,
) {
    assert!(matches!(row.selection_basis(), Selection::ProposedState));
    assert_eq!(row.effect_posture(), Effect::Invariant);
    assert_eq!(row.terminal_requirement(), Terminal::InvariantVerdict);
    assert!(row.invariant_requirement().is_some());
    assert_operation_resources(row, resources);
}

fn assert_operation_resources(
    row: &WorthQueryInstalledGraphObligation,
    expected: &WorthQueryExecutionResourceContract,
) {
    let Resource::ApplicationOperation(actual) = row.resource_posture() else {
        panic!("an operation obligation must retain its installed operation resources");
    };
    assert_eq!(actual, expected);
}
