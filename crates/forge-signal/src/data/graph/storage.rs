use std::collections::BTreeMap;

use crate::data::aspect::Aspect;
use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::output::PartitionSubscription;
use crate::data::trace::CausalityMetadata;

use super::node_builder::NodeBuilder;
use super::signal_graph::SignalGraph;
impl SignalGraph {
    #[doc(hidden)]
    pub fn create_node(&mut self) -> NodeId {
        let entry = NodeEntry::new();
        self.allocate_node(entry)
    }

    pub fn node(&mut self) -> NodeBuilder<'_> {
        NodeBuilder::new(self)
    }

    #[doc(hidden)]
    pub fn create_node_with_config(&mut self, config: NodeEvaluationConfig) -> NodeId {
        let mut entry = NodeEntry::new();
        entry.set_eval_config(config);
        self.allocate_node(entry)
    }

    pub fn get_state(&self, id: NodeId) -> Result<NodeState, SignalError> {
        Ok(*self.get_entry(id)?.get_state())
    }

    pub fn get_entry(&self, id: NodeId) -> Result<&NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &self.nodes[id.index() as usize];
        slot.data.as_ref().ok_or_else(|| stale_error(id))
    }

    pub fn get_entry_mut(&mut self, id: NodeId) -> Result<&mut NodeEntry, SignalError> {
        self.validate_handle(id)?;
        let slot = &mut self.nodes[id.index() as usize];
        slot.data.as_mut().ok_or_else(|| stale_error(id))
    }

    pub(crate) fn get_dep_snapshot(&self, id: NodeId) -> Result<&DependencySnapshot, SignalError> {
        let entry = self.get_entry(id)?;
        Ok(self.dependency_snapshots.get(entry.get_dep_snapshot_id()))
    }

    pub(crate) fn set_dep_snapshot(
        &mut self,
        id: NodeId,
        snapshot: DependencySnapshot,
    ) -> Result<(), SignalError> {
        let snapshot_id = self.dependency_snapshots.insert(snapshot);
        self.get_entry_mut(id)?.set_dep_snapshot_id(snapshot_id);
        Ok(())
    }

    pub fn is_alive(&self, id: NodeId) -> bool {
        let idx = id.index() as usize;
        if idx >= self.nodes.len() {
            return false;
        }
        let slot = &self.nodes[idx];
        slot.generation == id.generation() && slot.is_occupied()
    }

    pub fn active_node_count(&self) -> usize {
        self.nodes.iter().filter(|s| s.is_occupied()).count()
    }

    pub fn arena_capacity(&self) -> usize {
        self.nodes.len()
    }

    pub(crate) fn live_node_id_at(&self, index: usize) -> Option<NodeId> {
        let slot = self.nodes.get(index)?;
        if !slot.is_occupied() {
            return None;
        }
        Some(NodeId::new(index as u32, slot.generation))
    }

    pub(crate) fn replace_entry(
        &mut self,
        id: NodeId,
        entry: NodeEntry,
    ) -> Result<(), SignalError> {
        let target = self.get_entry_mut(id)?;
        *target = entry;
        Ok(())
    }

    pub fn causality_of(&self, node: NodeId) -> Result<Option<&CausalityMetadata>, SignalError> {
        Ok(self.get_entry(node)?.get_causality())
    }

    pub fn set_causality(
        &mut self,
        node: NodeId,
        causality: Option<CausalityMetadata>,
    ) -> Result<(), SignalError> {
        self.get_entry_mut(node)?.set_causality(causality);
        Ok(())
    }
}

impl SignalGraph {
    pub fn add_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let edge = DependencyEdge::new(upstream, aspect);
        let inserted = self.add_dependency_edge(downstream, edge)?;
        if inserted {
            self.add_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    pub fn add_partition_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        partition: impl Into<crate::data::output::PartitionToken>,
    ) -> Result<(), SignalError> {
        let scope = PartitionSubscription::whole_partition(partition);
        self.add_dependency_with_scope(downstream, upstream, aspect, scope)
    }

