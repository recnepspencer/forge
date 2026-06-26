use crate::source::{WorthUiArtifactInputProvenance, WorthUiSourceSpan};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum WorthUiArtifactSourceOrigin {
    ParsedSourceDeclaration {
        declaration_span: WorthUiSourceSpan,
        detail_span: Option<WorthUiSourceSpan>,
    },
    RustAuthoredDeclaration {
        authored_module_path: String,
        declaration_index: usize,
    },
}

impl WorthUiArtifactSourceOrigin {
    pub(crate) fn from_provenance(provenance: &WorthUiArtifactInputProvenance) -> Self {
        match provenance {
            WorthUiArtifactInputProvenance::ParsedSourceDeclaration {
                declaration_span,
                detail_span,
            } => Self::ParsedSourceDeclaration {
                declaration_span: declaration_span.clone(),
                detail_span: detail_span.clone(),
            },
            WorthUiArtifactInputProvenance::RustAuthoredDeclaration {
                authored_module_path,
                declaration_index,
            } => Self::RustAuthoredDeclaration {
                authored_module_path: authored_module_path.clone(),
                declaration_index: *declaration_index,
            },
        }
    }
}
