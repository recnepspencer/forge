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
        if topology_debug_asserts_enabled() {
            self.assert_bidirectional_consistency()
                .expect("signal topology should remain bidirectionally consistent");
        }
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

    #[cfg(test)]
    pub(crate) fn append_simple_dependency_edge(
        &mut self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        self.validate_handle(node)?;
        self.validate_handle(upstream)?;
        let dependency = self.build_dependency_edge(upstream, aspect, None);
        let mut updated = std::mem::take(&mut self.traversal.topology_dependency_buffer);
        updated.clear();
        updated.extend_from_slice(self.raw_dependencies_of(node)?);

        let changed =
            match updated.binary_search_by(|edge| compare_dependency_edges(edge, &dependency)) {
                Ok(_) => false,
                Err(index) => {
                    updated.insert(index, dependency.clone());
                    self.set_dependency_edges_sorted_with_delta(
                        node,
                        &updated,
                        DependencyTopologyDelta {
                            added_edges: vec![dependency],
                            removed_edges: Vec::new(),
                        },
                    )?;
                    if self.is_alive(upstream) {
                        self.add_subscriber_edge(upstream, node)?;
                    }
                    true
                }
            };

        self.debug_assert_bidirectional_consistency();
        updated.clear();
        self.traversal.topology_dependency_buffer = updated;
        Ok(changed)
    }

    #[cfg(test)]
    pub(crate) fn drop_simple_dependency_edge(
        &mut self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        self.validate_handle(node)?;
        self.validate_handle(upstream)?;
        let mut updated = std::mem::take(&mut self.traversal.topology_dependency_buffer);
        updated.clear();
        updated.extend_from_slice(self.raw_dependencies_of(node)?);
        let mut removed_edges = Vec::new();
        updated.retain(|edge| {
            let should_remove = edge.source() == upstream && edge.aspect() == aspect;
            if should_remove {
                removed_edges.push(edge.clone());
            }
            !should_remove
        });
        let changed = !removed_edges.is_empty();
        if changed {
            let upstream_still_present = updated.iter().any(|edge| edge.source() == upstream);
            self.set_dependency_edges_sorted_with_delta(
                node,
                &updated,
                DependencyTopologyDelta {
                    added_edges: Vec::new(),
                    removed_edges,
                },
            )?;
            if !upstream_still_present && self.is_alive(upstream) {
                self.remove_subscriber_edge(upstream, node)?;
            }
        }

        self.debug_assert_bidirectional_consistency();
        updated.clear();
        self.traversal.topology_dependency_buffer = updated;
        Ok(changed)
    }

    pub(crate) fn reconcile_dependencies(
        &mut self,
        node: NodeId,
        desired: &[DependencyEdge],
    ) -> Result<DependencyReconciliationReport, SignalError> {
        self.validate_handle(node)?;
        for edge in desired {
            self.validate_handle(edge.source())?;
        }
        let analysis = analyze_dependency_reconciliation(self.raw_dependencies_of(node)?, desired);
        if analysis.changed() {
            self.set_dependency_edges_sorted_with_delta(node, desired, analysis.delta)?;
            self.reconcile_dependency_subscribers(
                node,
                &analysis.current_sources,
                &analysis.desired_sources,
            )?;
        }
        self.debug_assert_bidirectional_consistency();
        Ok(analysis.report)
    }

    pub(crate) fn reconcile_dependencies_batch(
        &mut self,
        reconciliations: &[(NodeId, CanonicalDependencies)],
    ) -> Result<Vec<DependencyReconciliationReport>, SignalError> {
        self.reconcile_dependencies_batch_borrowed(
            &reconciliations
                .iter()
                .map(|(node, desired)| (*node, desired.as_slice()))
                .collect::<Vec<_>>(),
        )
    }

    pub(crate) fn reconcile_dependencies_batch_borrowed(
        &mut self,
        reconciliations: &[(NodeId, &[DependencyEdge])],
    ) -> Result<Vec<DependencyReconciliationReport>, SignalError> {
        let mut reports = vec![DependencyReconciliationReport::default(); reconciliations.len()];
        let mut subscriber_ops = Vec::<SubscriberBatchOp>::new();

        for (index, reconciliation) in reconciliations.iter().enumerate() {
            let (node, desired) = reconciliation;
            self.validate_handle(*node)?;
            for edge in *desired {
                self.validate_handle(edge.source())?;
            }
            let analysis =
                analyze_dependency_reconciliation(self.raw_dependencies_of(*node)?, desired);
            reports[index] = analysis.report;
            if !analysis.changed() {
                continue;
            }
            collect_subscriber_batch_ops(
                &mut subscriber_ops,
                *node,
                &analysis.current_sources,
                &analysis.desired_sources,
            );
            self.set_dependency_edges_sorted_with_delta(*node, desired, analysis.delta)?;
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
        let current = self.raw_dependencies_of(node)?;
        let delta = diff_dependency_topology(current, edges);
        if delta.added_edges.is_empty() && delta.removed_edges.is_empty() {
            return Ok(());
        }
        self.set_dependency_edges_sorted_with_delta(node, edges, delta)
    }

    fn set_dependency_edges_sorted_with_delta(
        &mut self,
        node: NodeId,
        edges: &[DependencyEdge],
        delta: DependencyTopologyDelta,
    ) -> Result<(), SignalError> {
        let dependencies_id = self.topology.dependency_edges.insert_from_slice(edges);
        self.get_entry_mut(node)?
            .set_dependencies_id(dependencies_id);
        if !delta.added_edges.is_empty() || !delta.removed_edges.is_empty() {
            self.record_branch_mutation_dependencies(node, delta);
        }
        self.record_graph_storage_pressure();
        Ok(())
    }
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

#[derive(Debug)]
struct DependencyReconciliationAnalysis {
    report: DependencyReconciliationReport,
    current_sources: Vec<NodeId>,
    desired_sources: Vec<NodeId>,
    delta: DependencyTopologyDelta,
}

impl DependencyReconciliationAnalysis {
    fn changed(&self) -> bool {
        self.report.added != 0 || self.report.removed != 0
    }
}

fn analyze_dependency_reconciliation(
    current: &[DependencyEdge],
    desired: &[DependencyEdge],
) -> DependencyReconciliationAnalysis {
    let mut report = DependencyReconciliationReport::default();
    let mut current_sources = Vec::new();
    let mut desired_sources = Vec::new();
    let mut added_edges = Vec::new();
    let mut removed_edges = Vec::new();

    let mut current_index = 0usize;
    let mut desired_index = 0usize;
    let mut last_current_source = None;
    let mut last_desired_source = None;

    while current_index < current.len() && desired_index < desired.len() {
        let current_edge = &current[current_index];
        let desired_edge = &desired[desired_index];
        let current_source = current_edge.source();
        if last_current_source != Some(current_source) {
            current_sources.push(current_source);
            last_current_source = Some(current_source);
        }
        let desired_source = desired_edge.source();
        if last_desired_source != Some(desired_source) {
            desired_sources.push(desired_source);
            last_desired_source = Some(desired_source);
        }

        match compare_dependency_edges(current_edge, desired_edge) {
            Ordering::Less => {
                report.removed += 1;
                removed_edges.push(current_edge.clone());
                current_index += 1;
            }
            Ordering::Greater => {
                report.added += 1;
                added_edges.push(desired_edge.clone());
                desired_index += 1;
            }
            Ordering::Equal => {
                report.unchanged += 1;
                current_index += 1;
                desired_index += 1;
            }
        }
    }

    while current_index < current.len() {
        let current_edge = &current[current_index];
        let current_source = current_edge.source();
        if last_current_source != Some(current_source) {
            current_sources.push(current_source);
            last_current_source = Some(current_source);
        }
        report.removed += 1;
        removed_edges.push(current_edge.clone());
        current_index += 1;
    }

    while desired_index < desired.len() {
        let desired_edge = &desired[desired_index];
        let desired_source = desired_edge.source();
        if last_desired_source != Some(desired_source) {
            desired_sources.push(desired_source);
            last_desired_source = Some(desired_source);
        }
        report.added += 1;
        added_edges.push(desired_edge.clone());
        desired_index += 1;
    }

    DependencyReconciliationAnalysis {
        report,
        current_sources,
        desired_sources,
        delta: DependencyTopologyDelta {
            added_edges,
            removed_edges,
        },
    }
}

fn diff_dependency_topology(
    current: &[DependencyEdge],
    desired: &[DependencyEdge],
) -> DependencyTopologyDelta {
    analyze_dependency_reconciliation(current, desired).delta
}

fn compare_dependency_identity(left: NodeId, right: NodeId) -> Ordering {
    (left.index(), left.generation()).cmp(&(right.index(), right.generation()))
}

#[cfg(debug_assertions)]
fn topology_debug_asserts_enabled() -> bool {
    std::env::var_os("FORGE_SIGNAL_SKIP_TOPOLOGY_DEBUG_ASSERTS").is_none()
}