    pub fn add_partition_detail_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        partition: impl Into<crate::data::output::PartitionToken>,
        detail: impl Into<String>,
    ) -> Result<(), SignalError> {
        let scope = PartitionSubscription::partition_and_detail(partition, detail);
        self.add_dependency_with_scope(downstream, upstream, aspect, scope)
    }

    fn add_dependency_with_scope(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: PartitionSubscription,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;
        let interned_scope = self.partition_interner.intern_subscription(&scope);
        let edge = DependencyEdge::with_scope(upstream, aspect, scope, interned_scope);
        let inserted = self.add_dependency_edge(downstream, edge)?;
        if inserted {
            self.add_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    pub(crate) fn connect_dependency_capture(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: Option<PartitionSubscription>,
    ) -> Result<bool, SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;
        let edge = match scope {
            Some(scope) => {
                let interned_scope = self.partition_interner.intern_subscription(&scope);
                DependencyEdge::with_scope(upstream, aspect, scope, interned_scope)
            }
            None => DependencyEdge::new(upstream, aspect),
        };
        let inserted = self.add_dependency_edge(downstream, edge)?;
        if inserted {
            self.add_subscriber_edge(upstream, downstream)?;
        }
        Ok(inserted)
    }

    pub(crate) fn disconnect_dependency_edge(
        &mut self,
        downstream: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(edge.source())?;
        let removed = self.remove_dependency_edge(downstream, edge.clone())?;
        if removed && !self.has_dependency_on(downstream, edge.source())? {
            self.remove_subscriber_edge(edge.source(), downstream)?;
        }
        Ok(removed)
    }

    pub fn remove_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let edge = DependencyEdge::new(upstream, aspect);
        let removed = self.remove_dependency_edge(downstream, edge)?;
        if removed && !self.has_dependency_on(downstream, upstream)? {
            self.remove_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    fn add_dependency_edge(
        &mut self,
        node: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        let current = self.dependencies_of(node)?.to_vec();
        if current.contains(&edge) {
            return Ok(false);
        }
        let mut updated = current;
        updated.push(edge);
        let dependencies_id = self.dependency_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        Ok(true)
    }

    fn remove_dependency_edge(
        &mut self,
        node: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        let current = self.dependencies_of(node)?.to_vec();
        let original_len = current.len();
        let updated: Vec<_> = current
            .into_iter()
            .filter(|candidate| *candidate != edge)
            .collect();
        if updated.len() == original_len {
            return Ok(false);
        }
        let dependencies_id = self.dependency_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        Ok(true)
    }

    pub(super) fn remove_dependencies_on(
        &mut self,
        node: NodeId,
        source: NodeId,
    ) -> Result<bool, SignalError> {
        let current = self.dependencies_of(node)?.to_vec();
        let original_len = current.len();
        let updated: Vec<_> = current
            .into_iter()
            .filter(|edge| edge.source() != source)
            .collect();
        if updated.len() == original_len {
            return Ok(false);
        }
        let dependencies_id = self.dependency_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        Ok(true)
    }

    fn has_dependency_on(&self, node: NodeId, source: NodeId) -> Result<bool, SignalError> {
        Ok(self
            .dependencies_of(node)?
            .iter()
            .any(|edge| edge.source() == source))
    }

    fn add_subscriber_edge(
        &mut self,
        node: NodeId,
        subscriber: NodeId,
    ) -> Result<bool, SignalError> {
        let current = self.subscribers_of(node)?.to_vec();
        if current.contains(&subscriber) {
            return Ok(false);
        }
        let mut updated = current;
        updated.push(subscriber);
        let subscribers_id = self.subscriber_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        Ok(true)
    }

    pub(super) fn remove_subscriber_edge(
        &mut self,
        node: NodeId,
        subscriber: NodeId,
    ) -> Result<bool, SignalError> {
        let current = self.subscribers_of(node)?.to_vec();
        let original_len = current.len();
        let updated: Vec<_> = current
            .into_iter()
            .filter(|candidate| *candidate != subscriber)
            .collect();
        if updated.len() == original_len {
            return Ok(false);
        }
        let subscribers_id = self.subscriber_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        Ok(true)
    }

    pub(crate) fn rebuild_subscriber_index_from_dependencies(&mut self) -> Result<(), SignalError> {
        let live_nodes = self.live_node_ids();
        let mut rebuilt = BTreeMap::<NodeId, Vec<NodeId>>::new();
        for node in &live_nodes {
            rebuilt.insert(*node, Vec::new());
        }

        for downstream in &live_nodes {
            let mut upstreams = self
                .dependencies_of(*downstream)?
                .iter()
                .map(|edge| edge.source())
                .collect::<Vec<_>>();
            upstreams.sort_by_key(|node| (node.index(), node.generation()));
            upstreams.dedup();
            for upstream in upstreams {
                if let Some(subscribers) = rebuilt.get_mut(&upstream) {
                    subscribers.push(*downstream);
                }
            }
        }

        for (node, subscribers) in rebuilt {
            let subscribers_id = self.subscriber_edges.insert_from_slice(&subscribers);
            self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        }

        Ok(())
    }
}

pub(super) fn stale_error(id: NodeId) -> SignalError {
    SignalError::invalid_input(format!("stale NodeId: {id}"))
}
