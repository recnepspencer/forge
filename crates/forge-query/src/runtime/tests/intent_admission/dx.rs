use super::*;

#[test]
fn intent_common_path_helper_executes_through_canonical_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let receipt = runtime
        .intent(ForgeQueryIntentDeclaration::strategy_commit(
            "helper-runtime-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1", "title": "Intent committed title"}),
        ))
        .execute()
        .expect("common-path helper should execute");

    assert_eq!(
        receipt.covered_entrypoint_label(),
        "ForgeQueryRuntime::execute_intent"
    );
    assert_eq!(
        trace_stages(receipt.decision_trace_envelope()),
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
fn intent_advanced_path_helper_exposes_request_eligibility_decision_and_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let review = runtime
        .intent(ForgeQueryIntentDeclaration::strategy_commit(
            "advanced-helper-runtime-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1"}),
        ))
        .review()
        .expect("advanced path should review");

    assert_eq!(
        review.request().entrypoint().as_str(),
        "ForgeQueryRuntime::execute_intent"
    );
    assert_eq!(
        review.request().request_digest(),
        review.eligibility().request().request_digest()
    );
    let decision = review.decision();
    let handoff = review
        .admitted_handoff()
        .expect("admitted review should expose a handoff");
    match decision {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            assert_eq!(plan.decision_digest(), handoff.decision_digest());
        }
        other => panic!("expected admitted review, got {other:?}"),
    }
}

#[test]
fn effect_write_intent_common_path_helper_executes_through_canonical_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let live = runtime
        .declare_live_view::<Value>("tasks.effect-dx", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-dx",
            ForgeQueryEffectTrigger::live_view(&live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: "task-1".to_string(),
            aspect_path: "title.value".to_string(),
            value: json!("title from helper"),
        })
        .expect("write should queue pending effect intent");

    let receipt = runtime
        .next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .execute()
        .expect("effect common-path helper should execute");

    assert_eq!(
        receipt.intent_receipt().covered_entrypoint_label(),
        "ForgeQueryRuntime::execute_next_effect_write_intent"
    );
    assert_eq!(receipt.effect_name(), "effects.reconcile-dx");
}

#[test]
fn canonical_admission_decision_round_trips_to_public_handoff_type() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "canonical-admission-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    );
    let request = crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
        declaration,
    )
    .expect("authoritative request should build");
    let decision = admit_runtime_intent_request(request);

    match decision {
        ForgeQueryIntentAdmissionDecision::Admitted(plan) => {
            let handoff: ForgeQueryAdmittedIntentExecutionHandoff = plan.into_execution_handoff();
            assert_eq!(
                handoff.entrypoint(),
                ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteIntent
            );
            assert!(!handoff.handoff_digest().is_empty());
        }
        other => panic!("expected admitted decision, got {other:?}"),
    }
}

#[test]
fn advisory_review_data_preserves_non_panicking_trace_shape() {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "advisory-runtime-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    );
    let request =
        crate::intent_admission::ForgeQueryRawIntentAdmissionRequest::authoritative_runtime_entrypoint(
            declaration,
        )
        .expect("authoritative request should build");
    let eligibility = crate::intent_admission::ForgeQueryIntentAdmissionEligibility::from_request(
        request.clone(),
    );
    let advisory = crate::intent_admission::ForgeQueryIntentAdvisoryDecision::new(
        request.family(),
        request.entrypoint(),
        "materialized-detail-advisory",
        "full execution is intentionally deferred",
        request.request_digest(),
        eligibility.eligibility_digest(),
    );
    let review =
        crate::intent_admission::dx::ForgeQueryRuntimeIntentAdmissionReviewData::from_decision(
            request,
            ForgeQueryIntentAdmissionDecision::Advisory(advisory),
        );
    let decision_trace = review
        .decision_trace_envelope()
        .expect("advisory review should still produce a trace");

    assert_eq!(
        decision_trace.kind(),
        ForgeQueryIntentDecisionTraceEnvelopeKind::AdvisoryStop
    );
    assert_eq!(
        trace_stages(decision_trace),
        vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdvisoryStop,
        ]
    );
}
