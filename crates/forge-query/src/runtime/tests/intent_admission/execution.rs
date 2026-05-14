use super::*;

#[test]
fn execute_intent_delegates_to_canonical_admission_and_execution_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "canonical-runtime-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1", "title": "Intent committed title"}),
    );
    let canonical = runtime
        .admit_authoritative_intent_for_execution(declaration.clone())
        .expect("canonical handoff should admit");
    let canonical_binding = runtime.prepare_authoritative_intent_execution_binding(canonical);
    let delegated = runtime
        .execute_intent(declaration)
        .expect("delegated entrypoint should execute");
    let canonical_receipt = runtime
        .execute_authoritative_intent_execution_binding(canonical_binding)
        .expect("canonical handoff should execute");

    assert_eq!(
        delegated.execution_binding_digest(),
        canonical_receipt.execution_binding_digest()
    );
    assert_eq!(
        delegated.execution_handoff_digest(),
        canonical_receipt.execution_handoff_digest()
    );
    assert_eq!(
        delegated.execution_provenance_chain_digest(),
        canonical_receipt.execution_provenance_chain_digest()
    );
    assert_eq!(
        delegated.decision_trace_envelope().trace_digest(),
        canonical_receipt.decision_trace_envelope().trace_digest()
    );
}

#[test]
fn execute_next_effect_write_intent_delegates_to_canonical_admission_and_execution_handoff() {
    let mut delegated_runtime = intent_runtime_with_authority(TestIntentAuthority);
    let delegated_live = delegated_runtime
        .declare_live_view::<Value>("tasks.effect-admission", task_live_request(), task_schema())
        .expect("live should declare");
    let delegated_effect = delegated_runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-admission",
            ForgeQueryEffectTrigger::live_view(&delegated_live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    delegated_runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: "task-1".to_string(),
            aspect_path: "title.value".to_string(),
            value: json!("title from write"),
        })
        .expect("write should queue pending effect intent");

    let delegated = delegated_runtime
        .execute_next_effect_write_intent(&delegated_effect, "1.0", "effect.intent.input.v1")
        .expect("legacy effect entrypoint should execute");

    let mut canonical_runtime = intent_runtime_with_authority(TestIntentAuthority);
    let canonical_live = canonical_runtime
        .declare_live_view::<Value>("tasks.effect-admission", task_live_request(), task_schema())
        .expect("live should declare");
    let canonical_effect = canonical_runtime
        .declare_effect::<Value>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-admission",
            ForgeQueryEffectTrigger::live_view(&canonical_live, ["title.value"]),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    canonical_runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: "task-1".to_string(),
            aspect_path: "title.value".to_string(),
            value: json!("title from write"),
        })
        .expect("write should queue pending effect intent");
    let (pending_delivery, canonical_handoff) = canonical_runtime
        .admit_next_effect_write_intent_for_execution(
            canonical_effect.name(),
            "1.0",
            "effect.intent.input.v1",
        )
        .expect("canonical effect handoff should admit");
    let canonical_binding = canonical_runtime
        .prepare_effect_intent_execution_binding(canonical_handoff, &pending_delivery);
    let canonical_receipt = canonical_runtime
        .execute_effect_intent_execution_binding(canonical_binding)
        .expect("canonical effect handoff should execute");

    assert_eq!(
        delegated.intent_receipt().execution_binding_digest(),
        canonical_receipt
            .intent_receipt()
            .execution_binding_digest()
    );
    assert_eq!(
        delegated.intent_receipt().execution_handoff_digest(),
        canonical_receipt
            .intent_receipt()
            .execution_handoff_digest()
    );
    assert_eq!(
        delegated
            .intent_receipt()
            .execution_provenance_chain_digest(),
        canonical_receipt
            .intent_receipt()
            .execution_provenance_chain_digest()
    );
    assert_eq!(
        delegated
            .intent_receipt()
            .decision_trace_envelope()
            .trace_digest(),
        canonical_receipt
            .intent_receipt()
            .decision_trace_envelope()
            .trace_digest()
    );
}

#[test]
fn denied_intent_stops_before_backend_execution() {
    let attempted = Rc::new(Cell::new(0));
    let mut runtime = intent_runtime_with_authority(CountingIntentAuthority {
        attempted: attempted.clone(),
    });
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "denied-runtime-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        json!({"entity": "task-1"}),
    )
    .with_source_lane(ForgeQueryIntentSourceLane::EffectTriggered);

    let error = runtime
        .execute_intent(declaration)
        .expect_err("mismatched source lane should deny");

    assert_eq!(attempted.get(), 0);
    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            stage, evidence, ..
        } => {
            assert_eq!(stage, "source-lane-admission");
            let decision_trace = evidence
                .decision_trace_envelope()
                .expect("covered denial should expose a decision trace");
            assert_eq!(
                decision_trace.kind(),
                ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop
            );
            assert_eq!(
                trace_stages(decision_trace),
                vec![
                    ForgeQueryIntentDecisionTraceStage::RawIntent,
                    ForgeQueryIntentDecisionTraceStage::Eligibility,
                    ForgeQueryIntentDecisionTraceStage::ViolationStop,
                ]
            );
        }
        other => panic!("expected intent denial, got {other:?}"),
    }
}

#[test]
fn admitted_intent_receipt_exposes_linear_decision_trace() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let receipt = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "traceable-runtime-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1", "title": "Intent committed title"}),
        ))
        .expect("intent should execute");
    let decision_trace = receipt.decision_trace_envelope();

    assert_eq!(
        decision_trace.kind(),
        ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution
    );
    assert_eq!(
        trace_stages(decision_trace),
        vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ]
    );
    assert_eq!(decision_trace.family().as_str(), receipt.admission_family());
    assert_eq!(
        decision_trace.entrypoint().as_str(),
        receipt.covered_entrypoint_label()
    );
    assert!(!decision_trace.trace_digest().is_empty());
}

#[test]
fn execution_denial_keeps_admission_and_execution_provenance_in_trace() {
    let mut runtime = intent_runtime_with_authority(InvariantViolationIntentAuthority);
    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "traceable-invariant-denial",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            json!({"entity": "task-1", "dependency": "cycle"}),
        ))
        .expect_err("invariant violation must deny");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => {
            let decision_trace = evidence
                .decision_trace_envelope()
                .expect("covered execution denial should expose a decision trace");
            assert_eq!(
                decision_trace.kind(),
                ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop
            );
            assert_eq!(
                trace_stages(decision_trace),
                vec![
                    ForgeQueryIntentDecisionTraceStage::RawIntent,
                    ForgeQueryIntentDecisionTraceStage::Eligibility,
                    ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
                    ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
                    ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
                    ForgeQueryIntentDecisionTraceStage::ViolationStop,
                ]
            );
            assert_eq!(
                decision_trace.trace_digest(),
                evidence
                    .decision_trace_envelope()
                    .expect("trace should still be present")
                    .trace_digest()
            );
        }
        other => panic!("expected invariant-denial evidence, got {other:?}"),
    }
}
