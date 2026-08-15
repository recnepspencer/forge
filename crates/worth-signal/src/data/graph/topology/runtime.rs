use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::runtime::graph::{EdgeTopology, SignalGraph};

impl SignalGraph {
    pub(crate) fn refresh_runtime_dependencies_of(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        EdgeTopology::prune_dead_dependency_edges(self, node)
    }

    pub(crate) fn runtime_dependencies_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        self.refresh_runtime_dependencies_of(node)?;
        self.current_runtime_dependencies_of(node)
    }

    pub(crate) fn current_runtime_dependencies_of(
        &self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        self.raw_dependencies_of(node)
    }

    pub(crate) fn has_current_unsettled_upstream(
        &self,
        target: NodeId,
    ) -> Result<bool, SignalError> {
        let mut visited = vec![false; self.arena_capacity()];
        let mut stack = self
            .current_runtime_dependencies_of(target)?
            .iter()
            .map(|edge| edge.source())
            .collect::<Vec<_>>();
        while let Some(node) = stack.pop() {
            let index = node.index() as usize;
            if visited.get(index).copied().unwrap_or(false) {
                continue;
            }
            if index >= visited.len() {
                return Err(SignalError::invalid_input(format!(
                    "dependency path references unavailable node {node}"
                )));
            }
            visited[index] = true;
            self.invalidation_performed_counter_state().add(
                crate::data::telemetry::InvalidationPerformedCounter::NonSemanticNodeVisits,
                1,
            );
            if self.get_state(node)? != crate::data::node::NodeState::Clean {
                return Ok(true);
            }
            stack.extend(
                self.current_runtime_dependencies_of(node)?
                    .iter()
                    .map(|edge| edge.source()),
            );
        }
        Ok(false)
    }

    pub(crate) fn refresh_runtime_subscribers_of(
        &mut self,
        node: NodeId,
    ) -> Result<(), SignalError> {
        EdgeTopology::prune_dead_subscriber_edges(self, node)
    }

    pub(crate) fn runtime_subscribers_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        self.refresh_runtime_subscribers_of(node)?;
        self.current_runtime_subscribers_of(node)
    }

    pub(crate) fn current_runtime_subscribers_of(
        &self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        self.raw_subscribers_of(node)
    }
}
