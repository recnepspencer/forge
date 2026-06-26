use crate::graph_read_access_plan_adoption::WorthGraphReadAccessSliceReceiptProjection;

use super::super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessCallerOwnedWorkBreakdown {
    strategy_rediscovery_count: usize,
    edge_scan_count: usize,
    per_result_neighbor_lookup_count: usize,
    persistent_artifact_bypass_count: usize,
    scalarized_caller_loop_count: usize,
    missing_counter_for_claimed_execution_count: usize,
    total_count: usize,
    breakdown_digest: String,
}

impl WorthGraphReadAccessCallerOwnedWorkBreakdown {
    pub(crate) fn from_phase_four_receipt(
        receipt: &WorthGraphReadAccessSliceReceiptProjection,
    ) -> Self {
        Self::new(
            receipt.local_strategy_recompute_count(),
            receipt.local_edge_scan_count(),
            receipt.local_neighbor_lookup_count(),
            receipt.persistent_artifact_bypass_count(),
            0,
            0,
        )
    }

    pub(crate) fn from_counts(
        scalarized_caller_loop_count: usize,
        missing_counter_for_claimed_execution_count: usize,
    ) -> Self {
        Self::new(
            0,
            0,
            0,
            0,
            scalarized_caller_loop_count,
            missing_counter_for_claimed_execution_count,
        )
    }

    fn new(
        strategy_rediscovery_count: usize,
        edge_scan_count: usize,
        per_result_neighbor_lookup_count: usize,
        persistent_artifact_bypass_count: usize,
        scalarized_caller_loop_count: usize,
        missing_counter_for_claimed_execution_count: usize,
    ) -> Self {
        let total_count = strategy_rediscovery_count
            + edge_scan_count
            + per_result_neighbor_lookup_count
            + persistent_artifact_bypass_count
            + scalarized_caller_loop_count
            + missing_counter_for_claimed_execution_count;
        let breakdown_digest = stable_digest(&[
            "worth_graph_read_access_caller_owned_work_breakdown_v1".to_string(),
            format!("strategy:{strategy_rediscovery_count}"),
            format!("edge_scan:{edge_scan_count}"),
            format!("neighbor_lookup:{per_result_neighbor_lookup_count}"),
            format!("persistent_bypass:{persistent_artifact_bypass_count}"),
            format!("scalarized:{scalarized_caller_loop_count}"),
            format!("missing_counter:{missing_counter_for_claimed_execution_count}"),
            format!("total:{total_count}"),
        ]);
        Self {
            strategy_rediscovery_count,
            edge_scan_count,
            per_result_neighbor_lookup_count,
            persistent_artifact_bypass_count,
            scalarized_caller_loop_count,
            missing_counter_for_claimed_execution_count,
            total_count,
            breakdown_digest,
        }
    }

    pub const fn strategy_rediscovery_count(&self) -> usize {
        self.strategy_rediscovery_count
    }

    pub const fn edge_scan_count(&self) -> usize {
        self.edge_scan_count
    }

    pub const fn per_result_neighbor_lookup_count(&self) -> usize {
        self.per_result_neighbor_lookup_count
    }

    pub const fn persistent_artifact_bypass_count(&self) -> usize {
        self.persistent_artifact_bypass_count
    }

    pub const fn scalarized_caller_loop_count(&self) -> usize {
        self.scalarized_caller_loop_count
    }

    pub const fn missing_counter_for_claimed_execution_count(&self) -> usize {
        self.missing_counter_for_claimed_execution_count
    }

    pub const fn total_count(&self) -> usize {
        self.total_count
    }

    pub fn breakdown_digest(&self) -> &str {
        &self.breakdown_digest
    }
}
