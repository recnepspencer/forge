use super::PerformanceAccess;

impl PerformanceAccess<'_> {
    #[cfg(test)]
    pub(crate) fn count_lineage_graph_snapshot_request(
        &self,
        node_count: usize,
        event_count: usize,
        candidate_count: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_graph_snapshot_requests += 1;
            counters.lineage_graph_snapshot_nodes_materialized += node_count;
            counters.lineage_graph_snapshot_events_materialized += event_count;
            counters.lineage_graph_snapshot_candidates_materialized += candidate_count;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_candidate_validation(
        &self,
        recorded_width: usize,
        validated_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_recorded_candidate_width += recorded_width;
            counters.lineage_validated_candidate_width += validated_width;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_promotion_rejection(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_rejection_count += 1;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_promotion_plan_lowering(&self, promotion_eligible_width: usize) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_eligible_candidate_width += promotion_eligible_width;
        });
    }

    pub(crate) fn count_lineage_finalization(
        &self,
        event_batch_width: usize,
        decision_log_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_finalization_event_batch_width += event_batch_width;
            counters.lineage_finalization_decision_log_width += decision_log_width;
        });
    }

    pub(crate) fn count_lineage_publication_artifact(
        &self,
        event_width: usize,
        decision_width: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_publication_event_width += event_width;
            counters.lineage_publication_decision_width += decision_width;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_promotion_accepted(&self) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_promotion_accepted_count += 1;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_graph_snapshot_visibility_cache(&self, hit: bool) {
        self.runtime.services.instrumentation.count(|counters| {
            if hit {
                counters.lineage_graph_snapshot_visibility_cache_hits += 1;
            } else {
                counters.lineage_graph_snapshot_visibility_cache_miss_reconstructions += 1;
            }
        });
    }

    pub(crate) fn count_lineage_historical_resolution(
        &self,
        branch_event_scans: usize,
        traversed_events: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_historical_resolution_requests += 1;
            counters.lineage_historical_resolution_branch_event_scans += branch_event_scans;
            counters.lineage_historical_resolution_traversed_events += traversed_events;
        });
    }

    #[cfg(test)]
    pub(crate) fn count_lineage_branch_divergence(
        &self,
        left_event_count: usize,
        right_event_count: usize,
        left_node_count: usize,
        right_node_count: usize,
    ) {
        self.runtime.services.instrumentation.count(|counters| {
            counters.lineage_branch_divergence_requests += 1;
            counters.lineage_branch_divergence_event_scans += left_event_count + right_event_count;
            counters.lineage_branch_divergence_node_scans += left_node_count + right_node_count;
        });
    }
}
