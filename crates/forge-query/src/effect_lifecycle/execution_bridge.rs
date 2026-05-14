use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity, BridgeWritebackAuthorityOutcome,
    BridgeWritebackCausalityBasis, BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity,
    BridgeWritebackIdempotenceIdentity, RuntimeBridge, TruthWritebackReceipt,
};

use crate::workflow::QueryWritebackDeclaration;

use super::execution::EffectExecutionDenialKind;

pub(super) fn execute_lowered_writeback(
    runtime: &RuntimeBridge,
    declaration: &QueryWritebackDeclaration,
) -> Result<
    (BridgeWritebackAuthorityOutcome, TruthWritebackReceipt),
    (EffectExecutionDenialKind, String),
> {
    let lowered_policy = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new(format!(
                "policy:{}",
                declaration.lowering_digest()
            )),
            declaration.causality_binding().request_kind(),
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .map(|contract| runtime.lower_admitted_policy(&contract))
        .map_err(|error| {
            (
                EffectExecutionDenialKind::BridgePolicyAdmissionFailed,
                format!("{error:?}"),
            )
        })?;
    let contract = runtime
        .admit_writeback_declaration(declaration.bridge_declaration().clone(), &lowered_policy)
        .map_err(|error| {
            (
                EffectExecutionDenialKind::BridgeWritebackExecutionFailed,
                format!("{error:?}"),
            )
        })?;
    let causality = BridgeWritebackCausalityBasis::new(
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
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:{}", declaration.lowering_digest())),
        declaration.lowering_digest().to_string(),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        declaration.causality_binding().basis_digest().to_string(),
        BridgeWritebackIdempotenceIdentity::new(format!(
            "idempotence:{}",
            declaration.lowering_digest()
        )),
        declaration.bridge_declaration().idempotence_class(),
    );
    runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .map_err(|error| {
            (
                EffectExecutionDenialKind::BridgeWritebackExecutionFailed,
                format!("{error:?}"),
            )
        })
}
