use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::facade::PreparedInvariantExecution;
use crate::authority::commit::preparation::planning::context::PreparationPlanningContext;
use crate::authority::commit::preparation::planning::strategy::{
    packet_width_is_profitable, ParallelLegality, ParallelProfitability, PreparationFallbackReason,
    PreparationStrategy, PreparationStrategySelection, MIN_PARALLEL_PACKET_WIDTH,
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
use crate::validation::data::InvariantRegistration;
use crate::validation::engine::InvariantExecutionRequest;
use std::collections::BTreeSet;

pub(crate) type PlannedInvariantExecution<'runtime> = PreparedInvariantExecution<'runtime>;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TestPreparationFault {
    PlanningProofInsufficient,
    PublicationIsolationViolation,
    ReductionIdentityConflict,
    WorkerEvaluationFailure,
}

#[cfg(test)]
static TEST_PREPARATION_FAULT_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

#[cfg(test)]
static TEST_PREPARATION_FAULT: std::sync::atomic::AtomicU8 = std::sync::atomic::AtomicU8::new(0);

#[cfg(test)]
pub(crate) fn current_test_preparation_fault() -> Option<TestPreparationFault> {
    match TEST_PREPARATION_FAULT.load(std::sync::atomic::Ordering::SeqCst) {
        1 => Some(TestPreparationFault::PlanningProofInsufficient),
        2 => Some(TestPreparationFault::PublicationIsolationViolation),
        3 => Some(TestPreparationFault::ReductionIdentityConflict),
        4 => Some(TestPreparationFault::WorkerEvaluationFailure),
        _ => None,
    }
}

#[cfg(test)]
pub(crate) fn with_test_preparation_fault<T>(
    fault: TestPreparationFault,
    run: impl FnOnce() -> T,
) -> T {
    let _guard = TEST_PREPARATION_FAULT_LOCK.lock().unwrap();
    TEST_PREPARATION_FAULT.store(
        match fault {
            TestPreparationFault::PlanningProofInsufficient => 1,
            TestPreparationFault::PublicationIsolationViolation => 2,
            TestPreparationFault::ReductionIdentityConflict => 3,
            TestPreparationFault::WorkerEvaluationFailure => 4,
        },
        std::sync::atomic::Ordering::SeqCst,
    );
    let result = run();
    TEST_PREPARATION_FAULT.store(0, std::sync::atomic::Ordering::SeqCst);
    result
}

#[cfg(test)]
pub(crate) fn has_test_preparation_fault() -> bool {
    current_test_preparation_fault().is_some()
}

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
        PreparationStrategy::serial(PreparationFallbackReason::ExecutionModelSerial)
    } else if !packet_width_is_profitable(packet_count, MIN_PARALLEL_PACKET_WIDTH) {
        PreparationStrategy {
            parallel_legality: ParallelLegality::ProvenParallel,
            parallel_profitability: ParallelProfitability::NotProfitable,
            selected_mode: PreparationStrategySelection::Serial,
            fallback_reason: Some(PreparationFallbackReason::InsufficientPacketBreadth),
        }
    } else if proof_kind == PreparationProofKind::RequiresSerial {
        PreparationStrategy::serial(PreparationFallbackReason::ProofRequiresSerial)
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
            let record_domain = if request.merged_plan().is_some() {
                PreparationRecordDomain::Mixed
            } else {
                PreparationRecordDomain::None
            };
            let locality = PreparationLocalityProof {
                observation_scope: request.observation().kind(),
                record_domain,
                partition_scope: if partition_scope.is_empty() {
                    PreparationPartitionScope::AllObserved
                } else {
                    PreparationPartitionScope::TouchedPartitions(partition_scope.clone())
                },
                invariant_group_scope,
                read_set_approximation: PreparationReadSetApproximation::SharedCommittedRead,
                write_exclusion: match proof_kind {
                    PreparationProofKind::RequiresSerial => {
                        PreparationWriteExclusionClass::RequiresSerialAuthority
                    }
                    _ => PreparationWriteExclusionClass::ReadOnly,
                },
            };
            let mut packet = crate::authority::commit::preparation::InvariantWorkPacket {
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
            };
            #[cfg(test)]
            match current_test_preparation_fault() {
                Some(TestPreparationFault::PlanningProofInsufficient) => {
                    packet.validity.context.invariant_registration_count += 1;
                }
                Some(TestPreparationFault::PublicationIsolationViolation) => {
                    packet.locality.write_exclusion =
                        PreparationWriteExclusionClass::RequiresSerialAuthority;
                }
                Some(TestPreparationFault::ReductionIdentityConflict) => {}
                Some(TestPreparationFault::WorkerEvaluationFailure) => {}
                None => {}
            }
            packet
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
        failure_count: 0,
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

fn packet_partition_scope(
    merged_plan: Option<&MergedCommitPlan>,
) -> Vec<crate::identity::data::PartitionId> {
    let mut touched = BTreeSet::new();
    if let Some(plan) = merged_plan {
        for intent in &plan.merged_intents {
            intent.seed_touched_partitions(&mut touched);
        }
    }
    touched.into_iter().collect()
}
