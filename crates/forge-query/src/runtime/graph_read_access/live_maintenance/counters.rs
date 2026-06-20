use crate::identity::hash_parts;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLiveGraphReadMaintenanceCounters {
    mutation_delta_count: usize,
    affected_requirement_row_count: usize,
    touched_edge_count: usize,
    touched_frontier_count: usize,
    index_update_count: usize,
    live_view_update_count: usize,
    per_result_neighbor_lookup_count: usize,
    skipped_unaffected_requirement_count: usize,
    strategy_recompute_count: usize,
    background_index_build_count: usize,
}

impl ForgeQueryLiveGraphReadMaintenanceCounters {
    pub(crate) fn observed_live_read(
        affected_requirement_row_count: usize,
        live_view_update_count: usize,
    ) -> Self {
        ForgeQueryLiveGraphReadMaintenanceCounterRecorder::live_read()
            .observe_affected_requirement_rows(affected_requirement_row_count)
            .observe_touched_edges(affected_requirement_row_count)
            .observe_touched_frontier_count(usize::from(affected_requirement_row_count > 0))
            .observe_index_updates(affected_requirement_row_count)
            .observe_live_view_updates(live_view_update_count)
            .finish()
    }

    pub(crate) fn observed_mutation_delivery(
        affected_requirement_row_count: usize,
        maintenance_delta_width: usize,
        patch_group_width: usize,
        live_view_update_count: usize,
    ) -> Self {
        let touched_edge_count = maintenance_delta_width.min(affected_requirement_row_count);
        let skipped_unaffected_requirement_count =
            affected_requirement_row_count.saturating_sub(touched_edge_count);
        ForgeQueryLiveGraphReadMaintenanceCounterRecorder::mutation_delivery()
            .observe_affected_requirement_rows(affected_requirement_row_count)
            .observe_touched_edges(touched_edge_count)
            .observe_touched_frontier_count(usize::from(touched_edge_count > 0))
            .observe_index_updates(patch_group_width)
            .observe_live_view_updates(live_view_update_count)
            .observe_skipped_unaffected_requirements(skipped_unaffected_requirement_count)
            .finish()
    }

    pub fn mutation_delta_count(&self) -> usize {
        self.mutation_delta_count
    }

    pub fn affected_requirement_row_count(&self) -> usize {
        self.affected_requirement_row_count
    }

    pub fn touched_edge_count(&self) -> usize {
        self.touched_edge_count
    }

    pub fn touched_frontier_count(&self) -> usize {
        self.touched_frontier_count
    }

    pub fn index_update_count(&self) -> usize {
        self.index_update_count
    }

    pub fn live_view_update_count(&self) -> usize {
        self.live_view_update_count
    }

    pub fn per_result_neighbor_lookup_count(&self) -> usize {
        self.per_result_neighbor_lookup_count
    }

    pub fn skipped_unaffected_requirement_count(&self) -> usize {
        self.skipped_unaffected_requirement_count
    }

    pub fn strategy_recompute_count(&self) -> usize {
        self.strategy_recompute_count
    }

    pub fn background_index_build_count(&self) -> usize {
        self.background_index_build_count
    }

    pub(crate) fn digest(&self) -> String {
        hash_parts(&[
            "forge_query_live_graph_read_maintenance_counters_v1".to_string(),
            format!("mutation_delta:{}", self.mutation_delta_count),
            format!(
                "affected_requirement_row:{}",
                self.affected_requirement_row_count
            ),
            format!("touched_edge:{}", self.touched_edge_count),
            format!("touched_frontier:{}", self.touched_frontier_count),
            format!("index_update:{}", self.index_update_count),
            format!("live_view_update:{}", self.live_view_update_count),
            format!(
                "per_result_neighbor_lookup:{}",
                self.per_result_neighbor_lookup_count
            ),
            format!(
                "skipped_unaffected_requirement:{}",
                self.skipped_unaffected_requirement_count
            ),
            format!("strategy_recompute:{}", self.strategy_recompute_count),
            format!(
                "background_index_build:{}",
                self.background_index_build_count
            ),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ForgeQueryLiveGraphReadMaintenanceCounterRecorder {
    mutation_delta_count: usize,
    affected_requirement_row_count: usize,
    touched_edge_count: usize,
    touched_frontier_count: usize,
    index_update_count: usize,
    live_view_update_count: usize,
    per_result_neighbor_lookup_count: usize,
    skipped_unaffected_requirement_count: usize,
    strategy_recompute_count: usize,
    background_index_build_count: usize,
}

impl ForgeQueryLiveGraphReadMaintenanceCounterRecorder {
    pub(crate) fn live_read() -> Self {
        Self::new(0)
    }

    pub(crate) fn mutation_delivery() -> Self {
        Self::new(1)
    }

    fn new(mutation_delta_count: usize) -> Self {
        Self {
            mutation_delta_count,
            affected_requirement_row_count: 0,
            touched_edge_count: 0,
            touched_frontier_count: 0,
            index_update_count: 0,
            live_view_update_count: 0,
            per_result_neighbor_lookup_count: 0,
            skipped_unaffected_requirement_count: 0,
            strategy_recompute_count: 0,
            background_index_build_count: 0,
        }
    }

    pub(crate) fn observe_affected_requirement_rows(mut self, count: usize) -> Self {
        self.affected_requirement_row_count = count;
        self
    }

    pub(crate) fn observe_touched_edges(mut self, count: usize) -> Self {
        self.touched_edge_count = count;
        self
    }

    pub(crate) fn observe_touched_frontier_count(mut self, count: usize) -> Self {
        self.touched_frontier_count = count;
        self
    }

    pub(crate) fn observe_index_updates(mut self, count: usize) -> Self {
        self.index_update_count = count;
        self
    }

    pub(crate) fn observe_live_view_updates(mut self, count: usize) -> Self {
        self.live_view_update_count = count;
        self
    }

    pub(crate) fn observe_skipped_unaffected_requirements(mut self, count: usize) -> Self {
        self.skipped_unaffected_requirement_count = count;
        self
    }

    pub(crate) fn finish(self) -> ForgeQueryLiveGraphReadMaintenanceCounters {
        ForgeQueryLiveGraphReadMaintenanceCounters {
            mutation_delta_count: self.mutation_delta_count,
            affected_requirement_row_count: self.affected_requirement_row_count,
            touched_edge_count: self.touched_edge_count,
            touched_frontier_count: self.touched_frontier_count,
            index_update_count: self.index_update_count,
            live_view_update_count: self.live_view_update_count,
            per_result_neighbor_lookup_count: self.per_result_neighbor_lookup_count,
            skipped_unaffected_requirement_count: self.skipped_unaffected_requirement_count,
            strategy_recompute_count: self.strategy_recompute_count,
            background_index_build_count: self.background_index_build_count,
        }
    }
}
