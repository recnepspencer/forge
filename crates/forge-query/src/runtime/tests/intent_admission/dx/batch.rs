use super::*;

#[test]
fn write_batch_intent_common_path_helper_executes_through_canonical_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let receipt = runtime
        .write_batch_intent(vec![
            ForgeQueryAspectMutationBuilder::new()
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-batch-helper-1"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("batch helper one"),
                )
                .build_insert("Task")
                .expect("batch command should build"),
            ForgeQueryAspectMutationBuilder::new()
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("task-batch-helper-2"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("batch helper two"),
                )
                .build_insert("Task")
                .expect("batch command should build"),
        ])
        .execute()
        .expect("batch write common-path helper should execute");

    assert_eq!(receipt.write_count(), 2);
    assert_eq!(
        receipt
            .decision_trace_envelope()
            .map(trace_stages)
            .unwrap_or_default(),
        vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ]
    );
}

#[test]
fn write_batch_intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let review = runtime
        .write_batch_intent(vec![ForgeQueryAspectMutationBuilder::new()
            .set_aspect(
                test_aspect_touch("identity.id"),
                test_authored_string_aspect_value("task-batch-advanced-1"),
            )
            .set_aspect(
                test_aspect_touch("title.value"),
                test_authored_string_aspect_value("batch advanced one"),
            )
            .build_insert("Task")
            .expect("batch command should build")])
        .review()
        .expect("advanced batch write path should review");

    assert_eq!(
        review.request().entrypoint().as_str(),
        "ForgeQueryRuntime::write_batch"
    );
    let decision_digest = match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => plan.decision_digest().to_string(),
        other => panic!("expected admitted batch review, got {other:?}"),
    };
    let handoff = review
        .admit()
        .expect("admitted batch review should expose handoff");
    assert_eq!(decision_digest, handoff.handoff().decision_digest());
}

#[test]
fn workspace_write_batch_intent_common_path_helper_executes_through_canonical_handoff() {
    let mut workspace = intent_runtime_with_authority(TestIntentAuthority)
        .workspace("intent-admission-workspace-batch")
        .expect("workspace should open");
    let receipt = workspace
        .write_batch_intent(vec![
            ForgeQueryAspectMutationBuilder::new()
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("workspace-batch-helper-1"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("workspace batch helper one"),
                )
                .build_insert("Task")
                .expect("batch command should build"),
            ForgeQueryAspectMutationBuilder::new()
                .set_aspect(
                    test_aspect_touch("identity.id"),
                    test_authored_string_aspect_value("workspace-batch-helper-2"),
                )
                .set_aspect(
                    test_aspect_touch("title.value"),
                    test_authored_string_aspect_value("workspace batch helper two"),
                )
                .build_insert("Task")
                .expect("batch command should build"),
        ])
        .execute()
        .expect("workspace batch write common-path helper should execute");

    assert_eq!(receipt.write_count(), 2);
    assert_eq!(
        receipt
            .decision_trace_envelope()
            .map(trace_stages)
            .unwrap_or_default(),
        vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ]
    );
}
