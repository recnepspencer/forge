use super::*;

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
        test_intent_input([("entity", "task-1")]),
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
            match decision_trace.rows()[1].evidence() {
                ForgeQueryIntentDecisionTraceEvidence::Eligibility(evidence) => {
                    assert_eq!(
                        evidence.capability_posture(),
                        ForgeQueryIntentAdmissionCapabilityEligibility::Violation {
                            stage: "source-lane-admission",
                            detail:
                                "covered-runtime-entrypoint-rejects-effect-triggered-source-lane",
                        }
                    );
                    assert_eq!(
                        evidence.source_lane_posture(),
                        ForgeQueryIntentAdmissionSourceLaneEligibility::Mismatch {
                            expected: ForgeQueryIntentSourceLane::UserAuthored,
                            actual: ForgeQueryIntentSourceLane::EffectTriggered,
                        }
                    );
                }
                other => panic!("expected structured eligibility evidence, got {other:?}"),
            }
        }
        other => panic!("expected intent denial, got {other:?}"),
    }
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
            test_intent_input([("entity", "task-1"), ("dependency", "cycle")]),
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
            let consumer = evidence.consumer_inspection();
            assert_eq!(
                consumer.outcome_class(),
                ForgeQueryIntentConsumerOutcomeClass::Violation
            );
            assert_eq!(
                consumer.decision_trace_envelope_kind(),
                Some(ForgeQueryIntentDecisionTraceEnvelopeKind::ViolationStop)
            );
            assert_eq!(
                consumer.execution_provenance_chain_digest(),
                evidence
                    .execution_provenance()
                    .map(|provenance| provenance.execution_provenance_chain_digest())
            );
            assert_eq!(
                consumer.terminal_stage(),
                Some(ForgeQueryIntentDecisionTraceStage::ViolationStop)
            );
        }
        other => panic!("expected invariant-denial evidence, got {other:?}"),
    }
}
