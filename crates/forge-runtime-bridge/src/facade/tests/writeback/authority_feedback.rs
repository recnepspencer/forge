use super::support::*;

#[test]
fn runtime_rejects_authority_execution_when_no_writeback_authority_is_bound() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-authority"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "missing-authority",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:missing-authority"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:missing-authority"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "missing-authority",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:missing-authority"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("missing authority binding must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::AuthorityDenied);

    let execution_record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("missing authority rejection should retain execution evidence");
    assert_eq!(
        execution_record.failure_class(),
        Some(BridgeWritebackFailureClass::AuthorityDenied)
    );
    assert!(execution_record.mapper_record_digest().is_some());
    assert!(execution_record.candidate_digest().is_some());
    assert_eq!(execution_record.request_digest(), None);
    assert_eq!(execution_record.receipt_digest(), None);
    assert_eq!(
        execution_record
            .counters()
            .writeback_authority_denial_count(),
        1
    );
}

#[test]
fn runtime_classifies_matching_feedback_as_canonical_noop() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:loop-classification"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "loop-classification",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:loop-classification"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:loop-classification"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "loop-classification",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:loop-classification"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);
    let feedback_context =
        crate::facade::BridgeWritebackFeedbackContext::from_provenance(&feedback_provenance);

    let loop_prevention =
        runtime.classify_writeback_loop_prevention(&effect, &idempotence, Some(&feedback_context));

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
    assert_eq!(
        loop_prevention.current_feedback_provenance().digest(),
        feedback_provenance.digest()
    );
    assert_eq!(
        loop_prevention
            .incoming_feedback_context()
            .expect("matching feedback must retain incoming feedback context")
            .digest(),
        feedback_context.digest()
    );
    assert_eq!(loop_prevention.idempotence().digest(), idempotence.digest());
}

#[test]
fn runtime_suppresses_matching_feedback_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:feedback-suppression"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "feedback-suppression",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:feedback-suppression"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:feedback-suppression"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "feedback-suppression",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:feedback-suppression"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);
    let feedback_context =
        crate::facade::BridgeWritebackFeedbackContext::from_provenance(&feedback_provenance);

    let (loop_prevention, outcome, receipt) = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(&feedback_context),
        )
        .expect("matching feedback should suppress before authority execution");

    assert_eq!(
        loop_prevention.disposition(),
        BridgeWritebackLoopDisposition::CanonicalNoop
    );
    assert_eq!(
        loop_prevention
            .incoming_feedback_context()
            .expect("matching feedback suppression must retain context")
            .provenance_digest(),
        feedback_context.provenance_digest()
    );
    assert_eq!(
        outcome,
        crate::facade::BridgeWritebackAuthorityOutcome::canonical_noop(&idempotence)
    );
    assert!(receipt.is_none());
}

#[test]
fn runtime_rejects_different_effect_same_causality_feedback_as_unsafe() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:unsafe-feedback"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "unsafe-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:unsafe-feedback"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:unsafe-feedback"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "unsafe-feedback",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:unsafe-feedback"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let changed_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:unsafe-feedback:changed"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "unsafe-feedback:changed",
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
        .expect_err("different-effect same-causality feedback must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}

#[test]
fn runtime_rejects_matching_feedback_when_repeated_authority_attempts_are_required() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:contradictory-feedback"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "contradictory-feedback",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:contradictory-feedback"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:contradictory-feedback"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "contradictory-feedback",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:contradictory-feedback"),
        BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt,
    );
    let feedback_provenance = runtime.derive_writeback_feedback_provenance(&effect);
    let feedback_context =
        crate::facade::BridgeWritebackFeedbackContext::from_provenance(&feedback_provenance);

    let error = runtime
        .execute_writeback_authority_with_feedback_context(
            &contract,
            &effect,
            &idempotence,
            Some(&feedback_context),
        )
        .expect_err("matching feedback with repeated-attempt idempotence must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
}
