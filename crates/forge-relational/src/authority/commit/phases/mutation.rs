use std::collections::BTreeSet;

use crate::authority::intent_merge::{
    entity_exists_in_version_basis, relation_exists_in_version_basis,
};
use crate::authority::mutation::{
    apply_plan_to_working_state, BranchLocalDeleteAllowance, MutationApplyOutcome, MutationEffect,
};
use crate::history::data::BranchId;
use crate::identity::data::VersionId;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::{
    AuthoritativeApplyPlan, MergedCommitPlan, MutationIntent, TransactionCommitError, TransactionId,
};
use crate::validation::engine::InvariantExecutionResult;

use super::prepare::record_mutation_counters;

pub(crate) struct MutationPhaseOutput {
    pub(crate) version_id: VersionId,
    pub(crate) effect: MutationEffect,
    pub(crate) invariant_results: InvariantExecutionResult,
}

pub(crate) fn run_authoritative_mutation_for_runtime(
    runtime: &mut RelationalRuntime,
    transaction_id: TransactionId,
    working_state: &mut crate::logic::runtime::WorkingState,
    merged_plan: &MergedCommitPlan,
    target_branch: Option<&BranchId>,
) -> Result<MutationPhaseOutput, TransactionCommitError> {
    let version_id = runtime.history().preview_next_version_id();
    let apply_plan = AuthoritativeApplyPlan {
        transaction_id,
        version_id,
        merged_intents: merged_plan.merged_intents.clone(),
    };
    let mutation_config = crate::config::data::MutationConfig {
        cascade_delete_policy: runtime.config.storage.cascade_delete_policy,
        adjacency_policy: runtime.config.storage.adjacency_policy.clone(),
        cross_context_policy: runtime.config.storage.cross_context_policy,
        execution_model: runtime.config.execution.execution_model,
    };
    let branch_local_delete_allowance =
        branch_local_delete_allowance_for_plan(runtime, merged_plan, target_branch);
    let MutationApplyOutcome {
        effect,
        preparation_telemetry,
    } = apply_plan_to_working_state(
        working_state,
        &apply_plan,
        &mutation_config,
        &runtime.config.schema.registry,
        &runtime.aspect_semantics.plans,
        &mut runtime.services.symbols,
        branch_local_delete_allowance,
    )
    .map_err(TransactionCommitError::conflict)?;
    runtime.performance_access().count_preparation_packet_shape(
        preparation_telemetry.packet_count,
        preparation_telemetry.packet_item_count,
        preparation_telemetry.packet_peak_width_total,
        preparation_telemetry.scope_unit_count,
    );
    for _ in 0..preparation_telemetry.parallel_legal_count {
        runtime
            .performance_access()
            .count_preparation_parallel_legal();
    }
    for _ in 0..preparation_telemetry.parallel_profitable_count {
        runtime
            .performance_access()
            .count_preparation_parallel_profitable();
    }
    for _ in 0..preparation_telemetry.serial_strategy_count {
        runtime
            .performance_access()
            .count_preparation_serial_strategy();
    }
    for _ in 0..preparation_telemetry.staged_parallel_strategy_count {
        runtime
            .performance_access()
            .count_preparation_staged_parallel_strategy();
    }
    record_mutation_counters(runtime, working_state);

    let invariant_results = runtime
        .invariant_authority()
        .enforce_mutation_sensitive_for_working_state(working_state, version_id, merged_plan)
        .map_err(TransactionCommitError::conflict)?;

    Ok(MutationPhaseOutput {
        version_id,
        effect,
        invariant_results,
    })
}

pub(crate) fn branch_local_delete_allowance_for_plan(
    runtime: &RelationalRuntime,
    merged_plan: &MergedCommitPlan,
    target_branch: Option<&BranchId>,
) -> BranchLocalDeleteAllowance {
    let Some(branch_id) = target_branch else {
        return BranchLocalDeleteAllowance::default();
    };
    let history = runtime.history();
    let Some(branch_head) = history.branch_head(branch_id) else {
        return BranchLocalDeleteAllowance::default();
    };
    let current_state = runtime.storage_access().current_state();
    let mut entity_ids = BTreeSet::new();
    let mut relation_ids = BTreeSet::new();

    for intent in &merged_plan.merged_intents {
        match intent {
            MutationIntent::Entity(crate::transactions::data::EntityMutationIntent::Delete(
                spec,
            )) => {
                if !crate::authority::intent_merge::entity_exists_in_state(
                    &current_state,
                    spec.entity_id,
                ) && entity_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    spec.entity_id,
                ) {
                    entity_ids.insert(spec.entity_id);
                }
            }
            MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::UpdateEndpoints(spec),
            ) => {
                let relation_id = spec.relation_id;
                if !crate::authority::intent_merge::relation_exists_in_state(
                    &current_state,
                    relation_id,
                ) && relation_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    relation_id,
                ) {
                    relation_ids.insert(relation_id);
                }
            }
            MutationIntent::Relation(
                crate::transactions::data::RelationMutationIntent::Delete(spec),
            ) => {
                let relation_id = spec.relation_id;
                if !crate::authority::intent_merge::relation_exists_in_state(
                    &current_state,
                    relation_id,
                ) && relation_exists_in_version_basis(
                    runtime,
                    branch_head.version_id,
                    relation_id,
                ) {
                    relation_ids.insert(relation_id);
                }
            }
            MutationIntent::Create(_) | MutationIntent::Entity(_) => {}
        }
    }

    BranchLocalDeleteAllowance {
        entity_ids,
        relation_ids,
    }
}
