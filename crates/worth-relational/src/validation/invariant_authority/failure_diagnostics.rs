use crate::diagnostics::data::{DiagnosticsArtifactKind, DiagnosticsScope};
use crate::publication::publication_failure_diagnostic;
use crate::runtime::RelationalRuntime;
use crate::validation::engine::{InvariantExecutionResult, InvariantFailure};

use super::diagnostic_projection::failure_diagnostic_fields;

pub(super) fn emit_conflict_diagnostic(
    runtime: &mut RelationalRuntime,
    result: &InvariantExecutionResult,
    failure: &InvariantFailure,
) {
    runtime
        .publication_authority()
        .diagnostic(DiagnosticsScope::Invariant)
        .failure()
        .emit_entry(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(failure)),
        );
}

pub(super) fn emit_collect_all_failure_diagnostics(
    runtime: &mut RelationalRuntime,
    result: &InvariantExecutionResult,
) -> bool {
    if !runtime
        .config
        .diagnostics
        .profile
        .collect_all_invariant_failures
    {
        return false;
    }

    let mut entries = Vec::new();
    for failure in result.blocking_failures() {
        entries.push(publication_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(&failure)),
        ));
    }
    for failure in result.publication_failures() {
        entries.push(publication_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(&failure)),
        ));
    }
    if entries.is_empty() {
        return false;
    }
    runtime.publication_authority().push_bounded_diagnostic(
        DiagnosticsScope::Invariant,
        DiagnosticsArtifactKind::Failure,
        entries,
    );
    true
}

pub(super) fn emit_publication_failure(
    runtime: &mut RelationalRuntime,
    result: &InvariantExecutionResult,
    failure: &InvariantFailure,
) {
    runtime.publication_authority().push_bounded_diagnostic(
        DiagnosticsScope::Invariant,
        DiagnosticsArtifactKind::Failure,
        vec![publication_failure_diagnostic(
            failure.code(),
            failure.detail().to_string(),
            failure_diagnostic_fields(&result.failure_artifact(failure)),
        )],
    );
}
