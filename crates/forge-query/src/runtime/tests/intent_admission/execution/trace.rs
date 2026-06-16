use super::*;

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
    match decision_trace.rows()[1].evidence() {
        ForgeQueryIntentDecisionTraceEvidence::Eligibility(evidence) => {
            assert_eq!(
                decision_trace.rows()[1].evidence_owner(),
                ForgeQueryIntentDecisionTraceEvidenceOwner::QueryIntentEligibility
            );
            assert_eq!(
                evidence.support_posture(),
                ForgeQueryIntentAdmissionSupportEligibility::Admitted
            );
            assert_eq!(
                evidence.capability_posture(),
                ForgeQueryIntentAdmissionCapabilityEligibility::Admitted
            );
            assert_eq!(
                evidence.policy_posture(),
                ForgeQueryIntentAdmissionPolicyEligibility::NotApplicableForRuntimeIntentFloor
            );
            assert_eq!(
                evidence.basis_posture(),
                ForgeQueryIntentAdmissionBasisEligibility::NotApplicableForRuntimeIntentFloor
            );
            assert_eq!(
                evidence.invariant_posture(),
                ForgeQueryIntentAdmissionInvariantEligibility::PreExecutionAuthorityRequired
            );
            assert_eq!(
                evidence.projection_source_posture(),
                ForgeQueryIntentAdmissionProjectionSourceEligibility::NotApplicableForRuntimeIntentFloor
            );
            assert_eq!(
                evidence.routing_support_posture(),
                ForgeQueryIntentAdmissionRoutingSupportEligibility::CoveredExecutionSeam(
                    crate::facade::runtime::ForgeQueryIntentAdmissionExecutionSeam::BackendIntentAuthorityRoute
                )
            );
            assert_eq!(
                evidence.source_lane_posture(),
                ForgeQueryIntentAdmissionSourceLaneEligibility::MatchesExpected(
                    ForgeQueryIntentSourceLane::UserAuthored
                )
            );
            assert_eq!(
                evidence.authority_lane_posture(),
                ForgeQueryIntentAdmissionAuthorityLaneEligibility::MatchesExpected(
                    ForgeQueryAuthorityLane::AuthoritativeTruth
                )
            );
        }
        other => panic!("expected structured eligibility evidence, got {other:?}"),
    }
    assert!(!decision_trace.trace_digest().is_empty());
    let consumer = receipt.consumer_inspection();
    assert_eq!(
        consumer.outcome_class(),
        ForgeQueryIntentConsumerOutcomeClass::Admitted
    );
    assert_eq!(
        consumer.decision_trace_envelope_kind(),
        Some(ForgeQueryIntentDecisionTraceEnvelopeKind::AdmittedExecution)
    );
    assert_eq!(
        consumer.execution_provenance_chain_digest(),
        Some(receipt.execution_provenance_chain_digest())
    );
    assert_eq!(
        consumer.terminal_stage(),
        Some(ForgeQueryIntentDecisionTraceStage::ExecutionOutcome)
    );
}

#[test]
fn effect_triggered_trace_eligibility_preserves_write_adjacent_trigger_proof() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let origin_identity = test_write_adjacent_origin_identity(
        ForgeQueryEffectWriteAdjacentTriggerClass::RemaskDrift,
        "remask-drift:cause:task-title",
    );
    let live = runtime
        .declare_live_view::<Value>("tasks.trace-follow-on", task_live_request(), task_schema())
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<Value>(
            ForgeQueryEffectDeclaration::write_intent(
                "effects.trace-follow-on",
                ForgeQueryEffectTrigger::live_view(&live, ["title.value"]),
                "strategy.intent.reconcile",
            )
            .with_write_adjacent_trigger(
                ForgeQueryEffectWriteAdjacentTriggerClass::RemaskDrift,
                origin_identity.clone(),
            ),
        )
        .expect("remask drift effect should declare");

    runtime
        .write(ForgeQueryWriteCommand::UpdateAspect {
            entity_identity: crate::memory_workspace::admit_authored_entity_label(
                "task-1",
            ),
            aspect_path: "title.value".to_string(),
            value: json!("title from remask drift"),
        })
        .expect("write should queue pending intent");

    let receipt = runtime
        .next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .execute()
        .expect("effect-triggered intent should execute");
    let decision_trace = receipt.intent_receipt().decision_trace_envelope();

    match decision_trace.rows()[1].evidence() {
        ForgeQueryIntentDecisionTraceEvidence::Eligibility(evidence) => {
            let trigger = evidence
                .write_adjacent_trigger()
                .expect("effect-triggered trace should retain write-adjacent trigger proof");
            assert_eq!(
                trigger.class(),
                ForgeQueryEffectWriteAdjacentTriggerClass::RemaskDrift
            );
            assert_eq!(trigger.origin_evidence_identity(), &origin_identity);
        }
        other => panic!("expected structured eligibility evidence, got {other:?}"),
    }
}
