use std::collections::HashMap;

use super::signal_graph::SignalGraph;

impl SignalGraph {
    pub(crate) fn compact_graph_storage(&mut self) {
        self.compact_dependency_edge_storage();
        self.compact_subscriber_edge_storage();
        self.compact_dependency_snapshot_storage();
    }

    pub(crate) fn maybe_compact_graph_storage(&mut self) {
        let active = self.active_node_count().max(1);
        let dependency_segments = self.dependency_edges.live_segment_count();
        let subscriber_segments = self.subscriber_edges.live_segment_count();
        let snapshot_count = self.dependency_snapshots.live_snapshot_count();
        let has_growth_debt = dependency_segments > active
            || subscriber_segments > active
            || snapshot_count > active;
        let has_high_pressure = dependency_segments >= active * 8
            || subscriber_segments >= active * 8
            || snapshot_count >= active * 8;
        if has_growth_debt || has_high_pressure {
            self.gc_compaction_debt = self.gc_compaction_debt.saturating_add(1).max(1);
        }
    }

    pub(super) fn gc_compaction_budget(&self) -> u8 {
        let active = self.active_node_count().max(1);
        let dependency_segments = self.dependency_edges.live_segment_count();
        let subscriber_segments = self.subscriber_edges.live_segment_count();
        let snapshot_count = self.dependency_snapshots.live_snapshot_count();
        let high_pressure = dependency_segments >= active * 8
            || subscriber_segments >= active * 8
            || snapshot_count >= active * 8
            || self.should_gc()
            || self.gc_compaction_debt >= 3;
        if high_pressure {
            3
        } else {
            1
        }
    }

    pub(super) fn compact_next_graph_storage_family(&mut self) {
        match self.gc_compaction_cursor % 3 {
            0 => self.compact_dependency_edge_storage(),
            1 => self.compact_subscriber_edge_storage(),
            _ => self.compact_dependency_snapshot_storage(),
        }
        self.gc_compaction_cursor = (self.gc_compaction_cursor + 1) % 3;
    }

    fn compact_dependency_edge_storage(&mut self) {
        let old_dependency_edges = std::mem::take(&mut self.dependency_edges);
        self.telemetry.graph_storage_compaction_count += 1;
        self.telemetry.graph_storage_dependency_segments_rewritten +=
            old_dependency_edges.live_segment_count() as u64;
        let mut dependency_id_map = HashMap::new();
        let mut compacted_dependency_edges = crate::data::graph::DependencyEdgeStore::default();

        for index in 0..self.nodes.len() {
            let Some(node) = self.live_node_id_at(index) else {
                continue;
            };
            let entry = match self.get_entry(node) {
                Ok(entry) => entry.clone(),
                Err(_) => continue,
            };
            let dependencies_id = *dependency_id_map
                .entry(entry.get_dependencies_id())
                .or_insert_with(|| {
                    compacted_dependency_edges
                        .insert_from_slice(old_dependency_edges.get(entry.get_dependencies_id()))
                });
            if let Ok(live_entry) = self.get_entry_mut(node) {
                live_entry.set_dependencies_id(dependencies_id);
            }
        }

        self.dependency_edges = compacted_dependency_edges;
    }

    fn compact_subscriber_edge_storage(&mut self) {
        let old_subscriber_edges = std::mem::take(&mut self.subscriber_edges);
        self.telemetry.graph_storage_compaction_count += 1;
        self.telemetry.graph_storage_subscriber_segments_rewritten +=
            old_subscriber_edges.live_segment_count() as u64;
        let mut subscriber_id_map = HashMap::new();
        let mut compacted_subscriber_edges = crate::data::graph::SubscriberEdgeStore::default();

        for index in 0..self.nodes.len() {
            let Some(node) = self.live_node_id_at(index) else {
                continue;
            };
            let entry = match self.get_entry(node) {
                Ok(entry) => entry.clone(),
                Err(_) => continue,
            };
            let subscribers_id = *subscriber_id_map
                .entry(entry.get_subscribers_id())
                .or_insert_with(|| {
                    compacted_subscriber_edges
                        .insert_from_slice(old_subscriber_edges.get(entry.get_subscribers_id()))
                });
            if let Ok(live_entry) = self.get_entry_mut(node) {
                live_entry.set_subscribers_id(subscribers_id);
            }
        }

        self.subscriber_edges = compacted_subscriber_edges;
    }

    fn compact_dependency_snapshot_storage(&mut self) {
        let old_dependency_snapshots = std::mem::take(&mut self.dependency_snapshots);
        self.telemetry.graph_storage_compaction_count += 1;
        self.telemetry.graph_storage_snapshot_rewrites +=
            old_dependency_snapshots.live_snapshot_count() as u64;
        let mut snapshot_id_map = HashMap::new();
        let mut compacted_dependency_snapshots =
            crate::data::dependency::DependencySnapshotStore::default();

        for index in 0..self.nodes.len() {
            let Some(node) = self.live_node_id_at(index) else {
                continue;
            };
            let entry = match self.get_entry(node) {
                Ok(entry) => entry.clone(),
                Err(_) => continue,
            };
            let dep_snapshot_id = *snapshot_id_map
                .entry(entry.get_dep_snapshot_id())
                .or_insert_with(|| {
                    compacted_dependency_snapshots.insert(
                        old_dependency_snapshots
                            .get(entry.get_dep_snapshot_id())
                            .clone(),
                    )
                });
            if let Ok(live_entry) = self.get_entry_mut(node) {
                live_entry.set_dep_snapshot_id(dep_snapshot_id);
            }
        }

        self.dependency_snapshots = compacted_dependency_snapshots;
    }
}
