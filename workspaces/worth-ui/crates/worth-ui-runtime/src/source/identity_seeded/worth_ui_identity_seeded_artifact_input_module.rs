use crate::source::WorthUiIdentitySeededArtifactInputNode;
use worth_ui_dsl::WorthUiSourceModuleId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInputModule {
    module_id: WorthUiSourceModuleId,
    nodes: Vec<WorthUiIdentitySeededArtifactInputNode>,
}

impl WorthUiIdentitySeededArtifactInputModule {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        nodes: Vec<WorthUiIdentitySeededArtifactInputNode>,
    ) -> Self {
        Self { module_id, nodes }
    }

    pub(crate) fn nodes(&self) -> &[WorthUiIdentitySeededArtifactInputNode] {
        &self.nodes
    }
}
