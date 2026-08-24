use crate::runtime::RelationalRuntime;
use crate::transactions::data::TransactionCommitError;

impl RelationalRuntime {
    pub(crate) fn prepare_validated_proposal(
        &mut self,
        proposal: crate::mvcc::ValidatedRelationalProposal,
    ) -> Result<crate::mvcc::PreparedRelationalCommitCandidate, TransactionCommitError> {
        let proposal = self.revalidate_proposal_for_publication(proposal)?;
        crate::authority::commit::pipeline::prepare_authoritative_commit(
            self,
            crate::authority::commit::pipeline::AuthoritativeCommitContext::from_validated_proposal(
                proposal,
            ),
        )
    }
}
