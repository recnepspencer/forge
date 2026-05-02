use super::support::*;

#[test]
fn writeback_batch_mutation_authority_bundle_counts_naming_components() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-naming",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-naming",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-naming:a",
        "trigger:sha256:batch-mutation-authority-naming:a",
        "effect:batch-mutation-authority-naming:a",
        "effect:sha256:batch-mutation-authority-naming:a",
        "idempotence:batch-mutation-authority-naming:a",
        "truth-state:sha256:batch-mutation-authority-naming:a",
    )
    .with_naming_mutation(
        crate::facade::BridgeNamingMutationBundle::attach_new_target(
            "persistent-name:first",
            "entity:first",
            Some("Task"),
        ),
    );
    let component_b = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-naming:b",
        "trigger:sha256:batch-mutation-authority-naming:b",
        "effect:batch-mutation-authority-naming:b",
        "effect:sha256:batch-mutation-authority-naming:b",
        "idempotence:batch-mutation-authority-naming:b",
        "truth-state:sha256:batch-mutation-authority-naming:b",
    );

    let aggregate = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[
        component_a,
        component_b,
    ])
    .expect("non-empty component set should aggregate");

    assert_eq!(aggregate.component_count(), 2);
    assert_eq!(aggregate.naming_mutation_count(), 1);
}

#[test]
fn writeback_batch_mutation_authority_bundle_counts_existing_and_symbolic_components() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-existing-symbolic",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-existing-symbolic",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-existing-symbolic:a",
        "trigger:sha256:batch-mutation-authority-existing-symbolic:a",
        "effect:batch-mutation-authority-existing-symbolic:a",
        "effect:sha256:batch-mutation-authority-existing-symbolic:a",
        "idempotence:batch-mutation-authority-existing-symbolic:a",
        "truth-state:sha256:batch-mutation-authority-existing-symbolic:a",
    )
    .with_existing_truth_binding(
        crate::facade::BridgeExistingTruthBindingBundle::direct_entity(
            "authority:task-existing",
            "entity:task-existing",
            Some("Task"),
        ),
    );
    let component_b = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-existing-symbolic:b",
        "trigger:sha256:batch-mutation-authority-existing-symbolic:b",
        "effect:batch-mutation-authority-existing-symbolic:b",
        "effect:sha256:batch-mutation-authority-existing-symbolic:b",
        "idempotence:batch-mutation-authority-existing-symbolic:b",
        "truth-state:sha256:batch-mutation-authority-existing-symbolic:b",
    )
    .with_symbolic_target_reference(
        crate::facade::BridgeSymbolicTargetReferenceBundle::same_batch_target(
            "draft-task",
            "entity:draft-task",
            Some("Task"),
        ),
    );

    let aggregate = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[
        component_a,
        component_b,
    ])
    .expect("non-empty component set should aggregate");

    assert_eq!(aggregate.component_count(), 2);
    assert_eq!(aggregate.existing_truth_binding_count(), 1);
    assert_eq!(aggregate.symbolic_target_reference_count(), 1);
    assert!(aggregate
        .aggregate_existing_truth_binding_digest()
        .is_some());
    assert!(aggregate
        .aggregate_symbolic_target_reference_digest()
        .is_some());
}

