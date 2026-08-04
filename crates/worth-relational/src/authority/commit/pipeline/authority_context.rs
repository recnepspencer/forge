use super::bulk_mutation_telemetry::{telemetry_from_strategy_batch, BulkMutationPlanTelemetry};
use super::rejection::invalid_merge_context;
use crate::capabilities::RuntimeConfigSource;
use crate::commit_strategies::data::{
    LoweredStrategyCommitPlan, StrategyCommitArtifactBundle, ValidatedStrategyCommitPlan,
};
use crate::transactions::data::{
    CommitPhaseTiming, LoweredCommitPlan, MergeCommitMutationPlan, TransactionCommitError,
    TransactionId, TransactionOptions,
};
use crate::validation::engine::InvariantExecutionResult;

#[derive(Debug)]
pub(super) enum CommitAuthorityInput {
    Lowered(LoweredCommitPlan),
    Merge(MergeCommitMutationPlan),
}

#[derive(Debug)]
pub(crate) struct AuthoritativeCommitContext {
    pub(super) transaction_id: TransactionId,
    pub(super) options: TransactionOptions,
    pub(super) phase_timing: CommitPhaseTiming,
    pub(super) authority_input: CommitAuthorityInput,
    pub(super) prepared_scope: Option<PreparedAuthorityScope>,
    pub(super) merge_execution_accounting: Option<MergeExecutionAccounting>,
    pub(super) bulk_mutation_telemetry: Option<BulkMutationPlanTelemetry>,
    pub(super) prevalidated_commit_boundary: Option<InvariantExecutionResult>,
    pub(super) validated_against_commit_id: Option<crate::history::data::CommitId>,
    pub(super) validated_against_version_id: Option<crate::identity::data::VersionId>,
    pub(super) strategy_commit_artifacts: Option<StrategyCommitArtifactBundle>,
}

#[derive(Debug)]
pub(super) struct PreparedAuthorityScope {
    pub(super) structural_summary:
        crate::authority::commit::structural_summary::CommitStructuralSummary,
    pub(super) working_state: crate::storage::overlay::WorkingState,
    pub(super) phase_timing: CommitPhaseTiming,
}

#[derive(Debug, Clone, Copy)]
pub(super) struct MergeExecutionAccounting {
    pub(super) admitted_records: usize,
    pub(super) emitted_mutation_intents: usize,
}

impl AuthoritativeCommitContext {
    pub(crate) fn from_validated_mutation(
        candidate: crate::transactions::logic::ValidatedRelationalMutation,
    ) -> Self {
        let crate::transactions::logic::ValidatedRelationalMutation {
            transaction_id,
            options,
            prepared,
            commit_boundary,
            validated_against_commit,
            validated_against_version,
            batch_count,
            ..
        } = candidate;
        let bulk_mutation_telemetry =
            super::bulk_mutation_telemetry::summarize_bulk_mutation_telemetry(
                &prepared.merged_plan,
                batch_count,
            );
        Self {
            transaction_id,
            options,
            phase_timing: prepared.phase_timing.clone(),
            authority_input: CommitAuthorityInput::Lowered(LoweredCommitPlan::Mutation(
                prepared.merged_plan,
            )),
            prepared_scope: Some(PreparedAuthorityScope {
                structural_summary: prepared.structural_summary,
                working_state: prepared.working_state,
                phase_timing: prepared.phase_timing,
            }),
            merge_execution_accounting: None,
            bulk_mutation_telemetry,
            prevalidated_commit_boundary: Some(commit_boundary),
            validated_against_commit_id: validated_against_commit,
            validated_against_version_id: Some(validated_against_version),
            strategy_commit_artifacts: None,
        }
    }

    pub(super) fn from_mutation(
        transaction_id: TransactionId,
        options: TransactionOptions,
        phase_timing: CommitPhaseTiming,
        prepared: crate::authority::commit::phases::prepare::PreparedWorkingStateScope,
        bulk_mutation_telemetry: Option<BulkMutationPlanTelemetry>,
    ) -> Self {
        Self {
            transaction_id,
            options,
            phase_timing: phase_timing.clone(),
            authority_input: CommitAuthorityInput::Lowered(LoweredCommitPlan::Mutation(
                prepared.merged_plan,
            )),
            prepared_scope: Some(PreparedAuthorityScope {
                structural_summary: prepared.structural_summary,
                working_state: prepared.working_state,
                phase_timing,
            }),
            merge_execution_accounting: None,
            bulk_mutation_telemetry,
            prevalidated_commit_boundary: None,
            validated_against_commit_id: None,
            validated_against_version_id: None,
            strategy_commit_artifacts: None,
        }
    }

