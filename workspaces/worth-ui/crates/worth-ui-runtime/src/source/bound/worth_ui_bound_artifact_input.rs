use std::collections::BTreeMap;
use worth_ui_dsl::WorthUiSourceModuleId;

#[cfg(test)]
use crate::source::WorthUiBoundArtifactInputEquivalentShape;
use crate::source::WorthUiBoundArtifactInputModule;

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

    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiBoundArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
