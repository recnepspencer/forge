#[cfg(test)]
use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::graph::signal_graph::DependencyTopologyDelta;
use crate::data::handle::NodeId;

use crate::data::graph::signal_graph::SignalGraph;

impl SignalGraph {
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

        let changed = match updated.binary_search_by(|edge| {
            super::classification::compare_dependency_edges(edge, &dependency)
        }) {
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

    #[cfg(test)]
    pub(crate) fn rewire_simple_dependency_edge(
        &mut self,
        node: NodeId,
        old_upstream: NodeId,
        new_upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        self.validate_handle(node)?;
        self.validate_handle(old_upstream)?;
        self.validate_handle(new_upstream)?;
        if old_upstream == new_upstream {
            return Ok(false);
        }

        let old_dependency = self.build_dependency_edge(old_upstream, aspect, None);
        let new_dependency = self.build_dependency_edge(new_upstream, aspect, None);

        let mut updated = std::mem::take(&mut self.traversal.topology_dependency_buffer);
        updated.clear();
        updated.extend_from_slice(self.raw_dependencies_of(node)?);

        let old_index = match updated.binary_search_by(|edge| {
            super::classification::compare_dependency_edges(edge, &old_dependency)
        }) {
            Ok(index) => index,
            Err(_) => {
                self.traversal.topology_dependency_buffer = updated;
                return Ok(false);
            }
        };
        updated.remove(old_index);

        let mut added_edges = Vec::new();
        let mut inserted_new = false;
        match updated.binary_search_by(|edge| {
            super::classification::compare_dependency_edges(edge, &new_dependency)
        }) {
            Ok(_) => {}
            Err(index) => {
                updated.insert(index, new_dependency.clone());
                added_edges.push(new_dependency);
                inserted_new = true;
            }
        }

        let old_upstream_still_present = updated.iter().any(|edge| edge.source() == old_upstream);
        self.set_dependency_edges_sorted_with_delta(
            node,
            &updated,
            DependencyTopologyDelta {
                added_edges,
                removed_edges: vec![old_dependency],
            },
        )?;
        if !old_upstream_still_present && self.is_alive(old_upstream) {
            self.remove_subscriber_edge(old_upstream, node)?;
        }
        if inserted_new && self.is_alive(new_upstream) {
            self.add_subscriber_edge(new_upstream, node)?;
        }

        self.debug_assert_bidirectional_consistency();
        updated.clear();
        self.traversal.topology_dependency_buffer = updated;
        Ok(true)
    }

    pub(super) fn set_dependency_edges_sorted_with_delta(
        &mut self,
        node: NodeId,
        edges: &[DependencyEdge],
        delta: DependencyTopologyDelta,
    ) -> Result<(), SignalError> {
        let dependencies_id = self.topology.dependency_edges.insert_from_slice(edges);
        self.set_dependencies_id_direct(node, dependencies_id)?;
        if !delta.added_edges.is_empty() || !delta.removed_edges.is_empty() {
            self.record_branch_mutation_dependencies(node, delta);
        }
        self.record_graph_storage_pressure();
        Ok(())
    }
}
