use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{
    SubscriberRecoveryDecision, SubscriberRecoveryDisposition, SubscriberRecoveryPlan,
    SubscriberRecoverySource, SubscriberResumeRequest, SubscriberStreamFailure,
};
use crate::publication::cdc::diagnostics::recovery_decision_artifact;
use crate::publication::cdc::planning::checkpoint_resolution::{
    durable_checkpoint_available, durable_envelopes, latest_available_checkpoint,
    resolve_checkpoint,
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

    let (start_after_position, mut diagnostics) =
        resolve_checkpoint(runtime, request.checkpoint())?;
    let use_durable_source = request.checkpoint().is_some_and(|checkpoint| {
        !runtime
            .history_access()
            .contains_patch_stream_position(checkpoint.position())
            && durable_checkpoint_available(runtime, checkpoint)
    });
    let source_envelopes = if use_durable_source {
        durable_envelopes(runtime)
    } else {
        Vec::new()
    };
    let decision = SubscriberRecoveryDecision {
        disposition: if start_after_position.is_some() {
            SubscriberRecoveryDisposition::ResumeAfterCheckpoint
        } else {
            SubscriberRecoveryDisposition::StartFromBeginning
        },
        source: if use_durable_source {
            SubscriberRecoverySource::DurableCanonicalRecovery
        } else {
            SubscriberRecoverySource::InMemoryHistory
        },
        start_after_position,
    };
    diagnostics.push(recovery_decision_artifact(&decision));
    Ok((
        SubscriberRecoveryPlan {
            latest_available_checkpoint: latest_available_checkpoint(runtime),
            start_after_position,
            decision,
            request,
            source_envelopes,
        },
        diagnostics,
    ))
}
