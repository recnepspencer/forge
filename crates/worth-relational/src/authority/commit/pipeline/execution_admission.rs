use super::authority_context::AuthoritativeCommitContext;
use super::bulk_mutation_telemetry::record_bulk_mutation_telemetry;
use super::rejection::stale_strategy_validation_basis;
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{
    CommitLog, CommitPhaseTiming, MergedCommitPlan, TransactionCommitError, TransactionId,
    TransactionOptions,
};

pub(super) struct AdmittedCommitExecution {
    transaction_id: TransactionId,
    options: TransactionOptions,
    selected_branch_state: crate::branch::SelectedRelationalBranchState,
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
    pub(super) fn transaction_id(&self) -> TransactionId {
        self.transaction_id
    }

    pub(super) fn merged_plan(&self) -> &MergedCommitPlan {
        match &self.authority_input {
            super::authority_context::CommitAuthorityInput::Lowered(plan) => plan.merged_plan(),
            super::authority_context::CommitAuthorityInput::Merge(plan) => &plan.merged_plan,
        }
    }

    pub(super) fn selected_branch_state(&self) -> &crate::branch::SelectedRelationalBranchState {
        &self.selected_branch_state
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
    enforce_validated_strategy_basis(
        runtime,
        context.transaction_id,
        &context.options,
        context.validated_against_commit_id,
        context.validated_against_version_id,
        context.validated_against_branch_version,
        context
            .prepared_scope
            .as_ref()
            .and_then(|scope| scope.proposal_identity.as_ref()),
    )?;
    let selected_branch_state = context
        .prepared_scope
        .as_ref()
        .map(|scope| scope.selected_branch_state.clone())
        .or_else(|| match &context.authority_input {
            super::authority_context::CommitAuthorityInput::Lowered(plan) => {
                plan.selected_branch_state().cloned()
            }
            super::authority_context::CommitAuthorityInput::Merge(_) => None,
        })
        .map(Ok)
        .unwrap_or_else(|| {
            runtime
                .selected_branch_state(context.options.branch_binding())
                .map_err(TransactionCommitError::preparation)
        })?;
    if let Some(telemetry) = context.bulk_mutation_telemetry.as_ref() {
        record_bulk_mutation_telemetry(runtime, telemetry);
    }
    if matches!(
        context.options.branch_binding().observation().target(),
        worth_foundational::FoundationalBranchTarget::Basis(_)
    ) {
        runtime
            .history
            .record_retained_history_head_lookup(context.options.target_branch());
    }
    Ok(AdmittedCommitExecution {
        transaction_id: context.transaction_id,
        options: context.options,
        selected_branch_state,
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
    transaction_id: TransactionId,
    options: &crate::transactions::data::TransactionOptions,
    validated_against_commit_id: Option<crate::history::data::CommitId>,
    validated_against_version_id: Option<crate::identity::data::VersionId>,
    validated_against_branch_version: Option<crate::branch::RelationalBranchVersion>,
    proposal_identity: Option<&crate::transactions::RelationalMutationProposalIdentity>,
) -> Result<(), TransactionCommitError> {
    let branch = options.target_branch().clone();
    if options.branch_binding().identity().runtime_instance_id() != runtime.runtime_instance_id() {
        return Err(stale_strategy_validation_basis(
            "transaction branch binding belongs to another Relational runtime",
        ));
    }
    if let Some(identity) = proposal_identity {
        if identity.runtime_instance_id() != runtime.runtime_instance_id()
            || identity.transaction_id() != transaction_id
            || identity.branch_observation() != options.branch_binding().observation()
            || identity.branch_version() != options.branch_binding().truth_version()
        {
            return Err(stale_strategy_validation_basis(
                "proposal identity does not match the owner-issued transaction basis",
            ));
        }
        if identity.proposed_version_id() != runtime.history().preview_next_version_id() {
            return Err(stale_strategy_validation_basis(
                "proposal identity no longer names the next runtime version",
            ));
        }
    }
    if !runtime.legacy_branch_binding_is_current(options.branch_binding()) {
        return Err(stale_strategy_validation_basis(
            "transaction branch binding is no longer current",
        ));
    }
    if let Some(validated_branch_version) = validated_against_branch_version {
        let binding = options.branch_binding();
        let Some(cell) = runtime.history.branch_cell(&branch) else {
            return Err(stale_strategy_validation_basis(
                "validated transaction branch is no longer registered",
            ));
        };
        if cell.identity() != binding.identity()
            || cell.observation() != binding.observation()
            || cell.truth_version() != validated_branch_version
        {
            return Err(stale_strategy_validation_basis(
                "validated transaction no longer matches the current branch reference",
            ));
        }
    }
    let current_commit = runtime.legacy_branch_binding_commit(options.branch_binding());
    let current_version = runtime.legacy_branch_binding_version(options.branch_binding());
    validate_version_basis(current_version, validated_against_version_id)?;
    if validated_against_commit_id.is_some()
        && current_commit.as_ref().map(|commit| commit.commit_id()) != validated_against_commit_id
    {
        return Err(stale_strategy_validation_basis(
            "validated strategy plan no longer matches the current committed commit basis",
        ));
    }
    Ok(())
}

fn validate_version_basis(
    current_version: Option<crate::identity::data::VersionId>,
    validated_version: Option<crate::identity::data::VersionId>,
) -> Result<(), TransactionCommitError> {
    let Some(validated_version) = validated_version else {
        return Ok(());
    };
    let Some(observed) = current_version else {
        return Err(stale_strategy_validation_basis(
            "owner-issued branch binding has no exact local version basis",
        ));
    };
    (validated_version == observed)
        .then_some(())
        .ok_or_else(|| {
            stale_strategy_validation_basis(
                "validated strategy plan no longer matches the current committed version basis",
            )
        })
}
