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

#[derive(Debug, Default)]
struct SubscriberReconciliationPlan {
    rewrites: Vec<SubscriberSetRewrite>,
}

#[derive(Debug)]
struct SubscriberSetRewrite {
    source: NodeId,
    subscribers: Vec<NodeId>,
}

impl SubscriberReconciliationPlan {
    fn push(&mut self, source: NodeId, subscribers: Vec<NodeId>) {
        self.rewrites.push(SubscriberSetRewrite { source, subscribers });
    }

    fn is_empty(&self) -> bool {
        self.rewrites.is_empty()
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

        let subscriber_plan =
            self.build_subscriber_reconciliation_plan(node, &current_sources, &desired_sources)?;

        self.set_dependency_edges_sorted(node, desired)?;
        self.apply_subscriber_reconciliation_plan(subscriber_plan)?;
        self.debug_assert_bidirectional_consistency();
        Ok(report)
    }

    fn add_dependency_edge(
        &mut self,
        node: NodeId,
        edge: DependencyEdge,
    ) -> Result<bool, SignalError> {
        self.mutate_dependency_edges(node, |updated| {
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
            match updated.binary_search(&subscriber) {
                Ok(index) => {
                    updated.remove(index);
                    true
                }
                Err(_) => false,
            }
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
        let plan = self.build_subscriber_reconciliation_plan(node, current_sources, desired_sources)?;
        self.apply_subscriber_reconciliation_plan(plan)?;
        Ok(())
    }

    fn build_subscriber_membership_repair_plan(
        &self,
        sources: &[NodeId],
    ) -> Result<SubscriberReconciliationPlan, SignalError> {
        let mut plan = SubscriberReconciliationPlan::default();

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
                plan.push(source, expected);
            }
        }

        Ok(plan)
    }

    fn build_subscriber_reconciliation_plan(
        &self,
        node: NodeId,
        current_sources: &[NodeId],
        desired_sources: &[NodeId],
    ) -> Result<SubscriberReconciliationPlan, SignalError> {
        let current_sources = sorted_unique_nodes(current_sources);
        let desired_sources = sorted_unique_nodes(desired_sources);
        let mut plan = SubscriberReconciliationPlan::default();
        let mut current_index = 0usize;
        let mut desired_index = 0usize;

        while current_index < current_sources.len() && desired_index < desired_sources.len() {
            let current = current_sources[current_index];
            let desired = desired_sources[desired_index];

            match node_id_sort_key(current).cmp(&node_id_sort_key(desired)) {
                Ordering::Less => {
                    self.plan_subscriber_membership_update(&mut plan, current, node, false)?;
                    current_index += 1;
                }
                Ordering::Greater => {
                    self.plan_subscriber_membership_update(&mut plan, desired, node, true)?;
                    desired_index += 1;
                }
                Ordering::Equal => {
                    current_index += 1;
                    desired_index += 1;
                }
            }
        }

        while current_index < current_sources.len() {
            self.plan_subscriber_membership_update(
                &mut plan,
                current_sources[current_index],
                node,
                false,
            )?;
            current_index += 1;
        }

        while desired_index < desired_sources.len() {
            self.plan_subscriber_membership_update(
                &mut plan,
                desired_sources[desired_index],
                node,
                true,
            )?;
            desired_index += 1;
        }

        Ok(plan)
    }

    fn plan_subscriber_membership_update(
        &self,
        plan: &mut SubscriberReconciliationPlan,
        source: NodeId,
        subscriber: NodeId,
        should_subscribe: bool,
    ) -> Result<(), SignalError> {
        if !self.is_alive(source) {
            return Ok(());
        }

        let current = self.raw_subscribers_of(source)?;
        let mut updated = current.to_vec();
        let changed = match updated.binary_search(&subscriber) {
            Ok(index) if !should_subscribe => {
                updated.remove(index);
                true
            }
            Ok(_) => false,
            Err(index) if should_subscribe => {
                updated.insert(index, subscriber);
                true
            }
            Err(_) => false,
        };

        if changed {
            plan.push(source, updated);
        }

        Ok(())
    }

    fn apply_subscriber_reconciliation_plan(
        &mut self,
        plan: SubscriberReconciliationPlan,
    ) -> Result<(), SignalError> {
        if plan.is_empty() {
            return Ok(());
        }

        for rewrite in plan.rewrites {
            self.set_subscribers_sorted(rewrite.source, &rewrite.subscribers)?;
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

fn sorted_unique_nodes(nodes: &[NodeId]) -> Vec<NodeId> {
    let mut nodes = nodes.to_vec();
    nodes.sort_by_key(|node| node_id_sort_key(*node));
    nodes.dedup();
    nodes
}

fn node_id_sort_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}
