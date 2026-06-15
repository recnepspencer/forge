use std::collections::BTreeMap;

use crate::source::{
    WorthUiIdentitySeededArtifactInputEquivalentShape, WorthUiIdentitySeededArtifactInputModule,
    WorthUiSourceModuleId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiIdentitySeededArtifactInput {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiIdentitySeededArtifactInputModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiIdentitySeededArtifactInput {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiIdentitySeededArtifactInputModule>,
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
    ) -> Option<&WorthUiIdentitySeededArtifactInputModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiIdentitySeededArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
