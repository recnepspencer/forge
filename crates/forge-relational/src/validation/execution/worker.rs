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
#[cfg(test)]
use crate::validation::data::InvariantRule;
use crate::validation::data::{
    CustomInvariantExecutionContext, CustomInvariantFailure, CustomInvariantFailureKind,
    CustomInvariantRuntimePhase, InvariantCheckResult, InvariantReportedRule, InvariantVerdict,
    InvariantWitnessKey,
};
use crate::validation::engine::context::InvariantExecutionContext;
use crate::validation::engine::evaluator::evaluate_rule;

use super::envelope::{InvariantWorkerEnvelope, InvariantWorkerResult};
use super::packets::InvariantWorkPacket;

pub(crate) fn evaluate_invariant_packet(
    runtime: &RelationalRuntime,
    packet: &InvariantWorkPacket<'_>,
) -> InvariantWorkerEnvelope {
    #[allow(unused_mut)]
    let mut preparation_failures = invariant_packet_failures(packet);
    #[cfg(test)]
    if matches!(
        packet.injected_test_fault,
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
        preparation_failures.is_empty() || packet.injected_test_fault.is_some(),
        "planned invariant packet violated preparation proof contract: {:?}",
        preparation_failures
    );
    let context = InvariantExecutionContext::new(
        runtime,
        packet.observation.clone(),
        packet.version_id,
        packet.merged_plan,
        packet.relation_integrity_scopes.clone(),
    );
    let (reported_rule, groups, cost, custom_provenance, verdicts) = match &packet.registration {
        crate::authority::commit::preparation::packets::invariant::InvariantPacketRegistration::Native(
            registration,
        ) => {
            let violations = evaluate_rule(
                &context,
                registration.execution_point.class(),
                &registration.rule,
            );
            let verdicts = if violations.is_empty() {
                vec![InvariantVerdict::Pass]
            } else {
                violations
                    .into_iter()
                    .map(|violation| registration.verdict_for_violation(violation))
                    .collect()
            };
            (
                InvariantReportedRule::Native(registration.rule.clone()),
                registration.groups(),
                registration.cost(),
                None,
                verdicts,
            )
        }
        crate::authority::commit::preparation::packets::invariant::InvariantPacketRegistration::Custom {
            registration,
            prepared_execution,
        } => {
            let custom_context = CustomInvariantExecutionContext::new(
                runtime,
                packet.observation,
                packet.version_id,
                packet.merged_plan,
            );
            let verdicts = match prepared_execution.evaluate(&custom_context) {
                crate::validation::data::PreparedCustomInvariantExecutionOutcome::Verdict(
                    crate::validation::data::CustomInvariantVerdict::Pass,
                ) => vec![InvariantVerdict::Pass],
                crate::validation::data::PreparedCustomInvariantExecutionOutcome::Verdict(
                    crate::validation::data::CustomInvariantVerdict::Violation,
                ) => vec![InvariantVerdict::Violation(crate::validation::data::InvariantViolation {
                    class: registration.execution_point().class(),
                    code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
                    detail: format!(
                        "custom invariant '{}' reported a structural violation",
                        registration.rule_id().as_str()
                    ),
                    fields: crate::validation::data::InvariantViolationFields::None,
                })],
                crate::validation::data::PreparedCustomInvariantExecutionOutcome::Failure(
                    failure,
                ) => {
                    if failure.kind == CustomInvariantFailureKind::Panic
                        && failure.phase == CustomInvariantRuntimePhase::Execution
                    {
                        runtime.performance_access().count_custom_invariant_panic();
                    }
                    vec![InvariantVerdict::Violation(custom_invariant_failure_violation(
                        registration.execution_point().class(),
                        &failure,
                    ))]
                }
            };
            (
                InvariantReportedRule::Custom(registration.descriptor().identity.clone()),
                registration.groups(),
                registration.cost_class(),
                Some(custom_context.provenance()),
                verdicts,
            )
        }
    };
    let mut results = Vec::with_capacity(verdicts.len());
    let mut diagnostic_observations = Vec::new();
    for (_index, verdict) in verdicts.into_iter().enumerate() {
        let witness = match &verdict {
            InvariantVerdict::Pass => InvariantWitnessKey::pass(),
            InvariantVerdict::Advisory { violation, .. } | InvariantVerdict::Violation(violation) => {
                violation.witness_key()
            }
        };
        let result = InvariantCheckResult {
            execution_point: packet.registration.execution_point(),
            failure_effect: packet.registration.failure_effect(),
            rule: reported_rule.clone(),
            witness: witness.clone(),
            groups,
            cost,
            custom_provenance: custom_provenance.clone(),
            verdict,
        };
        #[allow(unused_mut)]
        let mut result_identity = ValidationResultIdentity::from_parts(
            result.execution_point,
            result.failure_effect,
            reported_rule.clone(),
            witness,
        );
        #[cfg(test)]
        if _index == 0
            && matches!(
                packet.injected_test_fault,
                Some(crate::validation::execution::TestPreparationFault::ReductionIdentityConflict)
            )
        {
            result_identity = ValidationResultIdentity {
                execution_point: result.execution_point,
                failure_effect: result.failure_effect,
                rule: InvariantReportedRule::Native(InvariantRule::MaxMergedIntents(16)),
                witness: InvariantWitnessKey::new("reduction-conflict"),
            };
        }
        if !matches!(result.verdict, InvariantVerdict::Pass) {
            diagnostic_observations.push(ValidationDiagnosticObservation {
                packet_index: packet.packet_index,
                result_identity: result_identity.clone(),
            });
        }
        results.push(InvariantWorkerResult {
            result_identity,
            result,
        });
    }

    let worker_result_count = results.len();
    InvariantWorkerEnvelope {
        packet_index: packet.packet_index,
        reduction_key: packet.reduction_key.clone(),
        results,
        diagnostic_observations,
        preparation_failures: preparation_failures.clone(),
        counters: ValidationPreparationCounters {
            packet_count: 1,
            worker_result_count,
            reducer_input_count: 0,
            reducer_conflict_count: 0,
            failure_count: preparation_failures.len(),
        },
    }
}

