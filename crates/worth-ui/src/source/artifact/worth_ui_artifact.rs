use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactEquivalentShape, WorthUiArtifactHandle, WorthUiArtifactModule,
    WorthUiArtifactNode, WorthUiSourceModuleId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifact {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiArtifact {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactModule>,
        canonical_module_order: Vec<WorthUiSourceModuleId>,
    ) -> Self {
        Self {
            modules,
            canonical_module_order,
        }
    }

    pub(crate) fn module(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> Option<&WorthUiArtifactModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    pub(crate) fn node(&self, handle: &WorthUiArtifactHandle) -> Option<&WorthUiArtifactNode> {
        self.module(handle.module_id())
            .and_then(|module| module.node(handle.node_index()))
    }

    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiArtifactEquivalentShape::artifacts_are_equivalent(self, other)
    }
}
