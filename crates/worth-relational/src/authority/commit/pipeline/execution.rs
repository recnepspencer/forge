use super::artifact_execution::assemble_commit_artifacts;
use super::authority_context::AuthoritativeCommitContext;
use super::boundary_validation::validate_commit_boundary;
use super::draft_execution::prepare_commit_execution;
use super::execution_admission::admit_commit_execution;
use super::history_binding::bind_commit_history;
use super::mutation_execution::mutate_commit_execution;
use super::publication_execution::prepare_commit_publication_execution;
use super::snapshot_validation::validate_snapshot_publication;
use crate::transactions::data::{CommitResult, TransactionCommitError};

pub(crate) fn prepare_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<crate::mvcc::PreparedRelationalCommitCandidate, TransactionCommitError> {
    let runtime_instance_id = runtime.runtime_instance_id();
    let transaction_id = context.transaction_id;
    let branch_id = context.validation_input.target_branch().clone();
    let expected_basis = context.validation_input.basis().descriptor().clone();
    let expected_root = std::sync::Arc::clone(&context.validation_input.basis().inner.root);
    require_parent_publication_settled(runtime, &expected_root)?;
    let publication_cell = runtime
        .history
        .branch_cell(&branch_id)
        .expect("admitted commit branch remains registered")
        .publication_cell();
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
        expected_basis,
        expected_root,
        publication_cell,
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
    let outcome = runtime.publication_port().compare_and_publish(candidate);
    let performed = match outcome {
        worth_proof::TransitionOutcome::Success(performed) => performed,
        worth_proof::TransitionOutcome::Stale(stale) => {
            return Err(TransactionCommitError::conflict(
                crate::transactions::data::CommitConflict::new(
                    crate::transactions::data::ConflictClass::StaleValidationBasis {
                        detail: format!("branch moved before publication: {stale:?}"),
                    },
                ),
            ));
        }
        worth_proof::TransitionOutcome::Denied(denial) => {
            return Err(publication_outcome_error(format!(
                "branch publication denied: {denial:?}"
            )));
        }
        worth_proof::TransitionOutcome::Deferred(deferred) => {
            return Err(publication_outcome_error(format!(
                "branch publication deferred: {deferred:?}"
            )));
        }
        worth_proof::TransitionOutcome::Failed(failure) => {
            return Err(publication_outcome_error(format!(
                "branch publication failed before movement: {}",
                failure.detail()
            )));
        }
        worth_proof::TransitionOutcome::RebindRequired(impossible) => match impossible {},
    };
    runtime.settle_performed_publication(performed)
}

fn require_parent_publication_settled(
    runtime: &crate::runtime::RelationalRuntime,
    expected_root: &std::sync::Arc<crate::branch::RelationalBranchRoot>,
) -> Result<(), TransactionCommitError> {
    let Some(parent) = expected_root.canonical_envelope() else {
        return Ok(());
    };
    let commit_id = parent.commit.commit_id;
    if !runtime.history.publication_requires_settlement(commit_id) {
        return Ok(());
    }
    Err(publication_outcome_error(format!(
        "parent commit {} requires explicit owner settlement",
        commit_id.0
    )))
}

fn publication_outcome_error(detail: String) -> TransactionCommitError {
    TransactionCommitError::publication(crate::publication::data::PublicationError::new(
        crate::publication::bundle::PublicationStage::Visibility,
        detail,
    ))
}

pub(crate) fn execute_authoritative_commit(
    runtime: &mut crate::runtime::RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<CommitResult, TransactionCommitError> {
    let candidate = prepare_authoritative_commit(runtime, context)?;
    publish_prepared_authoritative_commit(runtime, candidate)
}
