use super::artifact_execution::assemble_commit_artifacts;
use super::authority_context::AuthoritativeCommitContext;
use super::boundary_validation::validate_commit_boundary;
use super::draft_execution::prepare_commit_execution;
use super::execution_admission::admit_commit_execution;
use super::history_binding::bind_commit_history;
use super::mutation_execution::mutate_commit_execution;
use super::publication_execution::{
    prepare_commit_publication_execution, publish_commit_execution,
};
use super::result_assembly::assemble_commit_result;
use super::snapshot_validation::validate_snapshot_publication;
use crate::transactions::data::{CommitResult, TransactionCommitError};

pub(crate) fn prepare_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<crate::mvcc::PreparedRelationalCommitCandidate, TransactionCommitError> {
    let runtime_instance_id = runtime.runtime_instance_id();
    let transaction_id = context.transaction_id;
    let branch_id = context.validation_input.target_branch().clone();
    let admitted = admit_commit_execution(runtime, context)?;
    let prepared = prepare_commit_execution(runtime, admitted)?;
    let boundary_validated = validate_commit_boundary(runtime, prepared)?;
    let mutated = mutate_commit_execution(runtime, boundary_validated)?;
    let history_bound = bind_commit_history(runtime, mutated)?;
    let snapshot_validated = validate_snapshot_publication(runtime, history_bound)?;
    let assembled = assemble_commit_artifacts(runtime, snapshot_validated)?;
    let prepared = prepare_commit_publication_execution(runtime, assembled)?;
    let candidate = crate::mvcc::PreparedRelationalCommitCandidate::new(
        runtime_instance_id,
        transaction_id,
        branch_id,
        prepared,
    );
    runtime
        .history
        .record_candidate_preparation(candidate.branch());
    Ok(candidate)
}

pub(crate) fn publish_prepared_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    candidate: crate::mvcc::PreparedRelationalCommitCandidate,
) -> Result<CommitResult, TransactionCommitError> {
    runtime
        .history
        .record_publication_attempt(candidate.branch());
    let published = publish_commit_execution(runtime, candidate.execution)?;
    Ok(assemble_commit_result(runtime, published))
}

pub(crate) fn execute_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<CommitResult, TransactionCommitError> {
    let candidate = prepare_authoritative_commit(runtime, context)?;
    publish_prepared_authoritative_commit(runtime, candidate)
}
