pub(crate) mod counters;
pub(crate) mod failures;
pub(crate) mod observations;

#[cfg(test)]
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticFields,
    RelationalDiagnosticValue, RelationalDiagnosticsEntry,
};
#[cfg(test)]
use crate::logic::runtime::RelationalRuntime;

#[cfg(test)]
pub(crate) fn emit_preparation_failure(
    runtime: &mut RelationalRuntime,
    scope: DiagnosticsScope,
    failure_class: failures::PreparationFailureClass,
    commit_id: crate::history::data::CommitId,
    patch_record_count: usize,
) {
    runtime.publication_authority().push_bounded_diagnostic(
        scope,
        DiagnosticsArtifactKind::DetailedTrace,
        vec![RelationalDiagnosticsEntry::new(
            DiagnosticCode::PreparationFailure,
            "preparation contract reported a structured failure",
            preparation_failure_fields(failure_class, commit_id, patch_record_count),
        )],
    );
}

#[cfg(test)]
fn preparation_failure_fields(
    failure_class: failures::PreparationFailureClass,
    commit_id: crate::history::data::CommitId,
    patch_record_count: usize,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "failure_class",
            RelationalDiagnosticValue::string(failure_class.diagnostic_label()),
        ),
        (
            "commit_id",
            RelationalDiagnosticValue::Unsigned(commit_id.0 as u64),
        ),
        (
            "patch_record_count",
            RelationalDiagnosticValue::unsigned(patch_record_count),
        ),
    ])
    .into()
}
