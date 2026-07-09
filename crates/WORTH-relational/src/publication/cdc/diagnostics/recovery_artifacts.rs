use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticFields, RelationalDiagnosticValue,
    RelationalDiagnosticsEntry,
};
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberContinuationAssessment, SubscriberRecoveryDecision,
    SubscriberRecoveryDisposition, SubscriberRecoverySource, SubscriberResumeRequest,
};
use crate::publication::patch::data::PatchStreamPosition;
use crate::replay::data::ReplaySchemaVersion;
use crate::schema::data::SchemaVersionId;

use super::continuation_summary_artifact;

pub(crate) fn checkpoint_resolution_artifact(
    checkpoint: Option<&SubscriberCheckpoint>,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact::new(
        DiagnosticsScope::Replay,
        DiagnosticsArtifactKind::MinimalSummary,
        DeterminismExpectation::Required,
        vec![RelationalDiagnosticsEntry::new(
            DiagnosticCode::RecoveryCheckpointSelected,
            "subscriber checkpoint resolution completed",
            checkpoint_resolution_fields(checkpoint),
        )],
    )
}

pub(crate) fn recovery_decision_artifact(
    decision: &SubscriberRecoveryDecision,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact::new(
        DiagnosticsScope::Replay,
        DiagnosticsArtifactKind::MinimalSummary,
        DeterminismExpectation::Required,
        vec![RelationalDiagnosticsEntry::new(
            DiagnosticCode::RecoveryRangeReplayed,
            "subscriber recovery plan lowered to execution",
            recovery_decision_fields(decision),
        )],
    )
}

pub(crate) fn continuation_assessment_artifact(
    request: &SubscriberResumeRequest,
    assessment: &SubscriberContinuationAssessment,
) -> RelationalDiagnosticArtifact {
    continuation_summary_artifact(assessment, &request.subscriber_contract().contract_id)
}

fn checkpoint_resolution_fields(
    checkpoint: Option<&SubscriberCheckpoint>,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "checkpoint_position",
            optional_patch_stream_position(checkpoint.map(SubscriberCheckpoint::position)),
        ),
        (
            "schema_version",
            optional_schema_version(checkpoint.map(SubscriberCheckpoint::schema_version)),
        ),
        (
            "replay_schema_version",
            optional_replay_schema_version(
                checkpoint
                    .map(SubscriberCheckpoint::replay_schema_version)
                    .cloned(),
            ),
        ),
    ])
    .into()
}

fn recovery_decision_fields(decision: &SubscriberRecoveryDecision) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "disposition",
            recovery_disposition_value(decision.disposition),
        ),
        ("source", recovery_source_value(decision.source)),
        (
            "start_after_position",
            optional_patch_stream_position(decision.start_after_position),
        ),
    ])
    .into()
}

fn optional_patch_stream_position(
    position: Option<PatchStreamPosition>,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(
        position.map(RelationalDiagnosticValue::PatchStreamPosition),
    )
}

fn optional_schema_version(schema_version: Option<SchemaVersionId>) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(
        schema_version.map(RelationalDiagnosticValue::SchemaVersionId),
    )
}

fn optional_replay_schema_version(
    replay_schema_version: Option<ReplaySchemaVersion>,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::optional(
        replay_schema_version.map(RelationalDiagnosticValue::ReplaySchemaVersion),
    )
}

fn recovery_disposition_value(
    disposition: SubscriberRecoveryDisposition,
) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{disposition:?}"))
}

fn recovery_source_value(source: SubscriberRecoverySource) -> RelationalDiagnosticValue {
    RelationalDiagnosticValue::string(format!("{source:?}"))
}
