use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberCheckpointBasis, SubscriberContinuationAssessment,
    SubscriberStreamFailureClass,
};
use crate::publication::cdc::diagnostics::{checkpoint_resolution_artifact, rejection_artifact};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::{
    publication::cdc::data::SubscriberStreamFailure,
};
#[cfg(test)]
use crate::schema::data::SchemaVersionId;

pub(crate) fn latest_available_checkpoint(
    runtime: &RelationalRuntime,
) -> Option<SubscriberCheckpoint> {
    let basis = latest_available_checkpoint_basis(runtime)?;
    Some(SubscriberCheckpoint::from_basis_with_assessment(
        basis,
        "default.subscriber.contract".to_string(),
        &SubscriberContinuationAssessment::unchanged(
            "default.subscriber.contract".to_string(),
            crate::schema::data::DescriptorSemanticsVersion::default(),
        ),
        crate::schema::data::DescriptorSemanticsVersion::default(),
    ))
}

pub(crate) fn latest_available_checkpoint_basis(
    runtime: &RelationalRuntime,
) -> Option<SubscriberCheckpointBasis> {
    let latest_position = runtime.history_access().latest_patch_stream_position()?;
    let commit_id = *runtime.history.patch_stream_index.get(&latest_position)?;
    let history = runtime.history_access();
    let envelope = history.commit_envelope(commit_id)?;
    Some(SubscriberCheckpointBasis::new(
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

        if envelope.descriptor_semantics_version != checkpoint.descriptor_semantics_version() {
            let detail = format!(
                "subscriber checkpoint descriptor semantics version {} does not match retained canonical descriptor semantics version {}",
                checkpoint.descriptor_semantics_version().0,
                envelope.descriptor_semantics_version.0
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::DescriptorVersionMismatch,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::DescriptorVersionMismatch,
                detail,
                latest,
                diagnostics,
            ));
        }

        if checkpoint
            .normalized_continuation_proof()
            .descriptor_semantics_version()
            != checkpoint.descriptor_semantics_version()
        {
            let detail = format!(
                "subscriber checkpoint normalized proof descriptor semantics version {} does not match checkpoint descriptor semantics version {}",
                checkpoint
                    .normalized_continuation_proof()
                    .descriptor_semantics_version()
                    .0,
                checkpoint.descriptor_semantics_version().0
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::DescriptorVersionMismatch,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::DescriptorVersionMismatch,
                detail,
                latest,
                diagnostics,
            ));
        }

        if checkpoint.continuation_summary().contract_id != checkpoint.subscriber_contract_id() {
            let detail = format!(
                "subscriber checkpoint continuation summary contract {} does not match checkpoint contract {}",
                checkpoint.continuation_summary().contract_id,
                checkpoint.subscriber_contract_id()
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                detail,
                latest,
                diagnostics,
            ));
        }

        if checkpoint.continuation_summary().descriptor_semantics_version
            != checkpoint.descriptor_semantics_version()
        {
            let detail = format!(
                "subscriber checkpoint continuation summary descriptor semantics version {} does not match checkpoint descriptor semantics version {}",
                checkpoint.continuation_summary().descriptor_semantics_version.0,
                checkpoint.descriptor_semantics_version().0
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                detail,
                latest,
                diagnostics,
            ));
        }

        if checkpoint.continuation_summary().normalized_boundary_count
            != checkpoint
                .normalized_continuation_proof()
                .normalized_boundary_count()
        {
            let detail = format!(
                "subscriber checkpoint continuation summary normalized boundary count {} does not match checkpoint proof normalized boundary count {}",
                checkpoint.continuation_summary().normalized_boundary_count,
                checkpoint
                    .normalized_continuation_proof()
                    .normalized_boundary_count()
            );
            diagnostics.push(rejection_artifact(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                &detail,
            ));
            return Err(SubscriberStreamFailure::new(
                SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
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

pub(crate) fn checkpoint_basis_from_patch_position(
    runtime: &RelationalRuntime,
    position: PatchStreamPosition,
) -> Option<SubscriberCheckpointBasis> {
    let commit_id = *runtime.history.patch_stream_index.get(&position)?;
    let history = runtime.history_access();
    let envelope = history.commit_envelope(commit_id)?;
    Some(SubscriberCheckpointBasis::new(
        position,
        ReplaySchemaVersion(1),
        envelope.schema_version,
    ))
}

#[cfg(test)]
pub(crate) fn checkpoint_for_schema_version(
    position: PatchStreamPosition,
    schema_version: SchemaVersionId,
) -> SubscriberCheckpoint {
    SubscriberCheckpoint::from_basis_with_assessment(
        SubscriberCheckpointBasis::new(position, ReplaySchemaVersion(1), schema_version),
        "default.subscriber.contract".to_string(),
        &SubscriberContinuationAssessment::unchanged(
            "default.subscriber.contract".to_string(),
            crate::schema::data::DescriptorSemanticsVersion::default(),
        ),
        crate::schema::data::DescriptorSemanticsVersion::default(),
    )
}
