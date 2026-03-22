pub(crate) mod counters;
pub(crate) mod failures;
pub(crate) mod observations;

#[cfg(test)]
use serde_json::Value;

#[cfg(test)]
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
#[cfg(test)]
use crate::logic::runtime::RelationalRuntime;

#[cfg(test)]
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
