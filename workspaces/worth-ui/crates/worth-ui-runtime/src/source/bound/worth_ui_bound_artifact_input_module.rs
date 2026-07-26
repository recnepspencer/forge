use crate::source::WorthUiBoundArtifactInputNode;
use worth_ui_dsl::WorthUiSourceModuleId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInputModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiBoundArtifactInputNode>,
}

impl WorthUiBoundArtifactInputModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        nodes: Vec<WorthUiBoundArtifactInputNode>,
    ) -> Self {
        Self { module_id, nodes }
    }

    pub(crate) fn nodes(&self) -> &[WorthUiBoundArtifactInputNode] {
        &self.nodes
    }
}
