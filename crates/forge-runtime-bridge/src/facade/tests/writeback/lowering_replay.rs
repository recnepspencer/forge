use super::support::*;

#[test]
fn runtime_replay_writeback_bundle_changes_when_outcome_changes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:replay",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:replay",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay"),
        "effect:sha256:authoritative-upsert",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let noop_outcome = crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&idempotence);
    let commit_outcome = crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
        &idempotence,
        "authoritative-artifact:sha256:commit-a",
    );

    let noop_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &noop_outcome);
    let commit_bundle =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &commit_outcome);
    let noop_bundle_repeat =
        runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &noop_outcome);

    assert_eq!(noop_bundle, noop_bundle_repeat);
    assert_eq!(noop_bundle.digest(), noop_bundle_repeat.digest());
    assert_eq!(
        noop_bundle.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        noop_bundle.strategy_descriptor_digest(),
        contract
            .validated_declaration()
            .strategy_basis()
            .expect("admitted writeback contract should preserve strategy basis")
            .strategy_descriptor_digest()
    );
    assert_ne!(
        noop_bundle.semantic_digest(),
        commit_bundle.semantic_digest()
    );
    assert_eq!(noop_bundle.causality_digest(), effect.causality_digest());
    assert_eq!(noop_bundle.lowered_policy_digest(), lowered_policy.digest());
    assert_eq!(
        noop_bundle.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(
        noop_bundle.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::CanonicalNoop
    );
    assert_eq!(
        commit_bundle.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        commit_bundle.authoritative_artifact_digest(),
        "authoritative-artifact:sha256:commit-a"
    );
    assert_ne!(noop_outcome.digest(), commit_outcome.digest());
    assert_ne!(noop_bundle.digest(), commit_bundle.digest());
}

#[test]
fn runtime_replay_writeback_bundle_changes_when_family_changes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let projected_declaration = writeback_declaration_with_shape(
        "writeback:replay-family:projected",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        "strategy:sha256:replay-family:projected",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_declaration = writeback_declaration_with_shape(
        "writeback:replay-family:aspect",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::AspectReconciliation,
        "strategy:sha256:replay-family:aspect",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let projected_contract = runtime
        .admit_writeback_declaration(projected_declaration, &lowered_policy)
        .expect("projected family declaration should admit");
    let aspect_contract = runtime
        .admit_writeback_declaration(aspect_declaration, &lowered_policy)
        .expect("aspect family declaration should admit");
    let projected_causality =
        causality_basis("causality:replay-family:projected", "trigger:sha256:shared");
    let aspect_causality =
        causality_basis("causality:replay-family:aspect", "trigger:sha256:shared");
    let projected_effect = runtime.lower_writeback_effect(
        &projected_contract,
        &projected_causality,
        BridgeWritebackEffectIdentity::new("effect:replay-family:projected"),
        "effect:sha256:shared",
    );
    let aspect_effect = runtime.lower_writeback_effect(
        &aspect_contract,
        &aspect_causality,
        BridgeWritebackEffectIdentity::new("effect:replay-family:aspect"),
        "effect:sha256:shared",
    );
    let projected_idempotence = runtime.classify_writeback_idempotence(
        &projected_effect,
        &lowered_policy,
        "truth-state:sha256:shared",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-family:projected"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let aspect_idempotence = runtime.classify_writeback_idempotence(
        &aspect_effect,
        &lowered_policy,
        "truth-state:sha256:shared",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-family:aspect"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let projected_outcome =
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&projected_idempotence);
    let aspect_outcome =
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&aspect_idempotence);
    let projected_bundle = runtime.replay_writeback_bundle(
        &projected_contract,
        &projected_effect,
        &projected_idempotence,
        &projected_outcome,
    );
    let aspect_bundle = runtime.replay_writeback_bundle(
        &aspect_contract,
        &aspect_effect,
        &aspect_idempotence,
        &aspect_outcome,
    );

    assert_eq!(
        projected_bundle.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        aspect_bundle.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
    assert_ne!(
        projected_bundle.semantic_digest(),
        aspect_bundle.semantic_digest()
    );
    assert_ne!(projected_bundle.digest(), aspect_bundle.digest());
}
