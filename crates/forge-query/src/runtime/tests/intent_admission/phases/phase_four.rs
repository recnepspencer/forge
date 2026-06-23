use super::*;

#[test]
fn authoritative_admitted_handoff_materializes_execution_binding() {
    let runtime = intent_runtime_with_authority(TestIntentAuthority);
    let handoff = runtime
        .admit_authoritative_intent_for_execution(ForgeQueryIntentDeclaration::strategy_commit(
            "phase-four-authoritative-intent",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1")]),
        ))
        .expect("authoritative handoff should admit");

    let binding = runtime.prepare_authoritative_intent_execution_binding(handoff.clone());

    assert_eq!(binding.family(), handoff.family());
    assert_eq!(binding.entrypoint(), handoff.entrypoint());
    assert_eq!(binding.execution_seam(), handoff.execution_seam());
    assert_eq!(binding.handoff().handoff_digest(), handoff.handoff_digest());
    assert!(!binding.binding_digest().is_empty());
}

#[test]
fn effect_admitted_handoff_materializes_self_contained_execution_binding() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.phase-four-effect",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.phase-four-binding",
            ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from binding",
        ))
        .expect("write should queue pending effect intent");

    let (pending_delivery, handoff) = runtime
        .admit_next_effect_write_intent_for_execution(
            effect.name(),
            "1.0",
            "effect.intent.input.v1",
        )
        .expect("effect handoff should admit");
    let binding =
        runtime.prepare_effect_intent_execution_binding(handoff.clone(), &pending_delivery);

    assert_eq!(binding.family(), handoff.family());
    assert_eq!(binding.entrypoint(), handoff.entrypoint());
    assert_eq!(binding.execution_seam(), handoff.execution_seam());
    assert_eq!(binding.handoff().handoff_digest(), handoff.handoff_digest());
    assert_eq!(binding.effect_name(), pending_delivery.effect_name());
    assert_eq!(
        binding.trigger_commit_identity(),
        pending_delivery.commit_identity()
    );
    assert!(!binding.pending_delivery_digest().is_empty());
    assert!(!binding.binding_digest().is_empty());
}

#[test]
fn effect_receipt_surfaces_provenance_without_nested_receipt_spelunking() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.phase-four-effect-surface",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.phase-four-surface",
            ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from outer receipt",
        ))
        .expect("write should queue pending effect intent");

    let receipt = runtime
        .execute_next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .expect("effect intent should execute");

    assert_eq!(
        receipt.covered_entrypoint_label(),
        "ForgeQueryRuntime::execute_next_effect_write_intent"
    );
    assert_eq!(
        receipt.execution_binding_digest(),
        receipt.intent_receipt().execution_binding_digest()
    );
    assert_eq!(
        receipt.execution_provenance_chain_digest(),
        receipt.intent_receipt().execution_provenance_chain_digest()
    );
    assert_eq!(
        receipt.decision_trace_envelope().trace_digest(),
        receipt
            .intent_receipt()
            .decision_trace_envelope()
            .trace_digest()
    );
}

#[test]
fn execution_denial_evidence_retains_execution_provenance_artifact() {
    let mut runtime = intent_runtime_with_authority(InvariantViolationIntentAuthority);
    let error = runtime
        .execute_intent(ForgeQueryIntentDeclaration::strategy_commit(
            "phase-four-provenance-denial",
            "strategy.intent.reconcile",
            "1.0",
            "intent.reconcile.input.v1",
            test_intent_input([("entity", "task-1"), ("dependency", "cycle")]),
        ))
        .expect_err("invariant violation must deny");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied { evidence, .. } => {
            let provenance = evidence
                .execution_provenance()
                .expect("execution-time denial should retain provenance");
            assert_eq!(
                provenance.execution_provenance_chain_digest(),
                evidence
                    .execution_provenance()
                    .expect("provenance should still be present")
                    .execution_provenance_chain_digest()
            );
            assert!(!provenance.execution_binding_digest().is_empty());
        }
        other => panic!("expected intent denial, got {other:?}"),
    }
}

