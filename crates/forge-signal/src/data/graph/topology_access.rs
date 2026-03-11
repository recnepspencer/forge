use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn dependencies_of(&self, node: NodeId) -> Result<&[DependencyEdge], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.dependency_edges.get(entry.get_dependencies_id()))
    }

    pub fn subscribers_of(&self, node: NodeId) -> Result<&[NodeId], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.subscriber_edges.get(entry.get_subscribers_id()))
    }

    pub(crate) fn runtime_dependencies_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        self.prune_stale_dependencies(node)?;
        self.dependencies_of(node)
    }

    pub(crate) fn runtime_subscribers_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        self.prune_stale_subscribers(node)?;
        self.subscribers_of(node)
    }

    pub fn depends_on(
        &self,
        node: NodeId,
        upstream: NodeId,
        aspect: Aspect,
    ) -> Result<bool, SignalError> {
        Ok(self
            .dependencies_of(node)?
            .iter()
            .any(|dependency| dependency.source() == upstream && dependency.aspect() == aspect))
    }

    fn prune_stale_dependencies(&mut self, node: NodeId) -> Result<(), SignalError> {
        let has_stale = {
            let current = self.dependencies_of(node)?;
            current.iter().any(|edge| !self.is_alive(edge.source()))
        };
        if has_stale {
            let updated = self
                .dependencies_of(node)?
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
            let current = self.subscribers_of(node)?;
            current.iter().any(|subscriber| !self.is_alive(*subscriber))
        };
        if has_stale {
            let updated = self
                .subscribers_of(node)?
                .iter()
                .copied()
                .filter(|subscriber| self.is_alive(*subscriber))
                .collect::<Vec<_>>();
            self.set_subscribers_sorted(node, &updated)?;
        }
        Ok(())
    }
}
