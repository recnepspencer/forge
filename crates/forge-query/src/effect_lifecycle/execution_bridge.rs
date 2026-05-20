use forge_runtime_bridge::facade::{
    BridgeAdmittedWritebackExecution, BridgeAdmittedWritebackExecutionError,
    BridgeAdmittedWritebackExecutionRequest, BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity,
    BridgeWritebackIdempotenceIdentity, RuntimeBridge,
};

use crate::workflow::QueryWritebackDeclaration;

use super::execution::EffectExecutionDenialKind;

pub(super) fn execute_lowered_writeback(
    runtime: &RuntimeBridge,
    declaration: &QueryWritebackDeclaration,
) -> Result<BridgeAdmittedWritebackExecution, (EffectExecutionDenialKind, String)> {
    let request = BridgeAdmittedWritebackExecutionRequest::new(
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new(format!(
                "policy:{}",
                declaration.lowering_digest()
            )),
            declaration.causality_binding().request_kind(),
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
        declaration.bridge_declaration().clone(),
        BridgeWritebackCausalityBasis::new(
            BridgeWritebackCausalityIdentity::new(format!(
                "causality:{}",
                declaration.lowering_digest()
            )),
            declaration.causality_binding().causality_digest(),
            format!(
                "route:{}",
                declaration.declaration().report().declaration_digest()
            ),
            format!("evaluation:{}", declaration.bridge_declaration().digest()),
            declaration.causality_binding().basis_digest().to_string(),
        ),
        BridgeWritebackEffectIdentity::new(format!("effect:{}", declaration.lowering_digest())),
        declaration.lowering_digest().to_string(),
        declaration.causality_binding().basis_digest().to_string(),
        BridgeWritebackIdempotenceIdentity::new(format!(
            "idempotence:{}",
            declaration.lowering_digest()
        )),
        declaration.bridge_declaration().idempotence_class(),
    );
    runtime
        .execute_admitted_writeback(request)
        .map_err(map_bridge_writeback_execution_error)
}

fn map_bridge_writeback_execution_error(
    error: BridgeAdmittedWritebackExecutionError,
) -> (EffectExecutionDenialKind, String) {
    match error {
        BridgeAdmittedWritebackExecutionError::PolicyAdmission(rejection) => (
            EffectExecutionDenialKind::BridgePolicyAdmissionFailed,
            format!("{rejection:?}"),
        ),
        BridgeAdmittedWritebackExecutionError::Writeback(error) => (
            EffectExecutionDenialKind::BridgeWritebackExecutionFailed,
            format!("{error:?}"),
        ),
    }
}
