use crate::source::{
    WorthUiIdentitySeededArtifactInputNode, WorthUiIdentitySeededArtifactInputSurfaceNode,
    WorthUiSourceModuleId,
};

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

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn nodes(&self) -> &[WorthUiIdentitySeededArtifactInputNode] {
        &self.nodes
    }

    pub(crate) fn surfaces(
        &self,
    ) -> impl Iterator<Item = &WorthUiIdentitySeededArtifactInputSurfaceNode> {
        self.nodes.iter().filter_map(|node| match node {
            WorthUiIdentitySeededArtifactInputNode::Surface(surface) => Some(surface),
            _ => None,
        })
    }
}
