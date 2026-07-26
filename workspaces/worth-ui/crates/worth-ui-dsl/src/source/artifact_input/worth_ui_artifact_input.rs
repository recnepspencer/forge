use std::collections::BTreeMap;

#[cfg(test)]
use crate::source::WorthUiArtifactInputEquivalentShape;
use crate::source::{WorthUiArtifactInputModule, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiArtifactInput {
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

    pub fn module(&self, module_id: &WorthUiSourceModuleId) -> Option<&WorthUiArtifactInputModule> {
        self.modules.get(module_id)
    }

    pub fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
