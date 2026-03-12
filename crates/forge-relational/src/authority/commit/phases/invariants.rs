use serde_json::json;

use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsScope,
};
use crate::logic::runtime::{InvariantExecutionPoint, PartitionAccess, RelationalRuntime};
use crate::publication::data::{PublicationError, PublicationStage};
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionCommitError};
use crate::validation::logic::{
    first_blocking_invariant_error, first_publication_invariant_error,
};

pub(crate) fn run_commit_boundary_invariants(
    runtime: &mut RelationalRuntime,
    merged_plan: &MergedCommitPlan,
) -> Result<(), TransactionCommitError> {
    let results = {
        let committed_state = runtime.current_state();
        runtime.run_invariants_for_state(
            &committed_state,
            runtime.current_version_id(),
            InvariantExecutionPoint::CommitBoundary,
            false,
            Some(merged_plan),
        )
    };
    if let Some(error) = first_blocking_invariant_error(&results) {
        runtime
            .diagnostic(DiagnosticsScope::Invariant)
            .failure()
            .emit_entry(
                DiagnosticCode::InvariantViolation,
                error.detail.clone(),
                json!({ "execution_point": "commit_boundary" }),
            );
        return Err(TransactionCommitError::Conflict(error));
    }
    Ok(())
}

pub(crate) fn run_mutation_sensitive_invariants(
    runtime: &RelationalRuntime,
    overlay_state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    merged_plan: &MergedCommitPlan,
) -> Result<(), CommitConflict> {
    let results = runtime.run_invariants_for_state(
        overlay_state,
        version_id,
        InvariantExecutionPoint::MutationSensitive,
        false,
        Some(merged_plan),
    );
    first_blocking_invariant_error(&results).map_or(Ok(()), Err)
}

pub(crate) fn run_snapshot_publication_invariants(
    runtime: &RelationalRuntime,
    overlay_state: &impl PartitionAccess,
    version_id: crate::identity::data::VersionId,
    merged_plan: &MergedCommitPlan,
) -> Result<(), PublicationError> {
    let snapshot_results = runtime.run_invariants_for_state(
        overlay_state,
        version_id,
        InvariantExecutionPoint::SnapshotPublication,
        false,
        Some(merged_plan),
    );
    if let Some(error) = first_publication_invariant_error(&snapshot_results) {
        return Err(PublicationError::new(
            PublicationStage::InvariantCheck,
            error.detail,
        ));
    }
    Ok(())
}
