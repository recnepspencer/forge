use std::collections::HashMap;
use std::time::Instant;

use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;

use super::scratch::ScratchLeaseKind;
use super::signal_graph::SignalGraph;

impl SignalGraph {
    fn purge_stale_subscribers_for(
        &mut self,
        node: NodeId,
        is_alive: impl Fn(NodeId) -> bool,
    ) -> Result<(), SignalError> {
        let current = self.subscribers_of(node)?.to_vec();
        let updated: Vec<_> = current
            .into_iter()
            .filter(|subscriber| is_alive(*subscriber))
            .collect();
        if updated.len() == self.subscribers_of(node)?.len() {
            return Ok(());
        }
        let subscribers_id = self.subscriber_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        Ok(())
    }

    pub fn tombstone_count(&self) -> u32 {
        self.tombstone_count
    }

    pub fn gc_threshold(&self) -> u32 {
        self.gc_threshold
    }

    pub fn unregister_node(&mut self, id: NodeId) -> Result<(), SignalError> {
        self.validate_handle(id)?;
        let mut scratch = self.acquire_scratch(ScratchLeaseKind::Churn)?;
        scratch.node_buffer_a.clear();
        scratch.node_buffer_b.clear();

        {
            scratch
                .node_buffer_a
                .extend(self.dependencies_of(id)?.iter().map(|edge| edge.source()));
            scratch
                .node_buffer_b
                .extend(self.subscribers_of(id)?.iter().copied());
        }

        for &source in &scratch.node_buffer_a {
            if self.is_alive(source) {
                self.remove_subscriber_edge(source, id)?;
            }
        }

        for &subscriber in &scratch.node_buffer_b {
            if self.is_alive(subscriber) {
                self.remove_dependencies_on(subscriber, id)?;
                self.get_entry_mut(subscriber)?.set_state(NodeState::Dirty);
            }
        }

        debug_assert!(
            !self.free_list.contains(&id.index()),
            "free list already contained slot {} before unregister",
            id.index()
        );
        self.nodes[id.index() as usize].vacate();
        self.tombstone_count += 1;
        self.free_list.push(id.index());
        self.restore_scratch(ScratchLeaseKind::Churn, scratch)?;
        Ok(())
    }

    pub fn run_gc_epoch(&mut self) {
        let gc_start = Instant::now();
        let Ok(mut scratch) = self.acquire_scratch(ScratchLeaseKind::Gc) else {
            return;
        };
        let len = self.nodes.len();
        if scratch.gc_liveness_generations.len() < len {
            scratch.gc_liveness_generations.resize(len, 0);
        }
        scratch.gc_liveness_alive.clear_all();
        scratch.gc_liveness_alive.ensure_len(len);

        for (index, slot) in self.nodes.iter().enumerate() {
            scratch.gc_liveness_generations[index] = slot.generation;
            if slot.is_occupied() {
                scratch.gc_liveness_alive.mark(index);
            }
        }

        let generations = &scratch.gc_liveness_generations;
        let alive_bits = &scratch.gc_liveness_alive;
        let alive_checker = |node_id: NodeId| -> bool {
            let idx = node_id.index() as usize;
            idx < generations.len()
                && generations[idx] == node_id.generation()
                && alive_bits.contains(idx)
        };

        for index in 0..self.nodes.len() {
            if let Some(node) = self.live_node_id_at(index) {
                let _ = self.purge_stale_subscribers_for(node, alive_checker);
            }
        }

        self.compact_graph_storage();

        self.tombstone_count = 0;
        if self.restore_scratch(ScratchLeaseKind::Gc, scratch).is_err() {
            return;
        }
        self.telemetry.gc_epoch_count += 1;
        self.telemetry.gc_epoch_nanos += gc_start.elapsed().as_nanos();
    }

    pub fn should_gc(&self) -> bool {
        self.tombstone_count >= self.gc_threshold
    }

    fn compact_graph_storage(&mut self) {
        let old_dependency_edges = self.dependency_edges.clone();
        let old_subscriber_edges = self.subscriber_edges.clone();
        let old_dependency_snapshots = self.dependency_snapshots.clone();

        let mut dependency_id_map = HashMap::new();
        let mut subscriber_id_map = HashMap::new();
        let mut snapshot_id_map = HashMap::new();

        let mut compacted_dependency_edges = crate::data::graph::DependencyEdgeStore::default();
        let mut compacted_subscriber_edges = crate::data::graph::SubscriberEdgeStore::default();
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

            let dependencies_id = *dependency_id_map
                .entry(entry.get_dependencies_id())
                .or_insert_with(|| {
                    compacted_dependency_edges
                        .insert_from_slice(old_dependency_edges.get(entry.get_dependencies_id()))
                });
            let subscribers_id = *subscriber_id_map
                .entry(entry.get_subscribers_id())
                .or_insert_with(|| {
                    compacted_subscriber_edges
                        .insert_from_slice(old_subscriber_edges.get(entry.get_subscribers_id()))
                });
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
                live_entry.set_dependencies_id(dependencies_id);
                live_entry.set_subscribers_id(subscribers_id);
                live_entry.set_dep_snapshot_id(dep_snapshot_id);
            }
        }

        self.dependency_edges = compacted_dependency_edges;
        self.subscriber_edges = compacted_subscriber_edges;
        self.dependency_snapshots = compacted_dependency_snapshots;
    }
}
