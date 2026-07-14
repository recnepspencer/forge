use crate::source::{WorthUiArtifactNode, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiArtifactNode>,
}

impl WorthUiArtifactModule {
    pub(crate) fn new(module_id: WorthUiSourceModuleId, nodes: Vec<WorthUiArtifactNode>) -> Self {
        Self { module_id, nodes }
    }

    pub(crate) fn nodes(&self) -> &[WorthUiArtifactNode] {
        &self.nodes
    }

    #[cfg(test)]
    pub(crate) fn node(&self, node_index: usize) -> Option<&WorthUiArtifactNode> {
        self.nodes.get(node_index)
    }
}
