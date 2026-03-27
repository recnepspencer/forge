use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    pub(in crate::data::graph) fn raw_dependencies_of(
        &self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        let (dependencies_id, _) = self.node_dependency_ids(node)?;
        Ok(self.topology.dependency_edges.get(dependencies_id))
    }

    pub(in crate::data::graph) fn raw_subscribers_of(
        &self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        let subscribers_id = self.node_subscribers_id(node)?;
        Ok(self.topology.subscriber_edges.get(subscribers_id))
    }

    pub(crate) fn dependency_sources_of(&self, node: NodeId) -> Result<Vec<NodeId>, SignalError> {
        Ok(self
            .raw_dependencies_of(node)?
            .iter()
            .map(|dependency| dependency.source())
            .collect())
    }
}
