use crate::authority::mutation::{
    apply_plan_to_working_state, MutationApplyOutcome, MutationEffect,
};
use crate::identity::data::VersionId;
use crate::transactions::data::{AuthoritativeApplyPlan, MergedCommitPlan, TransactionCommitError};
use crate::transactions::logic::RelationalTransaction;
use crate::validation::engine::InvariantExecutionResult;

use super::prepare::record_mutation_counters;

pub(crate) struct MutationPhaseOutput {
    pub(crate) version_id: VersionId,
    pub(crate) effect: MutationEffect,
    pub(crate) invariant_results: InvariantExecutionResult,
}

pub(crate) fn run_authoritative_mutation(
    transaction: &mut RelationalTransaction<'_>,
    working_state: &mut crate::logic::runtime::WorkingState,
    merged_plan: &MergedCommitPlan,
) -> Result<MutationPhaseOutput, TransactionCommitError> {
    let version_id = transaction
        .runtime
        .history_access()
        .preview_next_version_id();
    let apply_plan = AuthoritativeApplyPlan {
        transaction_id: transaction.transaction_id,
        version_id,
        merged_intents: merged_plan.merged_intents.clone(),
    };
    let mutation_config = crate::config::data::MutationConfig {
        patch_surface_policy: transaction
            .runtime
            .config
            .publication
            .policy
            .patch_surface_policy,
        cascade_delete_policy: transaction.runtime.config.storage.cascade_delete_policy,
        adjacency_policy: transaction.runtime.config.storage.adjacency_policy.clone(),
        cross_context_policy: transaction.runtime.config.storage.cross_context_policy,
        execution_model: transaction.runtime.config.execution.execution_model,
    };
    let MutationApplyOutcome {
        effect,
        preparation_telemetry,
    } = apply_plan_to_working_state(
        working_state,
        &apply_plan,
        &mutation_config,
        &transaction.runtime.config.schema.registry,
        &transaction.runtime.aspect_semantics.plans,
        &mut transaction.runtime.services.symbols,
    )
    .map_err(TransactionCommitError::conflict)?;
    transaction
        .runtime
        .performance_access()
        .count_preparation_packet_shape(
            preparation_telemetry.packet_count,
            preparation_telemetry.packet_item_count,
            preparation_telemetry.packet_peak_width_total,
            preparation_telemetry.scope_unit_count,
        );
    for _ in 0..preparation_telemetry.parallel_legal_count {
        transaction
            .runtime
            .performance_access()
            .count_preparation_parallel_legal();
    }
    for _ in 0..preparation_telemetry.parallel_profitable_count {
        transaction
            .runtime
            .performance_access()
            .count_preparation_parallel_profitable();
    }
    for _ in 0..preparation_telemetry.serial_strategy_count {
        transaction
            .runtime
            .performance_access()
            .count_preparation_serial_strategy();
    }
    for _ in 0..preparation_telemetry.staged_parallel_strategy_count {
        transaction
            .runtime
            .performance_access()
            .count_preparation_staged_parallel_strategy();
    }
    record_mutation_counters(transaction.runtime, working_state);

    let invariant_results = transaction
        .runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(working_state, version_id, merged_plan)
        .map_err(TransactionCommitError::conflict)?;

    Ok(MutationPhaseOutput {
        version_id,
        effect,
        invariant_results,
    })
}
