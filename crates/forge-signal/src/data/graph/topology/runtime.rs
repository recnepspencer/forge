use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    pub(crate) fn runtime_dependencies_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        self.prune_stale_dependencies(node)?;
        self.raw_dependencies_of(node)
    }

    pub(crate) fn runtime_subscribers_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        self.prune_stale_subscribers(node)?;
        self.raw_subscribers_of(node)
    }

    fn prune_stale_dependencies(&mut self, node: NodeId) -> Result<(), SignalError> {
        let has_stale = {
            let current = self.raw_dependencies_of(node)?;
            current.iter().any(|edge| !self.is_alive(edge.source()))
        };
        if has_stale {
            let updated = self
                .raw_dependencies_of(node)?
                .iter()
                .filter(|edge| self.is_alive(edge.source()))
                .cloned()
                .collect::<Vec<_>>();
            self.set_dependency_edges_sorted(node, &updated)?;
        }
        Ok(())
    }

    fn prune_stale_subscribers(&mut self, node: NodeId) -> Result<(), SignalError> {
        let has_stale = {
            let current = self.raw_subscribers_of(node)?;
            current.iter().any(|subscriber| !self.is_alive(*subscriber))
        };
        if has_stale {
            let updated = self
                .raw_subscribers_of(node)?
                .iter()
                .copied()
                .filter(|subscriber| self.is_alive(*subscriber))
                .collect::<Vec<_>>();
            self.set_subscribers_sorted(node, &updated)?;
        }
        Ok(())
    }
}
