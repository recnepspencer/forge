use crate::data::graph::DependencyEdgeStore;

use super::{remap_live_entry_handles, SignalGraph};

impl SignalGraph {
    pub(super) fn compact_dependency_edge_storage(&mut self) {
        let old_dependency_edges = std::mem::take(&mut self.topology.dependency_edges);
        self.observation
            .telemetry
            .storage
            .graph_storage_compaction_count += 1;
        self.observation
            .telemetry
            .storage
            .graph_storage_dependency_segments_rewritten +=
            old_dependency_edges.live_segment_count() as u64;

        let mut compacted_dependency_edges = DependencyEdgeStore::default();
        remap_live_entry_handles(
            self,
            |entry| entry.get_dependencies_id(),
            |dependencies_id| {
                compacted_dependency_edges
                    .insert_from_slice(old_dependency_edges.get(dependencies_id))
            },
            |entry, dependencies_id| entry.set_dependencies_id(dependencies_id),
        );

        self.topology.dependency_edges = compacted_dependency_edges;
    }
}
