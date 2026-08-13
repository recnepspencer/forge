use super::artifact_execution::assemble_commit_artifacts;
use super::authority_context::AuthoritativeCommitContext;
use super::boundary_validation::validate_commit_boundary;
use super::draft_execution::prepare_commit_execution;
use super::execution_admission::admit_commit_execution;
use super::history_binding::bind_commit_history;
use super::mutation_execution::mutate_commit_execution;
use super::publication_execution::{append_commit_durably, publish_commit_execution};
use super::result_assembly::assemble_commit_result;
use super::snapshot_validation::validate_snapshot_publication;
use crate::transactions::data::{CommitResult, TransactionCommitError};

pub(crate) fn execute_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<CommitResult, TransactionCommitError> {
    let admitted = admit_commit_execution(runtime, context)?;
    let prepared = prepare_commit_execution(runtime, admitted);
    let boundary_validated = validate_commit_boundary(runtime, prepared)?;
    let mutated = mutate_commit_execution(runtime, boundary_validated)?;
    let history_bound = bind_commit_history(runtime, mutated)?;
    let snapshot_validated = validate_snapshot_publication(runtime, history_bound)?;
    let assembled = assemble_commit_artifacts(runtime, snapshot_validated)?;
    let durable = append_commit_durably(runtime, assembled)?;
    let published = publish_commit_execution(runtime, durable);
    Ok(assemble_commit_result(runtime, published))
}
