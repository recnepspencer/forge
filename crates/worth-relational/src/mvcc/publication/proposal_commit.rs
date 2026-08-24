use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitResult, TransactionCommitError};

impl RelationalRuntime {
    pub fn commit_validated_proposal(
        &mut self,
        proposal: crate::mvcc::ValidatedRelationalProposal,
    ) -> Result<CommitResult, TransactionCommitError> {
        let proposal = self.revalidate_proposal_for_publication(proposal)?;
        self.history
            .record_publication_attempt(proposal.invariant_evidence().branch());
        crate::authority::commit::pipeline::execute_authoritative_commit(
            self,
            crate::authority::commit::pipeline::AuthoritativeCommitContext::from_validated_proposal(
                proposal,
            ),
        )
    }
}