    pub(crate) fn from_merge(
        options: TransactionOptions,
        merge_plan: MergeCommitMutationPlan,
    ) -> Result<Self, TransactionCommitError> {
        validate_merge_context_proof(&options, &merge_plan)?;

        Ok(Self {
            transaction_id: merge_plan.transaction_id,
            options,
            phase_timing: CommitPhaseTiming::default(),
            merge_execution_accounting: Some(MergeExecutionAccounting {
                admitted_records: merge_plan.structural_summary.executed_record_count,
                emitted_mutation_intents: merge_plan
                    .structural_summary
                    .emitted_mutation_intent_count,
            }),
            authority_input: CommitAuthorityInput::Merge(merge_plan),
            prepared_scope: None,
            bulk_mutation_telemetry: None,
            prevalidated_commit_boundary: None,
            validated_against_commit_id: None,
            validated_against_version_id: None,
            strategy_commit_artifacts: None,
        })
    }

    pub(crate) fn from_strategy(
        runtime: &crate::logic::runtime::RelationalRuntime,
        lowered_plan: LoweredStrategyCommitPlan,
    ) -> Self {
        let descriptor = runtime
            .commit_strategy_registry()
            .get_by_id(lowered_plan.request().strategy_id())
            .expect("strategy lowering provenance should resolve to a registered descriptor")
            .descriptor();
        Self {
            transaction_id: lowered_plan.transaction_id(),
            options: lowered_plan.options().clone(),
            phase_timing: CommitPhaseTiming::default(),
            authority_input: CommitAuthorityInput::Lowered(LoweredCommitPlan::Strategy(
                lowered_plan.clone(),
            )),
            prepared_scope: None,
            merge_execution_accounting: None,
            bulk_mutation_telemetry: lowered_plan
                .bulk_mutation_batch()
                .map(telemetry_from_strategy_batch),
            prevalidated_commit_boundary: None,
            validated_against_commit_id: None,
            validated_against_version_id: None,
            strategy_commit_artifacts: Some(StrategyCommitArtifactBundle::from_lowered(
                &lowered_plan,
                descriptor,
                runtime.runtime_config(),
            )),
        }
    }

    pub(crate) fn from_validated_strategy(
        runtime: &crate::logic::runtime::RelationalRuntime,
        validated_plan: ValidatedStrategyCommitPlan,
    ) -> Self {
        let descriptor = runtime
            .commit_strategy_registry()
            .get_by_id(validated_plan.lowered_plan().request().strategy_id())
            .expect("validated strategy plan should resolve to a registered descriptor")
            .descriptor();
        Self {
            transaction_id: validated_plan.lowered_plan().transaction_id(),
            options: validated_plan.lowered_plan().options().clone(),
            phase_timing: CommitPhaseTiming::default(),
            authority_input: CommitAuthorityInput::Lowered(LoweredCommitPlan::Strategy(
                validated_plan.lowered_plan().clone(),
            )),
            prepared_scope: Some(PreparedAuthorityScope {
                structural_summary: validated_plan.prepared_scope().structural_summary.clone(),
                working_state: validated_plan.prepared_scope().working_state.clone(),
                phase_timing: CommitPhaseTiming::default(),
            }),
            merge_execution_accounting: None,
            bulk_mutation_telemetry: validated_plan
                .lowered_plan()
                .bulk_mutation_batch()
                .map(telemetry_from_strategy_batch),
            prevalidated_commit_boundary: Some(validated_plan.commit_boundary_invariants().clone()),
            validated_against_commit_id: validated_plan.validated_against_commit_id(),
            validated_against_version_id: Some(validated_plan.validated_against_version_id()),
            strategy_commit_artifacts: Some(
                StrategyCommitArtifactBundle::from_lowered(
                    validated_plan.lowered_plan(),
                    descriptor,
                    runtime.runtime_config(),
                )
                .with_preview_validation(
                    validated_plan.validation_summary(),
                    validated_plan.preview_validation_cost(),
                    validated_plan.validated_against_commit_id(),
                    validated_plan.validated_against_version_id(),
                ),
            ),
        }
    }
}

fn validate_merge_context_proof(
    options: &TransactionOptions,
    merge_plan: &MergeCommitMutationPlan,
) -> Result<(), TransactionCommitError> {
    if options.target_branch.as_ref() != Some(&merge_plan.target_branch) {
        return Err(invalid_merge_context(
            "merge commit context target branch does not match merge proof",
        ));
    }
    if options.merge_parent_branches != merge_plan.merge_parent_branches.as_ref() {
        return Err(invalid_merge_context(
            "merge commit context merge parent branches do not match merge proof",
        ));
    }
    if merge_plan.requested_merge_parent_count != merge_plan.merge_parent_branches.len() {
        return Err(invalid_merge_context(
            "merge commit context requested merge parent count does not match merge proof",
        ));
    }
    let expected_parents = [
        merge_plan.merge_execution_summary.target_head_commit_id,
        merge_plan.merge_execution_summary.source_head_commit_id,
    ];
    if merge_plan.parent_commits.as_slice() != expected_parents {
        return Err(invalid_merge_context(
            "merge commit context ordered parent commits do not match merge proof",
        ));
    }
    Ok(())
}
