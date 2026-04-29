use super::support::*;

#[test]
fn runtime_lowers_writeback_effect_with_canonical_causality_and_strategy_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:effect",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:canonical",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration.clone(), &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis("causality:effect", "trigger:sha256:commit-a");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:canonical"),
        "effect:sha256:update-profile",
    );

    assert_eq!(
        effect.strategy_descriptor_digest(),
        declaration.strategy_descriptor_digest()
    );
    assert_eq!(
        effect.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert!(contract
        .validated_declaration()
        .strategy_basis()
        .expect("admitted writeback declaration should preserve strategy basis")
        .digest()
        .starts_with("bridge-writeback-strategy-basis:sha256:"));
    assert_eq!(effect.effect_digest(), "effect:sha256:update-profile");
    assert_eq!(effect.causality_digest(), causality.digest());
    assert!(effect
        .digest()
        .starts_with("bridge-derived-writeback-effect:sha256:"));
}

#[test]
fn runtime_maps_writeback_family_input_before_effect_lowering() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:mapped-family-input",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:mapped-family-input",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        "causality:mapped-family-input",
        "trigger:sha256:mapped-family-input",
    );
    let mapper_envelope = runtime.lower_writeback_mapper_envelope(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "evidence:sha256:mapped-family-input",
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "evidence:sha256:mapped-family-input",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mapped-family-input"),
        "effect:sha256:mapped-family-input",
    );
    let lowered_path_mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        "effect:sha256:mapped-family-input",
        "bridge-mapper-evidence:none",
    );

    assert_eq!(mapper_envelope.contract_digest(), contract.digest());
    assert_eq!(
        mapper_envelope.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(mapper_envelope.causality_digest(), causality.digest());
    assert_eq!(
        mapper_envelope.domain_payload_digest(),
        "effect:sha256:mapped-family-input"
    );
    assert_eq!(
        mapper_envelope.domain_evidence_digest(),
        "evidence:sha256:mapped-family-input"
    );
    assert_eq!(
        mapped_input.mapper_envelope_digest(),
        mapper_envelope.digest()
    );
    assert_eq!(mapped_input.contract_digest(), contract.digest());
    assert_eq!(
        mapped_input.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        mapped_input.effect_class(),
        BridgeWritebackEffectClass::ProjectedStateDiff
    );
    assert_eq!(
        mapped_input.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(mapped_input.causality_digest(), causality.digest());
    assert_eq!(
        mapped_input.domain_payload_digest(),
        "effect:sha256:mapped-family-input"
    );
    assert_eq!(
        mapped_input.domain_evidence_digest(),
        "evidence:sha256:mapped-family-input"
    );
    let retained_envelope = runtime
        .diagnostics()
        .writeback_mapper_envelope_for_digest(mapper_envelope.digest())
        .expect("runtime should retain mapper envelope records");
    assert_eq!(retained_envelope.digest(), mapper_envelope.digest());
    let mapper_envelope_explanation = runtime
        .diagnostics()
        .explain_writeback_mapper_envelope(&retained_envelope);
    assert_eq!(
        mapper_envelope_explanation.envelope_digest(),
        mapper_envelope.digest()
    );
    assert_eq!(
        mapper_envelope_explanation.domain_payload_digest(),
        mapper_envelope.domain_payload_digest()
    );
    assert_eq!(
        mapper_envelope_explanation.domain_evidence_digest(),
        mapper_envelope.domain_evidence_digest()
    );
    assert_eq!(effect.contract_digest(), mapped_input.contract_digest());
    assert_eq!(effect.family_kind(), mapped_input.family_kind());
    assert_eq!(effect.effect_class(), mapped_input.effect_class());
    assert_eq!(effect.strategy_class(), mapped_input.strategy_class());
    assert_eq!(effect.causality_digest(), mapped_input.causality_digest());
    assert_eq!(effect.effect_digest(), mapped_input.domain_payload_digest());
    assert_eq!(
        effect.mapper_envelope_digest(),
        lowered_path_mapped_input.mapper_envelope_digest()
    );
    assert_eq!(
        effect.mapped_input_digest(),
        lowered_path_mapped_input.digest()
    );
    assert_eq!(
        effect.contract_digest(),
        lowered_path_mapped_input.contract_digest()
    );
    assert_eq!(
        effect.family_kind(),
        lowered_path_mapped_input.family_kind()
    );
    assert_eq!(
        effect.effect_class(),
        lowered_path_mapped_input.effect_class()
    );
    assert_eq!(
        effect.strategy_class(),
        lowered_path_mapped_input.strategy_class()
    );
    assert_eq!(
        effect.causality_digest(),
        lowered_path_mapped_input.causality_digest()
    );
    assert_eq!(
        effect.effect_digest(),
        lowered_path_mapped_input.domain_payload_digest()
    );
    assert!(effect
        .canonical_basis()
        .contains(lowered_path_mapped_input.digest()));
}
