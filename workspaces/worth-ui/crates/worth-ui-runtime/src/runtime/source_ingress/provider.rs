use std::path::PathBuf;

use crate::runtime::source_ingress::digest::fold_texts;
use crate::runtime::source_ingress::watched_artifact_input::WorthUiWatchedArtifactInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceProvider {
    kind: WorthUiSourceProviderKind,
    id: String,
    workspace_root: PathBuf,
    source_modules: Vec<WorthUiProvidedSourceModule>,
    artifact_inputs: Vec<WorthUiWatchedArtifactInput>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSourceProviderKind {
    Filesystem,
    EditorBuffer,
    Generated,
    InMemory,
    RustAuthoredArtifact,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiProvidedSourceModule {
    relative_path: String,
    source_text: String,
}

impl WorthUiSourceProvider {
    pub fn filesystem_root(root: impl Into<PathBuf>) -> Self {
        let root = root.into();
        let id = root.to_string_lossy().into_owned();
        Self::new(WorthUiSourceProviderKind::Filesystem, id, root)
    }

    pub fn editor_buffer(id: impl Into<String>) -> Self {
        Self::new(
            WorthUiSourceProviderKind::EditorBuffer,
            id,
            PathBuf::from("."),
        )
    }

    pub fn generated(id: impl Into<String>) -> Self {
        Self::new(WorthUiSourceProviderKind::Generated, id, PathBuf::from("."))
    }

    pub fn in_memory(id: impl Into<String>) -> Self {
        Self::new(WorthUiSourceProviderKind::InMemory, id, PathBuf::from("."))
    }

    pub fn rust_authored_artifact(id: impl Into<String>) -> Self {
        Self::new(
            WorthUiSourceProviderKind::RustAuthoredArtifact,
            id,
            PathBuf::from("."),
        )
    }

    pub fn with_file(
        mut self,
        relative_path: impl Into<String>,
        source_text: impl Into<String>,
    ) -> Self {
        self.source_modules.push(WorthUiProvidedSourceModule {
            relative_path: relative_path.into(),
            source_text: source_text.into(),
        });
        self
    }

    pub fn with_artifact_input(mut self, input: WorthUiWatchedArtifactInput) -> Self {
        self.artifact_inputs.push(input);
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.source_modules.is_empty() && self.artifact_inputs.is_empty()
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn kind(&self) -> WorthUiSourceProviderKind {
        self.kind
    }

    pub(crate) fn workspace_root(&self) -> &PathBuf {
        &self.workspace_root
    }

    pub(crate) fn source_modules(&self) -> &[WorthUiProvidedSourceModule] {
        &self.source_modules
    }

    pub(crate) fn artifact_inputs(&self) -> &[WorthUiWatchedArtifactInput] {
        &self.artifact_inputs
    }

    pub(crate) fn final_package_digest(&self) -> u64 {
        let mut basis = vec![
            format!("provider-kind:{:?}", self.kind),
            format!("provider-id:{}", self.id),
        ];
        for module in &self.source_modules {
            basis.push(format!(
                "module:{}|source:{}",
                module.relative_path, module.source_text
            ));
        }
        for input in &self.artifact_inputs {
            basis.push(format!("artifact:{}|{}", input.label(), input.digest()));
        }
        basis.sort();
        fold_texts(basis)
    }
}

impl WorthUiProvidedSourceModule {
    pub(crate) fn relative_path(&self) -> &str {
        &self.relative_path
    }

    pub(crate) fn source_text(&self) -> &str {
        &self.source_text
    }
}

impl WorthUiSourceProvider {
    fn new(
        kind: WorthUiSourceProviderKind,
        id: impl Into<String>,
        workspace_root: PathBuf,
    ) -> Self {
        Self {
            kind,
            id: id.into(),
            workspace_root,
            source_modules: Vec::new(),
            artifact_inputs: Vec::new(),
        }
    }
}
