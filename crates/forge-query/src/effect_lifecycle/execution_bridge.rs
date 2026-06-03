use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::{
    BridgeAdmittedWritebackExecution, BridgeAdmittedWritebackExecutionError,
    BridgeAdmittedWritebackExecutionRequest, BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeRouteIdentity,
    BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity, BridgeWritebackEffectIntent,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackNativeCausalityInputs, RuntimeBridge,
    TruthCommitIdentity, TruthSnapshotIdentity,
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
        BridgeWritebackNativeCausalityInputs::new(
            BridgeWritebackCausalityIdentity::new(format!(
                "causality:{}",
                declaration.lowering_digest()
            )),
            TruthCommitIdentity::new(
                declaration
                    .causality_binding()
                    .causality_digest()
                    .to_string(),
            ),
            BridgeRouteIdentity::new(format!(
                "route:{}",
                declaration.declaration().report().declaration_digest()
            )),
            TruthSnapshotIdentity::new(format!(
                "evaluation:{}",
                declaration.bridge_declaration().digest()
            )),
            TruthSnapshotIdentity::new(declaration.causality_binding().basis_digest().to_string()),
        ),
        BridgeWritebackEffectIdentity::new(format!("effect:{}", declaration.lowering_digest())),
        query_writeback_effect_intent(declaration)?,
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

fn query_writeback_effect_intent(
    declaration: &QueryWritebackDeclaration,
) -> Result<BridgeWritebackEffectIntent, (EffectExecutionDenialKind, String)> {
    BridgeWritebackEffectIntent::validated_scalar_patch(
        declaration.bridge_declaration().effect_class(),
        AspectKey::new("query.writeback.effect")
            .expect("static query writeback effect aspect key is valid"),
        AspectValue::String(declaration.lowering_digest().to_string().into()),
    )
    .map_err(|error| {
        (
            EffectExecutionDenialKind::BridgeWritebackExecutionFailed,
            format!("{error:?}"),
        )
    })
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
