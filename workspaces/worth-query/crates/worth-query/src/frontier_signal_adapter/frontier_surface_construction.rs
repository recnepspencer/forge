use crate::frontier_planning::FrontierSurfaceDigest;
use worth_signal::facade::adapters::{
    InvalidationExecutionSummary, InvalidationPerformedCounter, InvalidationPlanningEstimate,
    SignalInvalidationExecutionReceipt, SignalInvalidationRealizedCounters,
};

use super::frontier_surface_model::SignalFrontierSurfaceEvidence;

impl SignalFrontierSurfaceEvidence {
    pub fn from_planning_estimate(estimate: &InvalidationPlanningEstimate) -> Self {
        let parts = [
            format!("seed_count:{}", estimate.seed_count()),
            format!(
                "direct_candidate_count:{}",
                estimate.direct_candidate_count()
            ),
            format!(
                "partition_scoped_check_count:{}",
                estimate.partition_scoped_check_count()
            ),
        ];
        let predicted_breadth = estimate.seed_count()
            + estimate.direct_candidate_count()
            + estimate.partition_scoped_check_count();
        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            predicted_breadth as usize,
            None,
        )
    }

    pub fn from_execution_receipt(receipt: &SignalInvalidationExecutionReceipt) -> Self {
        Self::from_execution_summary(&receipt.summary())
    }

    pub fn from_execution_summary(summary: &InvalidationExecutionSummary) -> Self {
        Self::from_realized_counters(summary.realized_counters())
    }

    pub(crate) fn from_realized_counters(counters: &SignalInvalidationRealizedCounters) -> Self {
        let parts = InvalidationPerformedCounter::ALL
            .into_iter()
            .map(|counter| format!("{}:{}", counter.name(), counters.value(counter)))
            .collect::<Vec<_>>();
        let realized_breadth = counters.value(InvalidationPerformedCounter::NodesEvaluated)
            + counters.value(InvalidationPerformedCounter::WorkItemsAdmitted)
            + counters.value(InvalidationPerformedCounter::ReverseIndexCandidatesReturned);
        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            0,
            Some(realized_breadth as usize),
        )
    }
}
