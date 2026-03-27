use crate::authority::commit::phases::prepare::prepare_authoritative_working_state_scope;
use crate::authority::commit::pipeline::{execute_authoritative_commit, CommitAuthorityInput};
use crate::logic::runtime::RelationalRuntime;
use crate::merge::logic::MergeAccess;
use crate::transactions::data::MergeExecutionOutcome;
use crate::transactions::data::TransactionOptions;

impl RelationalRuntime {
    pub fn merge_access(&self) -> MergeAccess<'_> {
        MergeAccess::new(self)
    }

    pub fn prepare_merge_execution(
        &self,
        request: crate::merge::data::MergeExecutionRequest,
    ) -> Result<
        crate::merge::data::PreparedMergeExecution,
        crate::merge::data::MergeExecutionPreparationError,
    > {
        self.merge_access().prepare_merge_execution(request)
    }

    pub fn execute_prepared_merge(
        &mut self,
        prepared: crate::merge::data::PreparedMergeExecution,
    ) -> Result<MergeExecutionOutcome, crate::merge::data::MergeExecutionError> {
        self.merge_access().verify_prepared_merge_execution(&prepared)?;
        let transaction_id = self.services.next_transaction_id();
        let mutation_plan = self
            .merge_access()
            .derive_merge_commit_mutation_plan(transaction_id, &prepared)?;
        let options = TransactionOptions {
            target_branch: Some(mutation_plan.target_branch.clone()),
            merge_parent_branches: vec![mutation_plan.source_branch.clone()],
            ..TransactionOptions::default()
        };
        let prepared_scope = prepare_authoritative_working_state_scope(
            self,
            mutation_plan.merged_plan.clone(),
            options.merge_parent_branches.len(),
        );
        let execution_summary = mutation_plan.merge_execution_summary.clone();
        let commit = execute_authoritative_commit(
            self,
            transaction_id,
            options,
            CommitAuthorityInput::Merge(mutation_plan),
            prepared_scope.structural_summary,
            prepared_scope.working_state,
        )?;
        Ok(MergeExecutionOutcome {
            commit,
            execution_summary,
        })
    }
}
