use super::super::runtime::graph::{EdgeTopology, NodeArena};

const STORAGE_PRESSURE_MULTIPLIER: usize = 8;
const HIGH_DEBT_EPOCH_THRESHOLD: u32 = 3;
const LOW_PRESSURE_EPOCH_BUDGET: u8 = 1;
const HIGH_PRESSURE_EPOCH_BUDGET: u8 = 3;

impl NodeArena {
    pub(in crate::data::graph) fn note_storage_pressure(
        &mut self,
        topology: &EdgeTopology,
        active_nodes: usize,
    ) {
        if topology.has_compaction_pressure(active_nodes) {
            self.compaction.debt = self.compaction.debt.saturating_add(1).max(1);
        }
    }

    pub(in crate::data::graph) fn should_gc(&self) -> bool {
        self.compaction.tombstone_count >= self.compaction.gc_threshold
    }

    pub(in crate::data::graph) fn should_run_compaction_epoch(
        &self,
        topology: &EdgeTopology,
        active_nodes: usize,
    ) -> bool {
        self.compaction.debt > 0 || self.should_gc() || topology.has_compaction_pressure(active_nodes)
    }

    pub(in crate::data::graph) fn compaction_epoch_budget(
        &self,
        topology: &EdgeTopology,
        active_nodes: usize,
    ) -> u8 {
        if topology.has_high_compaction_pressure(active_nodes) || self.should_gc()
            || self.compaction.debt >= HIGH_DEBT_EPOCH_THRESHOLD
        {
            HIGH_PRESSURE_EPOCH_BUDGET
        } else {
            LOW_PRESSURE_EPOCH_BUDGET
        }
    }

    pub(in crate::data::graph) fn record_retired_node(&mut self) {
        self.compaction.tombstone_count += 1;
        self.compaction.debt = self.compaction.debt.saturating_add(1);
    }

    pub(in crate::data::graph) fn complete_compaction_epoch(&mut self, families_compacted: u8) {
        self.compaction.tombstone_count = 0;
        self.compaction.debt = self.compaction.debt.saturating_sub(families_compacted as u32);
    }
}

impl EdgeTopology {
    pub(in crate::data::graph) fn has_compaction_pressure(&self, active_nodes: usize) -> bool {
        let (dependency_segments, subscriber_segments, snapshot_count) = self.storage_pressure_counts();
        let active = active_nodes.max(1);
        dependency_segments > active
            || subscriber_segments > active
            || snapshot_count > active
            || self.has_high_storage_pressure(active)
    }

    pub(in crate::data::graph) fn has_high_compaction_pressure(&self, active_nodes: usize) -> bool {
        self.has_high_storage_pressure(active_nodes)
    }

    fn has_high_storage_pressure(&self, active_nodes: usize) -> bool {
        let (dependency_segments, subscriber_segments, snapshot_count) = self.storage_pressure_counts();
        let active = active_nodes.max(1);
        dependency_segments >= active * STORAGE_PRESSURE_MULTIPLIER
            || subscriber_segments >= active * STORAGE_PRESSURE_MULTIPLIER
            || snapshot_count >= active * STORAGE_PRESSURE_MULTIPLIER
    }

    pub(in crate::data::graph) fn storage_pressure_counts(&self) -> (usize, usize, usize) {
        (
            self.dependency_edges.live_segment_count(),
            self.subscriber_edges.live_segment_count(),
            self.dependency_snapshots.live_snapshot_count(),
        )
    }
}

impl super::super::signal_graph::SignalGraph {
    pub(crate) fn record_graph_storage_pressure(&mut self) {
        self.arena
            .note_storage_pressure(&self.topology, self.arena.active_nodes as usize);
    }
}
