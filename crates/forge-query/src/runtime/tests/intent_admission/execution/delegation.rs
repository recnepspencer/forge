use super::*;

#[test]
fn execute_intent_delegates_to_canonical_admission_and_execution_handoff() {
    let mut runtime = intent_runtime_with_authority(TestIntentAuthority);
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "canonical-runtime-intent",
        "strategy.intent.reconcile",
        "1.0",
        "intent.reconcile.input.v1",
        test_intent_input([("entity", "task-1"), ("title", "Intent committed title")]),
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
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.effect-admission",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let delegated_effect = delegated_runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-admission",
            ForgeQueryEffectTrigger::live_view(
                &delegated_live,
                test_aspect_touches(["title.value"]),
            ),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    delegated_runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from write",
        ))
        .expect("write should queue pending effect intent");

    let delegated = delegated_runtime
        .execute_next_effect_write_intent(&delegated_effect, "1.0", "effect.intent.input.v1")
        .expect("legacy effect entrypoint should execute");

    let mut canonical_runtime = intent_runtime_with_authority(TestIntentAuthority);
    let canonical_live = canonical_runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.effect-admission",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let canonical_effect = canonical_runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.reconcile-admission",
            ForgeQueryEffectTrigger::live_view(
                &canonical_live,
                test_aspect_touches(["title.value"]),
            ),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    canonical_runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from write",
        ))
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
fn effect_write_intent_denies_when_graph_obligations_require_pre_execution_touch_descriptor() {
    let mut runtime = intent_runtime_with_collection_obligation(TestIntentAuthority, "Task");
    let live = runtime
        .declare_live_view::<ForgeQueryNativeRow>(
            "tasks.effect-obligation",
            task_live_request(),
            task_schema(),
        )
        .expect("live should declare");
    let effect = runtime
        .declare_effect::<ForgeQueryNativeRow>(ForgeQueryEffectDeclaration::write_intent(
            "effects.obligation-reconcile",
            ForgeQueryEffectTrigger::live_view(&live, test_aspect_touches(["title.value"])),
            "strategy.intent.reconcile",
        ))
        .expect("write-intent effect should declare");
    runtime
        .write(test_update_string_aspect_command(
            crate::memory_workspace::admit_authored_entity_label("task-1"),
            "title.value",
            "title from write",
        ))
        .expect("write should queue pending effect intent");

    let error = runtime
        .execute_next_effect_write_intent(&effect, "1.0", "effect.intent.input.v1")
        .expect_err("effect write intent should stop before execution without touch descriptor");

    match error {
        ForgeQueryRuntimeError::GraphObligationEffectTouchDescriptorMissing { effect_name } => {
            assert_eq!(effect_name, "effects.obligation-reconcile");
        }
        other => panic!("unexpected effect graph obligation error: {other:?}"),
    }
}

#[test]
fn scalar_write_delegates_to_canonical_admission_and_execution_handoff() {
    let mut delegated_runtime = intent_runtime_with_authority(TestIntentAuthority);
    let command = test_update_string_aspect_command(
        crate::memory_workspace::admit_authored_entity_label("task-1"),
        "title.value",
        "title from scalar write",
    );
    let delegated = delegated_runtime
        .write(command.clone())
        .expect("delegated scalar write should execute");

    let mut canonical_runtime = intent_runtime_with_authority(TestIntentAuthority);
    let review = canonical_runtime
        .review_authoritative_runtime_write(command)
        .expect("canonical write review should succeed");
    let canonical_handoff = canonical_runtime
        .resolve_reviewed_admitted_authoritative_write_handoff(review)
        .expect("canonical write handoff should admit");
    let canonical_binding =
        canonical_runtime.prepare_authoritative_mutation_execution_binding(canonical_handoff);
    let canonical_receipt = canonical_runtime
        .execute_authoritative_mutation_execution_binding(canonical_binding)
        .expect("canonical write binding should execute");

    assert_eq!(
        delegated.execution_provenance_chain_digest(),
        canonical_receipt.execution_provenance_chain_digest()
    );
    assert_eq!(
        delegated
            .decision_trace_envelope()
            .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest),
        canonical_receipt
            .decision_trace_envelope()
            .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest)
    );
    assert_eq!(
        delegated.covered_entrypoint_label(),
        Some("ForgeQueryRuntime::write")
    );
    assert_eq!(
        delegated.admission_family(),
        Some("authoritative-mutation-intent")
    );
}

fn intent_runtime_with_collection_obligation<T: ForgeQueryIntentAuthorityAdapter + 'static>(
    authority: T,
    collection: &str,
) -> ForgeQueryRuntime {
    complete_backend_from_parts_builder()
        .support_profile(intent_support_profile())
        .intent_authority(authority)
        .graph_obligation(
            ForgeQueryGraphObligationRegistration::schema_contract_validator(
                ForgeQueryGraphObligationRuleIdentity::new(
                    "test.effect-graph-obligation",
                    collection,
                    "v1",
                )
                .unwrap(),
                ForgeQueryGraphTouchSelector::collection(collection).unwrap(),
                ForgeQueryGraphObligationOperatingWorldSelector::any_committed_authority(),
            )
            .with_support_posture(ForgeQueryGraphObligationSupportPosture::supported(
                ForgeQueryGraphObligationSupportLane::EffectTriggeredWriteIntent,
            )),
        )
        .build_backend_from_parts()
        .build()
        .expect("runtime should build with intent authority and graph obligation")
}
