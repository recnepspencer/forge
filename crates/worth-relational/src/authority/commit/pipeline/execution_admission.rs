use super::authority_context::AuthoritativeCommitContext;
use super::bulk_mutation_telemetry::record_bulk_mutation_telemetry;
use super::rejection::stale_strategy_validation_basis;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPhaseTiming, MergedCommitPlan, TransactionCommitError, TransactionId,
    TransactionOptions,
};

pub(super) struct AdmittedCommitExecution {
    transaction_id: TransactionId,
    options: TransactionOptions,
    phase_timing: CommitPhaseTiming,
    commit_log: CommitLog,
    authority_input: super::authority_context::CommitAuthorityInput,
    prepared_scope: Option<super::authority_context::PreparedAuthorityScope>,
    merge_execution_accounting: Option<super::authority_context::MergeExecutionAccounting>,
    merge_execution_diagnostics_plan: Option<crate::merge::data::MergeExecutionDiagnosticsPlan>,
    prevalidated_commit_boundary: Option<crate::validation::engine::InvariantExecutionResult>,
    strategy_commit_artifacts: Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
    diagnostics_start: usize,
    complexity_before: crate::performance::data::RuntimeComplexityCounters,
}

pub(super) struct AdmittedCommitPhaseView<'a> {
    transaction_id: TransactionId,
    options: &'a TransactionOptions,
    authority_input: &'a super::authority_context::CommitAuthorityInput,
    commit_log: &'a mut CommitLog,
    phase_timing: &'a mut CommitPhaseTiming,
}

impl<'a> AdmittedCommitPhaseView<'a> {
    pub(super) fn into_parts(
        self,
    ) -> (
        TransactionId,
        &'a TransactionOptions,
        &'a MergedCommitPlan,
        Option<&'a crate::transactions::data::MergeCommitMutationPlan>,
        &'a mut CommitLog,
        &'a mut CommitPhaseTiming,
    ) {
        let (merged_plan, merge_history_plan) = match self.authority_input {
            super::authority_context::CommitAuthorityInput::Lowered(plan) => {
                (plan.merged_plan(), None)
            }
            super::authority_context::CommitAuthorityInput::Merge(plan) => {
                (&plan.merged_plan, Some(plan))
            }
        };
        (
            self.transaction_id,
            self.options,
            merged_plan,
            merge_history_plan,
            self.commit_log,
            self.phase_timing,
        )
    }
}

impl AdmittedCommitExecution {
    pub(super) fn merged_plan(&self) -> &MergedCommitPlan {
        match &self.authority_input {
            super::authority_context::CommitAuthorityInput::Lowered(plan) => plan.merged_plan(),
            super::authority_context::CommitAuthorityInput::Merge(plan) => &plan.merged_plan,
        }
    }

    pub(super) fn merge_history_plan(
        &self,
    ) -> Option<&crate::transactions::data::MergeCommitMutationPlan> {
        match &self.authority_input {
            super::authority_context::CommitAuthorityInput::Lowered(_) => None,
            super::authority_context::CommitAuthorityInput::Merge(plan) => Some(plan),
        }
    }

    pub(super) fn options(&self) -> &TransactionOptions {
        &self.options
    }

    pub(super) fn take_prepared_scope(
        &mut self,
    ) -> Option<super::authority_context::PreparedAuthorityScope> {
        self.prepared_scope.take()
    }

    pub(super) fn take_prevalidated_boundary(
        &mut self,
    ) -> Option<crate::validation::engine::InvariantExecutionResult> {
        self.prevalidated_commit_boundary.take()
    }

    pub(super) fn commit_phase_state(&mut self) -> (&mut CommitLog, &mut CommitPhaseTiming) {
        (&mut self.commit_log, &mut self.phase_timing)
    }

