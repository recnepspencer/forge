use serde_json::json;

use crate::diagnostics::data::{DiagnosticCode, DiagnosticsScope};
use crate::logic::runtime::RelationalRuntime;
use crate::replay::data::{RelationalReplayOutcome, RelationalReplayRequest, ReplayFailureClass};

pub(super) fn record_replay_diagnostic(
    runtime: &mut RelationalRuntime,
    request: &RelationalReplayRequest,
    outcome: &RelationalReplayOutcome,
) {
    let mismatch_classes = outcome
        .mismatches
        .iter()
        .map(|mismatch| format!("{:?}", mismatch.class))
        .collect::<Vec<_>>();
    let mismatch_surfaces = outcome
        .mismatches
        .iter()
        .map(|mismatch| format!("{:?}", mismatch.surface))
        .collect::<Vec<_>>();
    let mismatch_verification_layers = outcome
        .mismatches
        .iter()
        .map(|mismatch| format!("{:?}", mismatch.verification_layer))
        .collect::<Vec<_>>();
    let compared_surfaces = outcome
        .compared_surfaces
        .iter()
        .map(|surface| format!("{surface:?}"))
        .collect::<Vec<_>>();
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
            "verification_mode": format!("{:?}", request.verification_mode),
            "compared_surfaces": compared_surfaces,
            "mismatch_count": outcome.mismatches.len(),
            "mismatch_classes": mismatch_classes,
            "mismatch_surfaces": mismatch_surfaces,
            "mismatch_verification_layers": mismatch_verification_layers,
            "failure": outcome.failure.as_ref().map(|value| format!("{value:?}")),
        }),
    );
}
