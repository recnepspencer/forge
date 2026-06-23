use super::support::*;

#[test]
fn preflight_sequencing_blocks_missing_prerequisite_without_panic() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::PreflightSequencingObligation,
        "finish-before-witness",
        supported_command_batch_posture(),
    );
    let mut runtime = runtime_with_registration(registration);

    let error = runtime
        .write_batch(vec![task_insert_command("finish-before-witness")])
        .expect_err("missing preflight witness should deny before write execution");

    match error {
        ForgeQueryRuntimeError::GraphObligationDenied(denial) => {
            assert_eq!(denial.blocking_count(), 1);
            let row = denial
                .rows()
                .first()
                .expect("preflight denial should project the blocking row");
            assert_eq!(
                row.obligation_kind(),
                ForgeQueryGraphObligationKind::PreflightSequencingObligation
            );
            assert_eq!(
                row.execution_status(),
                ForgeQueryGraphObligationExecutionStatus::BlockedByPrerequisite
            );
            assert_eq!(
                row.verdict_context(),
                Some("preflight-prerequisite-not-satisfied")
            );
        }
        other => panic!("expected typed preflight graph obligation denial, got {other:?}"),
    }
}

#[test]
fn preflight_sequencing_allows_when_typed_witness_is_satisfied() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::PreflightSequencingObligation,
        "finish-after-witness",
        supported_command_batch_posture(),
    );
    let mut runtime = runtime_with_registration(registration);
    let execution_context = ForgeQueryGraphObligationExecutionContext::bounded()
        .with_preflight_witness(ForgeQueryGraphObligationPreflightWitness::satisfied(
            "phase-eleven-preflight-witness",
        ));

    let receipt = runtime
        .write_batch_with_graph_obligation_execution_context(
            vec![task_insert_command("finish-after-witness")],
            execution_context,
        )
        .expect("satisfied preflight witness should allow write execution");
    let row = receipt
        .obligation_dispatch()
        .expect("preflight dispatch should be attached")
        .execution_results()
        .expect("preflight dispatch should carry execution evidence")
        .rows()
        .first()
        .expect("preflight execution should project evidence")
        .clone();

    assert_eq!(
        row.status(),
        ForgeQueryGraphObligationExecutionStatus::Executed
    );
    assert_eq!(
        row.verdict()
            .and_then(ForgeQueryGraphObligationVerdict::context),
        Some("preflight-prerequisite-satisfied")
    );
}
