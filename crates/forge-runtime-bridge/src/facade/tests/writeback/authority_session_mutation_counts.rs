use super::authority_session_mutation_evidence::execute_bridge_mutation_bundle;
use super::support::*;

#[test]
fn writeback_batch_mutation_authority_bundle_counts_naming_components() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                BridgeWritebackDeclarationIdentity::new(
                    "writeback:batch-mutation-authority-naming",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-naming",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new("causality:batch-mutation-authority-naming:a"),
        "batch-mutation-authority-naming:a",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-naming:a"),
        "batch-mutation-authority-naming:a",
        BridgeWritebackIdempotenceIdentity::new("idempotence:batch-mutation-authority-naming:a"),
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
        BridgeWritebackCausalityIdentity::new("causality:batch-mutation-authority-naming:b"),
        "batch-mutation-authority-naming:b",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-naming:b"),
        "batch-mutation-authority-naming:b",
        BridgeWritebackIdempotenceIdentity::new("idempotence:batch-mutation-authority-naming:b"),
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
                BridgeWritebackDeclarationIdentity::new(
                    "writeback:batch-mutation-authority-existing-symbolic",
                ),
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "batch-mutation-authority-existing-symbolic",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let component_a = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-existing-symbolic:a",
        ),
        "batch-mutation-authority-existing-symbolic:a",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-existing-symbolic:a"),
        "batch-mutation-authority-existing-symbolic:a",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-existing-symbolic:a",
        ),
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
        BridgeWritebackCausalityIdentity::new(
            "causality:batch-mutation-authority-existing-symbolic:b",
        ),
        "batch-mutation-authority-existing-symbolic:b",
        BridgeWritebackEffectIdentity::new("effect:batch-mutation-authority-existing-symbolic:b"),
        "batch-mutation-authority-existing-symbolic:b",
        BridgeWritebackIdempotenceIdentity::new(
            "idempotence:batch-mutation-authority-existing-symbolic:b",
        ),
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
