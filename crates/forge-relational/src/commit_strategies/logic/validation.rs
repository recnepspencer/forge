use crate::authority::commit::phases::mutation::branch_local_delete_allowance_for_plan;
use crate::authority::commit::phases::prepare::prepare_authoritative_working_state_scope;
use crate::authority::mutation::apply_plan_to_working_state;
use crate::commit_strategies::data::{
    PreparedStrategyAuthorityScope, StrategyPreviewValidationCostSummary,
    ValidatedStrategyCommitPlan,
};
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{AuthoritativeApplyPlan, CommitValidation, TransactionCommitError};

pub(crate) fn validate_lowered_plan(
    runtime: &mut RelationalRuntime,
    lowered: crate::commit_strategies::data::LoweredStrategyCommitPlan,
) -> Result<ValidatedStrategyCommitPlan, TransactionCommitError> {
    let validated_against_version_id = runtime.current_version_id();
    let (structural_summary, working_state) = prepare_authoritative_working_state_scope(
        runtime,
        lowered.merged_plan(),
        lowered.options().merge_parent_branches.len(),
    );
    let commit_boundary_invariants = runtime
        .invariant_authority()
        .enforce_commit_boundary(lowered.merged_plan())?;
    let (preview_mutation_sensitive_invariants, preview_publication_invariants) =
        preview_strategy_post_mutation_validation(runtime, &lowered, &working_state)?;
    let validation_summary = CommitValidation::summarize(&[
        commit_boundary_invariants.clone(),
        preview_mutation_sensitive_invariants.clone(),
        preview_publication_invariants.clone(),
    ]);
    let preview_validation_cost = StrategyPreviewValidationCostSummary::new(
        runtime.history_access().preview_next_version_id(),
        lowered.merged_plan().merged_intents.len(),
        structural_summary.touched_partitions.len(),
        structural_summary.bulk_entity_slots_reserved,
        structural_summary.bulk_relation_slots_reserved,
        2,
    );

    Ok(ValidatedStrategyCommitPlan::new(
        lowered,
        validated_against_version_id,
        PreparedStrategyAuthorityScope {
            structural_summary,
            working_state,
        },
        commit_boundary_invariants,
        preview_mutation_sensitive_invariants,
        preview_publication_invariants,
        preview_validation_cost,
        validation_summary,
    ))
}

fn preview_strategy_post_mutation_validation(
    runtime: &mut RelationalRuntime,
    lowered: &crate::commit_strategies::data::LoweredStrategyCommitPlan,
    working_state: &crate::logic::runtime::WorkingState,
) -> Result<
    (
        crate::validation::engine::InvariantExecutionResult,
        crate::validation::engine::InvariantExecutionResult,
    ),
    TransactionCommitError,
> {
    let preview_version_id = runtime.history_access().preview_next_version_id();
    let apply_plan = AuthoritativeApplyPlan {
        transaction_id: lowered.transaction_id(),
        version_id: preview_version_id,
        merged_intents: lowered.merged_plan().merged_intents.clone(),
    };
    let mutation_config = crate::config::data::MutationConfig {
        patch_surface_policy: runtime.config().publication.policy.patch_surface_policy,
        cascade_delete_policy: runtime.config().storage.cascade_delete_policy,
        adjacency_policy: runtime.config().storage.adjacency_policy.clone(),
        cross_context_policy: runtime.config().storage.cross_context_policy,
        execution_model: runtime.config().execution.execution_model,
    };
    let mut preview_working_state = working_state.clone();
    let branch_local_delete_allowance = branch_local_delete_allowance_for_plan(
        runtime,
        lowered.merged_plan(),
        lowered.options().target_branch.as_ref(),
    );
    let mut preview_symbols = runtime.services.symbols.clone();
    apply_plan_to_working_state(
        &mut preview_working_state,
        &apply_plan,
        &mutation_config,
        &runtime.config().schema.registry,
        &runtime.aspect_semantics.plans,
        &mut preview_symbols,
        branch_local_delete_allowance,
    )
    .map_err(TransactionCommitError::conflict)?;

    let mutation_sensitive = runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(
            &preview_working_state,
            preview_version_id,
            lowered.merged_plan(),
        )
        .map_err(TransactionCommitError::conflict)?;
    let publication = runtime
        .invariant_authority()
        .enforce_snapshot_publication_for_working_state(
            &preview_working_state,
            preview_version_id,
            lowered.merged_plan(),
        )
        .map_err(TransactionCommitError::publication)?;

    Ok((mutation_sensitive, publication))
}
