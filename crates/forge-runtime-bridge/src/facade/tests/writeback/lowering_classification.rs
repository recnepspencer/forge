use super::support::*;

#[test]
fn runtime_classifies_writeback_idempotence_stably_for_same_inputs() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:idempotence",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:idempotence",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:idempotence", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:idempotence"),
        "effect:sha256:canonical-upsert",
    );

    let left = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let right = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:stable",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:drifted",
        BridgeWritebackIdempotenceIdentity::new("idempotence:stable"),
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
        "writeback:candidate",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:candidate",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:candidate", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:candidate"),
        "effect:sha256:candidate",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:candidate",
        BridgeWritebackIdempotenceIdentity::new("idempotence:candidate"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let strategy_compatibility =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);

    let left = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should succeed");
    let right = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should remain stable");

    assert_eq!(left, right);
    assert_eq!(left.digest(), right.digest());
    assert_eq!(
        left.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
}

#[test]
fn runtime_classifies_strategy_compatibility_for_matching_shapes() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:strategy-compatibility",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:strategy-compatibility",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            "causality:strategy-compatibility",
            "trigger:sha256:commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:strategy-compatibility"),
        "effect:sha256:strategy-compatibility",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:strategy-compatibility",
        BridgeWritebackIdempotenceIdentity::new("idempotence:strategy-compatibility"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let report =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);

    assert_eq!(
        report.disposition(),
        BridgeWritebackStrategyCompatibilityDisposition::Compatible
    );
}
