use std::cmp::Ordering;

use crate::source::{WorthUiArtifactInputProvenance, WorthUiSourceModuleId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiBindingDiagnosticCode {
    MissingSemanticSurfaceIconReference,
    DeferredSemanticSurfaceIconReference,
    UnsupportedSemanticSurfaceIconReference,
    PlatformInternalSemanticSurfaceIconReference,
    InvalidSemanticCommandReferenceId,
    MissingSemanticCommandIconReference,
    DeferredSemanticCommandIconReference,
    UnsupportedSemanticCommandIconReference,
    PlatformInternalSemanticCommandIconReference,
    MissingSemanticCommandProjectionReference,
    DeferredSemanticCommandProjectionReference,
    UnsupportedSemanticCommandProjectionReference,
    PlatformInternalSemanticCommandProjectionReference,
    MissingSemanticCommandReference,
    DeferredSemanticCommandReference,
    UnsupportedSemanticCommandReference,
    PlatformInternalSemanticCommandReference,
    InvalidSemanticViewBindingReferenceId,
    MissingSemanticViewBindingReference,
    DeferredSemanticViewBindingReference,
    UnsupportedSemanticViewBindingReference,
    PlatformInternalSemanticViewBindingReference,
    InvalidSemanticThemeTokenReferenceId,
    MissingSemanticThemeTokenReference,
    DeferredSemanticThemeTokenReference,
    UnsupportedSemanticThemeTokenReference,
    PlatformInternalSemanticThemeTokenReference,
    LocalPseudoQueryClaimRejected,
    MissingQueryCapabilityPosture,
    MissingQueryCompositionSupportProfile,
    MissingQueryViewShape,
    MissingQueryResultShape,
    MissingQueryBasisPosture,
    MissingQueryLiveCompatibility,
    MissingQueryDenialPresentation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiBindingDiagnostic {
    code: WorthUiBindingDiagnosticCode,
    module_id: WorthUiSourceModuleId,
    authored_text: String,
    semantic_locus: String,
    provenance: WorthUiArtifactInputProvenance,
}

impl WorthUiBindingDiagnostic {
    pub(crate) fn new(
        code: WorthUiBindingDiagnosticCode,
        module_id: WorthUiSourceModuleId,
        authored_text: impl Into<String>,
        semantic_locus: impl Into<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            code,
            module_id,
            authored_text: authored_text.into(),
            semantic_locus: semantic_locus.into(),
            provenance,
        }
    }

    pub(crate) fn code(&self) -> WorthUiBindingDiagnosticCode {
        self.code
    }

    pub(crate) fn stable_cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then_with(|| self.module_id.cmp(&other.module_id))
            .then_with(|| self.semantic_locus.cmp(&other.semantic_locus))
            .then_with(|| self.authored_text.cmp(&other.authored_text))
            .then_with(|| stable_provenance_cmp(&self.provenance, &other.provenance))
    }
}

fn stable_provenance_cmp(
    left: &WorthUiArtifactInputProvenance,
    right: &WorthUiArtifactInputProvenance,
) -> Ordering {
    match (left, right) {
        (
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span: left_declaration,
                detail_span: left_detail,
            },
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span: right_declaration,
                detail_span: right_detail,
            },
        ) => stable_span_cmp(left_declaration, right_declaration)
            .then_with(|| stable_optional_span_cmp(left_detail.as_ref(), right_detail.as_ref())),
        (
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
                authored_module_path: left_path,
                declaration_index: left_index,
            },
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
                authored_module_path: right_path,
                declaration_index: right_index,
            },
        ) => left_path
            .cmp(right_path)
            .then_with(|| left_index.cmp(&right_index)),
        (WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }, _) => Ordering::Less,
        (_, WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }) => Ordering::Greater,
    }
}

fn stable_optional_span_cmp(
    left: Option<&crate::source::WorthUiSourceSpan>,
    right: Option<&crate::source::WorthUiSourceSpan>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => stable_span_cmp(left, right),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn stable_span_cmp(
    left: &crate::source::WorthUiSourceSpan,
    right: &crate::source::WorthUiSourceSpan,
) -> Ordering {
    left.module_id()
        .cmp(right.module_id())
        .then_with(|| left.start_byte().cmp(&right.start_byte()))
        .then_with(|| left.end_byte().cmp(&right.end_byte()))
}
