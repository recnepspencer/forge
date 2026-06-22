use super::support::*;

#[test]
fn diagnostic_only_posture_remains_diagnostic_execution_evidence() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::CapabilityGapScreen,
        "diagnostic-only-capability-gap",
        ForgeQueryGraphObligationSupportPosture::diagnostic_only(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("diagnostic-only-capability-gap")])
        .expect("diagnostic-only posture should not block write execution");
    let execution = receipt
        .obligation_dispatch()
        .and_then(|dispatch| dispatch.execution_results())
        .expect("diagnostic-only posture should still attach execution evidence");
    let row = execution
        .rows()
        .first()
        .expect("diagnostic-only posture should record a result row");

    assert_eq!(
        row.status(),
        ForgeQueryGraphObligationExecutionStatus::DiagnosticOnly
    );
    assert!(row
        .verdict()
        .expect("diagnostic-only row should carry advisory evidence")
        .is_advisory());
}

#[test]
fn deferred_to_backstop_posture_cites_delegation_without_blocking_write() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::PreflightSequencingObligation,
        "deferred-preflight-backstop",
        ForgeQueryGraphObligationSupportPosture::deferred_to_backstop(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command("deferred-preflight-backstop")])
        .expect("deferred-to-backstop posture should not block at this lane");
    let execution = receipt
        .obligation_dispatch()
        .and_then(|dispatch| dispatch.execution_results())
        .expect("deferred-to-backstop posture should still attach execution evidence");
    let row = execution
        .rows()
        .first()
        .expect("deferred-to-backstop posture should record a result row");

    assert_eq!(
        row.status(),
        ForgeQueryGraphObligationExecutionStatus::DeferredToBackstop
    );
    assert_eq!(
        row.verdict()
            .and_then(ForgeQueryGraphObligationVerdict::context),
        Some("selected-deferred-to-backstop")
    );
}

#[test]
fn unsupported_posture_blocks_with_typed_support_denial() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        "unsupported-blocking-invariant",
        ForgeQueryGraphObligationSupportPosture::unsupported(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let error = runtime
        .write_batch(vec![task_insert_command("unsupported-blocking-invariant")])
        .expect_err("unsupported posture should fail closed at selected execution");

    match error {
        ForgeQueryRuntimeError::GraphObligationDenied(denial) => {
            let row = denial
                .rows()
                .first()
                .expect("unsupported denial should project one blocking row");
            assert_eq!(
                row.execution_status(),
                ForgeQueryGraphObligationExecutionStatus::Unsupported
            );
            assert_eq!(
                row.verdict_context(),
                Some("selected-obligation-unsupported")
            );
        }
        other => panic!("expected unsupported graph obligation denial, got {other:?}"),
    }
}

#[test]
fn not_applicable_posture_records_non_blocking_state_load_result() {
    let registration = task_collection_registration(
        ForgeQueryGraphObligationKind::BlockingInvariant,
        "not-applicable-blocking-invariant",
        ForgeQueryGraphObligationSupportPosture::not_applicable(
            ForgeQueryGraphObligationSupportLane::AuthoritativeCommandBatch,
        ),
    );
    let mut runtime = runtime_with_registration(registration);

    let receipt = runtime
        .write_batch(vec![task_insert_command(
            "not-applicable-blocking-invariant",
        )])
        .expect("not-applicable posture should not block write execution");
    let row = receipt
        .obligation_dispatch()
        .and_then(|dispatch| dispatch.execution_results())
        .and_then(|execution| execution.rows().first())
        .expect("not-applicable posture should record a result row");

    assert_eq!(
        row.status(),
        ForgeQueryGraphObligationExecutionStatus::NotApplicableAfterStateLoad
    );
    assert!(row.verdict().is_none());
}
