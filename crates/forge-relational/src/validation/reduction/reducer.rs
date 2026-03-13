use crate::authority::commit::preparation::diagnostics::counters::ValidationPreparationCounters;
use crate::authority::commit::preparation::planning::strategy::PreparationStrategy;
use crate::authority::commit::preparation::reduction::merge::canonical_merge_indices;
use crate::validation::data::InvariantCheckResult;
use crate::validation::engine::{
    InvariantExecutionDisposition, InvariantExecutionMetadata, InvariantExecutionRequest,
    InvariantExecutionResult,
};

use super::diagnostics::sort_diagnostic_observations;
use crate::validation::execution::{InvariantWorkerEnvelope, ValidationReducerConflict};

pub(crate) fn reduce_invariant_execution(
    request: &InvariantExecutionRequest<'_>,
    strategy: PreparationStrategy,
    mut envelopes: Vec<InvariantWorkerEnvelope>,
) -> (InvariantExecutionResult, ValidationPreparationCounters, Vec<ValidationReducerConflict>) {
    canonical_merge_indices(&mut envelopes, |left, right| {
        left.reduction_key
            .cmp(&right.reduction_key)
            .then_with(|| left.result_identity.cmp(&right.result_identity))
    });

    let mut reducer_conflicts = Vec::new();
    let mut diagnostics = envelopes
        .iter()
        .flat_map(|envelope| envelope.diagnostic_observations.clone())
        .collect::<Vec<_>>();
    sort_diagnostic_observations(&mut diagnostics);

    let mut results = Vec::with_capacity(envelopes.len());
    let mut last_identity = None;
    for envelope in envelopes {
        if let Some(previous_identity) = &last_identity {
            if previous_identity == &envelope.result_identity {
                reducer_conflicts.push(ValidationReducerConflict {
                    identity: envelope.result_identity.clone(),
                });
            }
        }
        last_identity = Some(envelope.result_identity.clone());
        results.push(envelope.result);
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
    );
    let result = InvariantExecutionResult::executed(metadata, results.clone());
    let counters = ValidationPreparationCounters {
        packet_count: results.len(),
        worker_result_count: results.len(),
        reducer_input_count: results.len(),
        reducer_conflict_count: reducer_conflicts.len(),
    };
    (result, counters, reducer_conflicts)
}
