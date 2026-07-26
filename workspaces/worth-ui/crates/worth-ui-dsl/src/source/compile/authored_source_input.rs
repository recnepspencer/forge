use std::path::PathBuf;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredSourceInput {
    workspace_root: PathBuf,
    modules: Vec<WorthUiAuthoredSourceModule>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorthUiAuthoredSourceModule {
    relative_path: String,
    source_text: String,
}

impl WorthUiAuthoredSourceInput {
    pub fn rooted_at(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
            modules: Vec::new(),
        }
    }

    pub fn with_module(
        mut self,
        relative_path: impl Into<String>,
        source_text: impl Into<String>,
    ) -> Self {
        self.modules.push(WorthUiAuthoredSourceModule {
            relative_path: relative_path.into(),
            source_text: source_text.into(),
        });
        self
    }

    pub(super) fn into_parts(self) -> (PathBuf, Vec<WorthUiAuthoredSourceModule>) {
        (self.workspace_root, self.modules)
    }
}

impl WorthUiAuthoredSourceModule {
    pub(super) fn into_parts(self) -> (String, String) {
        (self.relative_path, self.source_text)
    }
}
