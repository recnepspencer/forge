use super::support::*;

#[test]
fn capability_gap_screen_blocks_with_typed_obligation_denial() {
    let registration = task_collection_registration(
        WorthQueryGraphObligationKind::CapabilityGapScreen,
        "capability-gap",
        supported_command_batch_posture(),
    );
    let mut runtime = runtime_with_registration(registration);

    let error = runtime
        .write_batch(vec![task_insert_command("capability-gap")])
        .expect_err("capability gap screen should block before write execution");

    match error {
        WorthQueryRuntimeError::GraphObligationDenied(denial) => {
            assert_eq!(denial.blocking_count(), 1);
            let row = denial
                .rows()
                .first()
                .expect("capability gap denial should project the blocking row");
            assert_eq!(
                row.obligation_kind(),
                WorthQueryGraphObligationKind::CapabilityGapScreen
            );
            assert_eq!(
                row.execution_status(),
                WorthQueryGraphObligationExecutionStatus::Executed
            );
            assert_eq!(row.verdict(), "block");
            assert_eq!(
                row.verdict_context(),
                Some("capability-gap-screen-selected")
            );
            assert_eq!(
                row.support_lane(),
                WorthQueryGraphObligationSupportLane::AuthoritativeCommandBatch
            );
            assert!(!row.execution_input_digest().is_empty());
            assert!(!row.executor_contract_digest().is_empty());
            let attachment = denial.attachment_projection();
            let attachment_row = attachment
                .rows()
                .first()
                .expect("runtime denial should expose attachment row");
            assert_eq!(attachment.rows().len(), 1);
            assert_eq!(
                attachment.execution_point(),
                WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
            );
            assert_eq!(
                attachment_row.envelope_digest(),
                attachment.envelope_digest()
            );
            assert_eq!(
                attachment_row.execution_point(),
                WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
            );
            assert_eq!(
                attachment_row.execution_input_digest(),
                row.execution_input_digest()
            );
            assert_eq!(
                attachment_row.rule_identity_digest(),
                row.rule_identity_digest()
            );
            assert!(!attachment_row.touch_descriptor_digest().is_empty());
            assert!(!attachment_row.operating_world_digest().is_empty());
            assert!(!attachment_row.dispatch_plan_digest().is_empty());
            assert!(!attachment.projection_digest().is_empty());
        }
        other => panic!("expected graph obligation denial, got {other:?}"),
    }
}

#[test]
fn capability_gap_denial_attachment_cites_dispatch_and_world_evidence() {
    let mut runtime = runtime_with_registration(task_collection_registration(
        WorthQueryGraphObligationKind::CapabilityGapScreen,
        "capability-gap-attachment",
        supported_command_batch_posture(),
    ));
    let error = runtime
        .write_batch(vec![task_insert_command("capability-gap-attachment")])
        .expect_err("capability gap should deny through the runtime boundary");

    let WorthQueryRuntimeError::GraphObligationDenied(denial) = error else {
        panic!("expected graph obligation denial, got {error:?}");
    };
    let attachment = denial.attachment_projection();
    let row = denial.rows().first().expect("denial should have one row");
    let attachment_row = attachment
        .rows()
        .first()
        .expect("denial attachment should have one row");

    assert_eq!(
        attachment.execution_point(),
        WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
    );
    assert_eq!(
        attachment_row.envelope_digest(),
        attachment.envelope_digest()
    );
    assert_eq!(
        attachment_row.execution_point(),
        WorthQueryGraphObligationDispatchContextKind::AuthoritativeCommandBatch
    );
    assert_eq!(
        attachment_row.execution_status(),
        WorthQueryGraphObligationExecutionStatus::Executed
    );
    assert_eq!(
        attachment_row.execution_input_digest(),
        row.execution_input_digest()
    );
    assert!(!attachment_row.touch_descriptor_digest().is_empty());
    assert!(!attachment_row.operating_world_digest().is_empty());
    assert!(!attachment_row.dispatch_plan_digest().is_empty());
}
