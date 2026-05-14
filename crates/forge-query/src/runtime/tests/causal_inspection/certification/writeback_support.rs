use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity, BridgeRequestKind, BridgeWritebackCausalityBasis,
    BridgeWritebackCausalityIdentity, BridgeWritebackDeclaration,
    BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass, BridgeWritebackEffectIdentity,
    BridgeWritebackFamilyKind, BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackStrategyClass, LoweredBridgeExecutionPolicy, RuntimeBridge,
};

pub(super) struct RetainedWritebackRecordIdentities {
    pub(super) admission_record_identity: String,
    pub(super) mapper_envelope_identity: String,
    pub(super) mapped_family_input_identity: String,
    pub(super) mapper_record_identity: String,
    pub(super) execution_record_identity: String,
    pub(super) replay_record_identity: String,
}

pub(super) fn retain_writeback_record_identities(
    runtime: &RuntimeBridge,
    suffix: &str,
) -> RetainedWritebackRecordIdentities {
    let lowered_policy = lowered_writeback_policy(runtime);
    let contract = runtime
        .admit_writeback_declaration(writeback_declaration(suffix), &lowered_policy)
        .expect("writeback declaration should admit");
    let admission = runtime
        .diagnostics()
        .last_writeback_admission_record()
        .expect("writeback admission record should be retained");
    let causality = BridgeWritebackCausalityBasis::new(
        BridgeWritebackCausalityIdentity::new(format!("causality:slot:{suffix}")),
        format!("trigger:sha256:{suffix}"),
        "route:sha256:slot",
        "evaluation:sha256:slot",
        "truth-view:sha256:slot",
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        format!("effect:sha256:{suffix}"),
        format!("evidence:sha256:{suffix}"),
    );
    let mapper_envelope = runtime
        .diagnostics()
        .last_writeback_mapper_envelope()
        .expect("writeback mapper envelope should be retained");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:slot:{suffix}")),
        format!("effect:sha256:{suffix}"),
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        format!("truth-state:sha256:{suffix}"),
        BridgeWritebackIdempotenceIdentity::new(format!("idempotence:slot:{suffix}")),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("writeback authority should execute");
    let mapper_record = runtime
        .diagnostics()
        .last_writeback_mapper_record()
        .expect("writeback mapper record should be retained");
    let execution = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("writeback execution record should be retained");
    let replay_bundle = runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &outcome);
    let drifted_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:slot:{suffix}:drifted")),
        format!("effect:sha256:{suffix}:drifted"),
    );
    let drifted_idempotence = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        format!("truth-state:sha256:{suffix}"),
        BridgeWritebackIdempotenceIdentity::new(format!("idempotence:slot:{suffix}:drifted")),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted_bundle =
        runtime.replay_writeback_bundle(&contract, &drifted_effect, &drifted_idempotence, &outcome);
    runtime
        .validate_replayed_writeback_bundle(&replay_bundle, &drifted_bundle)
        .expect_err("drifted replay should retain a writeback replay record");
    let replay = runtime
        .diagnostics()
        .last_writeback_replay_record()
        .expect("writeback replay record should be retained");

    RetainedWritebackRecordIdentities {
        admission_record_identity: admission.record_identity().as_str().to_string(),
        mapper_envelope_identity: mapper_envelope.envelope_identity().as_str().to_string(),
        mapped_family_input_identity: mapped_input.mapped_input_identity().as_str().to_string(),
        mapper_record_identity: mapper_record.record_identity().as_str().to_string(),
        execution_record_identity: execution.record_identity().as_str().to_string(),
        replay_record_identity: replay.record_identity().as_str().to_string(),
    }
}

fn lowered_writeback_policy(runtime: &RuntimeBridge) -> LoweredBridgeExecutionPolicy {
    let contract = runtime
        .admit_policy_declaration(BridgePolicyDeclaration::new(
            BridgePolicyDeclarationIdentity::new("policy:slot-writeback"),
            BridgeRequestKind::Authoritative,
            BridgeExecutionPolicyClass::DeterministicCanonical,
            BridgeDiagnosticsTier::Standard,
            true,
            true,
        ))
        .expect("writeback policy should admit");
    runtime.lower_admitted_policy(&contract)
}

fn writeback_declaration(suffix: &str) -> BridgeWritebackDeclaration {
    BridgeWritebackDeclaration::writeback_capable(
        BridgeWritebackDeclarationIdentity::new(format!("writeback:slot:{suffix}")),
        BridgeRequestKind::Authoritative,
        BridgeWritebackFamilyKind::ProjectedStateDiff,
        BridgeWritebackEffectClass::ProjectedStateDiff,
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
        format!("strategy:sha256:{suffix}"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    )
}
