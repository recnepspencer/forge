use super::support::*;

#[test]
fn runtime_classifies_writeback_idempotence_stably_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::admit_bridge_owned("writeback:idempotence"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "idempotence",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:idempotence"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:idempotence"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "canonical-upsert",
        ),
    );

    let left = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let right = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted_effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:idempotence:drifted"),
            "commit-b",
        ),
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:idempotence:drifted"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "canonical-upsert-drifted",
        ),
    );
    let drifted = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        &truth_state_basis(&drifted_effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_ne!(left.digest(), drifted.digest());
}

#[test]
fn runtime_validates_writeback_candidate_stably_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::admit_bridge_owned("writeback:candidate"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "candidate",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:candidate"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:candidate"),
        writeback_effect_intent(BridgeWritebackEffectClass::ProjectedStateDiff, "candidate"),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned("idempotence:candidate"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(&effect, &idempotence, None);
    let strategy_coherence =
        runtime.classify_writeback_strategy_coherence(&contract, &effect, &idempotence);

    let left = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_coherence,
        )
        .expect("candidate validation should succeed");
    let right = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_coherence,
        )
        .expect("candidate validation should remain stable");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_eq!(left.writeback_effect_artifact_digest(), effect.digest());
    assert_eq!(left.effect_intent_digest(), effect.effect_intent_digest());
    assert_eq!(
        left.effect_intent_patch_canonical_basis(),
        effect.effect_intent().patch_canonical_basis()
    );
    assert_eq!(
        left.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
}

#[test]
fn runtime_classifies_strategy_coherence_for_matching_shapes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::admit_bridge_owned("writeback:strategy-coherence"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy-coherence",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::admit_bridge_owned("causality:strategy-coherence"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::admit_bridge_owned("effect:strategy-coherence"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "strategy-coherence",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::admit_bridge_owned("idempotence:strategy-coherence"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let report = runtime.classify_writeback_strategy_coherence(&contract, &effect, &idempotence);

    assert_eq!(
        report.disposition(),
        BridgeWritebackStrategyCoherenceDisposition::Coherent
    );
    assert_eq!(report.writeback_effect_artifact_digest(), effect.digest());
    assert_eq!(report.effect_intent_digest(), effect.effect_intent_digest());
    assert_eq!(
        report.effect_intent_patch_canonical_basis(),
        effect.effect_intent().patch_canonical_basis()
    );
}
