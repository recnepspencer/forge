use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{SubscriberCheckpoint, SubscriberStreamFailureClass};
use crate::publication::cdc::diagnostics::{checkpoint_resolution_artifact, rejection_artifact};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::{publication::cdc::data::SubscriberStreamFailure, schema::data::SchemaVersionId};

pub(crate) fn latest_available_checkpoint(
    runtime: &RelationalRuntime,
) -> Option<SubscriberCheckpoint> {
    let latest_position = runtime.history_access().latest_patch_stream_position()?;
    let commit_id = *runtime.history.patch_stream_index.get(&latest_position)?;
    let history = runtime.history_access();
    let envelope = history.commit_envelope(commit_id)?;
    Some(SubscriberCheckpoint::new(
        latest_position,
        ReplaySchemaVersion(1),
        envelope.schema_version,
    ))
}

pub(crate) fn resolve_checkpoint(
    runtime: &RelationalRuntime,
    checkpoint: Option<&SubscriberCheckpoint>,
) -> Result<
    (
        Option<PatchStreamPosition>,
        Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
    ),
    SubscriberStreamFailure,
> {
    let latest = latest_available_checkpoint(runtime);
    let mut diagnostics = vec![checkpoint_resolution_artifact(checkpoint)];

    let Some(checkpoint) = checkpoint else {
        return Ok((None, diagnostics));
    };

    if checkpoint.replay_schema_version().0 != 1 {
        let detail = format!(
            "subscriber checkpoint replay schema version {} is incompatible with runtime replay schema version 1",
            checkpoint.replay_schema_version().0
        );
        diagnostics.push(rejection_artifact(
            SubscriberStreamFailureClass::SchemaIncompatible,
            &detail,
        ));
        return Err(SubscriberStreamFailure::new(
            SubscriberStreamFailureClass::SchemaIncompatible,
            detail,
            latest,
            diagnostics,
        ));
    }

    if let Some(commit_id) = runtime
        .history
        .patch_stream_index
        .get(&checkpoint.position())
        .copied()
    {
        let history = runtime.history_access();
        let Some(envelope) = history.commit_envelope(commit_id) else {
            let detail = format!(
                "subscriber checkpoint {} has no retained canonical envelope coverage",
                checkpoint.position().0
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::RetainedHistoryUnavailable,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::RetainedHistoryUnavailable,
                detail,
                latest,
                diagnostics,
            ));
        };

        if envelope.schema_version != checkpoint.schema_version() {
            let detail = format!(
                "subscriber checkpoint schema version {} does not match retained canonical schema version {}",
                checkpoint.schema_version().0,
                envelope.schema_version.0
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::SchemaIncompatible,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::SchemaIncompatible,
                detail,
                latest,
                diagnostics,
            ));
        }

        return Ok((Some(checkpoint.position()), diagnostics));
    }

    if durable_checkpoint_available(runtime, checkpoint) {
        return Ok((Some(checkpoint.position()), diagnostics));
    }

    let detail = format!(
        "subscriber checkpoint {} is not present in canonical history or durable recovery coverage",
        checkpoint.position().0
    );
    diagnostics.push(rejection_artifact(
        SubscriberStreamFailureClass::DurableCoverageGap,
        &detail,
    ));
    Err(SubscriberStreamFailure::new(
        SubscriberStreamFailureClass::DurableCoverageGap,
        detail,
        latest,
        diagnostics,
    ))
}

pub(crate) fn durable_checkpoint_available(
    runtime: &RelationalRuntime,
    checkpoint: &SubscriberCheckpoint,
) -> bool {
    durable_envelopes(runtime).iter().any(|envelope| {
        envelope.patch.position == checkpoint.position()
            && envelope.schema_version == checkpoint.schema_version()
    })
}

pub(crate) fn durable_envelopes(
    runtime: &RelationalRuntime,
) -> Vec<crate::replay::data::CanonicalCommitEnvelope> {
    let recovery_plan = runtime.durability_access().recovery_plan();
    let mut envelopes = recovery_plan
        .checkpoint
        .map(|checkpoint| checkpoint.envelopes)
        .unwrap_or_default();
    envelopes.extend(recovery_plan.tail_log);
    envelopes.sort_by_key(|envelope| envelope.patch.position);
    envelopes
}

pub(crate) fn checkpoint_from_patch_position(
    runtime: &RelationalRuntime,
    position: PatchStreamPosition,
) -> Option<SubscriberCheckpoint> {
    let commit_id = *runtime.history.patch_stream_index.get(&position)?;
    let history = runtime.history_access();
    let envelope = history.commit_envelope(commit_id)?;
    Some(SubscriberCheckpoint::new(
        position,
        ReplaySchemaVersion(1),
        envelope.schema_version,
    ))
}

pub(crate) fn checkpoint_for_schema_version(
    position: PatchStreamPosition,
    schema_version: SchemaVersionId,
) -> SubscriberCheckpoint {
    SubscriberCheckpoint::new(position, ReplaySchemaVersion(1), schema_version)
}
