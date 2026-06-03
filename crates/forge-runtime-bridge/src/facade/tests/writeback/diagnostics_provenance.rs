use super::support::*;

#[test]
fn writeback_diagnostics_tier_variation_preserves_replay_meaning() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());

    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new("writeback:diagnostics-tier-standard"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "diagnostics-tier",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new("writeback:diagnostics-tier-exhaustive"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "diagnostics-tier",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:diagnostics-tier"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "diagnostics-tier",
        ),
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:diagnostics-tier"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "diagnostics-tier",
        ),
    );

    let standard_idempotence = standard_runtime.classify_writeback_idempotence(
        &standard_effect,
        &standard_lowered_policy,
        &truth_state_basis(&standard_effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-tier"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let exhaustive_idempotence = exhaustive_runtime.classify_writeback_idempotence(
        &exhaustive_effect,
        &exhaustive_lowered_policy,
        &truth_state_basis(&exhaustive_effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-tier"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let (standard_outcome, _) = standard_runtime
        .execute_writeback_authority(&standard_contract, &standard_effect, &standard_idempotence)
        .expect("standard authority execution should succeed");
    let (exhaustive_outcome, _) = exhaustive_runtime
        .execute_writeback_authority(
            &exhaustive_contract,
            &exhaustive_effect,
            &exhaustive_idempotence,
        )
        .expect("exhaustive authority execution should succeed");

    let standard_bundle = standard_runtime.replay_writeback_bundle(
        &standard_contract,
        &standard_effect,
        &standard_idempotence,
        &standard_outcome,
    );
    let exhaustive_bundle = exhaustive_runtime.replay_writeback_bundle(
        &exhaustive_contract,
        &exhaustive_effect,
        &exhaustive_idempotence,
        &exhaustive_outcome,
    );

    let standard_explanation = standard_runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&standard_bundle);
    let exhaustive_explanation = exhaustive_runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&exhaustive_bundle);

    assert_ne!(standard_bundle.digest(), exhaustive_bundle.digest());
    assert_eq!(
        standard_bundle.semantic_digest(),
        exhaustive_bundle.semantic_digest()
    );
    assert_eq!(
        standard_explanation.semantic_digest(),
        exhaustive_explanation.semantic_digest()
    );
    assert_eq!(
        standard_explanation.strategy_class(),
        exhaustive_explanation.strategy_class()
    );
    assert_eq!(
        standard_explanation.causality_digest(),
        exhaustive_explanation.causality_digest()
    );
    assert_eq!(
        standard_explanation.retry_disposition(),
        exhaustive_explanation.retry_disposition()
    );
    assert_eq!(
        standard_explanation.outcome_class(),
        exhaustive_explanation.outcome_class()
    );
    standard_runtime
        .validate_replayed_writeback_bundle(&standard_bundle, &exhaustive_bundle)
        .expect("replay validation should accept diagnostics-only detail drift");
}

#[test]
fn writeback_feedback_provenance_is_diagnostics_invariant_for_semantically_equal_effects() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());
    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new("writeback:feedback-provenance-standard"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "feedback-provenance",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new("writeback:feedback-provenance-exhaustive"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "feedback-provenance",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:feedback-provenance"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "feedback-provenance",
        ),
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:feedback-provenance"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "feedback-provenance",
        ),
    );

    let standard_provenance =
        standard_runtime.derive_writeback_feedback_provenance(&standard_effect);
    let exhaustive_provenance =
        exhaustive_runtime.derive_writeback_feedback_provenance(&exhaustive_effect);

    assert_ne!(standard_contract.digest(), exhaustive_contract.digest());
    assert_ne!(standard_effect.digest(), exhaustive_effect.digest());
    assert_eq!(
        standard_provenance.effect_intent_digest(),
        exhaustive_provenance.effect_intent_digest()
    );
    assert_eq!(
        standard_provenance.causality_digest(),
        exhaustive_provenance.causality_digest()
    );
    assert_eq!(standard_provenance.digest(), exhaustive_provenance.digest());
}
