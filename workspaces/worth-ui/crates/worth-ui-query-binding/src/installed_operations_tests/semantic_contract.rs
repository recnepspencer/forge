use worth_query::facade::domain;

#[test]
fn snapshot_measurement_declares_no_decision_fact_or_invariant_execution_authority() {
    let definition =
        crate::installed_domain::snapshot_measurement::snapshot_measurement_definition();
    assert_eq!(
        definition.semantics().decision_facts,
        domain::WorthQueryOperationDecisionFactContract::NotRequired
    );
    assert_eq!(
        definition.semantics().invariant_execution,
        domain::WorthQueryInvariantExecutionContract::NotRequired
    );
}
