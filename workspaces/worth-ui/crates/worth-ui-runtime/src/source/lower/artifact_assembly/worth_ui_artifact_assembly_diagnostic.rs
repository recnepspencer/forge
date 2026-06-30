use std::cmp::Ordering;

use crate::source::WorthUiSourceModuleId;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactAssemblyDiagnosticCode {
    DuplicateCanonicalArtifactNodeKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactAssemblyDiagnostic {
    code: WorthUiArtifactAssemblyDiagnosticCode,
    module_id: WorthUiSourceModuleId,
    semantic_locus: String,
    key_text: String,
}

impl WorthUiArtifactAssemblyDiagnostic {
    pub(crate) fn duplicate_canonical_artifact_node_key(
        module_id: WorthUiSourceModuleId,
        semantic_locus: impl Into<String>,
        key_text: impl Into<String>,
    ) -> Self {
        Self {
            code: WorthUiArtifactAssemblyDiagnosticCode::DuplicateCanonicalArtifactNodeKey,
            module_id,
            semantic_locus: semantic_locus.into(),
            key_text: key_text.into(),
        }
    }

    pub(crate) fn code(&self) -> WorthUiArtifactAssemblyDiagnosticCode {
        self.code
    }

    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    pub(crate) fn semantic_locus(&self) -> &str {
        &self.semantic_locus
    }

    pub(crate) fn key_text(&self) -> &str {
        &self.key_text
    }

    pub(crate) fn stable_cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then_with(|| self.module_id.cmp(&other.module_id))
            .then_with(|| self.semantic_locus.cmp(&other.semantic_locus))
            .then_with(|| self.key_text.cmp(&other.key_text))
    }
}
