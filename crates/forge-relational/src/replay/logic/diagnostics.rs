use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{RelationalReplayOutcome, RelationalReplayRequest, ReplayFailureClass};

pub(super) fn record_replay_diagnostic(
    runtime: &mut RelationalRuntime,
    request: &RelationalReplayRequest,
    outcome: &RelationalReplayOutcome,
) {
    let code = match outcome.failure.as_ref() {
        Some(ReplayFailureClass::SchemaMismatch | ReplayFailureClass::UnsupportedReplaySchema) => {
            DiagnosticCode::ReplaySchemaVersionMismatch
        }
        Some(_) => DiagnosticCode::InvariantViolation,
        None => DiagnosticCode::CommitPublished,
    };
    let builder = runtime
        .publication_authority()
        .diagnostic(DiagnosticsScope::Replay);
    let builder = if outcome.failure.is_some() {
        builder.failure()
    } else {
        builder.comparison()
    };
    builder.emit_entry(
        code,
        "replay comparison completed",
        json!({
            "commit_id": request.commit_id.0,
            "branch_id": request.branch_id.0,
            "mismatch_count": outcome.mismatches.len(),
            "failure": outcome.failure.as_ref().map(|value| format!("{value:?}")),
        }),
    );
}
