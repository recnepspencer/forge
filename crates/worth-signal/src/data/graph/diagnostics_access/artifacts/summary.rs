use crate::data::graph::signal_graph::SignalGraph;
use crate::diagnostics::policy::OrdinaryAccessLane;
use crate::diagnostics::summary::{ExecutionHistorySummary, GraphSummary};

impl SignalGraph {
    pub(super) fn refresh_snapshot_summaries(&mut self) {
        let retention_budget = self.runtime_policy().retention_budget;
        let profile = self.diagnostics_profile();
        let history = ExecutionHistorySummary::from_graph(
            self,
            profile,
            retention_budget.detail_limit,
            retention_budget.retain_history_details,
            OrdinaryAccessLane,
        );
        let graph_summary = GraphSummary::from_graph(
            self,
            profile,
            retention_budget.detail_limit,
            OrdinaryAccessLane,
        );
        self.diagnostics_state_mut()
            .refresh_retained_views(history, graph_summary);
    }

    #[cfg(test)]
    pub(crate) fn test_storage_counts(&self) -> ((usize, usize), (usize, usize), usize) {
        (
            self.topology.dependency_edges.storage_counts(),
            self.topology.subscriber_edges.storage_counts(),
            self.topology.dependency_snapshots.snapshot_count(),
        )
    }
}
