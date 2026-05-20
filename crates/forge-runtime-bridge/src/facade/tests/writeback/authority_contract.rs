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
            "writeback:execution-contract",
            BridgeRequestKind::Authoritative,
            BridgeWritebackRequestMode::WritebackCapable,
            "strategy:sha256:execution-contract",
        ),
        causality_basis(
            "causality:execution-contract",
            "trigger:sha256:execution-contract",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-contract"),
        "effect:sha256:execution-contract",
        "truth-state:sha256:execution-contract",
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
        execution.execution_receipt().request_digest(),
        request.digest()
    );
    assert_eq!(
        execution.execution_receipt().authority_outcome_digest(),
        execution.outcome().digest()
    );
    assert_eq!(
        execution.execution_receipt().authority_receipt_digest(),
        execution.authority_receipt().digest()
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
        MismatchedReceiptWritebackAuthority,
    );
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
            "writeback:execution-contract-mismatch",
            BridgeRequestKind::Authoritative,
            BridgeWritebackRequestMode::WritebackCapable,
            "strategy:sha256:execution-contract-mismatch",
        ),
        causality_basis(
            "causality:execution-contract-mismatch",
            "trigger:sha256:execution-contract-mismatch",
        ),
        BridgeWritebackEffectIdentity::new("effect:execution-contract-mismatch"),
        "effect:sha256:execution-contract-mismatch",
        "truth-state:sha256:execution-contract-mismatch",
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
