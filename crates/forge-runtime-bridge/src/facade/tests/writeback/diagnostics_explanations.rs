use super::support::*;

#[test]
fn writeback_diagnostics_explanations_are_artifact_derived_and_stable() {
    let runtime = runtime_with_writeback_authority(BridgeRuntimePolicy::development());
    let lowered_policy = lowered_policy(&runtime);
    let declaration = writeback_declaration(
        "writeback:diagnostics-artifact-derived",
        BridgeRequestKind::Authoritative,
        BridgeWritebackRequestMode::WritebackCapable,
        "strategy:sha256:diagnostics-artifact-derived",
    );
    let contract = runtime
        .admit_writeback_declaration(declaration, &lowered_policy)
        .expect("writeback declaration should admit");
    let effect = runtime.lower_writeback_effect(
        &contract,
        &causality_basis(
            "causality:diagnostics-artifact-derived",
            "trigger:sha256:commit-a",
        ),
        BridgeWritebackEffectIdentity::new("effect:diagnostics-artifact-derived"),
        "effect:sha256:diagnostics-artifact-derived",
    );
    let idempotence = runtime.classify_writeback_idempotence(
        &effect,
        &lowered_policy,
        "truth-state:sha256:diagnostics-artifact-derived",
        BridgeWritebackIdempotenceIdentity::new("idempotence:diagnostics-artifact-derived"),
        BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let loop_prevention = runtime.classify_writeback_loop_prevention(
        &effect,
        &idempotence,
        None::<std::sync::Arc<str>>,
        None::<std::sync::Arc<str>>,
    );
    let strategy_compatibility =
        runtime.classify_writeback_strategy_compatibility(&contract, &effect, &idempotence);
    let candidate = runtime
        .validate_writeback_candidate(
            &contract,
            &effect,
            &idempotence,
            &loop_prevention,
            &strategy_compatibility,
        )
        .expect("candidate validation should succeed");
    let (outcome, _) = runtime
        .execute_writeback_authority(&contract, &effect, &idempotence)
        .expect("authority execution should succeed");
    let replay_bundle = runtime.replay_writeback_bundle(&contract, &effect, &idempotence, &outcome);

    let candidate_explanation = runtime
        .diagnostics()
        .explain_writeback_candidate(&candidate);
    let loop_explanation = runtime
        .diagnostics()
        .explain_writeback_loop_prevention(&loop_prevention);
    let compatibility_explanation = runtime
        .diagnostics()
        .explain_writeback_strategy_compatibility(&strategy_compatibility);
    let outcome_explanation = runtime.diagnostics().explain_writeback_outcome(&outcome);
    let replay_explanation = runtime
        .diagnostics()
        .explain_writeback_replay_bundle(&replay_bundle);
    let mapper_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_mapper_record()
        .expect("writeback mapper explanation should exist");
    let execution_record_explanation = runtime
        .diagnostics()
        .explain_last_writeback_execution_record()
        .expect("writeback execution record explanation should exist");

    assert_eq!(candidate_explanation.candidate_digest(), candidate.digest());
    assert_eq!(
        candidate_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        candidate_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        candidate_explanation.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(
        loop_explanation.loop_prevention_digest(),
        loop_prevention.digest()
    );
    assert_eq!(
        loop_explanation.disposition(),
        BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    );
    assert_eq!(
        compatibility_explanation.compatibility_digest(),
        strategy_compatibility.digest()
    );
    assert_eq!(
        compatibility_explanation.disposition(),
        BridgeWritebackStrategyCompatibilityDisposition::Compatible
    );
    assert_eq!(
        mapper_record_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        mapper_record_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        mapper_record_explanation.causality_digest(),
        effect.causality_digest()
    );
    assert_eq!(
        mapper_record_explanation.proposed_effect_digest(),
        effect.effect_digest()
    );
    assert_eq!(outcome_explanation.outcome_digest(), outcome.digest());
    assert_eq!(
        outcome_explanation.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
    assert_eq!(
        execution_record_explanation.idempotence_digest(),
        idempotence.digest()
    );
    assert_eq!(
        execution_record_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        execution_record_explanation.loop_prevention_digest(),
        loop_prevention.digest()
    );
    assert_eq!(
        execution_record_explanation.strategy_compatibility_digest(),
        strategy_compatibility.digest()
    );
    assert_eq!(
        execution_record_explanation.mapper_record_digest(),
        Some(
            runtime
                .diagnostics()
                .last_writeback_mapper_record()
                .expect("writeback mapper record should exist")
                .digest()
        )
    );
    assert_eq!(
        replay_explanation.replay_bundle_digest(),
        replay_bundle.digest()
    );
    assert_eq!(
        replay_explanation.family_kind(),
        BridgeWritebackFamilyKind::ProjectedStateDiff
    );
    assert_eq!(
        replay_explanation.strategy_class(),
        BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation
    );
    assert_eq!(
        replay_explanation.causality_digest(),
        effect.causality_digest()
    );
    assert_eq!(
        replay_explanation.retry_disposition(),
        crate::facade::BridgeWritebackRetryDisposition::SemanticNoopSuppressionRequired
    );
    assert_eq!(
        replay_explanation.outcome_class(),
        crate::facade::BridgeWritebackOutcomeClass::AuthoritativeCommit
    );
}
