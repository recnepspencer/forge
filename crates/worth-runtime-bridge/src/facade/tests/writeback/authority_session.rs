use super::support::*;

#[test]
fn writeback_batch_mutation_authority_bundle_aggregates_component_evidence() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::admit_bridge_owned(
                    "writeback:batch-mutation-authority-bundle",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-bundle",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::admit_bridge_owned(
            "causality:batch-mutation-authority-bundle:a",
        ),
        "batch-mutation-authority-bundle:a",
        BridgeWritebackEffectIdentity::admit_bridge_owned(
            "effect:batch-mutation-authority-bundle:a",
        ),
        "batch-mutation-authority-bundle:a",
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned(
            "idempotence:batch-mutation-authority-bundle:a",
        ),
    );
    let component_b = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::admit_bridge_owned(
            "causality:batch-mutation-authority-bundle:b",
        ),
        "batch-mutation-authority-bundle:b",
        BridgeWritebackEffectIdentity::admit_bridge_owned(
            "effect:batch-mutation-authority-bundle:b",
        ),
        "batch-mutation-authority-bundle:b",
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned(
            "idempotence:batch-mutation-authority-bundle:b",
        ),
    );

    let aggregate = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[
        component_a,
        component_b,
    ])
    .expect("non-empty component set should aggregate");

    assert_eq!(aggregate.component_count(), 2);
    assert_eq!(aggregate.causality_bundle_count(), 2);
    assert_eq!(aggregate.provenance_bundle_count(), 2);
    assert_eq!(aggregate.outcome_class_count(), 2);
    assert_eq!(aggregate.authority_request_count(), 2);
    assert_eq!(aggregate.authority_receipt_count(), 2);
    assert!(!aggregate.aggregate_causality_digest().is_empty());
    assert!(!aggregate.aggregate_provenance_digest().is_empty());
}

fn execute_bridge_mutation_bundle(
    runtime: &RuntimeBridge,
    lowered_policy: &crate::facade::LoweredBridgeExecutionPolicy,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    causality_identity: BridgeWritebackCausalityIdentity,
    truth_trigger_evidence_text: &str,
    effect_identity: BridgeWritebackEffectIdentity,
    effect_intent_value: &str,
    idempotence_identity: BridgeWritebackIdempotenceIdentity,
) -> crate::facade::BridgeMutationAuthorityBundle {
    let effect_intent = writeback_effect_intent(
        BridgeWritebackEffectClass::ProjectedStateDiff,
        effect_intent_value,
    );
    let causality = mutation_causality_basis(
        causality_identity,
        truth_trigger_evidence_text,
        &effect_intent,
    );
    let effect =
        runtime.lower_writeback_effect(contract, &causality, effect_identity, effect_intent);
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        lowered_policy,
        &truth_state_basis(&effect),
        idempotence_identity,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    runtime
        .execute_writeback_mutation_authority(contract, &effect, &idempotence, &causality)
        .expect("matching writeback execution should mint mutation authority")
}

#[test]
fn runtime_records_native_writeback_execution_record_on_pre_authority_failure() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::admit_bridge_owned(
                    "writeback:execution-record-failure",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "execution-record-failure",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:execution-record-failure"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:execution-record-failure"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "execution-record-failure",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned(
            "idempotence:execution-record-failure",
        ),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let changed_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::admit_bridge_owned(
            "effect:execution-record-failure:changed",
        ),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "execution-record-failure:changed",
        ),
    );
    let changed_feedback_provenance = runtime.derive_writeback_feedback_provenance(&changed_effect);
    let changed_feedback_context = crate::facade::BridgeWritebackFeedbackContext::from_provenance(
        &changed_feedback_provenance,
    );

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(&changed_feedback_context),
        )
        .expect_err("different-effect feedback sharing causality must fail closed before authority execution");
    let record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("runtime should retain a native failure execution record");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert_eq!(
        record.failure_class(),
        Some(crate::facade::BridgeWritebackFailureClass::InvariantRejected)
    );
    assert_eq!(record.outcome_digest(), None);
    assert_eq!(record.request_digest(), None);
    assert_eq!(record.receipt_digest(), None);
    assert_eq!(record.counters().writeback_failure_count(), 1);
    assert_eq!(record.counters().writeback_validation_rejection_count(), 1);
}

