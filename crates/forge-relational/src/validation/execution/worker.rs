use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::diagnostics::failures::PreparationFailureClass;
use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;
use crate::authority::commit::preparation::proofs::kinds::PreparationProofKind;
use crate::authority::commit::preparation::proofs::locality::{
    PreparationPartitionScope, PreparationReadSetApproximation, PreparationRecordDomain,
    PreparationWriteExclusionClass,
};
use crate::authority::commit::preparation::reduction::identity::ValidationResultIdentity;
use crate::logic::runtime::RelationalRuntime;
use crate::validation::data::{InvariantCheckResult, InvariantVerdict};
#[cfg(test)]
use crate::validation::data::InvariantRule;
use crate::validation::engine::context::InvariantExecutionContext;
use crate::validation::engine::evaluator::evaluate_rule;

use super::envelope::InvariantWorkerEnvelope;
use super::packets::InvariantWorkPacket;

pub(crate) fn evaluate_invariant_packet(
    runtime: &RelationalRuntime,
    packet: &InvariantWorkPacket<'_>,
) -> InvariantWorkerEnvelope {
    #[allow(unused_mut)]
    let mut preparation_failures = invariant_packet_failures(packet);
    #[cfg(test)]
    if matches!(
        crate::validation::execution::current_test_preparation_fault(),
        Some(crate::validation::execution::TestPreparationFault::WorkerEvaluationFailure)
    ) {
        preparation_failures.push(PreparationFailureClass::WorkerEvaluationFailure);
    }
    #[cfg(not(test))]
    debug_assert!(
        preparation_failures.is_empty(),
        "planned invariant packet violated preparation proof contract: {:?}",
        preparation_failures
    );
    #[cfg(test)]
    debug_assert!(
        preparation_failures.is_empty()
            || crate::validation::execution::has_test_preparation_fault(),
        "planned invariant packet violated preparation proof contract: {:?}",
        preparation_failures
    );
    let context = InvariantExecutionContext::new(
        runtime,
        packet.observation.clone(),
        packet.version_id,
        packet.registration.execution_point,
        packet.merged_plan,
        packet.relation_integrity_scopes.clone(),
    );
    let verdict = if let Some(violation) = evaluate_rule(
        &context,
        packet.registration.execution_point.class(),
        &packet.registration.rule,
    ) {
        packet.registration.verdict_for_violation(violation)
    } else {
        InvariantVerdict::Pass
    };
    let result = InvariantCheckResult {
        execution_point: packet.registration.execution_point,
        failure_effect: packet.registration.failure_effect,
        rule: packet.registration.rule.clone(),
        verdict,
    };
    #[allow(unused_mut)]
    let mut result_identity = ValidationResultIdentity::from_parts(
        result.execution_point,
        result.failure_effect,
        result.rule.clone(),
        &result.verdict,
    );
    #[cfg(test)]
    if matches!(
        crate::validation::execution::current_test_preparation_fault(),
        Some(crate::validation::execution::TestPreparationFault::ReductionIdentityConflict)
    ) {
        result_identity = ValidationResultIdentity {
            execution_point: result.execution_point,
            failure_effect: result.failure_effect,
            rule: InvariantRule::MaxMergedIntents(16),
            target_scope_identity: "reduction-conflict".to_string(),
        };
    }
    let diagnostic_observations = if matches!(result.verdict, InvariantVerdict::Pass) {
        Vec::new()
    } else {
        vec![ValidationDiagnosticObservation {
            packet_index: packet.packet_index,
            result_identity: result_identity.clone(),
        }]
    };

    InvariantWorkerEnvelope {
        packet_index: packet.packet_index,
        reduction_key: packet.reduction_key.clone(),
        result_identity,
        result,
        diagnostic_observations,
        preparation_failures: preparation_failures.clone(),
        counters: ValidationPreparationCounters {
            packet_count: 1,
            worker_result_count: 1,
            reducer_input_count: 0,
            reducer_conflict_count: 0,
            failure_count: preparation_failures.len(),
        },
    }
}

fn invariant_packet_failures(packet: &InvariantWorkPacket<'_>) -> Vec<PreparationFailureClass> {
    let mut failures = Vec::new();
    if packet.validity.context != packet.planning_context {
        failures.push(PreparationFailureClass::PlanningProofInsufficient);
    }
    if packet.planning_context.execution_point != packet.registration.execution_point
        || packet.planning_context.observation_kind != packet.observation.kind()
        || packet.locality.observation_scope != packet.observation.kind()
        || packet.locality.invariant_group_scope != packet.registration.rule.groups()
    {
        failures.push(PreparationFailureClass::PlanningProofInsufficient);
    }
    match packet.locality.partition_scope {
        PreparationPartitionScope::AllObserved
        | PreparationPartitionScope::TouchedPartitions(_) => {}
    }
    match packet.locality.read_set_approximation {
        PreparationReadSetApproximation::SharedCommittedRead
        | PreparationReadSetApproximation::TouchedOnly
        | PreparationReadSetApproximation::FullObservedScan => {}
    }
    match packet.locality.record_domain {
        PreparationRecordDomain::Mixed | PreparationRecordDomain::None => {}
        PreparationRecordDomain::Entity | PreparationRecordDomain::Relation => {}
    }
    match packet.proof_kind {
        PreparationProofKind::RequiresSerial => {
            if packet.locality.write_exclusion
                != PreparationWriteExclusionClass::RequiresSerialAuthority
            {
                failures.push(PreparationFailureClass::PublicationIsolationViolation);
            }
        }
        _ => {
            if packet.locality.write_exclusion != PreparationWriteExclusionClass::ReadOnly {
                failures.push(PreparationFailureClass::PublicationIsolationViolation);
            }
        }
    }
    failures
}
