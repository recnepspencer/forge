use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticsEntry,
};
use crate::publication::cdc::data::SubscriberStreamFailureClass;
use serde_json::json;

pub(crate) fn rejection_artifact(
    class: SubscriberStreamFailureClass,
    detail: &str,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact {
        scope: DiagnosticsScope::Replay,
        kind: DiagnosticsArtifactKind::Failure,
        determinism: DeterminismExpectation::Required,
        entries: vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::ReplaySchemaVersionMismatch,
            message: "subscriber recovery request rejected".to_string(),
            fields: json!({
                "failure_class": format!("{:?}", class),
                "detail": detail,
            }),
        }],
    }
}
