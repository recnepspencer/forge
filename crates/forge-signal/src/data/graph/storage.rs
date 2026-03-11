use std::cmp::Ordering;

use crate::data::aspect::Aspect;
use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::node::{NodeEntry, NodeEvaluationConfig, NodeState};
use crate::data::output::PartitionSubscription;
use crate::data::trace::CausalityMetadata;

use super::node_builder::NodeBuilder;
use super::signal_graph::SignalGraph;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DependencyReconciliationReport {
    pub added: u32,
    pub removed: u32,
    pub unchanged: u32,
}

impl DependencyReconciliationReport {
    pub fn update_count(self) -> u32 {
        self.added + self.removed
    }
}

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
        self.maybe_compact_graph_storage();
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
        self.active_nodes as usize
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
        let edge = self.build_dependency_edge(upstream, aspect, Some(scope));
        let inserted = self.add_dependency_edge(downstream, edge)?;
        if inserted {
            self.add_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    pub fn remove_dependency(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let removed = self.remove_dependency_edges_matching(downstream, upstream, aspect, None)?;
        if removed && !self.has_dependency_on(downstream, upstream)? {
            self.remove_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    pub fn remove_dependency_with_scope(
        &mut self,
        downstream: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: &PartitionSubscription,
    ) -> Result<(), SignalError> {
        self.validate_handle(downstream)?;
        self.validate_handle(upstream)?;

        let removed =
            self.remove_dependency_edges_matching(downstream, upstream, aspect, Some(scope))?;
        if removed && !self.has_dependency_on(downstream, upstream)? {
            self.remove_subscriber_edge(upstream, downstream)?;
        }
        Ok(())
    }

    pub(crate) fn build_dependency_edge(
        &mut self,
        upstream: NodeId,
        aspect: Aspect,
        scope: Option<PartitionSubscription>,
    ) -> DependencyEdge {
        match scope {
            Some(scope) => {
                let token_count_before = self.partition_interner.token_count();
                let interned_scope = self.partition_interner.intern_subscription(&scope);
                self.telemetry.partition_interner_growth_delta += self
                    .partition_interner
                    .token_count()
                    .saturating_sub(token_count_before) as u64;
                DependencyEdge::with_scope(upstream, aspect, scope, interned_scope)
            }
            None => DependencyEdge::new(upstream, aspect),
        }
    }

    pub(crate) fn reconcile_dependencies(
        &mut self,
        node: NodeId,
        desired: &[DependencyEdge],
    ) -> Result<DependencyReconciliationReport, SignalError> {
        self.validate_handle(node)?;
        let current = self.dependencies_of(node)?.to_vec();
        let current_sources = current.iter().map(|edge| edge.source()).collect::<Vec<_>>();
        let desired_sources = desired.iter().map(|edge| edge.source()).collect::<Vec<_>>();
        let mut report = DependencyReconciliationReport::default();
        let mut current_index = 0usize;
        let mut desired_index = 0usize;

        while current_index < current.len() && desired_index < desired.len() {
            match compare_dependency_edges(&current[current_index], &desired[desired_index]) {
                Ordering::Less => {
                    report.removed += 1;
                    current_index += 1;
                }
                Ordering::Greater => {
                    report.added += 1;
                    desired_index += 1;
                }
                Ordering::Equal => {
                    report.unchanged += 1;
                    current_index += 1;
                    desired_index += 1;
                }
            }
        }

        report.removed += (current.len() - current_index) as u32;
        report.added += (desired.len() - desired_index) as u32;

        self.set_dependency_edges_sorted(node, desired)?;
        self.reconcile_subscriber_sets(node, &current_sources, &desired_sources)?;
        Ok(report)
    }

    fn add_dependency_edge(
        &mut self,
        node: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        self.mutate_dependency_edges(node, |updated| {
            updated.sort_by(compare_dependency_edges);
            match updated.binary_search_by(|candidate| compare_dependency_edges(candidate, &edge)) {
                Ok(_) => false,
                Err(index) => {
                    updated.insert(index, edge);
                    true
                }
            }
        })
    }

    pub(crate) fn set_dependency_edges_sorted(
        &mut self,
        node: NodeId,
        edges: &[DependencyEdge],
    ) -> Result<(), SignalError> {
        let dependencies_id = self.dependency_edges.insert_from_slice(edges);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        self.maybe_compact_graph_storage();
        Ok(())
    }

    fn remove_dependency_edges_matching(
        &mut self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
        scope: Option<&PartitionSubscription>,
    ) -> Result<bool, SignalError> {
        self.mutate_dependency_edges(node, |updated| {
            let original_len = updated.len();
            updated.retain(|candidate| {
                if candidate.source() != upstream || candidate.aspect() != aspect {
                    return true;
                }
                match scope {
                    Some(scope) => candidate.scope_ref() != Some(scope),
                    None => false,
                }
            });
            updated.len() != original_len
        })
    }

    pub(super) fn remove_dependencies_on(
        &mut self,
        node: NodeId,
        source: NodeId,
    ) -> Result<bool, SignalError> {
        self.mutate_dependency_edges(node, |updated| {
            let original_len = updated.len();
            updated.retain(|edge| edge.source() != source);
            updated.len() != original_len
        })
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
        self.mutate_subscriber_edges(node, |updated| {
            updated.sort();
            match updated.binary_search(&subscriber) {
                Ok(_) => false,
                Err(index) => {
                    updated.insert(index, subscriber);
                    true
                }
            }
        })
    }

    pub(crate) fn set_subscribers_sorted(
        &mut self,
        node: NodeId,
        subscribers: &[NodeId],
    ) -> Result<(), SignalError> {
        let subscribers_id = self.subscriber_edges.insert_from_slice(subscribers);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.maybe_compact_graph_storage();
        Ok(())
    }

    pub(super) fn remove_subscriber_edge(
        &mut self,
        node: NodeId,
        subscriber: NodeId,
    ) -> Result<bool, SignalError> {
        self.mutate_subscriber_edges(node, |updated| {
            let original_len = updated.len();
            updated.retain(|candidate| *candidate != subscriber);
            updated.len() != original_len
        })
    }

    pub(crate) fn rebuild_subscriber_index_from_dependencies(&mut self) -> Result<(), SignalError> {
        self.telemetry.subscriber_index_rebuild_count += 1;
        let live_nodes = self.live_node_ids();
        let mut rebuilt = vec![Vec::<NodeId>::new(); self.arena_capacity()];

        for downstream in &live_nodes {
            let mut upstreams = self
                .dependencies_of(*downstream)?
                .iter()
                .map(|edge| edge.source())
                .collect::<Vec<_>>();
            upstreams.sort_by_key(|node| (node.index(), node.generation()));
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

        Ok(())
    }

    fn reconcile_subscriber_sets(
        &mut self,
        node: NodeId,
        current_sources: &[NodeId],
        desired_sources: &[NodeId],
    ) -> Result<(), SignalError> {
        let mut current_sources = current_sources.to_vec();
        let mut desired_sources = desired_sources.to_vec();
        current_sources.sort_by_key(|source| (source.index(), source.generation()));
        desired_sources.sort_by_key(|source| (source.index(), source.generation()));
        current_sources.dedup();
        desired_sources.dedup();

        for source in &current_sources {
            if !desired_sources.contains(source) {
                self.remove_subscriber_edge(*source, node)?;
            }
        }

        for source in &desired_sources {
            if !current_sources.contains(source) {
                self.add_subscriber_edge(*source, node)?;
            }
        }

        Ok(())
    }

    fn mutate_dependency_edges(
        &mut self,
        node: NodeId,
        mutate: impl FnOnce(&mut Vec<DependencyEdge>) -> bool,
    ) -> Result<bool, SignalError> {
        let mut updated = self.dependencies_of(node)?.to_vec();
        if !mutate(&mut updated) {
            return Ok(false);
        }
        let dependencies_id = self.dependency_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_dependencies_id(dependencies_id);
        self.maybe_compact_graph_storage();
        Ok(true)
    }

    fn mutate_subscriber_edges(
        &mut self,
        node: NodeId,
        mutate: impl FnOnce(&mut Vec<NodeId>) -> bool,
    ) -> Result<bool, SignalError> {
        let mut updated = self.subscribers_of(node)?.to_vec();
        if !mutate(&mut updated) {
            return Ok(false);
        }
        let subscribers_id = self.subscriber_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.maybe_compact_graph_storage();
        Ok(true)
    }
}

pub(super) fn stale_error(id: NodeId) -> SignalError {
    SignalError::invalid_input(format!("stale NodeId: {id}"))
}

fn compare_dependency_edges(left: &DependencyEdge, right: &DependencyEdge) -> Ordering {
    left.sort_key().cmp(&right.sort_key())
}
