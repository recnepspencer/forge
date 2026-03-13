pub(crate) mod counters;
pub(crate) mod failures;
pub(crate) mod observations;

use serde_json::Value;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::logic::runtime::RelationalRuntime;

pub(crate) fn emit_preparation_failure(
    runtime: &mut RelationalRuntime,
    scope: DiagnosticsScope,
    _failure_class: failures::PreparationFailureClass,
    fields: Value,
) {
    runtime.publication_authority().push_bounded_diagnostic(
        scope,
        DiagnosticsArtifactKind::DetailedTrace,
        vec![RelationalDiagnosticsEntry {
            code: DiagnosticCode::PreparationFailure,
            message: "preparation contract reported a structured failure".to_string(),
            fields,
        }],
    );
}
