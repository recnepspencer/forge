use crate::authority::commit::phases::prepare::prepare_authoritative_working_state_scope_for_base;
use crate::authority::commit::phases::proposed_invariant_state::prepare_proposed_invariant_state;
use crate::commit_strategies::data::{
    PreparedStrategyAuthorityScope, StrategyPreviewValidationCostSummary,
    ValidatedStrategyCommitPlan,
};
use crate::runtime::RelationalRuntime;
use crate::transactions::data::{CommitValidation, TransactionCommitError};

pub(crate) fn validate_lowered_plan(
    runtime: &mut RelationalRuntime,
    lowered: crate::commit_strategies::data::LoweredStrategyCommitPlan,
) -> Result<ValidatedStrategyCommitPlan, TransactionCommitError> {
    let validated_against_commit =
        runtime.legacy_branch_binding_commit(lowered.options().branch_binding());
    let validated_against_commit_id = validated_against_commit
        .as_ref()
        .map(|commit| commit.commit_id());
    let validated_against_version_id = runtime
        .legacy_branch_binding_version(lowered.options().branch_binding())
        .ok_or_else(|| {
            TransactionCommitError::conflict(crate::transactions::data::CommitConflict::new(
                crate::transactions::data::ConflictClass::StaleValidationBasis {
                    detail: "owner-issued branch binding has no exact local version basis"
                        .to_owned(),
                },
            ))
        })?;
    let selected_branch_state = runtime
        .selected_branch_state(lowered.options().branch_binding())
        .map_err(TransactionCommitError::preparation)?;
    let (structural_summary, working_state, _) = prepare_authoritative_working_state_scope_for_base(
        runtime,
        selected_branch_state.state(),
        lowered.merged_plan(),
        lowered.options().merge_parent_bindings().len(),
    );
    let proposal_identity =
        runtime.issue_mutation_proposal_identity(lowered.transaction_id(), lowered.options())?;
    let preview_validation_version_id = proposal_identity.proposed_version_id();
    let proposed_working_state = prepare_proposed_invariant_state(
        runtime,
        &selected_branch_state,
        &working_state,
        lowered.merged_plan(),
        preview_validation_version_id,
    )?;
    let commit_boundary_invariants = runtime
        .invariant_authority()
        .enforce_commit_boundary_for_selected_branch(
            &selected_branch_state,
            &proposed_working_state,
            preview_validation_version_id,
            lowered.merged_plan(),
            Some(&proposal_identity),
        )?;
    let (preview_mutation_sensitive_invariants, preview_publication_invariants) =
        preview_strategy_post_mutation_validation(
            runtime,
            &lowered,
            &selected_branch_state,
            &proposed_working_state,
            preview_validation_version_id,
            Some(&proposal_identity),
        )?;
    let validation_summary = CommitValidation::summarize(&[
        commit_boundary_invariants.clone(),
        preview_mutation_sensitive_invariants.clone(),
        preview_publication_invariants.clone(),
    ]);
    let preview_validation_cost = StrategyPreviewValidationCostSummary::new(
        preview_validation_version_id,
        lowered.merged_plan().merged_intents.len(),
        structural_summary.touched_partitions.len(),
        structural_summary.bulk_entity_slots_reserved,
        structural_summary.bulk_relation_slots_reserved,
        2,
    );

    Ok(ValidatedStrategyCommitPlan::new(
        lowered,
        validated_against_commit_id,
        validated_against_version_id,
        PreparedStrategyAuthorityScope {
            selected_branch_state,
            structural_summary,
            working_state,
        },
        proposed_working_state,
        proposal_identity,
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
    selected_branch_state: &crate::branch::SelectedRelationalBranchState,
    proposed_working_state: &crate::runtime::WorkingState,
    preview_version_id: crate::identity::data::VersionId,
    proposal_identity: Option<&crate::transactions::RelationalMutationProposalIdentity>,
) -> Result<
    (
        crate::validation::engine::InvariantExecutionResult,
        crate::validation::engine::InvariantExecutionResult,
    ),
    TransactionCommitError,
> {
    let mutation_sensitive = runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(
            selected_branch_state,
            proposed_working_state,
            preview_version_id,
            lowered.merged_plan(),
            proposal_identity,
        )
        .map_err(TransactionCommitError::conflict)?;
    let publication = runtime
        .invariant_authority()
        .enforce_snapshot_publication_for_working_state(
            selected_branch_state,
            proposed_working_state,
            preview_version_id,
            lowered.merged_plan(),
            proposal_identity,
        )
        .map_err(TransactionCommitError::publication)?;

    Ok((mutation_sensitive, publication))
}
