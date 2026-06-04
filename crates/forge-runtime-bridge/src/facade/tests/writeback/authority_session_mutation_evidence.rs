use super::support::*;

#[test]
fn writeback_batch_mutation_authority_bundle_counts_continuity_components() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new(
                    "writeback:batch-mutation-authority-continuity",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-continuity",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new("causality:batch-mutation-authority-continuity:a"),
        "batch-mutation-authority-continuity:a",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-continuity:a"),
        "batch-mutation-authority-continuity:a",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity:a",
        ),
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            BridgeContinuityAuthoritativeIdentity::new("authority:task-existing")
                .expect("test continuity authoritative identity should be native"),
            Some(
                BridgeContinuityAuthoritativeIdentity::new("authority:task-existing-successor")
                    .expect("test continuity authoritative identity should be native"),
            ),
            Some(
                BridgeContinuityResolvedTargetIdentity::new("entity:task-existing")
                    .expect("test continuity resolved target should be native"),
            ),
            Some(
                BridgeContinuityTargetCollection::new("Task")
                    .expect("test continuity target collection should be native"),
            ),
        )
        .expect("semantic continuity mutation evidence should derive"),
    );
    let component_b = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new("causality:batch-mutation-authority-continuity:b"),
        "batch-mutation-authority-continuity:b",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-continuity:b"),
        "batch-mutation-authority-continuity:b",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity:b",
        ),
    );

    let aggregate = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[
        component_a,
        component_b,
    ])
    .expect("non-empty component set should aggregate");

    assert_eq!(aggregate.component_count(), 2);
    assert_eq!(aggregate.continuity_mutation_count(), 1);
    assert!(aggregate.aggregate_continuity_mutation_digest().is_some());
}

#[test]
fn writeback_batch_continuity_digest_changes_with_successor_resolution_evidence() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new(
                    "writeback:batch-mutation-authority-continuity-resolution-digest",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-continuity-resolution-digest",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let left = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-continuity-resolution-digest:left",
        ),
        "batch-mutation-authority-continuity-resolution-digest:left",
        BridgeWritebackEffectIdentity::new(
            "effect:batch-mutation-authority-continuity-resolution-digest:left",
        ),
        "batch-mutation-authority-continuity-resolution-digest:left",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity-resolution-digest:left",
        ),
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            BridgeContinuityAuthoritativeIdentity::new("authority:task-existing")
                .expect("test continuity authoritative identity should be native"),
            Some(
                BridgeContinuityAuthoritativeIdentity::new(
                    "authority:task-existing-successor:left",
                )
                .expect("test continuity authoritative identity should be native"),
            ),
            Some(
                BridgeContinuityResolvedTargetIdentity::new("entity:task-existing")
                    .expect("test continuity resolved target should be native"),
            ),
            Some(
                BridgeContinuityTargetCollection::new("Task")
                    .expect("test continuity target collection should be native"),
            ),
        )
        .expect("left semantic continuity mutation evidence should derive"),
    );
    let right = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-continuity-resolution-digest:right",
        ),
        "batch-mutation-authority-continuity-resolution-digest:right",
        BridgeWritebackEffectIdentity::new(
            "effect:batch-mutation-authority-continuity-resolution-digest:right",
        ),
        "batch-mutation-authority-continuity-resolution-digest:right",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity-resolution-digest:right",
        ),
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            BridgeContinuityAuthoritativeIdentity::new("authority:task-existing")
                .expect("test continuity authoritative identity should be native"),
            Some(
                BridgeContinuityAuthoritativeIdentity::new(
                    "authority:task-existing-successor:right",
                )
                .expect("test continuity authoritative identity should be native"),
            ),
            Some(
                BridgeContinuityResolvedTargetIdentity::new("entity:task-existing")
                    .expect("test continuity resolved target should be native"),
            ),
            Some(
                BridgeContinuityTargetCollection::new("Task")
                    .expect("test continuity target collection should be native"),
            ),
        )
        .expect("right semantic continuity mutation evidence should derive"),
    );

    let left_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[left])
        .expect("left component set should aggregate")
        .aggregate_continuity_mutation_digest()
        .expect("left digest should exist")
        .to_string();
    let right_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[right])
        .expect("right component set should aggregate")
        .aggregate_continuity_mutation_digest()
        .expect("right digest should exist")
        .to_string();

    assert_ne!(left_digest, right_digest);
}

