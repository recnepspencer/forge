use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::diagnostics::failures::PreparationFailureClass;
use crate::authority::commit::preparation::planning::strategy::PreparationStrategy;
use crate::authority::commit::preparation::reduction::merge::{
    canonical_merge_streams, OrderedReductionStream,
};
use crate::validation::engine::{
    InvariantExecutionMetadata, InvariantExecutionRequest, InvariantExecutionResult,
    InvariantProofBoundarySummary,
};

use super::diagnostics::assert_canonical_diagnostic_observations;
use crate::validation::execution::{InvariantWorkerEnvelope, ValidationReducerConflict};

pub(crate) fn reduce_invariant_execution(
    request: &InvariantExecutionRequest<'_>,
    strategy: PreparationStrategy,
    proof_boundary: InvariantProofBoundarySummary,
    envelopes: Vec<InvariantWorkerEnvelope>,
) -> (
    InvariantExecutionResult,
    ValidationPreparationCounters,
    Vec<ValidationReducerConflict>,
) {
    let packet_count = envelopes.len();
    let envelopes = canonical_merge_streams(
        envelopes
            .into_iter()
            .map(|envelope| {
                let first_identity = envelope
                    .results
                    .first()
                    .map(|result| result.result_identity.clone())
                    .expect("worker envelopes must contain at least one result");
                OrderedReductionStream::singleton(
                    (
                        envelope.reduction_key.clone(),
                        first_identity,
                    ),
                    envelope,
                )
            })
            .collect(),
    )
    .into_iter()
    .map(|(_, envelope)| envelope)
    .collect::<Vec<_>>();

    let mut reducer_conflicts = Vec::new();
    let mut diagnostics = envelopes
        .iter()
        .flat_map(|envelope| envelope.diagnostic_observations.clone())
        .collect::<Vec<_>>();
    diagnostics.sort_by(|left, right| left.canonical_key().cmp(&right.canonical_key()));
    assert_canonical_diagnostic_observations(&diagnostics);
    let mut preparation_failures = envelopes
        .iter()
        .flat_map(|envelope| envelope.preparation_failures.clone())
        .collect::<Vec<_>>();
    if strategy.fallback_reason.is_some() {
        preparation_failures.push(PreparationFailureClass::FallbackToSerial);
    }

    let mut results = Vec::new();
    let mut last_identity = None;
    for envelope in envelopes {
        for worker_result in envelope.results {
            if let Some(previous_identity) = &last_identity {
                if previous_identity == &worker_result.result_identity {
                    reducer_conflicts.push(ValidationReducerConflict {
                        identity: worker_result.result_identity.clone(),
                    });
                    preparation_failures.push(PreparationFailureClass::ReductionIdentityConflict);
                }
            }
            last_identity = Some(worker_result.result_identity.clone());
            results.push(worker_result.result);
        }
    }

    let metadata = InvariantExecutionMetadata::executed_with_strategy(
        request.execution_point(),
        request.observation().kind(),
        request.version_id(),
        request.current_version_id(),
        request.consumed_groups(),
        request.applicable_groups(),
        request.max_cost(),
        request.plan_contract(),
        request.merged_plan().is_some(),
        strategy,
        preparation_failures.clone(),
        Some(proof_boundary),
    );
    let result = InvariantExecutionResult::executed(metadata, results.clone());
    let counters = ValidationPreparationCounters {
        packet_count,
        worker_result_count: results.len(),
        reducer_input_count: results.len(),
        reducer_conflict_count: reducer_conflicts.len(),
        failure_count: preparation_failures.len(),
    };
    (result, counters, reducer_conflicts)
}
