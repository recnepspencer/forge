use super::support::*;

#[test]
fn runtime_lowers_writeback_effect_with_canonical_causality_and_strategy_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:effect"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "canonical",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration.clone(), &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:effect"),
        "commit-a",
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:canonical"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "update-profile",
        ),
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
    assert!(effect
        .effect_intent_digest()
        .starts_with("bridge-writeback-effect-intent:sha256:"));
    assert_eq!(effect.causality_digest(), causality.digest());
    assert!(effect.digest().starts_with("bridge-derived-writeback-"));
}

#[test]
fn runtime_maps_writeback_family_input_before_effect_lowering() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:mapped-family-input"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "mapped-family-input",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:mapped-family-input"),
        "mapped-family-input",
    );
    let mapper_envelope = runtime.lower_writeback_mapper_envelope(
        &contract,
        &causality,
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mapped-family-input",
        ),
    );
    let mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mapped-family-input",
        ),
    );
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:mapped-family-input"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mapped-family-input",
        ),
    );
    let lowered_path_mapped_input = runtime.map_writeback_family_input(
        &contract,
        &causality,
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "mapped-family-input",
        ),
    );

    assert_eq!(mapper_envelope.contract_digest(), contract.digest());
    assert_eq!(
        mapper_envelope.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(mapper_envelope.causality_digest(), causality.digest());
    assert_eq!(
        mapper_envelope.effect_intent_digest(),
        mapped_input.effect_intent_digest()
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
        mapped_input.effect_intent_digest(),
        mapper_envelope.effect_intent_digest()
    );
    let retained_envelope = runtime
        .diagnostics()
        .writeback_mapper_envelope_for_digest(mapper_envelope.digest())
        .expect("runtime should retain mapper envelope records");
    assert_eq!(retained_envelope.digest(), mapper_envelope.digest());
    let mapper_envelope_explanation = runtime
        .diagnostics()
        .explain_writeback_mapper_envelope(&retained_envelope);
    assert_eq!(mapper_envelope_explanation.envelope(), &retained_envelope);
    assert_eq!(
        mapper_envelope_explanation.envelope_digest(),
        mapper_envelope.digest()
    );
    assert_eq!(
        mapper_envelope_explanation.effect_intent_digest(),
        mapper_envelope.effect_intent_digest()
    );
    let retained_mapped_input = runtime
        .diagnostics()
        .writeback_mapped_family_input_for_digest(mapped_input.digest())
        .expect("runtime should retain mapped family input records");
    let mapped_input_explanation = runtime
        .diagnostics()
        .explain_writeback_mapped_family_input(&retained_mapped_input);
    assert_eq!(
        mapped_input_explanation.mapped_input(),
        &retained_mapped_input
    );
    assert_eq!(
        mapped_input_explanation.effect_intent_digest(),
        mapped_input.effect_intent_digest()
    );
    assert_eq!(effect.contract_digest(), mapped_input.contract_digest());
    assert_eq!(effect.family_kind(), mapped_input.family_kind());
    assert_eq!(effect.effect_class(), mapped_input.effect_class());
    assert_eq!(effect.strategy_class(), mapped_input.strategy_class());
    assert_eq!(effect.causality_digest(), mapped_input.causality_digest());
    assert_eq!(
        effect.effect_intent_digest(),
        mapped_input.effect_intent_digest()
    );
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
        effect.effect_intent_digest(),
        lowered_path_mapped_input.effect_intent_digest()
    );
    assert!(effect
        .canonical_basis()
        .contains(lowered_path_mapped_input.digest()));
}

#[test]
fn derived_writeback_effect_digest_is_bound_to_typed_effect_intent_basis() {
    let runtime = runtime(BridgeRuntimePolicy::default());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        BridgeWritebackDeclarationIdentity::new("writeback:effect-intent-proof"),
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "effect-intent-proof",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let causality = causality_basis(
        BridgeWritebackCausalityIdentity::new("causality:effect-intent-proof"),
        "commit-a",
    );
    let baseline_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:intent-proof"),
        writeback_effect_intent(BridgeWritebackEffectClass::ProjectedStateDiff, "baseline"),
    );
    let changed_effect = runtime.lower_writeback_effect(
        &contract,
        &causality,
        BridgeWritebackEffectIdentity::new("effect:intent-proof"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            "changed-native-value",
        ),
    );

    assert_ne!(
        baseline_effect.effect_intent_digest(),
        changed_effect.effect_intent_digest()
    );
    assert_ne!(baseline_effect.digest(), changed_effect.digest());
    assert!(baseline_effect
        .canonical_basis()
        .contains(baseline_effect.effect_intent().patch_canonical_basis()));
    assert!(changed_effect
        .canonical_basis()
        .contains(changed_effect.effect_intent().patch_canonical_basis()));
}
