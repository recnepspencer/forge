use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitResult, TransactionCommitError};

impl RelationalRuntime {
    pub fn commit_branch_transaction(
        &mut self,
        transaction: crate::mvcc::BranchBoundRelationalTransaction,
    ) -> Result<CommitResult, TransactionCommitError> {
        let proposal = self
            .validate_branch_transaction(transaction)
            .map_err(attach_validation_rejection)?;
        self.commit_validated_proposal(proposal)
    }
}

fn attach_validation_rejection(error: TransactionCommitError) -> TransactionCommitError {
    let mut commit_log = crate::transactions::data::CommitLog::new();
    let phase = crate::transactions::data::CommitPhase::DraftPreparation;
    commit_log.begin_phase(phase);
    match &error {
        TransactionCommitError::Conflict { error, .. } => {
            commit_log.record_rejection(phase, Some(error.code()), None, error.detail());
        }
        TransactionCommitError::Publication { error, .. } => {
            commit_log.record_rejection(phase, None, Some(error.stage), error.detail.clone());
        }
        TransactionCommitError::Preparation { error, .. } => {
            commit_log.record_rejection(phase, Some(error.code()), None, error.detail());
        }
    }
    error.with_commit_log(commit_log)
}
