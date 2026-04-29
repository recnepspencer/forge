use super::support::*;

#[test]
fn runtime_rejects_authority_execution_when_no_writeback_authority_is_bound() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:missing-authority",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:missing-authority",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:missing-authority", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:missing-authority"),
        "effect:sha256:missing-authority",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:before",
        BridgeWritebackIdempotenceIdentity::new("idempotence:missing-authority"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("missing authority binding must fail closed");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::AuthorityBypassRejected
    );
    assert!(error.to_string().contains("no truth writeback authority"));
}

#[test]
fn runtime_classifies_matching_feedback_as_canonical_noop() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:loop-classification",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:loop-classification",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:loop-classification", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:loop-classification"),
        "effect:sha256:loop-classification",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:before",
        BridgeWritebackIdempotenceIdentity::new("idempotence:loop-classification"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        Some(feedback_provenance.digest()),
        Some(causality.digest()),
    );

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
}

#[test]
fn runtime_suppresses_matching_feedback_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:feedback-suppression",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:feedback-suppression",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:feedback-suppression", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:feedback-suppression"),
        "effect:sha256:feedback-suppression",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:feedback-suppression"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let (loop_prevention, outcome, receipt) = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            Some(causality.digest()),
        )
        .expect("matching feedback should suppress before authority execution");

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
    assert_eq!(
        outcome,
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&idempotence)
    );
    assert!(receipt.is_none());
}

#[test]
fn runtime_rejects_partial_feedback_context_as_unsafe() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:unsafe-feedback",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:unsafe-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:unsafe-feedback", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:unsafe-feedback"),
        "effect:sha256:unsafe-feedback",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:unsafe-feedback"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            None::<std::sync::Arc<str>>,
        )
        .expect_err("partial feedback context must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}

#[test]
fn runtime_rejects_contradictory_feedback_context_as_unsafe() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:contradictory-feedback",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:contradictory-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:contradictory-feedback",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:contradictory-feedback"),
        "effect:sha256:contradictory-feedback",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:after-feedback",
        BridgeWritebackIdempotenceIdentity::new("idempotence:contradictory-feedback"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(feedback_provenance.digest()),
            Some("truth-trigger:sha256:other-commit"),
        )
        .expect_err("contradictory feedback context must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}
