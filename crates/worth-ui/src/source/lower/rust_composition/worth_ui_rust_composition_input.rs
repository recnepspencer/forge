use crate::source::WorthUiRustCompositionModule;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthUiRustCompositionInput {
    modules: Vec<WorthUiRustCompositionModule>,
}

impl WorthUiRustCompositionInput {
    pub(crate) fn from_modules(
        modules: impl IntoIterator<Item = WorthUiRustCompositionModule>,
    ) -> Self {
        Self {
            modules: modules.into_iter().collect(),
        }
    }

    pub(crate) fn modules(&self) -> &[WorthUiRustCompositionModule] {
        &self.modules
    }
}
