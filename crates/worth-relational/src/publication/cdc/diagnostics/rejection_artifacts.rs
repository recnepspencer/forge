use crate::diagnostics::data::{
    DeterminismExpectation, DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope,
    RelationalDiagnosticArtifact, RelationalDiagnosticFields, RelationalDiagnosticValue,
    RelationalDiagnosticsEntry,
};
use crate::publication::cdc::data::SubscriberStreamFailureClass;

pub(crate) fn rejection_artifact(
    class: SubscriberStreamFailureClass,
    detail: &str,
) -> RelationalDiagnosticArtifact {
    RelationalDiagnosticArtifact::new(
        DiagnosticsScope::Replay,
        DiagnosticsArtifactKind::Failure,
        DeterminismExpectation::Required,
        vec![RelationalDiagnosticsEntry::new(
            DiagnosticCode::ReplaySchemaVersionMismatch,
            "subscriber recovery request rejected",
            subscriber_recovery_request_rejection_fields(class, detail),
        )],
    )
}

fn subscriber_recovery_request_rejection_fields(
    failure_class: SubscriberStreamFailureClass,
    detail: &str,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "failure_class",
            RelationalDiagnosticValue::string(format!("{failure_class:?}")),
        ),
        ("detail", RelationalDiagnosticValue::string(detail)),
    ])
    .into()
}
