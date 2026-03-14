use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{SubscriberStreamBatch, SubscriberStreamFailure};
use crate::publication::cdc::planning::checkpoint_from_patch_position;

pub(crate) fn execute_subscriber_stream(
    runtime: &RelationalRuntime,
    plan: crate::publication::cdc::data::SubscriberRecoveryPlan,
    diagnostics: Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
) -> Result<SubscriberStreamBatch, SubscriberStreamFailure> {
    let patches = match plan.decision.source {
        crate::publication::cdc::data::SubscriberRecoverySource::InMemoryHistory => runtime
            .history_access()
            .patches_after(plan.start_after_position, plan.request.max_commits()),
        crate::publication::cdc::data::SubscriberRecoverySource::DurableCanonicalRecovery => plan
            .source_envelopes
            .iter()
            .filter(|envelope| {
                plan.start_after_position
                    .is_none_or(|position| envelope.patch.position > position)
            })
            .map(|envelope| envelope.patch.clone())
            .take(plan.request.max_commits())
            .collect(),
    };
    let next_checkpoint = patches
        .last()
        .and_then(|patch| checkpoint_from_patch_position(runtime, patch.position));
    let latest_commit_id = runtime
        .history_access()
        .latest_commit()
        .map(|commit| commit.commit_id);

    Ok(SubscriberStreamBatch {
        resumed_from: plan.request.checkpoint().cloned(),
        next_checkpoint,
        latest_available_checkpoint: plan.latest_available_checkpoint,
        recovery_decision: plan.decision,
        latest_commit_id,
        patches,
        diagnostics,
    })
}
