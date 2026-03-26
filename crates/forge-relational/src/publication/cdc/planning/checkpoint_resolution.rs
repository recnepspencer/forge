use crate::capabilities::RuntimeConfigSource;
use crate::logic::runtime::RelationalRuntime;
use crate::publication::cdc::data::SubscriberStreamFailure;
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberCheckpointBasis, SubscriberContinuationAssessment,
    SubscriberStreamFailureClass,
};
use crate::publication::cdc::diagnostics::{checkpoint_resolution_artifact, rejection_artifact};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
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
            runtime
                .runtime_config()
                .schema
                .descriptor_semantics_policy
                .current_write_version(),
        ),
        runtime
            .runtime_config()
            .schema
            .descriptor_semantics_policy
            .current_write_version(),
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
    durable_envelopes: Option<&[crate::replay::data::CanonicalCommitEnvelope]>,
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

        validate_checkpoint_against_envelope(
            checkpoint,
            envelope,
            "retained canonical",
            latest.clone(),
            &mut diagnostics,
        )?;

        return Ok((Some(checkpoint.position()), diagnostics));
    }

    if let Some(envelope) = durable_checkpoint_envelope(runtime, checkpoint, durable_envelopes) {
        validate_checkpoint_against_envelope(
            checkpoint,
            &envelope,
            "durable canonical",
            latest.clone(),
            &mut diagnostics,
        )?;
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

pub(crate) fn durable_checkpoint_envelope(
    runtime: &RelationalRuntime,
    checkpoint: &SubscriberCheckpoint,
    preloaded_envelopes: Option<&[crate::replay::data::CanonicalCommitEnvelope]>,
) -> Option<crate::replay::data::CanonicalCommitEnvelope> {
    preloaded_envelopes
        .map(|envelopes| envelopes.to_vec())
        .unwrap_or_else(|| durable_envelopes(runtime))
        .into_iter()
        .find(|envelope| envelope.patch.position == checkpoint.position())
}

pub(crate) fn durable_envelopes(
    runtime: &RelationalRuntime,
) -> Vec<crate::replay::data::CanonicalCommitEnvelope> {
    let recovery_plan = runtime.durability_access().recovery_plan(
        crate::durability::data::RecoveryVerificationMode::NormalRecoveryVerification,
    );
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
            crate::schema::data::runtime_descriptor_semantics_policy().current_write_version(),
        ),
        crate::schema::data::runtime_descriptor_semantics_policy().current_write_version(),
    )
}

