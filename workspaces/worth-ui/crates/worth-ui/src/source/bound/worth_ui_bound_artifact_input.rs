use std::collections::BTreeMap;

use crate::source::{
    WorthUiBoundArtifactInputEquivalentShape, WorthUiBoundArtifactInputModule,
    WorthUiSourceModuleId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiBoundArtifactInput {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiBoundArtifactInputModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiBoundArtifactInput {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiBoundArtifactInputModule>,
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
    ) -> Option<&WorthUiBoundArtifactInputModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiBoundArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
