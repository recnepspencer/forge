use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitConflict, ConflictClass, TransactionCommitError};

use super::proposal_invariants::stale_validated_proposal;
use super::validated_proposal::ValidatedRelationalProposal;

impl RelationalRuntime {
    pub(crate) fn revalidate_proposal_for_publication(
        &mut self,
        candidate: ValidatedRelationalProposal,
    ) -> Result<ValidatedRelationalProposal, TransactionCommitError> {
        self.ensure_validated_proposal_branch_is_current(&candidate)?;
        Ok(candidate)
    }

    fn ensure_validated_proposal_branch_is_current(
        &self,
        candidate: &ValidatedRelationalProposal,
    ) -> Result<(), TransactionCommitError> {
        let binding = candidate.validation_input.basis();
        if binding.identity().runtime_instance_id() != self.runtime_instance_id() {
            return Err(TransactionCommitError::conflict(CommitConflict::new(
                ConflictClass::ForeignRuntime {
                    expected_runtime_instance_id: self.runtime_instance_id(),
                    actual_runtime_instance_id: binding.identity().runtime_instance_id(),
                },
            )));
        }
        if !self.admitted_branch_basis_is_current(binding) {
            return Err(stale_validated_proposal(
                "validated mutation branch binding is no longer current",
            ));
        }
        let Some(cell) = self
            .history
            .branch_cell(candidate.validation_input.target_branch())
        else {
            return Err(stale_validated_proposal(
                "validated mutation branch is no longer registered",
            ));
        };
        if cell.identity() != binding.identity()
            || cell.observation() != *binding.reference()
            || cell.truth_version() != candidate.validated_against_branch_version
        {
            return Err(stale_validated_proposal(
                "validated mutation no longer matches the current branch reference",
            ));
        }
        Ok(())
    }
}
