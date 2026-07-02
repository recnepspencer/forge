use crate::source::WorthUiSourceSpan;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactInputProvenance {
    ParsedSourceDeclaration {
        declaration_span: WorthUiSourceSpan,
        detail_span: Option<WorthUiSourceSpan>,
        declaration_index: usize,
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
        declaration_index: usize,
    ) -> Self {
        Self::ParsedSourceDeclaration {
            declaration_span,
            detail_span,
            declaration_index,
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

    pub(crate) fn module_path(&self) -> &str {
        match self {
            Self::ParsedSourceDeclaration {
                declaration_span, ..
            } => declaration_span.module_id().as_str(),
            Self::RustAuthoredDeclaration {
                authored_module_path,
                ..
            } => authored_module_path,
        }
    }

    pub(crate) fn declaration_index(&self) -> usize {
        match self {
            Self::ParsedSourceDeclaration {
                declaration_index, ..
            }
            | Self::RustAuthoredDeclaration {
                declaration_index, ..
            } => *declaration_index,
        }
    }
}
