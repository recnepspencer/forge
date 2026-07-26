use std::path::PathBuf;

use worth_ui_dsl::WorthUiRustAuthoredArtifactInput;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiSourceProvider {
    kind: WorthUiSourceProviderKind,
    id: String,
    workspace_root: PathBuf,
    source_modules: Vec<WorthUiProvidedSourceModule>,
    rust_authored_inputs: Vec<WorthUiRustAuthoredArtifactInput>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiSourceProviderKind {
    Filesystem,
    EditorBuffer,
    Generated,
    InMemory,
    RustAuthoredComposition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiProvidedSourceModule {
    relative_path: String,
    source_text: String,
}

impl WorthUiSourceProvider {
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

    pub fn rust_authored(id: impl Into<String>) -> Self {
        Self::new(
            WorthUiSourceProviderKind::RustAuthoredComposition,
            id,
            PathBuf::from("."),
        )
    }

    pub fn with_file(
        mut self,
        relative_path: impl Into<String>,
        source_text: impl Into<String>,
    ) -> Self {
        assert_ne!(
            self.kind,
            WorthUiSourceProviderKind::Filesystem,
            "filesystem snapshots cannot accept caller-injected source text"
        );
        self.source_modules.push(WorthUiProvidedSourceModule {
            relative_path: relative_path.into(),
            source_text: source_text.into(),
        });
        self
    }

    pub fn with_rust_authored_input(mut self, input: WorthUiRustAuthoredArtifactInput) -> Self {
        self.rust_authored_inputs.push(input);
        self
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.source_modules.is_empty() && self.rust_authored_inputs.is_empty()
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

    pub(crate) fn rust_authored_inputs(&self) -> &[WorthUiRustAuthoredArtifactInput] {
        &self.rust_authored_inputs
    }

    pub(crate) fn final_package_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        fold_text(&mut digest, "worth-ui:source-provider-revision:v1");
        fold_text(&mut digest, self.kind.source_revision_tag());
        fold_text(&mut digest, &self.id);

        let mut modules = self.source_modules.iter().collect::<Vec<_>>();
        modules.sort_by(|left, right| {
            left.relative_path
                .cmp(&right.relative_path)
                .then_with(|| left.source_text.cmp(&right.source_text))
        });
        fold_u64(&mut digest, modules.len() as u64);
        for module in modules {
            fold_text(&mut digest, &module.relative_path);
            fold_text(&mut digest, &module.source_text);
        }

        let mut rust_inputs = self
            .rust_authored_inputs
            .iter()
            .map(WorthUiRustAuthoredArtifactInput::source_revision_digest)
            .collect::<Vec<_>>();
        rust_inputs.sort_unstable();
        fold_u64(&mut digest, rust_inputs.len() as u64);
        for input_digest in rust_inputs {
            fold_u64(&mut digest, input_digest);
        }
        digest
    }
}

fn fold_text(digest: &mut u64, text: &str) {
    fold_u64(digest, text.len() as u64);
    for byte in text.as_bytes() {
        *digest ^= u64::from(*byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

fn fold_u64(digest: &mut u64, value: u64) {
    for byte in value.to_le_bytes() {
        *digest ^= u64::from(byte);
        *digest = digest.wrapping_mul(0x100_0000_01b3);
    }
}

impl WorthUiSourceProviderKind {
    fn source_revision_tag(self) -> &'static str {
        match self {
            Self::Filesystem => "filesystem",
            Self::EditorBuffer => "editor-buffer",
            Self::Generated => "generated",
            Self::InMemory => "in-memory",
            Self::RustAuthoredComposition => "rust-authored-composition",
        }
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
    pub(crate) fn filesystem_snapshot(
        workspace_root: PathBuf,
        source_modules: impl IntoIterator<Item = (String, String)>,
    ) -> Self {
        let id = workspace_root.to_string_lossy().into_owned();
        Self {
            kind: WorthUiSourceProviderKind::Filesystem,
            id,
            workspace_root,
            source_modules: source_modules
                .into_iter()
                .map(|(relative_path, source_text)| WorthUiProvidedSourceModule {
                    relative_path,
                    source_text,
                })
                .collect(),
            rust_authored_inputs: Vec::new(),
        }
    }

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
            rust_authored_inputs: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::WorthUiSourceProvider;

    #[test]
    fn source_revision_digest_is_order_independent_and_field_delimited() {
        let ordered = WorthUiSourceProvider::in_memory("provider")
            .with_file("app/a.wui", "token a = \"a\";")
            .with_file("app/b.wui", "token b = \"b\";");
        let reordered = WorthUiSourceProvider::in_memory("provider")
            .with_file("app/b.wui", "token b = \"b\";")
            .with_file("app/a.wui", "token a = \"a\";");
        let left_ambiguous_under_delimiter_concatenation =
            WorthUiSourceProvider::in_memory("provider").with_file("a|source:b", "c");
        let right_ambiguous_under_delimiter_concatenation =
            WorthUiSourceProvider::in_memory("provider").with_file("a", "b|source:c");

        assert_eq!(
            ordered.final_package_digest(),
            reordered.final_package_digest()
        );
        assert_ne!(
            left_ambiguous_under_delimiter_concatenation.final_package_digest(),
            right_ambiguous_under_delimiter_concatenation.final_package_digest()
        );
    }
}
