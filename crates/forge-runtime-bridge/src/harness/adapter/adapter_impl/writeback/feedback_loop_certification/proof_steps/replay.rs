use super::super::*;

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) struct FeedbackReplayContextProof
{
    pub replayed_causality: crate::facade::BridgeWritebackNativeCausalityInputs,
    pub replayed_feedback_provenance: crate::facade::BridgeWritebackFeedbackProvenance,
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) struct ChangedEffectFeedbackDenialProof
{
    pub changed_effect: crate::writeback::BridgeDerivedWritebackEffect,
    pub changed_idempotence: crate::facade::BridgeWritebackIdempotenceBasis,
    pub changed_effect_error: crate::facade::BridgeWritebackError,
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) struct ReplayedFeedbackAuthorityProof
{
    pub replayed_idempotence: crate::facade::BridgeWritebackIdempotenceBasis,
    pub loop_prevention: crate::facade::BridgeWritebackLoopPreventionReport,
    pub replayed_outcome: crate::facade::BridgeWritebackAuthorityOutcome,
    pub replayed_receipt: Option<crate::adapter::TruthWritebackReceipt>,
    pub replayed_bundle: crate::facade::BridgeWritebackReplayBundle,
    pub replayed_strategy_coherence: crate::facade::BridgeWritebackStrategyCoherenceReport,
    pub replayed_candidate: Option<crate::facade::BridgeValidatedWritebackCandidate>,
    pub feedback_authority_request: Option<crate::adapter::TruthWritebackRequest>,
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) fn verify_replayed_feedback_context(
    runtime_bridge: &crate::facade::RuntimeBridge,
    original_commit: &crate::facade::BridgeCommittedPatchEnvelope,
    initial_route_digest: &str,
    effect: &crate::writeback::BridgeDerivedWritebackEffect,
    carried_feedback_context: &crate::facade::BridgeWritebackFeedbackContext,
) -> Result<FeedbackReplayContextProof, BridgeHarnessError> {
    let replayed_causality = writeback_causality_basis(
        "harness:writeback-feedback-causality",
        original_commit.commit_identity().as_str(),
        initial_route_digest.to_owned(),
        "feedback",
        original_commit.snapshot_identity().as_str(),
    );
    if replayed_causality.digest() != carried_feedback_context.causality_digest() {
        return Err(BridgeHarnessError::new(format!(
            "feedback patch carried causality `{}` but replayed causality was `{}`",
            carried_feedback_context.causality_digest(),
            replayed_causality.digest()
        )));
    }
    let replayed_feedback_provenance = runtime_bridge.derive_writeback_feedback_provenance(effect);
    if replayed_feedback_provenance.digest() != carried_feedback_context.provenance_digest() {
        return Err(BridgeHarnessError::new(format!(
            "feedback patch carried provenance `{}` but replayed provenance was `{}`",
            carried_feedback_context.provenance_digest(),
            replayed_feedback_provenance.digest()
        )));
    }
    Ok(FeedbackReplayContextProof {
        replayed_causality,
        replayed_feedback_provenance,
    })
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) fn reject_changed_effect_feedback(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    lowered_policy_bundle: &crate::facade::LoweredBridgeExecutionPolicy,
    replayed_causality: &crate::facade::BridgeWritebackNativeCausalityInputs,
    effect: &crate::writeback::BridgeDerivedWritebackEffect,
    carried_feedback_context: &crate::facade::BridgeWritebackFeedbackContext,
) -> ChangedEffectFeedbackDenialProof {
    let changed_effect = runtime_bridge.lower_writeback_effect(
        contract,
        replayed_causality,
        crate::facade::BridgeWritebackEffectIdentity::new(
            "harness:writeback-feedback-effect:changed",
        ),
        writeback_effect_intent(
            BridgeWritebackEffectClass::ProjectedStateDiff,
            effect.digest().to_owned(),
        ),
    );
    let changed_idempotence = runtime_bridge.classify_writeback_idempotence(
        &changed_effect,
        lowered_policy_bundle,
        &crate::facade::BridgeWritebackAuthoritativeStateBasis::from_effect(&changed_effect),
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:changed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let changed_effect_error = runtime_bridge
        .execute_writeback_authority_with_feedback_context(
            contract,
            &changed_effect,
            &changed_idempotence,
            Some(carried_feedback_context),
        )
        .expect_err("same-causality changed-effect feedback must fail closed");

    ChangedEffectFeedbackDenialProof {
        changed_effect,
        changed_idempotence,
        changed_effect_error,
    }
}

pub(in crate::harness::adapter::adapter_impl::writeback::feedback_loop_certification) fn execute_replayed_feedback_authority(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    lowered_policy_bundle: &crate::facade::LoweredBridgeExecutionPolicy,
    effect: &crate::writeback::BridgeDerivedWritebackEffect,
    carried_feedback_context: &crate::facade::BridgeWritebackFeedbackContext,
    replayed_feedback_provenance: &crate::facade::BridgeWritebackFeedbackProvenance,
) -> Result<ReplayedFeedbackAuthorityProof, BridgeHarnessError> {
    let replayed_idempotence = runtime_bridge.classify_writeback_idempotence(
        effect,
        lowered_policy_bundle,
        &crate::facade::BridgeWritebackAuthoritativeStateBasis::from_effect(effect),
        crate::facade::BridgeWritebackIdempotenceIdentity::new(
            "harness:writeback-feedback-idempotence:replayed",
        ),
        crate::facade::BridgeWritebackIdempotenceClass::RequireSemanticNoopSuppression,
    );
    let (loop_prevention, replayed_outcome, replayed_receipt) = runtime_bridge
        .execute_writeback_authority_with_feedback_context(
            contract,
            effect,
            &replayed_idempotence,
            Some(carried_feedback_context),
        )
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification replayed authority execution failed: {error}"
            ))
        })?;
    let replayed_bundle = runtime_bridge.replay_writeback_bundle(
        contract,
        effect,
        &replayed_idempotence,
        &replayed_outcome,
    );
    let replayed_strategy_coherence = runtime_bridge.classify_writeback_strategy_coherence(
        contract,
        effect,
        &replayed_idempotence,
    );
    let replayed_candidate = replayed_feedback_candidate(
        runtime_bridge,
        contract,
        effect,
        &replayed_idempotence,
        &loop_prevention,
        &replayed_strategy_coherence,
    )?;
    let feedback_authority_request = replayed_candidate.as_ref().map(|candidate| {
        feedback_authority_request_from_candidate(
            runtime_bridge,
            contract,
            effect,
            candidate,
            replayed_feedback_provenance,
            &loop_prevention,
            &replayed_strategy_coherence,
            &replayed_idempotence,
        )
    });

    Ok(ReplayedFeedbackAuthorityProof {
        replayed_idempotence,
        loop_prevention,
        replayed_outcome,
        replayed_receipt,
        replayed_bundle,
        replayed_strategy_coherence,
        replayed_candidate,
        feedback_authority_request,
    })
}

