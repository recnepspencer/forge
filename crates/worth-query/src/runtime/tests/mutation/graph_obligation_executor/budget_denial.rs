use super::support::*;

#[test]
fn budget_exceeded_records_status_and_counters_before_unbounded_state_load() {
    let budget = WorthQueryGraphObligationExecutionBudget::bounded_sparse(
        WorthQueryGraphObligationExecutionScope::CandidateTopologyComponent,
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory,
    )
    .with_max_state_scope(0);
    let registration = task_collection_registration(
        WorthQueryGraphObligationKind::CapabilityGapScreen,
        "capability-gap-budget-advisory",
        supported_command_batch_posture().with_execution_budget(budget),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("budget-advisory")])
        .expect("advisory budget overrun should retain evidence without blocking");
    let projection = receipt
        .obligation_dispatch()
        .expect("budgeted obligation should attach dispatch")
        .evidence_projection();
    let row = projection
        .rows()
        .first()
        .expect("budgeted obligation should project a row");

    assert_eq!(
        row.execution_status(),
        Some(WorthQueryGraphObligationExecutionStatus::BudgetExceeded)
    );
    assert_eq!(row.loaded_state_scope_count(), Some(2));
    assert_eq!(row.traversed_edge_count(), Some(0));
    assert_eq!(row.materialized_row_count(), Some(0));
    assert_eq!(
        row.execution_cost_class(),
        WorthQueryGraphObligationExecutionCostClass::SparseTopology
    );
    assert_eq!(
        row.execution_scope(),
        WorthQueryGraphObligationExecutionScope::CandidateTopologyComponent
    );
    assert_eq!(
        row.budget_exceeded_policy(),
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory
    );
    assert!(!row.execution_budget_digest().is_empty());
    assert!(!row.state_load_plan_digest().is_empty());
}

#[test]
fn sequencing_budget_overrun_records_budget_status_before_prerequisite_check() {
    let budget = WorthQueryGraphObligationExecutionBudget::declared(
        WorthQueryGraphObligationExecutionCostClass::ConstructionContext,
        WorthQueryGraphObligationExecutionScope::ConstructionFamily,
        WorthQueryGraphObligationBudgetExceededPolicy::Advisory,
    )
    .with_max_state_scope(1);
    let registration = task_collection_registration(
        WorthQueryGraphObligationKind::PreflightSequencingObligation,
        "preflight-budget-advisory",
        supported_command_batch_posture().with_execution_budget(budget),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("preflight-budget-advisory")])
        .expect("advisory budget overrun should stop preflight execution without blocking");
    let execution = receipt
        .obligation_dispatch()
        .and_then(|dispatch| dispatch.execution_results())
        .expect("budgeted preflight obligation should execute into evidence");
    let row = execution
        .rows()
        .first()
        .expect("budgeted preflight obligation should record a result row");

    assert_eq!(
        row.status(),
        WorthQueryGraphObligationExecutionStatus::BudgetExceeded
    );
    assert_eq!(row.state_load_counters().loaded_state_scope_count(), 2);
    assert_eq!(row.state_load_counters().traversed_edge_count(), 0);
    assert_eq!(row.state_load_counters().materialized_row_count(), 0);
}
