use worth_proof::TransitionOutcome;

use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::proof::invalidation::progression::{
    InvalidationProgressionOwner, ReadyInvalidationBatch,
};
use crate::data::telemetry::InvalidationPerformedCounter;

pub(crate) fn execute_ready<Outcome>(
    graph: &SignalGraph,
    ready: ReadyInvalidationBatch,
    effect: impl FnOnce() -> Result<Outcome, SignalError>,
) -> Result<Outcome, SignalError> {
    if let Err(error) = super::readiness::ensure_ready_is_current(graph, &ready) {
        graph
            .invalidation_performed_counter_state()
            .add(InvalidationPerformedCounter::StaleWorkRejected, 1);
        return Err(error);
    }
    let executed_binding = InvalidationProgressionOwner::ready_binding(&ready).clone();
    match InvalidationProgressionOwner::execute(ready, |_| effect()) {
        TransitionOutcome::Success(executed) => {
            graph
                .invalidation_performed_counter_state()
                .record_executed_work(executed_binding);
            graph
                .invalidation_performed_counter_state()
                .add(InvalidationPerformedCounter::NodesEvaluated, 1);
            Ok(InvalidationProgressionOwner::into_executed_outcome(
                executed,
            ))
        }
        TransitionOutcome::Failed(error) => Err(error),
        TransitionOutcome::Denied(never)
        | TransitionOutcome::Deferred(never)
        | TransitionOutcome::Stale(never)
        | TransitionOutcome::RebindRequired(never) => match never {},
    }
}
