use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::runtime::graph::{EdgeTopology, SignalGraph};

impl SignalGraph {
    pub(crate) fn runtime_dependencies_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[DependencyEdge], SignalError> {
        EdgeTopology::prune_dead_dependency_edges(self, node)?;
        self.raw_dependencies_of(node)
    }

    pub(crate) fn runtime_subscribers_of(
        &mut self,
        node: NodeId,
    ) -> Result<&[NodeId], SignalError> {
        EdgeTopology::prune_dead_subscriber_edges(self, node)?;
        self.raw_subscribers_of(node)
    }
}
