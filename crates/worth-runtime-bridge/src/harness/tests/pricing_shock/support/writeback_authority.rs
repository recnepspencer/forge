use super::*;

pub(in crate::harness::tests::pricing_shock) fn pricing_writeback_declaration(
    declaration_identity: &str,
    request_kind: BridgeRequestKind,
    request_mode: BridgeWritebackRequestMode,
    _strategy_descriptor_evidence_text: &str,
) -> BridgeWritebackDeclaration {
    match request_mode {
        BridgeWritebackRequestMode::ReadOnly => BridgeWritebackDeclaration::read_only(
            BridgeWritebackDeclarationIdentity::admit_bridge_owned(declaration_identity),
            request_kind,
            BridgeWritebackEffectClass::ProjectedStateDiff,
            BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
        ),
        BridgeWritebackRequestMode::WritebackCapable => {
            BridgeWritebackDeclaration::writeback_capable(
                BridgeWritebackDeclarationIdentity::admit_bridge_owned(declaration_identity),
                request_kind,
                BridgeWritebackFamilyKind::ProjectedStateDiff,
                BridgeWritebackEffectClass::ProjectedStateDiff,
                BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            )
        }
    }
}

pub(in crate::harness::tests::pricing_shock) fn pricing_lowered_policy(
    runtime: &RuntimeBridge,
) -> crate::facade::LoweredBridgeExecutionPolicy {
    let policy_contract = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::admit_bridge_owned("policy:pricing-writeback"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .expect("pricing writeback policy should admit");
    runtime.lower_admitted_policy(&policy_contract)
}

pub(in crate::harness::tests::pricing_shock) fn pricing_writeback_causality_basis(
    identity: &str,
    truth_trigger_evidence_text: &str,
) -> crate::facade::BridgeWritebackNativeCausalityInputs {
    crate::facade::BridgeWritebackNativeCausalityInputs::new(
        BridgeWritebackCausalityIdentity::admit_bridge_owned(identity),
        crate::truth_identity_fixtures::truth_commit_fixture(truth_trigger_evidence_text),
        crate::facade::BridgeRouteIdentity::admit_bridge_owned(identity),
        crate::truth_identity_fixtures::truth_snapshot_fixture(identity),
        crate::truth_identity_fixtures::truth_snapshot_fixture(identity),
    )
}
