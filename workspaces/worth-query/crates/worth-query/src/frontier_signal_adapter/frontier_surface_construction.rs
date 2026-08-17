use crate::frontier_planning::FrontierSurfaceDigest;
use worth_signal::facade::adapters::{
    InvalidationPerformedCounter, InvalidationPlanningEstimate, SignalInvalidationExecutionReceipt,
};

use super::frontier_surface_model::SignalFrontierSurfaceEvidence;

impl SignalFrontierSurfaceEvidence {
    /// Project only Signal's caller-visible pre-execution estimate.
    pub fn from_invalidation_planning_estimate(estimate: &InvalidationPlanningEstimate) -> Self {
        let seed_count = estimate.seed_count();
        let direct_candidate_count = estimate.direct_candidate_count();
        let partition_scoped_check_count = estimate.partition_scoped_check_count();
        let parts = [
            format!("seed_count:{seed_count}"),
            format!("direct_candidate_count:{direct_candidate_count}"),
            format!("partition_scoped_check_count:{partition_scoped_check_count}"),
        ];

        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            usize::try_from(seed_count.saturating_add(direct_candidate_count))
                .unwrap_or(usize::MAX),
            None,
        )
    }

    /// Project only counters sealed by Signal after performed execution.
    pub fn from_invalidation_execution_receipt(
        receipt: &SignalInvalidationExecutionReceipt,
    ) -> Self {
        let counters = receipt.realized_counters();
        let parts = InvalidationPerformedCounter::ALL
            .into_iter()
            .map(|counter| format!("{}:{}", counter.name(), counters.value(counter)))
            .collect::<Vec<_>>();
        let predicted_breadth = counters
            .work_items_admitted()
            .saturating_add(counters.work_items_merged());
        let realized_breadth = counters
            .ready_items_popped()
            .saturating_add(counters.retained_ready_frontier_width());

        Self::from_materialized_surface(
            FrontierSurfaceDigest::from_label(&parts.join("|")),
            usize::try_from(predicted_breadth).unwrap_or(usize::MAX),
            Some(usize::try_from(realized_breadth).unwrap_or(usize::MAX)),
        )
    }
}
