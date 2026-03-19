use std::cmp::Ordering;

use crate::data::aspect::Aspect;
use crate::data::dependency::{CanonicalDependencies, DependencyEdge};
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::DependencyTopologyDelta;
use crate::data::handle::NodeId;
use crate::data::output::PartitionSubscription;
use crate::data::proof::DependencyBatchEdit;
use crate::data::proof::OrderedStreamItem;

use super::super::signal_graph::SignalGraph;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct DependencyReconciliationReport {
    pub added: u32,
    pub removed: u32,
    pub unchanged: u32,
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

            for dependency in self
                .topology
                .dependency_edges
                .get(entry.get_dependencies_id())
            {
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

            for &subscriber in self
                .topology
                .subscriber_edges
                .get(entry.get_subscribers_id())
            {
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
    pub(super) fn debug_assert_bidirectional_consistency(&self) {
        #[cfg(debug_assertions)]
        self.assert_bidirectional_consistency()
            .expect("signal topology should remain bidirectionally consistent");
    }

    pub fn set_dependencies(
        &mut self,
        node: NodeId,
        desired: impl IntoIterator<Item = DependencyEdge>,
    ) -> Result<(), SignalError> {
        let desired = self.normalize_dependency_edges(desired)?;
        let _ = self.reconcile_dependencies(node, desired.as_slice())?;
        Ok(())
    }

    pub fn clear_dependencies(&mut self, node: NodeId) -> Result<(), SignalError> {
        self.set_dependencies(node, std::iter::empty())
    }

    #[cfg(test)]
    pub(crate) fn edit_dependencies(
        &mut self,
        node: NodeId,
        edit: impl FnOnce(&mut Vec<DependencyEdge>),
    ) -> Result<(), SignalError> {
        let mut desired = self.dependencies_of(node)?.to_vec();
        edit(&mut desired);
        self.set_dependencies(node, desired)
    }

    pub fn apply_dependency_batch_edit(
        &mut self,
        edit: &DependencyBatchEdit,
    ) -> Result<(), SignalError> {
        let reconciliations = edit
            .as_slice()
            .iter()
            .map(|entry| (entry.node, entry.dependencies.clone()))
            .collect::<Vec<_>>();
        let _ = self.reconcile_dependencies_batch(&reconciliations)?;
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
                self.observation
                    .telemetry
                    .invalidation
                    .partition_interner_growth_delta +=
                    self.observation
                        .partition_interner
                        .token_count()
                        .saturating_sub(token_count_before) as u64;
                DependencyEdge::with_scope(upstream, aspect, scope, interned_scope)
            }
            None => DependencyEdge::new(upstream, aspect),
        }
    }

    fn normalize_dependency_edges(
        &mut self,
        desired: impl IntoIterator<Item = DependencyEdge>,
    ) -> Result<CanonicalDependencies, SignalError> {
        let mut normalized = Vec::new();
        for edge in desired {
            self.validate_handle(edge.source())?;
            normalized.push(self.build_dependency_edge(
                edge.source(),
                edge.aspect(),
                edge.scope_ref().cloned(),
            ));
        }
        Ok(CanonicalDependencies::new(normalized))
    }

    pub(crate) fn reconcile_dependencies(
        &mut self,
        node: NodeId,
        desired: &[DependencyEdge],
    ) -> Result<DependencyReconciliationReport, SignalError> {
        self.validate_handle(node)?;
        let mut report = DependencyReconciliationReport::default();
        let (current_sources, desired_sources) = {
            let current = self.raw_dependencies_of(node)?;
            let current_sources = unique_sources_from_sorted_dependencies(current);
            let desired_sources = unique_sources_from_sorted_dependencies(desired);
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
            (current_sources, desired_sources)
        };

        self.set_dependency_edges_sorted(node, desired)?;
        self.reconcile_dependency_subscribers(node, &current_sources, &desired_sources)?;
        self.debug_assert_bidirectional_consistency();
        Ok(report)
    }

    pub(crate) fn reconcile_dependencies_batch(
        &mut self,
        reconciliations: &[(NodeId, CanonicalDependencies)],
    ) -> Result<Vec<DependencyReconciliationReport>, SignalError> {
        let mut reports = vec![DependencyReconciliationReport::default(); reconciliations.len()];
        let mut subscriber_ops = Vec::<SubscriberBatchOp>::new();

        for (index, reconciliation) in reconciliations.iter().enumerate() {
            let (node, desired) = reconciliation;
            self.validate_handle(*node)?;
            let current = self.raw_dependencies_of(*node)?;
            let desired = desired.as_slice();
            let current_sources = unique_sources_from_sorted_dependencies(current);
            let desired_sources = unique_sources_from_sorted_dependencies(desired);
            let report = reconcile_dependency_slices(current, desired);
            reports[index] = report;
            collect_subscriber_batch_ops(
                &mut subscriber_ops,
                *node,
                &current_sources,
                &desired_sources,
            );
            self.set_dependency_edges_sorted(*node, desired)?;
        }

        self.apply_subscriber_batch_ops(&subscriber_ops)?;
        self.debug_assert_bidirectional_consistency();
        Ok(reports)
    }

    pub(crate) fn set_dependency_edges_sorted(
        &mut self,
        node: NodeId,
        edges: &[DependencyEdge],
    ) -> Result<(), SignalError> {
        let current = self.raw_dependencies_of(node)?.to_vec();
        let dependencies_id = self.topology.dependency_edges.insert_from_slice(edges);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        self.record_branch_mutation_dependencies(
            node,
            DependencyTopologyDelta {
                added_edges: diff_dependency_edges(edges, &current),
                removed_edges: diff_dependency_edges(&current, edges),
            },
        );
        self.record_graph_storage_pressure();
        Ok(())
    }
}

