use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::facade::PreparedInvariantExecution;
use crate::authority::commit::preparation::planning::context::PreparationPlanningContext;
use crate::authority::commit::preparation::planning::strategy::{
    ParallelLegality, ParallelProfitability, PreparationFallbackReason, PreparationStrategy,
    PreparationStrategySelection,
};
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationLocalityProof, PreparationPartitionScope, PreparationReadSetApproximation,
    PreparationRecordDomain, PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::proofs::validity::PreparationProofValidity;
use crate::authority::commit::preparation::reduction::keys::ValidationReductionKey;
use crate::logic::planning::RelationalExecutionModel;
use crate::logic::runtime::RelationalRuntime;
use crate::transactions::data::MergedCommitPlan;
use crate::validation::data::{InvariantGroupSet, InvariantRegistration};
use crate::validation::engine::InvariantExecutionRequest;
use std::collections::BTreeSet;

pub(crate) type PlannedInvariantExecution<'runtime> = PreparedInvariantExecution<'runtime>;

pub(crate) fn plan_invariant_execution<'runtime>(
    runtime: &'runtime RelationalRuntime,
    request: &InvariantExecutionRequest<'runtime>,
) -> PlannedInvariantExecution<'runtime> {
    let registrations = eligible_registrations(runtime, &request);
    let context = planning_context(runtime, &request);
    let proof_kind = if matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        PreparationProofKind::ReadOnlyShared
    } else {
        PreparationProofKind::RequiresSerial
    };
    let packet_count = registrations.len();
    let strategy = if !matches!(
        runtime.config.execution.execution_model,
        RelationalExecutionModel::StagedParallelPreparation
    ) {
        PreparationStrategy {
            parallel_legality: ParallelLegality::RequiresSerial,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::ExecutionModelSerial),
        }
    } else if packet_count <= 1 {
        PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::InsufficientPacketBreadth),
        }
    } else if proof_kind == PreparationProofKind::RequiresSerial {
        PreparationStrategy {
            parallel_legality: ParallelLegality::RequiresSerial,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::ProofRequiresSerial),
        }
    } else {
        PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::Profitable,
            selected_mode: PreparationStrategySelection::StagedParallel,
            fallback_reason: None,
        }
    };

    let packets = registrations
        .into_iter()
        .enumerate()
        .map(|(packet_index, registration)| {
            let partition_scope = packet_partition_scope(request.merged_plan());
            let invariant_group_scope = registration.rule.groups();
            let locality = PreparationLocalityProof {
                observation_scope: request.observation().kind(),
                record_domain: PreparationRecordDomain::Mixed,
                partition_scope: PreparationPartitionScope::TouchedPartitions(
                    partition_scope.clone(),
                ),
                invariant_group_scope,
                read_set_approximation: PreparationReadSetApproximation::SharedCommittedRead,
                write_exclusion: PreparationWriteExclusionClass::ReadOnly,
            };
            crate::authority::commit::preparation::InvariantWorkPacket {
                packet_index,
                registration,
                reduction_key: ValidationReductionKey::new(
                    request.execution_point(),
                    request.observation().kind(),
                    partition_scope,
                    invariant_group_scope,
                    packet_index,
                ),
                proof_kind,
                locality,
                validity: PreparationProofValidity {
                    context: context.clone(),
                },
                planning_context: context.clone(),
                observation: request.observation().clone(),
                version_id: request.version_id(),
                merged_plan: request.merged_plan(),
            }
        })
        .collect();

    PreparedInvariantExecution {
        context,
        strategy,
        packets,
    }
}

pub(crate) fn planned_packet_counters(
    planned: &PreparedInvariantExecution<'_>,
) -> ValidationPreparationCounters {
    ValidationPreparationCounters {
        packet_count: planned.packets.len(),
        worker_result_count: 0,
        reducer_input_count: 0,
        reducer_conflict_count: 0,
    }
}

fn planning_context(
    runtime: &RelationalRuntime,
    request: &InvariantExecutionRequest<'_>,
) -> PreparationPlanningContext {
    PreparationPlanningContext {
        transaction_id: request.merged_plan().map(|plan| plan.transaction_id),
        execution_point: request.execution_point(),
        observation_kind: request.observation().kind(),
        version_id: request.version_id(),
        current_version_id: request.current_version_id(),
        structural_summary: None,
        plan_contract: request.plan_contract(),
        schema_registry_entry_count: runtime.config.schema.registry.entity_kinds.len()
            + runtime.config.schema.registry.relation_kinds.len(),
        invariant_registration_count: runtime.config.schema.invariant_catalog.registrations.len(),
        planning_contract: runtime.config.execution.planning.clone(),
    }
}

fn eligible_registrations(
    runtime: &RelationalRuntime,
    request: &InvariantExecutionRequest<'_>,
) -> Vec<InvariantRegistration> {
    runtime
        .config
        .schema
        .invariant_catalog
        .registrations_for_execution_point(request.execution_point())
        .filter(|registration| request.includes_registration(registration))
        .cloned()
        .collect()
}

fn packet_partition_scope(merged_plan: Option<&MergedCommitPlan>) -> Vec<crate::identity::data::PartitionId> {
    let mut touched = BTreeSet::new();
    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched);
        }
    }
    touched.into_iter().collect()
}
