use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::proof::{DedupedNodeBatch, SubscriberRepair, SubscriberRepairBatch};

use super::super::signal_graph::SignalGraph;
use super::mutation::SubscriberBatchOp;

impl SignalGraph {
    pub(super) fn add_subscriber_edge(
        &mut self,
        node: NodeId,
        subscriber: NodeId,
    ) -> Result<bool, SignalError> {
        self.mutate_subscriber_edges(node, |updated| match updated.binary_search(&subscriber) {
            Ok(_) => false,
            Err(index) => {
                updated.insert(index, subscriber);
                true
            }
        })
    }

    pub(crate) fn set_subscribers_sorted(
        &mut self,
        node: NodeId,
        subscribers: &[NodeId],
    ) -> Result<(), SignalError> {
        let subscribers_id = self
            .topology
            .subscriber_edges
            .insert_from_slice(subscribers);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn remove_subscriber_edge(
        &mut self,
        node: NodeId,
        subscriber: NodeId,
    ) -> Result<bool, SignalError> {
        self.mutate_subscriber_edges(node, |updated| match updated.binary_search(&subscriber) {
            Ok(index) => {
                updated.remove(index);
                true
            }
            Err(_) => false,
        })
    }

    #[cfg(test)]
    pub(crate) fn rebuild_subscriber_index_from_dependencies(&mut self) -> Result<(), SignalError> {
        self.observation
            .telemetry
            .storage
            .subscriber_index_rebuild_count += 1;
        let live_nodes = self.live_node_ids();
        let mut rebuilt = vec![Vec::<NodeId>::new(); self.arena_capacity()];

        for downstream in &live_nodes {
            let mut upstreams = self
                .raw_dependencies_of(*downstream)?
                .iter()
                .map(|edge| edge.source())
                .collect::<Vec<_>>();
            upstreams.sort_by_key(|node| node_id_sort_key(*node));
            upstreams.dedup();
            for upstream in upstreams {
                if !self.is_alive(upstream) {
                    continue;
                }
                rebuilt[upstream.index() as usize].push(*downstream);
            }
        }

        for node in live_nodes {
            let subscribers = std::mem::take(&mut rebuilt[node.index() as usize]);
            self.set_subscribers_sorted(node, &subscribers)?;
        }

        self.debug_assert_bidirectional_consistency();
        Ok(())
    }

    pub(crate) fn reconcile_subscriber_membership_for_sources(
        &mut self,
        sources: &[NodeId],
    ) -> Result<(), SignalError> {
        let plan = self.build_subscriber_membership_repair_plan(sources)?;
        self.apply_subscriber_reconciliation_plan(plan)?;
        self.debug_assert_bidirectional_consistency();
        Ok(())
    }

    pub(in crate::data::graph) fn reconcile_subscriber_sets(
        &mut self,
        node: NodeId,
        current_sources: &[NodeId],
        desired_sources: &[NodeId],
    ) -> Result<(), SignalError> {
        self.reconcile_dependency_subscribers(node, current_sources, desired_sources)?;
        Ok(())
    }

    pub(super) fn reconcile_dependency_subscribers(
        &mut self,
        node: NodeId,
        current_sources: &[NodeId],
        desired_sources: &[NodeId],
    ) -> Result<(), SignalError> {
        let current_sources = sorted_unique_nodes(current_sources);
        let desired_sources = sorted_unique_nodes(desired_sources);
        let mut current_index = 0usize;
        let mut desired_index = 0usize;

        while current_index < current_sources.len() && desired_index < desired_sources.len() {
            let current = current_sources[current_index];
            let desired = desired_sources[desired_index];

            match node_id_sort_key(current).cmp(&node_id_sort_key(desired)) {
                std::cmp::Ordering::Less => {
                    if self.is_alive(current) {
                        self.remove_subscriber_edge(current, node)?;
                    }
                    current_index += 1;
                }
                std::cmp::Ordering::Greater => {
                    if self.is_alive(desired) {
                        self.add_subscriber_edge(desired, node)?;
                    }
                    desired_index += 1;
                }
                std::cmp::Ordering::Equal => {
                    current_index += 1;
                    desired_index += 1;
                }
            }
        }

        while current_index < current_sources.len() {
            let current = current_sources[current_index];
            if self.is_alive(current) {
                self.remove_subscriber_edge(current, node)?;
            }
            current_index += 1;
        }

        while desired_index < desired_sources.len() {
            let desired = desired_sources[desired_index];
            if self.is_alive(desired) {
                self.add_subscriber_edge(desired, node)?;
            }
            desired_index += 1;
        }

        Ok(())
    }

    fn build_subscriber_membership_repair_plan(
        &self,
        sources: &[NodeId],
    ) -> Result<SubscriberRepairBatch, SignalError> {
        let mut repairs = Vec::<SubscriberRepair>::new();

        for source in sorted_unique_nodes(sources) {
            if !self.is_alive(source) {
                continue;
            }

            let current = self.raw_subscribers_of(source)?;
            let mut expected = Vec::with_capacity(current.len());
            let mut changed = false;

            for subscriber in current.iter().copied() {
                if !self.is_alive(subscriber) {
                    changed = true;
                    continue;
                }

                let still_subscribed = self
                    .raw_dependencies_of(subscriber)?
                    .iter()
                    .any(|edge| edge.source() == source);
                if still_subscribed {
                    expected.push(subscriber);
                } else {
                    changed = true;
                }
            }

            if changed {
                repairs.push(SubscriberRepair {
                    source,
                    subscribers: DedupedNodeBatch::new(expected),
                });
            }
        }

        Ok(SubscriberRepairBatch::new(repairs))
    }

    fn apply_subscriber_reconciliation_plan(
        &mut self,
        plan: SubscriberRepairBatch,
    ) -> Result<(), SignalError> {
        if plan.is_empty() {
            return Ok(());
        }

        for rewrite in plan.as_slice() {
            self.set_subscribers_sorted(rewrite.source, rewrite.subscribers.as_slice())?;
        }

        Ok(())
    }

    pub(super) fn apply_subscriber_batch_ops(
        &mut self,
        ops: &[SubscriberBatchOp],
    ) -> Result<(), SignalError> {
        if ops.is_empty() {
            return Ok(());
        }

        let mut sorted = ops.to_vec();
        sorted.sort_by_key(|op| {
            (
                node_id_sort_key(op.source),
                node_id_sort_key(op.subscriber),
                !op.should_subscribe,
            )
        });

        let mut op_index = 0usize;
        while op_index < sorted.len() {
            let source = sorted[op_index].source;
            let start = op_index;
            while op_index < sorted.len() && sorted[op_index].source == source {
                op_index += 1;
            }

            if !self.is_alive(source) {
                continue;
            }

            let mut updated = std::mem::take(&mut self.traversal.topology_node_buffer);
            updated.clear();
            updated.extend_from_slice(self.raw_subscribers_of(source)?);

            let mut changed = false;
            let mut range_index = start;
            while range_index < op_index {
                let op = sorted[range_index];
                match updated.binary_search(&op.subscriber) {
                    Ok(index) if !op.should_subscribe => {
                        updated.remove(index);
                        changed = true;
                    }
                    Ok(_) => {}
                    Err(index) if op.should_subscribe => {
                        updated.insert(index, op.subscriber);
                        changed = true;
                    }
                    Err(_) => {}
                }
                range_index += 1;
            }

            if changed {
                let subscribers_id = self.topology.subscriber_edges.insert_from_slice(&updated);
                self.get_entry_mut(source)?
                    .set_subscribers_id(subscribers_id);
                self.record_graph_storage_pressure();
            }

            updated.clear();
            self.traversal.topology_node_buffer = updated;
        }

        Ok(())
    }

    fn mutate_subscriber_edges(
        &mut self,
        node: NodeId,
        mutate: impl FnOnce(&mut Vec<NodeId>) -> bool,
    ) -> Result<bool, SignalError> {
        let mut updated = std::mem::take(&mut self.traversal.topology_node_buffer);
        updated.clear();
        updated.extend_from_slice(self.raw_subscribers_of(node)?);
        if !mutate(&mut updated) {
            self.traversal.topology_node_buffer = updated;
            return Ok(false);
        }
        let subscribers_id = self.topology.subscriber_edges.insert_from_slice(&updated);
        updated.clear();
        self.traversal.topology_node_buffer = updated;
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.record_graph_storage_pressure();
        Ok(true)
    }
}

fn sorted_unique_nodes(nodes: &[NodeId]) -> Vec<NodeId> {
    let mut nodes = nodes.to_vec();
    nodes.sort_by_key(|node| node_id_sort_key(*node));
    nodes.dedup();
    nodes
}

fn node_id_sort_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}
