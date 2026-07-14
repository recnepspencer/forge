use std::collections::BTreeMap;

use crate::source::{WorthUiParsedSourceModule, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiParsedSourcePackage {
    modules: BTreeMap<WorthUiSourceModuleId, WorthUiParsedSourceModule>,
    canonical_module_order: Vec<WorthUiSourceModuleId>,
}

impl WorthUiParsedSourcePackage {
    pub(crate) fn new(
        modules: BTreeMap<WorthUiSourceModuleId, WorthUiParsedSourceModule>,
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
    ) -> Option<&WorthUiParsedSourceModule> {
        self.modules.get(module_id)
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.canonical_module_order
    }

    #[cfg(test)]
    pub(crate) fn equivalent_shape(&self, other: &Self) -> bool {
        self.canonical_module_order == other.canonical_module_order
            && self.modules.len() == other.modules.len()
            && self.modules.iter().all(|(module_id, left_module)| {
                other
                    .modules
                    .get(module_id)
                    .is_some_and(|right_module| left_module.equivalent_shape(right_module))
            })
    }
}
