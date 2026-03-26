use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{
    SubscriberRecoveryDecision, SubscriberRecoveryPlan, SubscriberRecoverySource,
    SubscriberResumeRequest, SubscriberStreamFailure,
};
use crate::publication::cdc::diagnostics::{
    continuation_assessment_artifact, recovery_decision_artifact,
};
use crate::publication::cdc::planning::checkpoint_resolution::{
    checkpoint_basis_from_patch_position, durable_envelopes, latest_available_checkpoint,
    resolve_checkpoint,
};
use crate::publication::cdc::planning::{
    assess_subscriber_continuity, disposition_for_assessment, select_execution_envelopes,
};

pub(crate) fn plan_subscriber_recovery(
    runtime: &RelationalRuntime,
    request: SubscriberResumeRequest,
) -> Result<
    (
        SubscriberRecoveryPlan,
        Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    ),
    SubscriberStreamFailure,
> {
    if request.max_commits() == 0 {
        return Err(SubscriberStreamFailure::new(
            crate::publication::cdc::data::SubscriberStreamFailureClass::InvalidBatchSize,
            "subscriber stream request must ask for at least one commit",
            latest_available_checkpoint(runtime),
            vec![crate::publication::cdc::diagnostics::rejection_artifact(
                crate::publication::cdc::data::SubscriberStreamFailureClass::InvalidBatchSize,
                "subscriber stream request must ask for at least one commit",
            )],
        ));
    }

    let preloaded_durable_envelopes = request.checkpoint().and_then(|checkpoint| {
        (!runtime
            .history_access()
            .contains_patch_stream_position(checkpoint.position()))
        .then(|| durable_envelopes(runtime))
    });
    let (start_after_position, mut diagnostics) = resolve_checkpoint(
        runtime,
        request.checkpoint(),
        preloaded_durable_envelopes.as_deref(),
    )?;
    let use_durable_source = preloaded_durable_envelopes
        .as_ref()
        .is_some_and(|envelopes| {
            request.checkpoint().is_some_and(|checkpoint| {
                envelopes
                    .iter()
                    .any(|envelope| envelope.patch.position == checkpoint.position())
            })
        });
    let available_envelopes = if let Some(envelopes) = preloaded_durable_envelopes {
        envelopes
    } else {
        runtime
            .history_access()
            .envelopes_after(start_after_position, usize::MAX)
    };
    if let Some(checkpoint) = request.checkpoint() {
        if checkpoint.subscriber_contract_id() != request.subscriber_contract().contract_id {
            let detail = format!(
                "subscriber checkpoint contract {} does not match requested subscriber contract {}",
                checkpoint.subscriber_contract_id(),
                request.subscriber_contract().contract_id
            );
            return Err(SubscriberStreamFailure::new(
                crate::publication::cdc::data::SubscriberStreamFailureClass::SubscriberContractMismatch,
                detail.clone(),
                latest_available_checkpoint(runtime),
                vec![crate::publication::cdc::diagnostics::rejection_artifact(
                    crate::publication::cdc::data::SubscriberStreamFailureClass::SubscriberContractMismatch,
                    &detail,
                )],
            ));
        }
    }
    let selected_envelopes = select_execution_envelopes(
        &available_envelopes,
        start_after_position,
        request.max_commits(),
    );
    let prior_proof = request
        .checkpoint()
        .map(|checkpoint| checkpoint.normalized_continuation_proof().clone())
        .unwrap_or_default();
    let fallback_descriptor_semantics_version = request
        .checkpoint()
        .map(|checkpoint| checkpoint.descriptor_semantics_version())
        .or_else(|| {
            selected_envelopes
                .last()
                .map(|envelope| envelope.descriptor_semantics_version)
        })
        .unwrap_or_default();
    let continuation_assessment = assess_subscriber_continuity(
        runtime,
        &selected_envelopes,
        request.subscriber_contract(),
        &prior_proof,
        fallback_descriptor_semantics_version,
    )
    .map_err(|failure| {
        SubscriberStreamFailure::new(
            failure.class,
            failure.detail,
            latest_available_checkpoint(runtime),
            failure.diagnostics,
        )
    })?;
    let latest_available_assessment = if selected_envelopes.len() == available_envelopes.len() {
        None
    } else {
        Some(
            assess_subscriber_continuity(
                runtime,
                &available_envelopes,
                request.subscriber_contract(),
                &prior_proof,
                fallback_descriptor_semantics_version,
            )
            .map_err(|failure| {
                SubscriberStreamFailure::new(
                    failure.class,
                    failure.detail,
                    latest_available_checkpoint(runtime),
                    failure.diagnostics,
                )
            })?,
        )
    };
    let decision = SubscriberRecoveryDecision {
        disposition: disposition_for_assessment(
            start_after_position,
            continuation_assessment.continuation_outcome,
        ),
        source: if use_durable_source {
            SubscriberRecoverySource::DurableCanonicalRecovery
        } else {
            SubscriberRecoverySource::InMemoryHistory
        },
        start_after_position,
    };
    let latest_available_checkpoint = available_envelopes
        .last()
        .and_then(|envelope| checkpoint_basis_from_patch_position(runtime, envelope.patch.position))
        .map(|basis| {
            let latest_assessment = latest_available_assessment
                .as_ref()
                .unwrap_or(&continuation_assessment);
            let descriptor_semantics_version = latest_assessment
                .normalized_continuation_proof
                .descriptor_semantics_version();
            crate::publication::cdc::data::SubscriberCheckpoint::from_basis_with_assessment(
                basis,
                request.subscriber_contract().contract_id.clone(),
                latest_assessment,
                descriptor_semantics_version,
            )
        })
        .or_else(|| request.checkpoint().cloned());
    diagnostics.push(continuation_assessment_artifact(
        &request,
        &continuation_assessment,
    ));
    diagnostics.push(recovery_decision_artifact(&decision));
    Ok((
        SubscriberRecoveryPlan::new(
            request,
            decision,
            latest_available_checkpoint,
            start_after_position,
            selected_envelopes,
            continuation_assessment,
        ),
        diagnostics,
    ))
}
