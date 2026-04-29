use super::support::*;

#[test]
fn runtime_rejects_replayed_writeback_bundle_when_semantic_meaning_drifts() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:replay-mismatch",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:replay-mismatch",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let original_effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay-mismatch", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-mismatch:original"),
        "effect:sha256:replay-mismatch:original",
    );
    let drifted_effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis("causality:replay-mismatch", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-mismatch:drifted"),
        "effect:sha256:replay-mismatch:drifted",
    );
    let original_idempotence = runtime.classify_writeback_idempotence(
        &original_effect,
        &lowered_policy,
        "truth-state:sha256:replay-mismatch",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-mismatch:original"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let drifted_idempotence = runtime.classify_writeback_idempotence(
        &drifted_effect,
        &lowered_policy,
        "truth-state:sha256:replay-mismatch",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-mismatch:drifted"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );

    let original_bundle = runtime.replay_writeback_bundle(
        &contract,
        &original_effect,
        &original_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &original_idempotence,
            "authoritative-artifact:sha256:replay-mismatch",
        ),
    );
    let drifted_bundle = runtime.replay_writeback_bundle(
        &contract,
        &drifted_effect,
        &drifted_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &drifted_idempotence,
            "authoritative-artifact:sha256:replay-mismatch",
        ),
    );

    let error = runtime
        .validate_replayed_writeback_bundle(&original_bundle, &drifted_bundle)
        .expect_err("replayed writeback bundle should reject semantic drift");

    assert_eq!(error.kind(), BridgeWritebackErrorKind::ReplayMismatch);
    assert!(error.to_string().contains("semantic mismatch"));
    assert_ne!(original_bundle.digest(), drifted_bundle.digest());
    assert_ne!(
        original_bundle.semantic_digest(),
        drifted_bundle.semantic_digest()
    );

    let replay_record = runtime
        .diagnostics()
        .last_writeback_replay_record()
        .expect("runtime should retain a native writeback replay record");
    assert_eq!(
        replay_record.failure_class(),
        Some(crate::facade::BridgeWritebackFailureClass::ReplayMismatch)
    );
    assert_eq!(
        replay_record.expected_replay_digest(),
        original_bundle.digest()
    );
    assert_eq!(
        replay_record.replayed_replay_digest(),
        drifted_bundle.digest()
    );
    assert_eq!(replay_record.counters().writeback_replay_request_count(), 1);
    assert_eq!(
        replay_record.counters().writeback_replay_mismatch_count(),
        1
    );

    let replay_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_replay_record()
        .expect("writeback replay record explanation should exist");
    assert_eq!(
        replay_record_explanation.expected_causality_digest(),
        original_bundle.causality_digest()
    );
    assert_eq!(
        replay_record_explanation.replayed_causality_digest(),
        drifted_bundle.causality_digest()
    );
}

#[test]
fn runtime_accepts_replayed_writeback_bundle_when_only_diagnostics_detail_differs() {
    let standard_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let exhaustive_runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::forensic());
    let standard_lowered_policy = lowered_policy(&standard_runtime);
    let exhaustive_lowered_policy = lowered_policy(&exhaustive_runtime);

    let standard_contract = standard_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:replay-semantic-standard",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:replay-semantic",
            ),
            &standard_lowered_policy,
        )
        .expect("standard writeback declaration should admit");
    let exhaustive_contract = exhaustive_runtime
        .admit_writeback_declaration(
            writeback_declaration(
                "writeback:replay-semantic-exhaustive",
                BridgeRequestKind::Authoritative,
                BridgeWritebackRequestMode::WritebackCapable,
                "strategy:sha256:replay-semantic",
            ),
            &exhaustive_lowered_policy,
        )
        .expect("exhaustive writeback declaration should admit");

    let standard_effect = standard_runtime.lower_writeback_effect(
        &standard_contract,
        &causality_basis("causality:replay-semantic", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-semantic"),
        "effect:sha256:replay-semantic",
    );
    let exhaustive_effect = exhaustive_runtime.lower_writeback_effect(
        &exhaustive_contract,
        &causality_basis("causality:replay-semantic", "trigger:sha256:commit-a"),
        BridgeWritebackEffectIdentity::new("effect:replay-semantic"),
        "effect:sha256:replay-semantic",
    );
    let standard_idempotence = standard_runtime.classify_writeback_idempotence(
        &standard_effect,
        &standard_lowered_policy,
        "truth-state:sha256:replay-semantic",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-semantic"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let exhaustive_idempotence = exhaustive_runtime.classify_writeback_idempotence(
        &exhaustive_effect,
        &exhaustive_lowered_policy,
        "truth-state:sha256:replay-semantic",
        BridgeWritebackIdempotenceIdentity::new("idempotence:replay-semantic"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let standard_bundle = standard_runtime.replay_writeback_bundle(
        &standard_contract,
        &standard_effect,
        &standard_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &standard_idempotence,
            "authoritative-artifact:sha256:replay-semantic-standard",
        ),
    );
    let exhaustive_bundle = exhaustive_runtime.replay_writeback_bundle(
        &exhaustive_contract,
        &exhaustive_effect,
        &exhaustive_idempotence,
        &crate::facade::BridgeWritebackAuthorityOutcome::authoritative_commit(
            &exhaustive_idempotence,
            "authoritative-artifact:sha256:replay-semantic-exhaustive",
        ),
    );

    standard_runtime
        .validate_replayed_writeback_bundle(&standard_bundle, &exhaustive_bundle)
        .expect("replay validation should accept diagnostics-only detail drift");
}
