use crate::source::WorthUiSourceModuleId;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiSourceSpan {
    module_id: WorthUiSourceModuleId,
    start_byte: usize,
    end_byte: usize,
}

impl WorthUiSourceSpan {
    pub(crate) fn new(
        module_id: WorthUiSourceModuleId,
        start_byte: usize,
        end_byte: usize,
    ) -> Self {
        Self {
            module_id,
            start_byte,
            end_byte,
        }
    }

    pub fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub fn start_byte(&self) -> usize {
        self.start_byte
    }

    pub fn end_byte(&self) -> usize {
        self.end_byte
    }
}
