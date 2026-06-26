use std::collections::BTreeMap;

use crate::source::{
    WorthUiArtifactInputEquivalentShape, WorthUiArtifactInputModule, WorthUiSourceModuleId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiArtifactInput {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactInputModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiArtifactInput {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiArtifactInputModule>,
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
    ) -> Option<&WorthUiArtifactInputModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
