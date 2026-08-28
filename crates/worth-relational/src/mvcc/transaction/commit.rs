use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitResult, TransactionCommitError};

impl RelationalRuntime {
    pub fn commit_branch_transaction(
        &mut self,
        transaction: crate::mvcc::BranchBoundRelationalTransaction,
    ) -> Result<CommitResult, TransactionCommitError> {
        let candidate = self.prepare_branch_transaction(transaction)?;
        self.publish_prepared_candidate(candidate)
    }

    pub fn prepare_branch_transaction(
        &self,
        transaction: crate::mvcc::BranchBoundRelationalTransaction,
    ) -> Result<crate::mvcc::PreparedRelationalCommitCandidate, TransactionCommitError> {
        self.preparation_port()
            .prepare_branch_transaction(transaction)
    }

    pub fn preparation_port(&self) -> crate::mvcc::RelationalPreparationPort {
        crate::mvcc::RelationalPreparationPort::new(
            crate::runtime::RelationalPreparationOwnerBinding::from_runtime(self),
        )
    }
}
