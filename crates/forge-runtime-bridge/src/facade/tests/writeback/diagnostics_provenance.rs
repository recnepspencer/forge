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
                "writeback:diagnostics-tier-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:diagnostics-tier",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:diagnostics-tier-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:diagnostics-tier",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:diagnostics-tier", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        "effect:sha256:diagnostics-tier",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:diagnostics-tier", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-tier"),
        "effect:sha256:diagnostics-tier",
    );

    let standard_idempotence = standard_runtime.classify_writeback_idempotence(
        &standard_effect,
        &standard_lowered_policy,
        "truth-state:sha256:diagnostics-tier",
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-tier"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let exhaustive_idempotence = exhaustive_runtime.classify_writeback_idempotence(
        &exhaustive_effect,
        &exhaustive_lowered_policy,
        "truth-state:sha256:diagnostics-tier",
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
                "writeback:feedback-provenance-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:feedback-provenance",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:feedback-provenance-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:feedback-provenance",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:feedback-provenance", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        "effect:sha256:feedback-provenance",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:feedback-provenance", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:feedback-provenance"),
        "effect:sha256:feedback-provenance",
    );

    let standard_provenance =
        standard_runtime.derive_writeback_feedback_provenance(&standard_effect);
    let exhaustive_provenance =
        exhaustive_runtime.derive_writeback_feedback_provenance(&exhaustive_effect);

    assert_ne!(standard_contract.digest(), exhaustive_contract.digest());
    assert_ne!(standard_effect.digest(), exhaustive_effect.digest());
    assert_eq!(
        standard_provenance.effect_digest(),
        exhaustive_provenance.effect_digest()
    );
    assert_eq!(
        standard_provenance.causality_digest(),
        exhaustive_provenance.causality_digest()
    );
    assert_eq!(standard_provenance.digest(), exhaustive_provenance.digest());
}
