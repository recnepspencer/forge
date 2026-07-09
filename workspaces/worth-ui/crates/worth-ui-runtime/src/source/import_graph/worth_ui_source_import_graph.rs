use std::collections::BTreeMap;

use crate::source::{WorthUiSourceImport, WorthUiSourceModuleId};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiSourceImportGraph {
    adjacency: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceImport>>,
}

impl WorthUiSourceImportGraph {
    pub(crate) fn new(
        adjacency: BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceImport>>,
    ) -> Self {
        Self { adjacency }
    }

    pub(crate) fn imports_for(
        &self,
        module_id: &WorthUiSourceModuleId,
    ) -> Option<&[WorthUiSourceImport]> {
        self.adjacency.get(module_id).map(Vec::as_slice)
    }

    pub(crate) fn module_ids(&self) -> impl Iterator<Item = &WorthUiSourceModuleId> {
        self.adjacency.keys()
    }

    #[cfg(test)]
    pub(crate) fn adjacency(&self) -> &BTreeMap<WorthUiSourceModuleId, Vec<WorthUiSourceImport>> {
        &self.adjacency
    }
}