fn custom_invariant_failure_violation(
    class: crate::validation::data::InvariantClass,
    failure: &CustomInvariantFailure,
) -> crate::validation::data::InvariantViolation {
    let phase = match failure.phase {
        CustomInvariantRuntimePhase::Preparation => "preparation",
        CustomInvariantRuntimePhase::Execution => "execution",
    };
    let failure_kind = match failure.kind {
        CustomInvariantFailureKind::PreparationError => "preparation_error",
        CustomInvariantFailureKind::ExecutionError => "execution_error",
        CustomInvariantFailureKind::Panic => "panic",
    };
    crate::validation::data::InvariantViolation {
        class,
        code: crate::diagnostics::data::DiagnosticCode::InvariantViolation,
        detail: format!(
            "custom invariant '{}' failed during {}: {}",
            failure.identity.rule_id.as_str(),
            phase,
            failure.detail
        ),
        fields: crate::validation::data::InvariantViolationFields::CustomInvariantFailure {
            rule_id: failure.identity.rule_id.as_str().to_string(),
            semantic_version_major: failure.identity.semantic_version.major,
            semantic_version_minor: failure.identity.semantic_version.minor,
            phase: phase.to_string(),
            failure_kind: failure_kind.to_string(),
            detail: failure.detail.to_string(),
        },
    }
}

fn invariant_packet_failures(packet: &InvariantWorkPacket<'_>) -> Vec<PreparationFailureClass> {
    let mut failures = Vec::new();
    if packet.validity.context != packet.planning_context {
        failures.push(PreparationFailureClass::PlanningProofInsufficient);
    }
    if packet.planning_context.execution_point != packet.registration.execution_point()
        || packet.planning_context.observation_kind != packet.observation.kind()
        || packet.locality.observation_scope != packet.observation.kind()
        || packet.locality.invariant_group_scope != packet.registration.groups()
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
