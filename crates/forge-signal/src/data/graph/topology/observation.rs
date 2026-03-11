use crate::data::aspect::Aspect;
use crate::data::dependency::DependencyEdge;
use crate::data::error::SignalError;
use crate::data::handle::NodeId;

use super::super::signal_graph::SignalGraph;

impl SignalGraph {
    pub fn dependencies_of(&self, node: NodeId) -> Result<&[DependencyEdge], SignalError> {
        self.raw_dependencies_of(node)
    }

    pub fn subscribers_of(&self, node: NodeId) -> Result<&[NodeId], SignalError> {
        self.raw_subscribers_of(node)
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
}
