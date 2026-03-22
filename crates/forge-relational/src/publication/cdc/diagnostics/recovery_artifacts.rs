use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::publication::cdc::data::{
    SubscriberCheckpoint, SubscriberContinuationAssessment, SubscriberRecoveryDecision,
    SubscriberResumeRequest,
};
use serde_json::json;

pub(crate) fn checkpoint_resolution_artifact(
    checkpoint: Option<&SubscriberCheckpoint>,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Replay,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: DeterminismExpectation::Required,
        entries: vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::RecoveryCheckpointSelected,
            message: "subscriber checkpoint resolution completed".to_string(),
            fields: json!({
                "checkpoint_position": checkpoint.map(|candidate| candidate.position().0),
                "schema_version": checkpoint.map(|candidate| candidate.schema_version().0),
                "replay_schema_version": checkpoint.map(|candidate| candidate.replay_schema_version().0),
            }),
        }],
    }
}

pub(crate) fn recovery_decision_artifact(
    decision: &SubscriberRecoveryDecision,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Replay,
        kind: DiagnosticsArtifactKind::MinimalSummary,
        determinism: DeterminismExpectation::Required,
        entries: vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::RecoveryRangeReplayed,
            message: "subscriber recovery plan lowered to execution".to_string(),
            fields: json!({
                "disposition": format!("{:?}", decision.disposition),
                "source": format!("{:?}", decision.source),
                "start_after_position": decision.start_after_position.map(|position| position.0),
            }),
        }],
    }
}

pub(crate) fn continuation_assessment_artifact(
    request: &SubscriberResumeRequest,
    assessment: &SubscriberContinuationAssessment,
) -> RelationalDiagnosticArtifact {
    assessment.to_summary_artifact(&request.subscriber_contract().contract_id)
}