    pub(super) fn phase_view(&mut self) -> AdmittedCommitPhaseView<'_> {
        AdmittedCommitPhaseView {
            transaction_id: self.transaction_id,
            options: &self.options,
            authority_input: &self.authority_input,
            commit_log: &mut self.commit_log,
            phase_timing: &mut self.phase_timing,
        }
    }

    pub(super) fn phase_timing_mut(&mut self) -> &mut CommitPhaseTiming {
        &mut self.phase_timing
    }

    pub(super) fn strategy_artifacts(
        &self,
    ) -> Option<&crate::commit_strategies::data::StrategyCommitArtifactBundle> {
        self.strategy_commit_artifacts.as_ref()
    }

    pub(super) fn take_merge_accounting(
        &mut self,
    ) -> Option<super::authority_context::MergeExecutionAccounting> {
        self.merge_execution_accounting.take()
    }

    pub(super) fn merge_execution_diagnostics_plan(
        &self,
    ) -> Option<&crate::merge::data::MergeExecutionDiagnosticsPlan> {
        self.merge_execution_diagnostics_plan.as_ref()
    }

    pub(super) fn into_result_parts(
        self,
    ) -> (
        TransactionId,
        CommitPhaseTiming,
        CommitLog,
        Option<crate::commit_strategies::data::StrategyCommitArtifactBundle>,
        usize,
        crate::performance::data::RuntimeComplexityCounters,
    ) {
        (
            self.transaction_id,
            self.phase_timing,
            self.commit_log,
            self.strategy_commit_artifacts,
            self.diagnostics_start,
            self.complexity_before,
        )
    }
}

pub(super) fn admit_commit_execution(
    runtime: &mut RelationalRuntime,
    context: AuthoritativeCommitContext,
) -> Result<AdmittedCommitExecution, TransactionCommitError> {
    let diagnostics_start = runtime.publication().diagnostic_access().artifact_count();
    let complexity_before = context.complexity_baseline.clone().unwrap_or_else(|| {
        runtime
            .services
            .instrumentation
            .complexity_counters
            .lock()
            .expect("complexity counter lock poisoned")
            .clone()
    });
    if let Some(telemetry) = context.bulk_mutation_telemetry.as_ref() {
        record_bulk_mutation_telemetry(runtime, telemetry);
    }
    enforce_validated_strategy_basis(
        runtime,
        &context.options,
        context.validated_against_commit_id,
        context.validated_against_version_id,
    )?;
    Ok(AdmittedCommitExecution {
        transaction_id: context.transaction_id,
        options: context.options,
        phase_timing: context.phase_timing,
        commit_log: CommitLog::new(),
        authority_input: context.authority_input,
        prepared_scope: context.prepared_scope,
        merge_execution_accounting: context.merge_execution_accounting,
        merge_execution_diagnostics_plan: context.merge_execution_diagnostics_plan,
        prevalidated_commit_boundary: context.prevalidated_commit_boundary,
        strategy_commit_artifacts: context.strategy_commit_artifacts,
        diagnostics_start,
        complexity_before,
    })
}

fn enforce_validated_strategy_basis(
    runtime: &RelationalRuntime,
    options: &crate::transactions::data::TransactionOptions,
    validated_against_commit_id: Option<crate::history::data::CommitId>,
    validated_against_version_id: Option<crate::identity::data::VersionId>,
) -> Result<(), TransactionCommitError> {
    let branch = options
        .target_branch
        .clone()
        .unwrap_or_else(|| runtime.config.history.main_branch.clone());
    let head = runtime.history().branch_head(&branch).cloned();
    validate_version_basis(runtime, head.as_ref(), validated_against_version_id)?;
    if validated_against_commit_id.is_some()
        && head.as_ref().map(|commit| commit.commit_id) != validated_against_commit_id
    {
        return Err(stale_strategy_validation_basis(
            "validated strategy plan no longer matches the current committed commit basis",
        ));
    }
    Ok(())
}

fn validate_version_basis(
    runtime: &RelationalRuntime,
    head: Option<&crate::history::data::CommitReference>,
    validated_version: Option<crate::identity::data::VersionId>,
) -> Result<(), TransactionCommitError> {
    let Some(validated_version) = validated_version else {
        return Ok(());
    };
    let observed = head
        .map(|commit| commit.version_id)
        .unwrap_or_else(|| runtime.current_version_id());
    (validated_version == observed)
        .then_some(())
        .ok_or_else(|| {
            stale_strategy_validation_basis(
                "validated strategy plan no longer matches the current committed version basis",
            )
        })
}
