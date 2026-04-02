use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
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
        self.performance_access().count_merge_execution_attempt();
        if let Err(error) = self
            .merge_access()
            .verify_prepared_merge_execution(&prepared)
        {
            emit_merge_execution_failure_artifact(self, &prepared, &error);
            return Err(error);
        }
        let transaction_id = self.services.next_transaction_id();
        let mutation_plan = match self
            .merge_access()
            .derive_merge_commit_mutation_plan(transaction_id, &prepared)
        {
            Ok(plan) => plan,
            Err(error) => {
                let error = crate::merge::data::MergeExecutionError::from(error);
                emit_merge_execution_failure_artifact(self, &prepared, &error);
                return Err(error);
            }
        };
        let options = TransactionOptions {
            target_branch: Some(mutation_plan.target_branch.clone()),
            merge_parent_branches: mutation_plan
                .merge_parent_branches
                .iter()
                .cloned()
                .collect(),
            ..TransactionOptions::default()
        };
        let execution_summary = mutation_plan.merge_execution_summary.clone();
        let structural_summary_public = mutation_plan.structural_summary.clone();
        let mut commit = match AuthoritativeCommitContext::from_merge(options, mutation_plan) {
            Ok(context) => execute_authoritative_commit(self, context),
            Err(error) => Err(error),
        }
        .map_err(|error| {
            let error = crate::merge::data::MergeExecutionError::from(error);
            emit_merge_execution_failure_artifact(self, &prepared, &error);
            error
        })?;
        let execution_artifact = crate::merge::logic::merge_execution_success_artifact(
            &execution_summary,
            &prepared.bound_executable_plan().diagnostics_plan,
            commit.outcome.commit.commit_id,
            self.config.diagnostics.profile.max_entries_per_artifact,
        );
        self.publication_authority()
            .push_diagnostic_artifact(execution_artifact.clone());
        commit.publication.diagnostics.push(execution_artifact);
        commit.execution.complexity_delta.merge_execution_attempts += 1;
        Ok(MergeExecutionOutcome {
            commit,
            execution_summary,
            structural_summary: structural_summary_public,
        })
    }
}

pub(crate) fn emit_merge_execution_failure_artifact(
    runtime: &mut RelationalRuntime,
    prepared: &crate::merge::data::PreparedMergeExecution,
    error: &crate::merge::data::MergeExecutionError,
) {
    runtime.publication_authority().push_diagnostic_artifact(
        crate::merge::logic::merge_execution_failure_artifact(prepared, error),
    );
}
