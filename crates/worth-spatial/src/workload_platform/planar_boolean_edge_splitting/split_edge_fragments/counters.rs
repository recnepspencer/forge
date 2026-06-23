#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeFragmentCounters {
    schedules_inspected: usize,
    source_edges_covered: usize,
    split_vertices_consumed: usize,
    original_endpoint_boundaries_synthesized: usize,
    fragments_emitted: usize,
    interval_attributed_fragments: usize,
    endpoint_noop_boundaries_skipped: usize,
    collapsed_fragments_rejected: usize,
    coverage_gaps_rejected: usize,
    foreign_schedule_rows_rejected: usize,
}

impl PlanarBooleanSplitEdgeFragmentCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        schedules_inspected: usize,
        source_edges_covered: usize,
        split_vertices_consumed: usize,
        original_endpoint_boundaries_synthesized: usize,
        fragments_emitted: usize,
        interval_attributed_fragments: usize,
        endpoint_noop_boundaries_skipped: usize,
        collapsed_fragments_rejected: usize,
        coverage_gaps_rejected: usize,
        foreign_schedule_rows_rejected: usize,
    ) -> Self {
        Self {
            schedules_inspected,
            source_edges_covered,
            split_vertices_consumed,
            original_endpoint_boundaries_synthesized,
            fragments_emitted,
            interval_attributed_fragments,
            endpoint_noop_boundaries_skipped,
            collapsed_fragments_rejected,
            coverage_gaps_rejected,
            foreign_schedule_rows_rejected,
        }
    }

    pub fn schedules_inspected(self) -> usize {
        self.schedules_inspected
    }
    pub fn source_edges_covered(self) -> usize {
        self.source_edges_covered
    }
    pub fn split_vertices_consumed(self) -> usize {
        self.split_vertices_consumed
    }
    pub fn original_endpoint_boundaries_synthesized(self) -> usize {
        self.original_endpoint_boundaries_synthesized
    }
    pub fn fragments_emitted(self) -> usize {
        self.fragments_emitted
    }
    pub fn interval_attributed_fragments(self) -> usize {
        self.interval_attributed_fragments
    }
    pub fn endpoint_noop_boundaries_skipped(self) -> usize {
        self.endpoint_noop_boundaries_skipped
    }
    pub fn collapsed_fragments_rejected(self) -> usize {
        self.collapsed_fragments_rejected
    }
    pub fn coverage_gaps_rejected(self) -> usize {
        self.coverage_gaps_rejected
    }
    pub fn foreign_schedule_rows_rejected(self) -> usize {
        self.foreign_schedule_rows_rejected
    }
}
