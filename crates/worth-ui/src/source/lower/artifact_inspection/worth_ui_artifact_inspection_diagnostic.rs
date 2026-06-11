use std::cmp::Ordering;

use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiArtifactInspectionDiagnosticCode {
    ArtifactBasisAlignmentMismatch,
    MissingArtifactSourceOrigin,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiArtifactInspectionDiagnostic {
    code: WorthUiArtifactInspectionDiagnosticCode,
    handle: Option<WorthUiArtifactHandle>,
    detail: String,
}

impl WorthUiArtifactInspectionDiagnostic {
    pub(crate) fn artifact_basis_alignment_mismatch(detail: impl Into<String>) -> Self {
        Self {
            code: WorthUiArtifactInspectionDiagnosticCode::ArtifactBasisAlignmentMismatch,
            handle: None,
            detail: detail.into(),
        }
    }

    pub(crate) fn missing_artifact_source_origin(
        handle: WorthUiArtifactHandle,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            code: WorthUiArtifactInspectionDiagnosticCode::MissingArtifactSourceOrigin,
            handle: Some(handle),
            detail: detail.into(),
        }
    }

    pub(crate) fn code(&self) -> WorthUiArtifactInspectionDiagnosticCode {
        self.code
    }

    pub(crate) fn handle(&self) -> Option<&WorthUiArtifactHandle> {
        self.handle.as_ref()
    }

    pub(crate) fn detail(&self) -> &str {
        &self.detail
    }

    pub(crate) fn stable_cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then_with(|| self.handle.cmp(&other.handle))
            .then_with(|| self.detail.cmp(&other.detail))
    }
}
