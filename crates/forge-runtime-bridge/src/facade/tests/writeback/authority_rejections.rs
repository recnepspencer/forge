use super::support::*;

#[test]
fn runtime_maps_typed_authority_rejection_into_bridge_error_kind() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        RejectingWritebackAuthority {
            failure_class: BridgeWritebackFailureClass::StaleTruthBasis,
        },
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:typed-rejection"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "typed-rejection",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:typed-rejection"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:typed-rejection"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "typed-rejection",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:typed-rejection"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("typed authority rejection should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StaleTruthBasis);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::StaleTruthBasis,
        ExpectedAuthorityStage::RejectedReceipt,
    );
}

#[test]
fn runtime_maps_authority_transport_failure_into_strategy_failed() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        FailingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:transport-failure"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "transport-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:transport-failure"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:transport-failure"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "transport-failure",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:transport-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority transport failure should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyFailed);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::StrategyFailed,
        ExpectedAuthorityStage::RequestDispatch,
    );
}

#[test]
fn runtime_maps_authority_panic_into_strategy_panicked() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        PanickingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:panic-failure"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "panic-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:panic-failure"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:panic-failure"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "panic-failure",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:panic-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority panic should surface as typed bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyPanicked);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::StrategyPanicked,
        ExpectedAuthorityStage::RequestDispatch,
    );
}

#[test]
fn runtime_rejects_receipt_with_mismatched_request_digest() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MismatchedReceiptWritebackAuthority::default(),
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:mismatched-receipt"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "mismatched-receipt",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:mismatched-receipt"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mismatched-receipt"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mismatched-receipt",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:mismatched-receipt"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("first request establishes a native prior receipt basis");

    let second_causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:mismatched-receipt:second"),
        "commit-b",
    );
    let second_effect = runtime.lower_writeback_effect(
        &contract,
        &second_causality,
        BridgeWritebackEffectIdentity::new("effect:mismatched-receipt:second"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mismatched-receipt:second",
        ),
    );
    let second_idempotence = runtime.classify_writeback_idempotence(
        &second_effect,
        &lowered_policy,
        &truth_state_basis(&second_effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:mismatched-receipt:second"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &second_effect, &second_idempotence)
        .expect_err("mismatched receipt request digests must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::InvariantRejected,
        ExpectedAuthorityStage::ValidatedReceipt,
    );
}

#[test]
fn runtime_rejects_rejected_receipt_without_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedRejectedReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:rejected-without-failure-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "rejected-without-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:rejected-without-failure-class"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:rejected-without-failure-class"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "rejected-without-failure-class",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:rejected-without-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("rejected receipts without failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::InvariantRejected,
        ExpectedAuthorityStage::ValidatedReceipt,
    );
}

#[test]
fn runtime_rejects_successful_receipt_with_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedSuccessfulReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:success-with-failure-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "success-with-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:success-with-failure-class"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:success-with-failure-class"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "success-with-failure-class",
        ),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &truth_state_basis(&effect),
        BridgeWritebackIdempotenceIdentity::new("idempotence:success-with-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("successful receipts with failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert_last_execution_failure(
        &runtime,
        BridgeWritebackFailureClass::InvariantRejected,
        ExpectedAuthorityStage::ValidatedReceipt,
    );
}

#[test]
fn runtime_rejects_strategy_coherence_mismatch_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let contract_a = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new("writeback:coherence-a"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "coherence-a",
            ),
            &lowered_policy,
        )
        .expect("first writeback declaration should admit");
    let contract_b = runtime
        .admit_writeback_declaration(
            writeback_declaration_with_shape(
                BridgeWritebackDeclarationIdentity::new("writeback:coherence-b"),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                BridgeWritebackEffectClass::AspectReconciliation,
                "coherence-b",
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &lowered_policy,
        )
        .expect("second writeback declaration should admit");
    let effect_a = runtime.lower_writeback_effect(
        &contract_a,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:coherence-a"),
            "commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:coherence-a"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "coherence-a",
        ),
    );
    let effect_b = runtime.lower_writeback_effect(
        &contract_b,
        &causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:coherence-b"),
            "commit-b",
        ),
        BridgeWritebackEffectIdentity::new("effect:coherence-b"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::AspectReconciliation,
            "coherence-b",
        ),
    );
    let mismatched_idempotence = runtime.classify_writeback_idempotence(
        &effect_b,
        &lowered_policy,
        &truth_state_basis(&effect_b),
        BridgeWritebackIdempotenceIdentity::new("idempotence:coherence-b"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract_a, &effect_a, &mismatched_idempotence)
        .expect_err("strategy coherence drift should fail before authority execution");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::FamilyBindingMismatch
    );
}
