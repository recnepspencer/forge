use forge_query::facade::{ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryReadReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologyReadGraphAccessProof {
    pub(crate) admission_posture: ForgeQueryGraphReadAccessAdmissionPosture,
    pub(crate) plan_digest: String,
    pub(crate) admission_digest: String,
    pub(crate) requirement_set_digest: String,
    pub(crate) cost_estimate_digest: String,
    pub(crate) budget_digest: String,
    pub(crate) graph_index_inventory_match_report_digest: String,
    pub(crate) planned_access_step_count: usize,
    pub(crate) consumed_access_step_count: usize,
    pub(crate) executor_entry_count: usize,
    pub(crate) executor_strategy_rediscovery_count: usize,
    pub(crate) edge_scan_execution_count: usize,
    pub(crate) per_result_neighbor_lookup_count: usize,
    pub(crate) persistent_artifact_bypass_count: usize,
    pub(crate) adjacency_buffer_build_count: usize,
    pub(crate) frontier_buffer_build_count: usize,
    pub(crate) visited_buffer_build_count: usize,
    pub(crate) result_buffer_build_count: usize,
}

impl TopologyReadGraphAccessProof {
    pub(crate) fn from_receipt(receipt: &ForgeQueryReadReceipt) -> Option<Self> {
        let summary = receipt.graph_read_access_summary()?;
        let counters = receipt.graph_read_access_complexity_counters()?;
        let authority_counters = receipt
            .graph_read_access_admission()?
            .authority_receipt()
            .counters();
        Some(Self {
            admission_posture: summary.admission_posture().clone(),
            plan_digest: summary.plan_digest().to_string(),
            admission_digest: summary.admission_digest().to_string(),
            requirement_set_digest: summary.requirement_set_digest().to_string(),
            cost_estimate_digest: summary.cost_estimate_digest().to_string(),
            budget_digest: summary.budget_digest().to_string(),
            graph_index_inventory_match_report_digest: summary
                .graph_index_inventory_match_report_digest()
                .to_string(),
            planned_access_step_count: counters.planned_access_step_count(),
            consumed_access_step_count: counters.consumed_access_step_count(),
            executor_entry_count: counters.executor_entry_count(),
            executor_strategy_rediscovery_count: counters.executor_strategy_rediscovery_count(),
            edge_scan_execution_count: counters.edge_scan_execution_count(),
            per_result_neighbor_lookup_count: counters.per_result_neighbor_lookup_count(),
            persistent_artifact_bypass_count: counters.persistent_artifact_bypass_count(),
            adjacency_buffer_build_count: authority_counters.adjacency_buffer_build_count(),
            frontier_buffer_build_count: authority_counters.frontier_buffer_build_count(),
            visited_buffer_build_count: authority_counters.visited_buffer_build_count(),
            result_buffer_build_count: authority_counters.result_buffer_build_count(),
        })
    }

    pub fn no_caller_owned_graph_work(&self) -> bool {
        self.executor_strategy_rediscovery_count == 0
            && self.edge_scan_execution_count == 0
            && self.per_result_neighbor_lookup_count == 0
            && self.persistent_artifact_bypass_count == 0
            && self.adjacency_buffer_build_count == 0
            && self.frontier_buffer_build_count == 0
            && self.visited_buffer_build_count == 0
            && self.result_buffer_build_count == 0
    }
}