fn diff_dependency_edges(left: &[DependencyEdge], right: &[DependencyEdge]) -> Vec<DependencyEdge> {
    let mut delta = Vec::new();
    let mut left_index = 0usize;
    let mut right_index = 0usize;

    while left_index < left.len() && right_index < right.len() {
        match compare_dependency_edges(&left[left_index], &right[right_index]) {
            Ordering::Less => {
                delta.push(left[left_index].clone());
                left_index += 1;
            }
            Ordering::Equal => {
                left_index += 1;
                right_index += 1;
            }
            Ordering::Greater => {
                right_index += 1;
            }
        }
    }

    if left_index < left.len() {
        delta.extend_from_slice(&left[left_index..]);
    }

    delta
}

fn compare_dependency_edges(left: &DependencyEdge, right: &DependencyEdge) -> Ordering {
    left.sort_key().cmp(&right.sort_key())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct SubscriberBatchOp {
    pub(super) source: NodeId,
    pub(super) subscriber: NodeId,
    pub(super) should_subscribe: bool,
}

impl OrderedStreamItem for SubscriberBatchOp {
    type OrderKey = ((u32, u32), (u32, u32), bool);

    fn order_key(&self) -> Self::OrderKey {
        (
            compare_dependency_identity_key(self.source),
            compare_dependency_identity_key(self.subscriber),
            !self.should_subscribe,
        )
    }
}

fn collect_subscriber_batch_ops(
    ops: &mut Vec<SubscriberBatchOp>,
    node: NodeId,
    current_sources: &[NodeId],
    desired_sources: &[NodeId],
) {
    let mut current_index = 0usize;
    let mut desired_index = 0usize;

    while current_index < current_sources.len() && desired_index < desired_sources.len() {
        let current = current_sources[current_index];
        let desired = desired_sources[desired_index];
        match compare_dependency_identity(current, desired) {
            Ordering::Less => {
                ops.push(SubscriberBatchOp {
                    source: current,
                    subscriber: node,
                    should_subscribe: false,
                });
                current_index += 1;
            }
            Ordering::Greater => {
                ops.push(SubscriberBatchOp {
                    source: desired,
                    subscriber: node,
                    should_subscribe: true,
                });
                desired_index += 1;
            }
            Ordering::Equal => {
                current_index += 1;
                desired_index += 1;
            }
        }
    }

    while current_index < current_sources.len() {
        ops.push(SubscriberBatchOp {
            source: current_sources[current_index],
            subscriber: node,
            should_subscribe: false,
        });
        current_index += 1;
    }

    while desired_index < desired_sources.len() {
        ops.push(SubscriberBatchOp {
            source: desired_sources[desired_index],
            subscriber: node,
            should_subscribe: true,
        });
        desired_index += 1;
    }
}

fn compare_dependency_identity_key(node: NodeId) -> (u32, u32) {
    (node.index(), node.generation())
}

fn reconcile_dependency_slices(
    current: &[DependencyEdge],
    desired: &[DependencyEdge],
) -> DependencyReconciliationReport {
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
    report
}

fn unique_sources_from_sorted_dependencies(edges: &[DependencyEdge]) -> Vec<NodeId> {
    let mut sources = Vec::new();
    let mut last = None;

    for edge in edges {
        let source = edge.source();
        if last != Some(source) {
            sources.push(source);
            last = Some(source);
        }
    }

    sources
}

fn compare_dependency_identity(left: NodeId, right: NodeId) -> Ordering {
    (left.index(), left.generation()).cmp(&(right.index(), right.generation()))
}
