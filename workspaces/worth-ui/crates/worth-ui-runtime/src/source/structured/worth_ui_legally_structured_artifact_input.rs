use std::collections::BTreeMap;
use worth_ui_dsl::WorthUiSourceModuleId;

#[cfg(test)]
use crate::source::WorthUiLegallyStructuredArtifactInputEquivalentShape;
use crate::source::WorthUiLegallyStructuredArtifactInputModule;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiLegallyStructuredArtifactInput {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiLegallyStructuredArtifactInputModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiLegallyStructuredArtifactInput {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiLegallyStructuredArtifactInputModule>,
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
    ) -> Option<&WorthUiLegallyStructuredArtifactInputModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        WorthUiLegallyStructuredArtifactInputEquivalentShape::packages_are_equivalent(self, other)
    }
}
