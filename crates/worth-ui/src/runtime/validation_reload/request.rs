use crate::runtime::{
    WorthUiObservedAuthoredEdit, WorthUiObservedAuthoredEditDenial, WorthUiSourceProvider,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiValidationReloadRequest {
    modules: Vec<WorthUiValidationReloadSourceModule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthUiValidationReloadSourceModule {
    relative_path: String,
    source_text: String,
}

impl WorthUiValidationReloadRequest {
    pub fn from_source_module(
        relative_path: impl Into<String>,
        source_text: impl Into<String>,
    ) -> Self {
        Self {
            modules: vec![WorthUiValidationReloadSourceModule {
                relative_path: relative_path.into(),
                source_text: source_text.into(),
            }],
        }
    }

    pub fn from_source_modules(
        modules: impl IntoIterator<Item = (impl Into<String>, impl Into<String>)>,
    ) -> Self {
        Self {
            modules: modules
                .into_iter()
                .map(
                    |(relative_path, source_text)| WorthUiValidationReloadSourceModule {
                        relative_path: relative_path.into(),
                        source_text: source_text.into(),
                    },
                )
                .collect(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.modules.is_empty()
    }

    pub(super) fn into_observed_authored_edit(
        self,
    ) -> Result<WorthUiObservedAuthoredEdit, WorthUiObservedAuthoredEditDenial> {
        let provider = self.modules.into_iter().fold(
            WorthUiSourceProvider::in_memory("validation-app-reload"),
            |provider, module| provider.with_file(module.relative_path, module.source_text),
        );
        WorthUiObservedAuthoredEdit::from_source_provider(provider)
    }
}
