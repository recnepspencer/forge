use super::support::*;

#[test]
fn writeback_batch_naming_digest_changes_with_attachment_identity() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let contract = runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:batch-mutation-authority-naming-digest",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:batch-mutation-authority-naming-digest",
            ),
            &lowered_policy,
        )
        .expect("writeback declaration should admit");

    let left = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-naming-digest:left",
        "trigger:sha256:batch-mutation-authority-naming-digest:left",
        "effect:batch-mutation-authority-naming-digest:left",
        "effect:sha256:batch-mutation-authority-naming-digest:left",
        "idempotence:batch-mutation-authority-naming-digest:left",
        "truth-state:sha256:batch-mutation-authority-naming-digest:left",
    )
    .with_naming_mutation(
        crate::facade::BridgeNamingMutationBundle::attach_new_target(
            "persistent-name:left",
            "entity:task-left",
            Some("Task"),
        ),
    );
    let right = execute_bridge_mutation_bundle(
        &runtime,
        &lowered_policy,
        &contract,
        "causality:batch-mutation-authority-naming-digest:right",
        "trigger:sha256:batch-mutation-authority-naming-digest:right",
        "effect:batch-mutation-authority-naming-digest:right",
        "effect:sha256:batch-mutation-authority-naming-digest:right",
        "idempotence:batch-mutation-authority-naming-digest:right",
        "truth-state:sha256:batch-mutation-authority-naming-digest:right",
    )
    .with_naming_mutation(
        crate::facade::BridgeNamingMutationBundle::attach_new_target(
            "persistent-name:right",
            "entity:task-left",
            Some("Task"),
        ),
    );

    let left_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[left])
        .expect("left component set should aggregate")
        .aggregate_naming_mutation_digest()
        .expect("left naming digest should exist")
        .to_string();
    let right_digest = crate::facade::BridgeBatchMutationAuthorityBundle::from_components(&[right])
        .expect("right component set should aggregate")
        .aggregate_naming_mutation_digest()
        .expect("right naming digest should exist")
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
