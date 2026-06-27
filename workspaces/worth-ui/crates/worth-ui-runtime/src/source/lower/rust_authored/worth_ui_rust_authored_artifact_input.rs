use crate::source::WorthUiRustAuthoredArtifactInputModule;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub(crate) struct WorthUiRustAuthoredArtifactInput {
    modules: Vec<WorthUiRustAuthoredArtifactInputModule>,
}

impl WorthUiRustAuthoredArtifactInput {
    pub(crate) fn from_modules(
        modules: impl IntoIterator<Item = WorthUiRustAuthoredArtifactInputModule>,
    ) -> Self {
        Self {
            modules: modules.into_iter().collect(),
        }
    }

    pub(crate) fn modules(&self) -> &[WorthUiRustAuthoredArtifactInputModule] {
        &self.modules
    }
}