#[test]
fn post_execution_routing_failure_preserves_proof_chain() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "phase-four-routing-failure",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        test_intent_input([("entity", "task-1")]),
    );
    let handoff = runtime
        .admit_authoritative_intent_for_execution(declaration.clone())
        .expect("authoritative handoff should admit");
    let binding = runtime.prepare_authoritative_intent_execution_binding(handoff.clone());
    let execution = runtime
        .backend
        .execute_intent(binding.declaration())
        .expect("backend execution should succeed");
    let admitted_handoff = ForgeQueryAdmittedIntentExecutionHandoff::from(handoff);
    let snapshot_evidence_identity = execution
        .mutation_receipt()
        .snapshot_identity
        .evidence_identity();
    let execution_provenance =
        ForgeQueryIntentExecutionProvenance::for_shared_execution_typed_parts(
            binding.family(),
            binding.entrypoint(),
            binding.execution_seam(),
            binding.handoff().decision_digest(),
            binding.handoff().handoff_digest(),
            binding.binding_digest(),
            execution.outcome_digest(),
            &snapshot_evidence_identity,
        );
    let decision_trace_envelope = ForgeQueryIntentDecisionTraceEnvelope::for_admitted_execution(
        &admitted_handoff,
        &execution,
    );
    let error = runtime.intent_execution_routing_error(
        &declaration,
        &execution,
        execution_provenance.clone(),
        decision_trace_envelope.clone(),
        ForgeQueryRuntimeError::LiveSubscriptionInstallation {
            view_name: "tasks.phase-four-routing".to_string(),
            stage: "delivery-window",
            message: "simulated route failure".to_string(),
        },
    );

    match error {
        ForgeQueryRuntimeError::IntentExecutionRoutingFailed {
            stage,
            message,
            evidence,
            source,
            ..
        } => {
            assert_eq!(stage, "post-execution-routing");
            assert!(message.contains("simulated route failure"));
            assert_eq!(
                evidence
                    .execution_provenance()
                    .execution_provenance_chain_digest(),
                execution_provenance.execution_provenance_chain_digest()
            );
            assert_eq!(
                evidence.decision_trace_envelope().trace_digest(),
                decision_trace_envelope.trace_digest()
            );
            assert!(source.to_string().contains("simulated route failure"));
        }
        other => panic!("expected intent execution routing failure, got {other:?}"),
    }
}

#[test]
fn stale_effect_execution_binding_fails_as_typed_handoff_violation() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.phase-four-stale-effect",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.phase-four-stale",
            ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title before stale execution",
        ))
        .expect("write should queue pending effect intent");

    let (pending_delivery, handoff) = runtime
        .admit_next_effect_write_intent_for_execution(
            effect.name(),
            "1.0",
            "effect.intent.input.v1",
        )
        .expect("effect handoff should admit");
    let binding = runtime.prepare_effect_intent_execution_binding(handoff, &pending_delivery);
    let effect_target = ForgeQueryEffectTarget::from_name(effect.name());
    runtime.remove_pending_effect_delivery(&effect_target, 0, &pending_delivery);

    let error = runtime
        .execute_effect_intent_execution_binding(binding)
        .expect_err("stale binding should fail as a typed violation");

    match error {
        ForgeQueryRuntimeError::IntentCommitDenied {
            stage, evidence, ..
        } => {
            assert_eq!(stage, "pending-write-intent-binding");
            let decision_trace = evidence
                .decision_trace_envelope()
                .expect("stale binding denial should expose a decision trace");
            assert_eq!(
                trace_stages(decision_trace),
                vec![
                    ForgeQueryIntentDecisionTraceStage::RawIntent,
                    ForgeQueryIntentDecisionTraceStage::Eligibility,
                    ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
                    ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
                    ForgeQueryIntentDecisionTraceStage::ViolationStop,
                ]
            );
        }
        other => panic!("expected typed stale-binding denial, got {other:?}"),
    }
}