fn validate_checkpoint_against_envelope(
    checkpoint: &SubscriberCheckpoint,
    envelope: &crate::replay::data::CanonicalCommitEnvelope,
    authority_label: &str,
    latest: Option<SubscriberCheckpoint>,
    diagnostics: &mut Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
) -> Result<(), SubscriberStreamFailure> {
    if envelope.schema_version != checkpoint.schema_version() {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::SchemaIncompatible,
            format!(
                "subscriber checkpoint schema version {} does not match {authority_label} schema version {}",
                checkpoint.schema_version().0,
                envelope.schema_version.0
            ),
            latest,
            diagnostics,
        );
    }

    if envelope.descriptor_semantics_version != checkpoint.descriptor_semantics_version() {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::DescriptorVersionMismatch,
            format!(
                "subscriber checkpoint descriptor semantics version {} does not match {authority_label} descriptor semantics version {}",
                checkpoint.descriptor_semantics_version().0,
                envelope.descriptor_semantics_version.0
            ),
            latest,
            diagnostics,
        );
    }

    if checkpoint
        .normalized_continuation_proof()
        .descriptor_semantics_version()
        != checkpoint.descriptor_semantics_version()
    {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::DescriptorVersionMismatch,
            format!(
                "subscriber checkpoint normalized proof descriptor semantics version {} does not match checkpoint descriptor semantics version {}",
                checkpoint
                    .normalized_continuation_proof()
                    .descriptor_semantics_version()
                    .0,
                checkpoint.descriptor_semantics_version().0
            ),
            latest,
            diagnostics,
        );
    }

    if checkpoint.continuation_summary().contract_id != checkpoint.subscriber_contract_id() {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
            format!(
                "subscriber checkpoint continuation summary contract {} does not match checkpoint contract {}",
                checkpoint.continuation_summary().contract_id,
                checkpoint.subscriber_contract_id()
            ),
            latest,
            diagnostics,
        );
    }

    if checkpoint
        .continuation_summary()
        .descriptor_semantics_version
        != checkpoint.descriptor_semantics_version()
    {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
            format!(
                "subscriber checkpoint continuation summary descriptor semantics version {} does not match checkpoint descriptor semantics version {}",
                checkpoint.continuation_summary().descriptor_semantics_version.0,
                checkpoint.descriptor_semantics_version().0
            ),
            latest,
            diagnostics,
        );
    }

    if checkpoint.continuation_summary().normalized_boundary_count
        != checkpoint
            .normalized_continuation_proof()
            .normalized_boundary_count()
    {
        return checkpoint_validation_error(
            SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
            format!(
                "subscriber checkpoint continuation summary normalized boundary count {} does not match checkpoint proof normalized boundary count {}",
                checkpoint.continuation_summary().normalized_boundary_count,
                checkpoint
                    .normalized_continuation_proof()
                    .normalized_boundary_count()
            ),
            latest,
            diagnostics,
        );
    }

    match &envelope.schema_continuation_descriptor {
        Some(descriptor) => {
            if checkpoint.authoritative_boundary_fingerprint()
                != Some(descriptor.boundary_fingerprint)
            {
                return checkpoint_validation_error(
                    SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                    format!(
                        "subscriber checkpoint authoritative boundary fingerprint {:?} does not match {authority_label} continuation boundary {:?}",
                        checkpoint.authoritative_boundary_fingerprint(),
                        descriptor.boundary_fingerprint
                    ),
                    latest,
                    diagnostics,
                );
            }
            if checkpoint.authoritative_descriptor_continuation()
                != Some(descriptor.bridge.continuation)
            {
                return checkpoint_validation_error(
                    SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                    format!(
                        "subscriber checkpoint authoritative descriptor continuation {:?} does not match {authority_label} continuation {:?}",
                        checkpoint.authoritative_descriptor_continuation(),
                        descriptor.bridge.continuation
                    ),
                    latest,
                    diagnostics,
                );
            }
            if checkpoint.authoritative_contract_consumes_boundary()
                && !checkpoint
                    .normalized_continuation_proof()
                    .boundary_fingerprints()
                    .contains(&descriptor.boundary_fingerprint)
            {
                return checkpoint_validation_error(
                    SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                    format!(
                        "subscriber checkpoint normalized continuation proof does not contain authoritative boundary {:?}",
                        descriptor.boundary_fingerprint
                    ),
                    latest,
                    diagnostics,
                );
            }
            if checkpoint
                .authoritative_subscriber_outcome()
                .is_some_and(|outcome| {
                    continuation_priority(checkpoint.continuation_summary().continuation_outcome)
                        < continuation_priority(outcome)
                })
            {
                return checkpoint_validation_error(
                    SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                    format!(
                        "subscriber checkpoint continuation outcome {:?} is weaker than authoritative boundary outcome {:?}",
                        checkpoint.continuation_summary().continuation_outcome,
                        checkpoint.authoritative_subscriber_outcome()
                    ),
                    latest,
                    diagnostics,
                );
            }
        }
        None => {
            if checkpoint.authoritative_boundary_fingerprint().is_some()
                || checkpoint.authoritative_descriptor_continuation().is_some()
                || checkpoint.authoritative_subscriber_outcome().is_some()
                || checkpoint.authoritative_contract_consumes_boundary()
            {
                return checkpoint_validation_error(
                    SubscriberStreamFailureClass::CheckpointContinuitySummaryMismatch,
                    format!(
                        "subscriber checkpoint carries authoritative boundary binding for position {} but {authority_label} canonical envelope has no continuation descriptor",
                        checkpoint.position().0
                    ),
                    latest,
                    diagnostics,
                );
            }
        }
    }

    Ok(())
}

fn checkpoint_validation_error(
    class: SubscriberStreamFailureClass,
    detail: String,
    latest: Option<SubscriberCheckpoint>,
    diagnostics: &mut Vec<crate::diagnostics::data::RelationalDiagnosticArtifact>,
) -> Result<(), SubscriberStreamFailure> {
    diagnostics.push(rejection_artifact(class, &detail));
    Err(SubscriberStreamFailure::new(
        class,
        detail,
        latest,
        diagnostics.clone(),
    ))
}

fn continuation_priority(
    classification: crate::schema::data::SchemaContinuationClassification,
) -> u8 {
    match classification {
        crate::schema::data::SchemaContinuationClassification::ContinueUnchanged => 0,
        crate::schema::data::SchemaContinuationClassification::ContinueWithTransparentBridge => 1,
        crate::schema::data::SchemaContinuationClassification::ContinueWithVisibleBridge => 2,
        crate::schema::data::SchemaContinuationClassification::ContinueWithContractUpgrade => 3,
        crate::schema::data::SchemaContinuationClassification::RequireRenegotiation => 4,
        crate::schema::data::SchemaContinuationClassification::Rejected => 5,
    }
}
