use crate::capabilities::DiagnosticArtifactSink;
use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::publication::invariant_failure_diagnostic;
use crate::validation::engine::{InvariantExecutionResult, InvariantFailure};

use super::diagnostic_projection::failure_diagnostic_fields;

pub(crate) fn emit_conflict_diagnostic(
    runtime: &impl DiagnosticArtifactSink,
    result: &InvariantExecutionResult,
    failure: &InvariantFailure,
) {
    runtime.push_diagnostic_entries(
        DiagnosticsScope::Invariant,
        DiagnosticsArtifactKind::Failure,
        vec![invariant_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(failure)),
        )],
    );
}

pub(crate) fn emit_collect_all_failure_diagnostics(
    runtime: &(impl DiagnosticArtifactSink + crate::capabilities::RuntimeConfigSource),
    result: &InvariantExecutionResult,
) -> bool {
    if !runtime
        .runtime_config()
        .diagnostics
        .profile
        .collect_all_invariant_failures
    {
        return false;
    }

    let mut entries = Vec::new();
    for failure in result.blocking_failures() {
        entries.push(invariant_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(&failure)),
        ));
    }
    for failure in result.publication_failures() {
        entries.push(invariant_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(&failure)),
        ));
    }
    if entries.is_empty() {
        return false;
    }
    runtime.push_diagnostic_entries(
        DiagnosticsScope::Invariant,
        DiagnosticsArtifactKind::Failure,
        entries,
    );
    true
}

pub(crate) fn emit_publication_failure(
    runtime: &impl DiagnosticArtifactSink,
    result: &InvariantExecutionResult,
    failure: &InvariantFailure,
) {
    runtime.push_diagnostic_entries(
        DiagnosticsScope::Invariant,
        DiagnosticsArtifactKind::Failure,
        vec![invariant_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(failure)),
        )],
    );
}
