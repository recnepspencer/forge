use std::collections::BTreeMap;

use crate::capability::SurfaceId;
use crate::source::{WorthUiArtifact, WorthUiArtifactHandle, WorthUiArtifactNode};

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(super) struct WorthUiArtifactDependencyIndex {
    handles: Vec<WorthUiArtifactHandle>,
    surface_handles: BTreeMap<SurfaceId, WorthUiArtifactHandle>,
}

pub(super) fn build_dependency_index(artifact: &WorthUiArtifact) -> WorthUiArtifactDependencyIndex {
    let mut handles = Vec::new();
    let mut surface_handles = BTreeMap::new();

    for module_id in artifact.module_ids() {
        let module = artifact.module(module_id).expect("artifact module");
        for node in module.nodes() {
            handles.push(node.handle().clone());
            if let WorthUiArtifactNode::Surface(surface) = node {
                surface_handles.insert(surface.surface().id().clone(), surface.handle().clone());
            }
        }
    }

    WorthUiArtifactDependencyIndex {
        handles,
        surface_handles,
    }
}

impl WorthUiArtifactDependencyIndex {
    pub(super) fn handles(&self) -> &[WorthUiArtifactHandle] {
        &self.handles
    }

    pub(super) fn surface_handle(&self, surface_id: &SurfaceId) -> Option<&WorthUiArtifactHandle> {
        self.surface_handles.get(surface_id)
    }
}
