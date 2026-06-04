use super::support::*;

#[test]
fn admitted_writeback_execution_contract_emits_proof_receipt() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let request = crate::facade::BridgeAdmittedWritebackExecutionRequest::new(
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:writeback-contract"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
        writeback_declaration(
            BridgeWritebackDeclarationIdentity::new("writeback:execution-contract"),
            BridgeRequestKind::Authoritative,
            BridgeWritebackRequestMode::WritebackCapable,
            "execution-contract",
        ),
        causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:execution-contract"),
            "execution-contract",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-contract"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "execution-contract",
        ),
        BridgeWritebackIdempotenceIdentity::new("idempotence:execution-contract"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let execution = runtime
        .execute_admitted_writeback(request.clone())
        .expect("bridge should execute the admitted writeback contract");

    assert_eq!(
        execution.authority_receipt().outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        execution.authority_receipt().effect_intent(),
        request.effect_intent()
    );
    assert_eq!(
        execution
            .authority_receipt()
            .effect_intent()
            .authoritative_patch(),
        request.effect_intent().authoritative_patch()
    );
    assert_eq!(
        execution.execution_receipt().request_digest(),
        request.digest()
    );
    assert_eq!(
        execution
            .execution_receipt()
            .admitted_execution_request()
            .effect_intent(),
        request.effect_intent()
    );
    assert_eq!(
        execution
            .execution_receipt()
            .admitted_execution_request()
            .effect_intent()
            .authoritative_patch(),
        request.effect_intent().authoritative_patch()
    );
    assert_eq!(
        execution.execution_receipt().effect_intent_digest(),
        request.effect_intent_digest()
    );
    assert_eq!(
        execution
            .execution_receipt()
            .effect_intent_patch_canonical_basis(),
        request.effect_intent_patch_canonical_basis()
    );
    assert_eq!(
        execution.execution_receipt().authority_outcome_digest(),
        execution.outcome().digest()
    );
    assert_eq!(
        execution.execution_receipt().authority_receipt_digest(),
        execution.authority_receipt().digest()
    );
    assert_eq!(
        execution
            .execution_receipt()
            .authority_receipt()
            .effect_intent(),
        request.effect_intent()
    );
    assert!(execution
        .execution_receipt()
        .replay_bundle_digest()
        .starts_with("bridge-writeback-replay-bundle:sha256:"));
    let record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("admitted writeback execution should annotate the retained execution record");
    assert_eq!(
        record.execution_receipt_digest(),
        Some(execution.execution_receipt().digest())
    );
}

#[test]
fn admitted_writeback_execution_contract_rejects_mismatched_authority_receipt() {
    let runtime = runtime_with_custom_writeback_authority(
        BridgeRuntimePolicy::development(),
        MismatchedReceiptWritebackAuthority::default(),
    );
    let first_request = crate::facade::BridgeAdmittedWritebackExecutionRequest::new(
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:writeback-contract-mismatch:first"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
        writeback_declaration(
            BridgeWritebackDeclarationIdentity::new("writeback:execution-contract-mismatch"),
            BridgeRequestKind::Authoritative,
            BridgeWritebackRequestMode::WritebackCapable,
            "execution-contract-mismatch",
        ),
        causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:execution-contract-mismatch:first"),
            "execution-contract-mismatch:first",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-contract-mismatch:first"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "execution-contract-mismatch:first",
        ),
        BridgeWritebackIdempotenceIdentity::new("idempotence:execution-contract-mismatch:first"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    runtime
        .execute_admitted_writeback(first_request)
        .expect("first admitted writeback establishes native prior receipt basis");

    let request = crate::facade::BridgeAdmittedWritebackExecutionRequest::new(
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:writeback-contract-mismatch"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
        writeback_declaration(
            BridgeWritebackDeclarationIdentity::new("writeback:execution-contract-mismatch"),
            BridgeRequestKind::Authoritative,
            BridgeWritebackRequestMode::WritebackCapable,
            "execution-contract-mismatch",
        ),
        causality_basis(
            BridgeWritebackCausalityIdentity::new("causality:execution-contract-mismatch"),
            "execution-contract-mismatch",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-contract-mismatch"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "execution-contract-mismatch",
        ),
        BridgeWritebackIdempotenceIdentity::new("idempotence:execution-contract-mismatch"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let error = runtime
        .execute_admitted_writeback(request)
        .expect_err("mismatched authority receipt should fail the admitted contract");

    match error {
        crate::facade::BridgeAdmittedWritebackExecutionError::Writeback(error) => {
            assert_eq!(error.kind(), BridgeWritebackErrorKind::InvariantRejected);
        }
        other => panic!("expected writeback rejection, got {other:?}"),
    }
}
