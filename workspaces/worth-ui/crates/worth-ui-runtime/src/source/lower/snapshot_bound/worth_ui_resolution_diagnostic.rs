use std::cmp::Ordering;
use worth_ui_dsl::{WorthUiArtifactInputProvenance, WorthUiSourceModuleId};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub(crate) enum WorthUiResolutionDiagnosticCode {
    InvalidComponentReferenceId,
    MissingComponentReference,
    DeferredComponentReference,
    UnsupportedComponentReference,
    PlatformInternalComponentReference,
    InvalidSurfaceReferenceId,
    MissingSurfaceReference,
    DeferredSurfaceReference,
    UnsupportedSurfaceReference,
    PlatformInternalSurfaceReference,
    InvalidViewBindingReferenceId,
    MissingViewBindingReference,
    DeferredViewBindingReference,
    UnsupportedViewBindingReference,
    PlatformInternalViewBindingReference,
    InvalidThemeTokenReferenceId,
    MissingThemeTokenReference,
    DeferredThemeTokenReference,
    UnsupportedThemeTokenReference,
    PlatformInternalThemeTokenReference,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiResolutionDiagnostic {
    code: WorthUiResolutionDiagnosticCode,
    module_id: WorthUiSourceModuleId,
    authored_text: String,
    provenance: Box<WorthUiArtifactInputProvenance>,
}

impl WorthUiResolutionDiagnostic {
    pub(crate) fn new(
        code: WorthUiResolutionDiagnosticCode,
        module_id: WorthUiSourceModuleId,
        authored_text: impl Into<String>,
        provenance: WorthUiArtifactInputProvenance,
    ) -> Self {
        Self {
            code,
            module_id,
            authored_text: authored_text.into(),
            provenance: Box::new(provenance),
        }
    }

    #[cfg(test)]
    pub(crate) fn code(&self) -> WorthUiResolutionDiagnosticCode {
        self.code
    }

    #[cfg(test)]
    pub(crate) fn authored_text(&self) -> &str {
        &self.authored_text
    }

    #[cfg(test)]
    pub(crate) fn module_id(&self) -> &WorthUiSourceModuleId {
        &self.module_id
    }

    #[cfg(test)]
    pub(crate) fn provenance(&self) -> &WorthUiArtifactInputProvenance {
        &self.provenance
    }

    pub(crate) fn stable_cmp(&self, other: &Self) -> Ordering {
        self.code
            .cmp(&other.code)
            .then_with(|| self.module_id.cmp(&other.module_id))
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
                declaration_index: left_index,
            },
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span: right_declaration,
                detail_span: right_detail,
                declaration_index: right_index,
            },
        ) => stable_span_cmp(left_declaration, right_declaration)
            .then_with(|| stable_optional_span_cmp(left_detail.as_ref(), right_detail.as_ref()))
            .then_with(|| left_index.cmp(right_index)),
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
            .then_with(|| left_index.cmp(right_index)),
        (WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }, _) => Ordering::Less,
        (_, WorthUiArtifactInputProvenance::ParsedSourceDeclaration { .. }) => Ordering::Greater,
    }
}

fn stable_optional_span_cmp(
    left: Option<&worth_ui_dsl::WorthUiSourceSpan>,
    right: Option<&worth_ui_dsl::WorthUiSourceSpan>,
) -> Ordering {
    match (left, right) {
        (Some(left), Some(right)) => stable_span_cmp(left, right),
        (None, Some(_)) => Ordering::Less,
        (Some(_), None) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn stable_span_cmp(
    left: &worth_ui_dsl::WorthUiSourceSpan,
    right: &worth_ui_dsl::WorthUiSourceSpan,
) -> Ordering {
    left.module_id()
        .cmp(right.module_id())
        .then_with(|| left.start_byte().cmp(&right.start_byte()))
        .then_with(|| left.end_byte().cmp(&right.end_byte()))
}
