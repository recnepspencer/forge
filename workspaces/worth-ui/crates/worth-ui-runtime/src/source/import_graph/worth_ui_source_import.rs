use crate::source::WorthUiSourceModuleId;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct WorthUiSourceImport {
    target_module_id: WorthUiSourceModuleId,
}

impl WorthUiSourceImport {
    pub(crate) fn new(target_module_id: WorthUiSourceModuleId) -> Self {
        Self { target_module_id }
    }

    pub(crate) fn target_module_id(&self) -> &WorthUiSourceModuleId {
        &self.target_module_id
    }
}