#[test]
fn writeback_batch_mutation_authority_bundle_counts_continuity_components() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-continuity",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-continuity",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity:a",
        "trigger:sha256:batch-mutation-authority-continuity:a",
        "effect:batch-mutation-authority-continuity:a",
        "effect:sha256:batch-mutation-authority-continuity:a",
        "idempotence:batch-mutation-authority-continuity:a",
        "truth-state:sha256:batch-mutation-authority-continuity:a",
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-existing",
            Some("authority:task-existing-successor"),
            Some("binding:sha256:task-existing"),
            Some("entity:task-existing"),
            Some("Task"),
            "lineage:sha256:task-existing",
            "continuity:sha256:task-existing",
        ),
    );
    let component_b = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity:b",
        "trigger:sha256:batch-mutation-authority-continuity:b",
        "effect:batch-mutation-authority-continuity:b",
        "effect:sha256:batch-mutation-authority-continuity:b",
        "idempotence:batch-mutation-authority-continuity:b",
        "truth-state:sha256:batch-mutation-authority-continuity:b",
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
fn writeback_batch_continuity_digest_changes_with_resolution_digest() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-continuity-resolution-digest",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-continuity-resolution-digest",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let left = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity-resolution-digest:left",
        "trigger:sha256:batch-mutation-authority-continuity-resolution-digest:left",
        "effect:batch-mutation-authority-continuity-resolution-digest:left",
        "effect:sha256:batch-mutation-authority-continuity-resolution-digest:left",
        "idempotence:batch-mutation-authority-continuity-resolution-digest:left",
        "truth-state:sha256:batch-mutation-authority-continuity-resolution-digest:left",
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-existing",
            Some("authority:task-existing-successor"),
            Some("binding:sha256:task-existing"),
            Some("entity:task-existing"),
            Some("Task"),
            "lineage:sha256:task-existing",
            "continuity:sha256:task-existing:left",
        ),
    );
    let right = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity-resolution-digest:right",
        "trigger:sha256:batch-mutation-authority-continuity-resolution-digest:right",
        "effect:batch-mutation-authority-continuity-resolution-digest:right",
        "effect:sha256:batch-mutation-authority-continuity-resolution-digest:right",
        "idempotence:batch-mutation-authority-continuity-resolution-digest:right",
        "truth-state:sha256:batch-mutation-authority-continuity-resolution-digest:right",
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-existing",
            Some("authority:task-existing-successor"),
            Some("binding:sha256:task-existing"),
            Some("entity:task-existing"),
            Some("Task"),
            "lineage:sha256:task-existing",
            "continuity:sha256:task-existing:right",
        ),
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
fn writeback_batch_continuity_digest_changes_with_binding_basis() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-continuity-binding-digest",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-continuity-binding-digest",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let left = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity-binding-digest:left",
        "trigger:sha256:batch-mutation-authority-continuity-binding-digest:left",
        "effect:batch-mutation-authority-continuity-binding-digest:left",
        "effect:sha256:batch-mutation-authority-continuity-binding-digest:left",
        "idempotence:batch-mutation-authority-continuity-binding-digest:left",
        "truth-state:sha256:batch-mutation-authority-continuity-binding-digest:left",
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-existing",
            Some("authority:task-existing-successor"),
            Some("binding:sha256:task-existing:left"),
            Some("entity:task-existing"),
            Some("Task"),
            "lineage:sha256:task-existing",
            "continuity:sha256:task-existing",
        ),
    );
    let right = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-continuity-binding-digest:right",
        "trigger:sha256:batch-mutation-authority-continuity-binding-digest:right",
        "effect:batch-mutation-authority-continuity-binding-digest:right",
        "effect:sha256:batch-mutation-authority-continuity-binding-digest:right",
        "idempotence:batch-mutation-authority-continuity-binding-digest:right",
        "truth-state:sha256:batch-mutation-authority-continuity-binding-digest:right",
    )
    .with_continuity_mutation(
        crate::facade::BridgeContinuityMutationBundle::rebind_existing_target(
            crate::continuity::BridgeContinuityOutcomeClass::ContinuesAsSingleSuccessor,
            "authority:task-existing",
            Some("authority:task-existing-successor"),
            Some("binding:sha256:task-existing:right"),
            Some("entity:task-existing"),
            Some("Task"),
            "lineage:sha256:task-existing",
            "continuity:sha256:task-existing",
        ),
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

fn execute_bridge_mutation_bundle(
    runtime: &RuntimeBridge,
    lowered_policy: &crate::facade::LoweredBridgeExecutionPolicy,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    causality_identity: &str,
    truth_trigger_digest: &str,
    effect_identity: &str,
    effect_digest: &str,
    idempotence_identity: &str,
    truth_state_digest: &str,
) -> crate::facade::BridgeMutationAuthorityBundle {
    let causality = causality_basis(causality_identity, truth_trigger_digest);
    let effect = runtime.lower_writeback_effect(
        contract,
        &causality,
        BridgeWritebackEffectIdentity::new(effect_identity),
        effect_digest,
    );
    let feedback = crate::facade::BridgeWritebackFeedbackProvenance::new(&effect);
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        lowered_policy,
        truth_state_digest,
        BridgeWritebackIdempotenceIdentity::new(idempotence_identity),
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
