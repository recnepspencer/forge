#[cfg(test)]
use crate::source::WorthUiResolvedArtifactInputComponentNode;
use crate::source::{WorthUiResolvedArtifactInputNode, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiResolvedArtifactInputModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiResolvedArtifactInputNode>,
}

impl WorthUiResolvedArtifactInputModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        nodes: Vec<WorthUiResolvedArtifactInputNode>,
    ) -> Self {
        Self { module_id, nodes }
    }

    #[cfg(test)]
    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn nodes(&self) -> &[WorthUiResolvedArtifactInputNode] {
        &self.nodes
    }

    #[cfg(test)]
    pub(crate) fn components(&self) -> Vec<&WorthUiResolvedArtifactInputComponentNode> {
        self.nodes
            .iter()
            .filter_map(|node| match node {
                WorthUiResolvedArtifactInputNode::Component(component_node) => Some(component_node),
                _ => None,
            })
            .collect()
    }
}
