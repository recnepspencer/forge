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
        "writeback:typed-rejection",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:typed-rejection",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:typed-rejection", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:typed-rejection"),
        "effect:sha256:typed-rejection",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:typed-rejection",
        BridgeWritebackIdempotenceIdentity::new("idempotence:typed-rejection"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("typed authority rejection should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StaleTruthBasis);
    assert!(error.to_string().contains("StaleTruthBasis"));
}

#[test]
fn runtime_maps_authority_transport_failure_into_strategy_failed() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        FailingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:transport-failure",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:transport-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:transport-failure", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:transport-failure"),
        "effect:sha256:transport-failure",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:transport-failure",
        BridgeWritebackIdempotenceIdentity::new("idempotence:transport-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority transport failure should surface as bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyFailed);
    assert!(error.to_string().contains("transport failure"));
}

#[test]
fn runtime_maps_authority_panic_into_strategy_panicked() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        PanickingWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:panic-failure",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:panic-failure",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:panic-failure", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:panic-failure"),
        "effect:sha256:panic-failure",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:panic-failure",
        BridgeWritebackIdempotenceIdentity::new("idempotence:panic-failure"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("authority panic should surface as typed bridge error");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::StrategyPanicked);
    assert!(error.to_string().contains("writeback strategy panic"));
}

#[test]
fn runtime_rejects_receipt_with_mismatched_request_digest() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MismatchedReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:mismatched-receipt",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:mismatched-receipt",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:mismatched-receipt", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mismatched-receipt"),
        "effect:sha256:mismatched-receipt",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:mismatched-receipt",
        BridgeWritebackIdempotenceIdentity::new("idempotence:mismatched-receipt"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("mismatched receipt request digests must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error.to_string().contains("returned receipt"));
}

#[test]
fn runtime_rejects_rejected_receipt_without_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedRejectedReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:rejected-without-failure-class",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:rejected-without-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:rejected-without-failure-class",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:rejected-without-failure-class"),
        "effect:sha256:rejected-without-failure-class",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:rejected-without-failure-class",
        BridgeWritebackIdempotenceIdentity::new("idempotence:rejected-without-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("rejected receipts without failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error.to_string().contains("without a failure class"));
}

#[test]
fn runtime_rejects_successful_receipt_with_failure_class() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::default(),
        MalformedSuccessfulReceiptWritebackAuthority,
    );
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:success-with-failure-class",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:success-with-failure-class",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:success-with-failure-class",
        "trigger:sha256:commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:success-with-failure-class"),
        "effect:sha256:success-with-failure-class",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:success-with-failure-class",
        BridgeWritebackIdempotenceIdentity::new("idempotence:success-with-failure-class"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect_err("successful receipts with failure class must fail closed");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
    assert!(error.to_string().contains("non-rejected receipt"));
}

#[test]
fn runtime_rejects_strategy_compatibility_mismatch_before_authority_execution() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let contract_a = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:compatibility-a",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:compatibility-a",
            ),
            &lowered_policy,
        )
        .expect("first writeback declaration should admit");
    let contract_b = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:compatibility-b",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:compatibility-b",
            ),
            &lowered_policy,
        )
        .expect("second writeback declaration should admit");
    let effect_a = runtime.lower_writeback_effect(
        &contract_a,
        &causality_basis("causality:compatibility-a", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:compatibility-a"),
        "effect:sha256:compatibility-a",
    );
    let effect_b = runtime.lower_writeback_effect(
        &contract_b,
        &causality_basis("causality:compatibility-b", "trigger:sha256:commit-b"),
        BridgeWritebackEffectIdentity::new("effect:compatibility-b"),
        "effect:sha256:compatibility-b",
    );
    let mismatched_idempotence = runtime.classify_writeback_idempotence(
        &effect_b,
        &lowered_policy,
        "truth-state:sha256:compatibility-b",
        BridgeWritebackIdempotenceIdentity::new("idempotence:compatibility-b"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_writeback_authority(&contract_a, &effect_a, &mismatched_idempotence)
        .expect_err("strategy compatibility drift should fail before authority execution");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::StrategyDescriptorMismatch
    );
}
