use crate::authority::commit::pipeline::{
    execute_authoritative_commit, AuthoritativeCommitContext,
};
use crate::logic::runtime::RelationalRuntime;
use crate::merge::logic::MergeAccess;
use crate::transactions::data::MergeExecutionOutcome;
use crate::transactions::data::TransactionOptions;

impl RelationalRuntime {
    pub(crate) fn merge_access(&self) -> MergeAccess<'_> {
        MergeAccess::new(self)
    }

    pub fn prepare_merge_execution(
        &self,
        request: crate::merge::data::MergeExecutionRequest,
    ) -> Result<
        crate::merge::data::PreparedMergeExecution,
        crate::merge::data::MergeExecutionPreparationError,
    > {
        self.merge().prepare_merge_execution(request)
    }

    pub fn execute_prepared_merge(
        &mut self,
        prepared: crate::merge::data::PreparedMergeExecution,
    ) -> Result<MergeExecutionOutcome, crate::merge::data::MergeExecutionError> {
        let complexity_baseline = current_complexity_counters(self);
        self.performance_access().count_merge_execution_attempt();
        if let Err(error) = self.merge().verify_prepared_merge_execution(&prepared) {
            emit_merge_execution_failure_artifact(self, &prepared, &error);
            return Err(error);
        }
        let transaction_id = self.services.next_transaction_id();
        let mutation_plan = match self
            .merge()
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
        let diagnostics_plan = prepared.bound_executable_plan().diagnostics_plan.clone();
        let commit = match AuthoritativeCommitContext::from_prepared_merge(
            options,
            mutation_plan,
            diagnostics_plan,
            complexity_baseline,
        ) {
            Ok(context) => execute_authoritative_commit(self, context),
            Err(error) => Err(error),
        }
        .map_err(|error| {
            let error = crate::merge::data::MergeExecutionError::from(error);
            emit_merge_execution_failure_artifact(self, &prepared, &error);
            error
        })?;
        Ok(MergeExecutionOutcome {
            commit,
            execution_summary,
            structural_summary: structural_summary_public,
        })
    }
}

fn current_complexity_counters(
    runtime: &RelationalRuntime,
) -> crate::performance::data::RuntimeComplexityCounters {
    runtime
        .services
        .instrumentation
        .complexity_counters
        .lock()
        .expect("complexity counter lock poisoned")
        .clone()
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