#[test]
fn writeback_batch_continuity_digest_changes_with_target_binding_evidence() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new(
                    "writeback:batch-mutation-authority-continuity-binding-digest",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-continuity-binding-digest",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let left = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-continuity-binding-digest:left",
        ),
        "batch-mutation-authority-continuity-binding-digest:left",
        BridgeWritebackEffectIdentity::new(
            "effect:batch-mutation-authority-continuity-binding-digest:left",
        ),
        "batch-mutation-authority-continuity-binding-digest:left",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity-binding-digest:left",
        ),
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            BridgeContinuityAuthoritativeIdentity::new("authority:task-existing")
                .expect("test continuity authoritative identity should be native"),
            Some(
                BridgeContinuityAuthoritativeIdentity::new("authority:task-existing-successor")
                    .expect("test continuity authoritative identity should be native"),
            ),
            Some(
                BridgeContinuityResolvedTargetIdentity::new("entity:task-existing:left")
                    .expect("test continuity resolved target should be native"),
            ),
            Some(
                BridgeContinuityTargetCollection::new("Task")
                    .expect("test continuity target collection should be native"),
            ),
        )
        .expect("left target binding evidence should derive"),
    );
    let right = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-continuity-binding-digest:right",
        ),
        "batch-mutation-authority-continuity-binding-digest:right",
        BridgeWritebackEffectIdentity::new(
            "effect:batch-mutation-authority-continuity-binding-digest:right",
        ),
        "batch-mutation-authority-continuity-binding-digest:right",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-continuity-binding-digest:right",
        ),
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            BridgeContinuityAuthoritativeIdentity::new("authority:task-existing")
                .expect("test continuity authoritative identity should be native"),
            Some(
                BridgeContinuityAuthoritativeIdentity::new("authority:task-existing-successor")
                    .expect("test continuity authoritative identity should be native"),
            ),
            Some(
                BridgeContinuityResolvedTargetIdentity::new("entity:task-existing:right")
                    .expect("test continuity resolved target should be native"),
            ),
            Some(
                BridgeContinuityTargetCollection::new("Task")
                    .expect("test continuity target collection should be native"),
            ),
        )
        .expect("right target binding evidence should derive"),
    );

    let left_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[left])
        .expect("left component set should aggregate")
        .aggregate_continuity_mutation_digest()
        .expect("left digest should exist")
        .to_string();
    let right_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[right])
        .expect("right component set should aggregate")
        .aggregate_continuity_mutation_digest()
        .expect("right digest should exist")
        .to_string();

    assert_ne!(left_digest, right_digest);
}

pub(super) fn execute_bridge_mutation_bundle(
    runtime: &RuntimeBridge,
    lowered_policy: &crate::facade::LoweredBridgeExecutionPolicy,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    causality_identity: BridgeWritebackCausalityIdentity,
    truth_trigger_evidence_text: &str,
    effect_identity: BridgeWritebackEffectIdentity,
    effect_intent_value: &str,
    idempotence_identity: BridgeWritebackIdempotenceIdentity,
) -> crate::facade::BridgeMutationAuthorityBundle {
    let causality = causality_basis(causality_identity, truth_trigger_evidence_text);
    let effect = runtime.lower_writeback_effect(
        contract,
        &causality,
        effect_identity,
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            effect_intent_value,
        ),
    );
    let feedback = crate::facade::BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        lowered_policy,
        &truth_state_basis(&effect),
        idempotence_identity,
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (outcome, _) = runtime
        .execute_writeback_authority(contract, &effect, &idempotence)
        .expect("authority execution should succeed");
    let execution_record = runtime
        .diagnostics()
        .last_writeback_execution_record()
        .expect("runtime should retain a native writeback execution record");

    crate::facade::BridgeMutationAuthorityBundle::from_writeback_artifacts(
        &causality,
        &effect,
        &feedback,
        &execution_record,
        Some(&outcome),
    )
}
