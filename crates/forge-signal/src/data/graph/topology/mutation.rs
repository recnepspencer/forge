use std::cmp::Ordering;

use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;

use super::super::signal_graph::SignalGraph;

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
    pub(crate) fn assert_bidirectional_consistency(&self) -> Result<(), SignalError> {
        for (index, slot) in self.arena.nodes.iter().enumerate() {
            let Some(entry) = slot.data.as_ref() else {
                continue;
            };
            if entry.is_tombstoned() {
                continue;
            }
            let node = NodeId::new(index as u32, slot.generation);

            for dependency in self.topology.dependency_edges.get(entry.get_dependencies_id()) {
                if !self.is_alive(dependency.source()) {
                    continue;
                }
                if !self
                    .topology
                    .subscriber_edges
                    .get(self.get_entry(dependency.source())?.get_subscribers_id())
                    .contains(&node)
                {
                    return Err(SignalError::internal(format!(
                        "topology inconsistency: missing subscriber edge {} -> {}",
                        dependency.source(),
                        node
                    )));
                }
            }

            for &subscriber in self.topology.subscriber_edges.get(entry.get_subscribers_id()) {
                if !self.is_alive(subscriber) {
                    continue;
                }
                if !self
                    .topology
                    .dependency_edges
                    .get(self.get_entry(subscriber)?.get_dependencies_id())
                    .iter()
                    .any(|dependency| dependency.source() == node)
                {
                    return Err(SignalError::internal(format!(
                        "topology inconsistency: missing dependency edge {} -> {}",
                        node, subscriber
                    )));
                }
            }
        }

        Ok(())
    }

    #[inline]
    fn debug_assert_bidirectional_consistency(&self) {
        #[cfg(debug_assertions)]
        self.assert_bidirectional_consistency()
            .expect("signal topology should remain bidirectionally consistent");
    }

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
        self.debug_assert_bidirectional_consistency();
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
        self.debug_assert_bidirectional_consistency();
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
        self.debug_assert_bidirectional_consistency();
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
                let token_count_before = self.observation.partition_interner.token_count();
                let interned_scope = self
                    .observation
                    .partition_interner
                    .intern_subscription(&scope);
                self.observation.telemetry.invalidation.partition_interner_growth_delta += self
                    .observation
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
        let current = self.raw_dependencies_of(node)?.to_vec();
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
        self.debug_assert_bidirectional_consistency();
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
        let dependencies_id = self.topology.dependency_edges.insert_from_slice(edges);
        self.get_entry_mut(node)?.set_dependencies_id(dependencies_id);
        self.record_graph_storage_pressure();
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

    fn has_dependency_on(&self, node: NodeId, source: NodeId) -> Result<bool, SignalError> {
        Ok(self
            .raw_dependencies_of(node)?
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
        let subscribers_id = self.topology.subscriber_edges.insert_from_slice(subscribers);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.record_graph_storage_pressure();
        Ok(())
    }

    pub(crate) fn remove_subscriber_edge(
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

    #[cfg(test)]
    pub(crate) fn rebuild_subscriber_index_from_dependencies(&mut self) -> Result<(), SignalError> {
        self.observation.telemetry.storage.subscriber_index_rebuild_count += 1;
        let live_nodes = self.live_node_ids();
        let mut rebuilt = vec![Vec::<NodeId>::new(); self.arena_capacity()];

        for downstream in &live_nodes {
            let mut upstreams = self
                .raw_dependencies_of(*downstream)?
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

        self.debug_assert_bidirectional_consistency();
        Ok(())
    }

    pub(crate) fn reconcile_subscriber_membership_for_sources(
        &mut self,
        sources: &[NodeId],
    ) -> Result<(), SignalError> {
        let mut sources = sources.to_vec();
        sources.sort_by_key(|node| (node.index(), node.generation()));
        sources.dedup();

        for source in sources {
            if !self.is_alive(source) {
                continue;
            }
            let current = self.raw_subscribers_of(source)?.to_vec();
            let mut expected = current
                .iter()
                .copied()
                .filter(|subscriber| self.is_alive(*subscriber))
                .filter(|subscriber| {
                    self.raw_dependencies_of(*subscriber)
                        .map(|dependencies| dependencies.iter().any(|edge| edge.source() == source))
                        .unwrap_or(false)
                })
                .collect::<Vec<_>>();
            expected.sort_by_key(|node| (node.index(), node.generation()));
            expected.dedup();
            if current != expected {
                self.set_subscribers_sorted(source, &expected)?;
            }
        }

        self.debug_assert_bidirectional_consistency();
        Ok(())
    }

    pub(in crate::data::graph) fn reconcile_subscriber_sets(
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
        let mut updated = self.raw_dependencies_of(node)?.to_vec();
        if !mutate(&mut updated) {
            return Ok(false);
        }
        let dependencies_id = self.topology.dependency_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_dependencies_id(dependencies_id);
        self.record_graph_storage_pressure();
        Ok(true)
    }

    fn mutate_subscriber_edges(
        &mut self,
        node: NodeId,
        mutate: impl FnOnce(&mut Vec<NodeId>) -> bool,
    ) -> Result<bool, SignalError> {
        let mut updated = self.raw_subscribers_of(node)?.to_vec();
        if !mutate(&mut updated) {
            return Ok(false);
        }
        let subscribers_id = self.topology.subscriber_edges.insert_from_slice(&updated);
        self.get_entry_mut(node)?.set_subscribers_id(subscribers_id);
        self.record_graph_storage_pressure();
        Ok(true)
    }
}

fn compare_dependency_edges(left: &DependencyEdge, right: &DependencyEdge) -> Ordering {
    left.sort_key().cmp(&right.sort_key())
}
