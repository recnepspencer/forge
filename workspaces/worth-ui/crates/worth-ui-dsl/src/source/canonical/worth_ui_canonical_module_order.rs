use crate::source::WorthUiSourceModuleId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorthUiCanonicalModuleOrder {
    module_ids: Vec<WorthUiSourceModuleId>,
}

impl WorthUiCanonicalModuleOrder {
    pub(crate) fn from_module_ids(mut module_ids: Vec<WorthUiSourceModuleId>) -> Self {
        module_ids.sort();
        module_ids.dedup();
        Self { module_ids }
    }

    pub(crate) fn module_ids(&self) -> &[WorthUiSourceModuleId] {
        &self.module_ids
    }
}
