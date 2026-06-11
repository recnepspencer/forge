use crate::source::{WorthUiLegallyStructuredArtifactInputNode, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiLegallyStructuredArtifactInputModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiLegallyStructuredArtifactInputNode>,
}

impl WorthUiLegallyStructuredArtifactInputModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        nodes: Vec<WorthUiLegallyStructuredArtifactInputNode>,
    ) -> Self {
        Self { module_id, nodes }
    }

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn nodes(&self) -> &[WorthUiLegallyStructuredArtifactInputNode] {
        &self.nodes
    }
}
