use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::diagnostics::observations::ValidationDiagnosticObservation;
use crate::authority::commit::preparation::reduction::identity::ValidationResultIdentity;
use crate::logic::runtime::RelationalRuntime;
use crate::validation::data::{InvariantCheckResult, InvariantVerdict};
use crate::validation::engine::context::InvariantExecutionContext;
use crate::validation::engine::evaluator::evaluate_rule;

use super::envelope::InvariantWorkerEnvelope;
use super::packets::InvariantWorkPacket;

pub(crate) fn evaluate_invariant_packet(
    runtime: &RelationalRuntime,
    packet: &InvariantWorkPacket<'_>,
) -> InvariantWorkerEnvelope {
    let context = InvariantExecutionContext::new(
        runtime,
        packet.observation.clone(),
        packet.version_id,
        packet.registration.execution_point,
        packet.merged_plan,
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
    let result_identity = ValidationResultIdentity::from_parts(
        result.execution_point,
        result.failure_effect,
        result.rule.clone(),
        &result.verdict,
    );
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
        counters: ValidationPreparationCounters {
            packet_count: 1,
            worker_result_count: 1,
            reducer_input_count: 0,
            reducer_conflict_count: 0,
        },
    }
}
