use super::support::*;

#[test]
fn runtime_rejects_preview_writeback_declarations() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:preview"),
        BridgeRequestKind::Preview,
        BridgeWritebackRequestMode::WritebackCapable,
        "preview",
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("preview writeback must fail closed");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::PreviewWritebackRejected
    );
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_strategy_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        Some(projected_strategy_descriptor_basis()),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind strategy digests");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::WritebackNotRequested
    );
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_strategy_class_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        None,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind strategy classes");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::WritebackNotRequested
    );
}

#[test]
fn runtime_rejects_read_only_writeback_declarations_with_family_binding() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:readonly-family"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::ReadOnly,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        None,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("read-only declarations must not bind writeback family");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::WritebackNotRequested
    );
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_strategy_descriptor() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-strategy"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        None,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind a non-empty strategy descriptor");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::StrategyDescriptorMismatch
    );
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_family_kind() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-family"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        None,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        Some(projected_strategy_descriptor_basis()),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind an explicit writeback family");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::FamilyBindingMismatch
    );
}

#[test]
fn runtime_rejects_writeback_capable_declaration_without_strategy_class() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:missing-class"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        None,
        Some(projected_strategy_descriptor_basis()),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("writeback-capable declaration must bind an explicit strategy class");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::StrategyDescriptorMismatch
    );
}

#[test]
fn runtime_rejects_writeback_capable_declaration_with_contradictory_strategy_descriptor_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let declaration = BridgeWritebackDeclaration::new(
        BridgeWritebackDeclarationIdentity::new("writeback:contradictory-strategy-basis"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        Some(BridgeWritebackFamilyKind::ProjectedStateDiff),
        BridgeWritebackEffectClass::ProjectedStateDiff,
        Some(BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation),
        Some(
            BridgeWritebackStrategyDescriptorBasis::for_writeback_contract(
                BridgeWritebackFamilyKind::AspectReconciliation,
                BridgeWritebackEffectClass::AspectReconciliation,
                BridgeWritebackStrategyClass::AspectReconciliationCommit,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
        ),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .validate_writeback_declaration(declaration)
        .expect_err("contradictory strategy descriptor basis must fail closed");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::StrategyDescriptorMismatch
    );
}

#[test]
fn runtime_rejects_writeback_admission_when_runtime_disables_replay_artifacts() {
    let permissive_runtime = runtime(BridgeRuntimePolicy::default());
    let runtime = runtime(BridgeRuntimePolicy::operational().with_replay_artifacts(false));
    let lowered_policy = lowered_policy(&permissive_runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:replay-disabled"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "authoritative",
    );

    let error = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect_err("writeback must fail closed when replay artifacts are unavailable");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::PolicyRejected);
}

#[test]
fn runtime_admits_family_distinct_aspect_reconciliation_writeback() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration_with_shape(
        BridgeWritebackDeclarationIdentity::new("writeback:aspect-reconciliation"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::AspectReconciliation,
        "aspect-reconciliation",
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("phase 1 family admission should admit aspect reconciliation family");
    let family_basis = contract
        .validated_declaration()
        .family_basis()
        .expect("admitted writeback contract should preserve family basis");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("writeback:aspect-reconciliation:causality"),
        "truth-trigger:aspect",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("writeback:aspect-reconciliation:effect"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::AspectReconciliation,
            "aspect-reconciliation",
        ),
    );

    assert_eq!(
        family_basis.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
    assert_eq!(
        effect.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
    assert_eq!(
        effect.effect_class(),
        BridgeWritebackEffectClass::AspectReconciliation
    );
    let admission_record = runtime
        .diagnostics()
        .last_writeback_admission_record()
        .expect("writeback family admission should retain native admission record");
    let admission_explanation = runtime
        .diagnostics()
        .explain_last_writeback_admission_record()
        .expect("writeback family admission explanation should exist");
    assert_eq!(admission_record.contract_digest(), contract.digest());
    assert_eq!(
        admission_record.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
    assert_eq!(admission_explanation.record(), &admission_record);
    assert_eq!(admission_explanation.contract_digest(), contract.digest());
    assert_eq!(
        admission_explanation.family_kind(),
        BridgeWritebackFamilyKind::AspectReconciliation
    );
}

#[test]
fn runtime_rejects_phase_1_unadmitted_repeated_authority_attempts() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration_with_shape(
        BridgeWritebackDeclarationIdentity::new("writeback:repeated-authority-attempt"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        "repeated-authority-attempt",
        BridgeWritebackIdempotenceClass::AllowRepeatedAuthorityAttempt,
    );

    let error = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect_err("phase 1 writeback should reject repeated authority attempt admission");

    assert_eq!(
        error.kind(),
        BridgeWritebackErrorKind::FamilyBindingMismatch
    );
    assert!(runtime
        .diagnostics()
        .last_writeback_admission_record()
        .is_none());
}
