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

    pub(super) fn modules(&self) -> &[WorthUiValidationReloadSourceModule] {
        &self.modules
    }
}

impl WorthUiValidationReloadSourceModule {
    pub(crate) fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub(crate) fn source_text(&self) -> &str {
        &self.source_text
    }
}
