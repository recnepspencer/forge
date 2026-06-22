use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsReportContext {
    workspace_root: PathBuf,
}

impl WorthDocsReportContext {
    pub fn for_workspace_root(workspace_root: impl Into<PathBuf>) -> Self {
        Self {
            workspace_root: workspace_root.into(),
        }
    }

    pub fn current_workspace() -> Self {
        Self::for_workspace_root(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .and_then(Path::parent)
                .expect("worth-kernel crate should live under crates/")
                .to_path_buf(),
        )
    }

    pub fn workspace_root(&self) -> &Path {
        &self.workspace_root
    }

    pub fn crate_docs_dir(&self, crate_name: &str) -> PathBuf {
        self.workspace_root
            .join("crates")
            .join(crate_name)
            .join("docs")
    }
}
