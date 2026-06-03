use super::super::*;

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) struct RestartFeedbackReplayProof
{
    pub rebuilt_contract: crate::facade::AdmittedBridgeWritebackContract,
    pub rebuilt_effect: crate::writeback::BridgeDerivedWritebackEffect,
    pub rebuilt_idempotence: crate::facade::BridgeWritebackIdempotenceBasis,
    pub rebuilt_loop_prevention: crate::facade::BridgeWritebackLoopPreventionReport,
    pub rebuilt_outcome: crate::facade::BridgeWritebackAuthorityOutcome,
    pub rebuilt_replay_bundle: crate::facade::BridgeWritebackReplayBundle,
    pub rebuilt_receipt: Option<crate::adapter::TruthWritebackReceipt>,
    pub counter_snapshot: WritebackCounterSnapshot,
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) fn rebuild_feedback_replay_proof(
    runtime: &crate::harness::adapter::BridgeHarnessSession,
    runtime_bridge: &crate::facade::RuntimeBridge,
    fixture: &BridgeHarnessFixture,
    replayed_causality: &crate::facade::BridgeWritebackCausalityBasis,
    carried_feedback_context: &crate::facade::BridgeWritebackFeedbackContext,
) -> Result<RestartFeedbackReplayProof, BridgeHarnessError> {
    let rebuilt_runtime = build_writeback_runtime(runtime, fixture, true)?;
    let rebuilt_lowered_policy = lowered_policy(&rebuilt_runtime)?;
    let rebuilt_contract = rebuilt_runtime
        .admit_writeback_declaration(
            crate::facade::BridgeWritebackDeclaration::writeback_capable(
                crate::facade::BridgeWritebackDeclarationIdentity::new(
                    "harness:writeback-feedback",
                ),
                crate::facade::BridgeRequestKind::Authoritative,
                crate::facade::BridgeWritebackFamilyKind::ProjectedStateDiff,
                crate::facade::BridgeWritebackEffectClass::ProjectedStateDiff,
                crate::facade::BridgeWritebackStrategyClass::ProjectedStateDiffReconciliation,
                crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
            ),
            &rebuilt_lowered_policy,
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification failed to admit rebuilt contract: {error}"
            ))
        })?;
    let rebuilt_effect = rebuilt_runtime.lower_writeback_effect(
        &rebuilt_contract,
        replayed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new("harness:writeback-feedback-effect"),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            rebuilt_contract.digest().to_owned(),
        ),
    );
    let rebuilt_idempotence = rebuilt_runtime.classify_writeback_idempotence(
        &rebuilt_effect,
        &rebuilt_lowered_policy,
        &crate::facade::BridgeWritebackAuthoritativeStateBasis::from_effect(&rebuilt_effect),
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (rebuilt_loop_prevention, rebuilt_outcome, rebuilt_receipt) = rebuilt_runtime
        .execute_writeback_authority_with_feedback_context(
            &rebuilt_contract,
            &rebuilt_effect,
            &rebuilt_idempotence,
            Some(carried_feedback_context),
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification replay-after-rebuild execution failed: {error}"
            ))
        })?;
    let rebuilt_replay_bundle = rebuilt_runtime.replay_writeback_bundle(
        &rebuilt_contract,
        &rebuilt_effect,
        &rebuilt_idempotence,
        &rebuilt_outcome,
    );
    let counters = aggregate_runtime_writeback_counters(&[runtime_bridge, &rebuilt_runtime]);
    let counter_snapshot = snapshot_from_counters(&counters);

    Ok(RestartFeedbackReplayProof {
        rebuilt_contract,
        rebuilt_effect,
        rebuilt_idempotence,
        rebuilt_loop_prevention,
        rebuilt_outcome,
        rebuilt_replay_bundle,
        rebuilt_receipt,
        counter_snapshot,
    })
}
