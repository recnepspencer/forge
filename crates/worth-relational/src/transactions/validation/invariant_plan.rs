//! Branch-bound invariant planning owned by a live transaction.

use crate::branch::SelectedRelationalBranchState;
use crate::transactions::data::{CommitConflict, ConflictClass};
use crate::transactions::RelationalTransaction;
use crate::validation::engine::InvariantExecutionResult;

impl RelationalTransaction<'_> {
    /// Evaluate commit-boundary planning against this transaction's exact
    /// owner-issued branch basis.
    pub fn commit_boundary_plan(&mut self) -> Result<InvariantExecutionResult, CommitConflict> {
        let (selected_state, merged_plan) = self.branch_bound_invariant_plan()?;
        Ok(self
            .runtime
            .validation()
            .commit_boundary_for_selected_branch_plan(&selected_state, &merged_plan))
    }

    /// Evaluate graph-composition planning against this transaction's exact
    /// owner-issued branch basis.
    pub fn graph_composition_plan(&mut self) -> Result<InvariantExecutionResult, CommitConflict> {
        let (selected_state, merged_plan) = self.branch_bound_invariant_plan()?;
        Ok(self
            .runtime
            .validation()
            .graph_composition_for_selected_branch_plan(&selected_state, &merged_plan))
    }

    fn branch_bound_invariant_plan(
        &mut self,
    ) -> Result<
        (
            SelectedRelationalBranchState,
            crate::transactions::data::MergedCommitPlan,
        ),
        CommitConflict,
    > {
        let selected_state = self
            .runtime
            .selected_branch_state(self.options.branch_binding())
            .map_err(|error| {
                CommitConflict::new(ConflictClass::StaleValidationBasis {
                    detail: error.detail(),
                })
            })?;
        let merged_plan = self.merged_plan()?.clone();
        Ok((selected_state, merged_plan))
    }
}
