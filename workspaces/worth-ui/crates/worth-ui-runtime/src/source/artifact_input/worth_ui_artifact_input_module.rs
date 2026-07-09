use crate::source::{WorthUiArtifactInputNode, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInputModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiArtifactInputNode>,
}

impl WorthUiArtifactInputModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        nodes: Vec<WorthUiArtifactInputNode>,
    ) -> Self {
        Self { module_id, nodes }
    }

    #[cfg(test)]
    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn nodes(&self) -> &[WorthUiArtifactInputNode] {
        &self.nodes
    }
}