fn replayed_feedback_candidate(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    effect: &crate::writeback::BridgeDerivedWritebackEffect,
    replayed_idempotence: &crate::facade::BridgeWritebackIdempotenceBasis,
    loop_prevention: &crate::facade::BridgeWritebackLoopPreventionReport,
    replayed_strategy_coherence: &crate::facade::BridgeWritebackStrategyCoherenceReport,
) -> Result<Option<crate::facade::BridgeValidatedWritebackCandidate>, BridgeHarnessError> {
    if loop_prevention.disposition()
        != crate::facade::BridgeWritebackLoopDisposition::AllowAuthoritativeAttempt
    {
        return Ok(None);
    }
    runtime_bridge
        .validate_writeback_candidate(
            contract,
            effect,
            replayed_idempotence,
            loop_prevention,
            replayed_strategy_coherence,
        )
        .map(Some)
        .map_err(|error| {
            BridgeHarnessError::new(format!(
                "writeback feedback certification replayed candidate validation failed: {error}"
            ))
        })
}

fn feedback_authority_request_from_candidate(
    runtime_bridge: &crate::facade::RuntimeBridge,
    contract: &crate::facade::AdmittedBridgeWritebackContract,
    effect: &crate::writeback::BridgeDerivedWritebackEffect,
    candidate: &crate::facade::BridgeValidatedWritebackCandidate,
    replayed_feedback_provenance: &crate::facade::BridgeWritebackFeedbackProvenance,
    loop_prevention: &crate::facade::BridgeWritebackLoopPreventionReport,
    replayed_strategy_coherence: &crate::facade::BridgeWritebackStrategyCoherenceReport,
    replayed_idempotence: &crate::facade::BridgeWritebackIdempotenceBasis,
) -> crate::adapter::TruthWritebackRequest {
    let mapped_input = runtime_bridge
        .diagnostics()
        .writeback_mapped_family_input_for_digest(effect.mapped_input_digest())
        .expect("writeback harness should retain mapped-family input for feedback certification");
    let mapper_witness = crate::facade::BridgeWritebackMapperWitness::issue(&mapped_input);
    crate::adapter::TruthWritebackRequest::from_evidence(
        crate::adapter::TruthWritebackRequestEvidence {
            contract,
            candidate,
            effect,
            mapper_witness: &mapper_witness,
            feedback_provenance: replayed_feedback_provenance,
            loop_prevention,
            strategy_coherence: replayed_strategy_coherence,
            idempotence: replayed_idempotence,
        },
    )
}