#[test]
fn runtime_passes_explicit_semantic_fields_to_bound_authority() {
    let authority = InspectingWritebackAuthority::default();
    let runtime =
        runtime_with_custom_writeback_authority(BridgeRuntimePolicy::default(), authority.clone());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::admit_bridge_owned("writeback:authority-request-shape"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "authority-request-shape",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration.clone(), &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:authority-request-shape"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:authority-request-shape"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "authority-request-shape",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned(
            "idempotence:authority-request-shape",
        ),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(&effect, &idempotence, None);
    let strategy_coherence =
        runtime.classify_writeback_strategy_coherence(&contract, &effect, &idempotence);
    let candidate = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_coherence,
        )
        .expect("writeback candidate validation should succeed");

    let (_, receipt) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("bound writeback authority should execute");
    assert_eq!(receipt.effect_intent(), effect.effect_intent());
    assert_eq!(
        receipt.effect_intent().authoritative_patch(),
        effect.effect_intent().authoritative_patch()
    );

    let request = authority
        .take_last_request()
        .expect("authority should have received exactly one request");
    assert_eq!(request.contract_digest(), contract.digest());
    assert_eq!(request.writeback_effect_artifact_digest(), effect.digest());
    assert_eq!(request.effect_intent(), effect.effect_intent());
    assert_eq!(
        request.effect_intent().authoritative_patch(),
        effect.effect_intent().authoritative_patch()
    );
    assert_eq!(
        request.effect_intent_digest(),
        effect.effect_intent_digest()
    );
    assert_eq!(
        request.effect_intent_patch_canonical_basis(),
        effect.effect_intent().patch_canonical_basis()
    );
    assert_eq!(request.family_kind(), effect.family_kind());
    assert_eq!(request.effect_class(), effect.effect_class());
    assert_eq!(request.strategy_class(), effect.strategy_class());
    assert_eq!(request.candidate_digest(), candidate.digest());
    assert_eq!(request.mapped_input_digest(), effect.mapped_input_digest());
    assert_eq!(request.causality_digest(), idempotence.causality_digest());
    assert_eq!(request.idempotence_digest(), idempotence.digest());
    assert_eq!(request.idempotence_class(), idempotence.idempotence_class());
    assert_eq!(
        request.strategy_descriptor_digest(),
        declaration.strategy_descriptor_digest()
    );
    assert_eq!(request.loop_prevention_digest(), loop_prevention.digest());
    assert_eq!(
        request.loop_prevention_disposition(),
        BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    );
    assert_eq!(
        request.strategy_coherence_digest(),
        strategy_coherence.digest()
    );
    let mapper_record = runtime
        .diagnostics()
        .last_writeback_mapper_record()
        .expect("runtime should retain a native writeback mapper record");
    assert_eq!(
        request.mapper_witness_digest(),
        mapper_record.witness_digest()
    );
    assert_eq!(mapper_record.candidate_digest(), candidate.digest());
    assert_eq!(mapper_record.family_kind(), effect.family_kind());
    assert_eq!(mapper_record.effect_class(), effect.effect_class());
    assert_eq!(mapper_record.strategy_class(), effect.strategy_class());
    assert_eq!(
        mapper_record.mapper_envelope_digest(),
        effect.mapper_envelope_digest()
    );
    assert_eq!(
        mapper_record.mapped_input_digest(),
        effect.mapped_input_digest()
    );
    let mapper_explanation = runtime
        .diagnostics()
        .explain_last_writeback_mapper_record()
        .expect("writeback mapper explanation should exist");
    assert_eq!(
        mapper_explanation.witness_digest(),
        mapper_record.witness_digest()
    );
    assert_eq!(mapper_explanation.candidate_digest(), candidate.digest());
    assert_eq!(
        mapper_explanation.envelope_digest(),
        effect.mapper_envelope_digest()
    );
    assert_eq!(
        mapper_explanation.mapped_input_digest(),
        effect.mapped_input_digest()
    );
    let mapper_envelope = runtime
        .diagnostics()
        .writeback_mapper_envelope_for_digest(effect.mapper_envelope_digest())
        .expect("runtime should retain mapper envelope for effect lineage");
    assert_eq!(
        mapper_envelope.causality_digest(),
        effect.causality_digest()
    );
    let execution_explanation = runtime
        .diagnostics()
        .explain_last_writeback_execution_record()
        .expect("writeback execution explanation should exist");
    assert_eq!(
        execution_explanation.mapper_record_digest(),
        Some(mapper_record.digest())
    );
}
