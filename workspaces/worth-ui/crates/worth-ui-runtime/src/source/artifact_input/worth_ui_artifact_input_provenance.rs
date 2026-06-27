use crate::source::WorthUiSourceSpan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactInputProvenance {
    ParsedSourceDeclaration {
        declaration_span: WorthUiSourceSpan,
        detail_span: Option<WorthUiSourceSpan>,
    },
    RustAuthoredDeclaration {
        authored_module_path: String,
        declaration_index: usize,
    },
}

impl WorthUiArtifactInputProvenance {
    pub(crate) fn parsed_source(
        declaration_span: WorthUiSourceSpan,
        detail_span: Option<WorthUiSourceSpan>,
    ) -> Self {
        Self::ParsedSourceDeclaration {
            declaration_span,
            detail_span,
        }
    }

    pub(crate) fn rust_authored(
        authored_module_path: impl Into<String>,
        declaration_index: usize,
    ) -> Self {
        Self::RustAuthoredDeclaration {
            authored_module_path: authored_module_path.into(),
            declaration_index,
        }
    }
}
