use super::support::*;

#[test]
fn advisory_obligation_executes_as_advice_without_blocking_write() {
    let registration = task_collection_registration(
        WorthQueryGraphObligationKind::AdvisoryObligation,
        "advisory-selected",
        supported_command_batch_posture(),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("advisory-selected")])
        .expect("advisory obligation should not block write");
    let dispatch = receipt
        .obligation_dispatch()
        .expect("selected advisory obligation should attach dispatch");
    let execution = dispatch
        .execution_results()
        .expect("selected advisory obligation should execute");
    let reduction = execution.reduce();

    assert_eq!(reduction.advisory_count(), 1);
    assert_eq!(reduction.blocking_count(), 0);
    assert_eq!(
        execution.rows()[0].status(),
        WorthQueryGraphObligationExecutionStatus::Executed
    );
    assert_eq!(
        execution.rows()[0]
            .state_load_counters()
            .loaded_state_scope_count(),
        0
    );
}
