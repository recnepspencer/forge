use forge_runtime_bridge::facade::{
    BridgeDiagnosticsTier, BridgeExecutionPolicyClass, BridgePolicyDeclaration,
    BridgePolicyDeclarationIdentity, BridgeRequestKind, BridgeRouteIdentity,
    BridgeWritebackAuthoritativeStateBasis, BridgeWritebackCausalityIdentity,
    BridgeWritebackDeclaration, BridgeWritebackDeclarationIdentity, BridgeWritebackEffectClass,
    BridgeWritebackEffectIdentity, BridgeWritebackEffectIntent, BridgeWritebackFamilyKind,
    BridgeWritebackIdempotenceClass, BridgeWritebackIdempotenceIdentity,
    BridgeWritebackNativeCausalityInputs, BridgeWritebackStrategyClass,
    LoweredBridgeExecutionPolicy, RuntimeBridge, TruthCommitIdentity, TruthSnapshotIdentity,
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
    let causality = BridgeWritebackNativeCausalityInputs::new(
        BridgeWritebackCausalityIdentity::new(format!("causality:slot:{suffix}")),
        TruthCommitIdentity::new(format!("query-trigger:{suffix}")),
        BridgeRouteIdentity::new("query-route:slot"),
        TruthSnapshotIdentity::new("query-evaluation:slot"),
        TruthSnapshotIdentity::new("query-truth-view:slot"),
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        writeback_effect_intent(BridgeWritebackEffectClass::ProjectedStateDiff, suffix),
    );
    let mapper_envelope = runtime
        .diagnostics()
        .last_writeback_mapper_envelope()
        .expect("writeback mapper envelope should be retained");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new(format!("effect:slot:{suffix}")),
        writeback_effect_intent(BridgeWritebackEffectClass::ProjectedStateDiff, suffix),
    );
    let authoritative_state_basis = BridgeWritebackAuthoritativeStateBasis::from_effect(&effect);
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        &authoritative_state_basis,
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
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            &format!("{suffix}:drifted"),
        ),
    );
    let drifted_authoritative_state_basis =
        BridgeWritebackAuthoritativeStateBasis::from_effect(&drifted_effect);
    let drifted_idempotence = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        &drifted_authoritative_state_basis,
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

fn writeback_effect_intent(
    effect_class: BridgeWritebackEffectClass,
    suffix: &str,
) -> BridgeWritebackEffectIntent {
    BridgeWritebackEffectIntent::validated_scalar_patch(
        effect_class,
        forge_foundational::facade::AspectKey::new("forge.query.writeback")
            .expect("valid writeback effect aspect key"),
        forge_foundational::facade::AspectValue::String(format!("query-effect:{suffix}").into()),
    )
    .expect("causal certification writeback effect intent should validate")
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
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    )
}
