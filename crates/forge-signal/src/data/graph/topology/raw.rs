use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    pub(in crate::data::graph) fn raw_dependencies_of(
        &self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.dependency_edges.get(entry.get_dependencies_id()))
    }

    pub(in crate::data::graph) fn raw_subscribers_of(
        &self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        let entry = self.get_entry(node)?;
        Ok(self.subscriber_edges.get(entry.get_subscribers_id()))
    }

    pub(crate) fn dependency_sources_of(&self, node: NodeId) -> Result<Vec<NodeId>, SignalError> {
        Ok(self
            .raw_dependencies_of(node)?
            .iter()
            .map(|dependency| dependency.source())
            .collect())
    }
}
