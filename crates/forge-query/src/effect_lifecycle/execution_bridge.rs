use forge_foundational::facade::{AspectKey, AspectValue};
use forge_runtime_bridge::facade::{
    BridgeAdmittedWritebackExecution, BridgeAdmittedWritebackExecutionError,
    BridgeAdmittedWritebackExecutionRequest, BridgeDiagnosticsTier, BridgeExecutionPolicyClass,
    BridgePolicyDeclaration, BridgePolicyDeclarationIdentity, BridgeRouteIdentity,
    BridgeWritebackCausalityIdentity, BridgeWritebackEffectIdentity, BridgeWritebackEffectIntent,
    BridgeWritebackIdempotenceIdentity, BridgeWritebackNativeCausalityInputs, RuntimeBridge,
};

use crate::application::{query_truth_commit_identity, query_truth_snapshot_identity};
use crate::evidence_identity::{
    ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::workflow::QueryWritebackDeclaration;

use super::execution::EffectExecutionDenialKind;

pub(super) fn execute_lowered_writeback(
    runtime: &RuntimeBridge,
    declaration: &QueryWritebackDeclaration,
) -> Result<BridgeAdmittedWritebackExecution, (EffectExecutionDenialKind, String)> {
    let request = BridgeAdmittedWritebackExecutionRequest::new(
        BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::from_external_authority_evidence(
                bridge_writeback_identity("policy", declaration.lowering_identity()),
            ),
            declaration.causality_binding().request_kind(),
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ),
        declaration.bridge_declaration().clone(),
        BridgeWritebackNativeCausalityInputs::new(
            BridgeWritebackCausalityIdentity::from_external_authority_evidence(
                bridge_writeback_identity("causality", declaration.lowering_identity()),
            ),
            query_truth_commit_identity(
                "effect-causality",
                bridge_writeback_identity(
                    "truth-commit-causality",
                    declaration.causality_binding().causality_identity(),
                ),
            ),
            BridgeRouteIdentity::from_external_authority_evidence(bridge_writeback_identity(
                "route",
                declaration.lowering_identity(),
            )),
            query_truth_snapshot_identity(
                "effect-evaluation",
                bridge_writeback_identity(
                    "truth-snapshot-evaluation",
                    declaration.lowering_identity(),
                ),
            ),
            query_truth_snapshot_identity(
                "effect-basis",
                bridge_writeback_identity(
                    "truth-snapshot-basis",
                    declaration.causality_binding().basis_identity(),
                ),
            ),
        ),
        BridgeWritebackEffectIdentity::from_external_authority_evidence(bridge_writeback_identity(
            "effect",
            declaration.lowering_identity(),
        )),
        query_writeback_effect_intent(declaration)?,
        BridgeWritebackIdempotenceIdentity::from_external_authority_evidence(
            bridge_writeback_identity("idempotence", declaration.lowering_identity()),
        ),
        declaration.bridge_declaration().idempotence_class(),
    );
    runtime
        .execute_admitted_writeback(request)
        .map_err(map_bridge_writeback_execution_error)
}

fn bridge_writeback_identity(
    role: &str,
    source_identity: &ForgeQueryEvidenceIdentity,
) -> ForgeQueryEvidenceIdentity {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::RuntimeBridgeWritebackAuthority)
        .field_shape(ForgeQueryEvidenceTag::new("role"), role)
        .field_evidence_identity(ForgeQueryEvidenceTag::new("source"), source_identity)
        .seal()
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
