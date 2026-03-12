use crate::logic::runtime::{RelationalRuntime, WorkingState};
use crate::publication::data::PublicationError;
use crate::transactions::data::{CommitConflict, MergedCommitPlan, TransactionCommitError};

pub(crate) fn run_commit_boundary_invariants(
    runtime: &mut RelationalRuntime,
    merged_plan: &MergedCommitPlan,
) -> Result<(), TransactionCommitError> {
    runtime.invariant_authority().enforce_commit_boundary(merged_plan)
}

pub(crate) fn run_mutation_sensitive_invariants(
    runtime: &mut RelationalRuntime,
    working_state: &WorkingState,
    version_id: crate::identity::data::VersionId,
    merged_plan: &MergedCommitPlan,
) -> Result<(), CommitConflict> {
    runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(working_state, version_id, merged_plan)
}

pub(crate) fn run_snapshot_publication_invariants(
    runtime: &mut RelationalRuntime,
    working_state: &WorkingState,
    version_id: crate::identity::data::VersionId,
    merged_plan: &MergedCommitPlan,
) -> Result<(), PublicationError> {
    runtime
        .invariant_authority()
        .enforce_snapshot_publication_for_working_state(working_state, version_id, merged_plan)
}
