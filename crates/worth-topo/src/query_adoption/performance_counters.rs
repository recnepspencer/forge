use crate::projection::runtime_boundary::query_runtime::{
    TopologyQueryMutationLaneExecutionShape, TopologyRuntimeSupport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopoPhaseEightPerformanceCounters {
    runtime_posture_row_count: usize,
    query_read_family_touched_scope_count: usize,
    snapshot_read_family_touched_scope_count: usize,
    query_mutation_family_touched_scope_count: usize,
    query_mutation_lane_touched_scope_count: usize,
    graph_composed_mutation_lane_count: usize,
    runtime_closeout_row_count: usize,
}

impl WorthTopoPhaseEightPerformanceCounters {
    pub const fn runtime_posture_row_count(&self) -> usize {
        self.runtime_posture_row_count
    }

    pub const fn query_read_family_touched_scope_count(&self) -> usize {
        self.query_read_family_touched_scope_count
    }

    pub const fn snapshot_read_family_touched_scope_count(&self) -> usize {
        self.snapshot_read_family_touched_scope_count
    }

    pub const fn query_mutation_family_touched_scope_count(&self) -> usize {
        self.query_mutation_family_touched_scope_count
    }

    pub const fn query_mutation_lane_touched_scope_count(&self) -> usize {
        self.query_mutation_lane_touched_scope_count
    }

    pub const fn graph_composed_mutation_lane_count(&self) -> usize {
        self.graph_composed_mutation_lane_count
    }

    pub const fn runtime_closeout_row_count(&self) -> usize {
        self.runtime_closeout_row_count
    }
}

pub fn current_topology_phase_eight_performance_counters() -> WorthTopoPhaseEightPerformanceCounters
{
    let current_head = TopologyRuntimeSupport::current_head_authoritative();
    let snapshot = TopologyRuntimeSupport::snapshot_read_only();

    WorthTopoPhaseEightPerformanceCounters {
        runtime_posture_row_count: current_head.runtime_posture_rows().len(),
        query_read_family_touched_scope_count: current_head.query_read_family_support_rows().len(),
        snapshot_read_family_touched_scope_count: snapshot.query_read_family_support_rows().len(),
        query_mutation_family_touched_scope_count: current_head
            .query_mutation_family_support_rows()
            .len(),
        query_mutation_lane_touched_scope_count: current_head
            .query_mutation_lane_support_rows()
            .len(),
        graph_composed_mutation_lane_count: current_head
            .query_mutation_lane_support_rows()
            .iter()
            .filter(|row| {
                matches!(
                    row.execution_shape(),
                    TopologyQueryMutationLaneExecutionShape::GraphComposition
                )
            })
            .count(),
        runtime_closeout_row_count: current_head.closeout().rows().len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_eight_topology_runtime_counter_report_exposes_exact_touched_scope() {
        let counters = current_topology_phase_eight_performance_counters();

        assert_eq!(counters.runtime_posture_row_count(), 8);
        assert_eq!(counters.query_read_family_touched_scope_count(), 4);
        assert_eq!(counters.snapshot_read_family_touched_scope_count(), 4);
        assert_eq!(counters.query_mutation_family_touched_scope_count(), 10);
        assert_eq!(counters.query_mutation_lane_touched_scope_count(), 14);
        assert_eq!(counters.graph_composed_mutation_lane_count(), 7);
        assert_eq!(counters.runtime_closeout_row_count(), 5);
    }
}
