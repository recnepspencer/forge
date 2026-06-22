use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryBoundaryAuditErrorKind {
    EmptyCrateName,
    EmptySourceLabel,
    EmptySourcePath,
    EmptySourceText,
    DuplicateSourceLabel,
    RustParseFailed,
    MissingRequiredRoot,
    SourceInventoryReadFailed,
}

impl ForgeQueryBoundaryAuditErrorKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EmptyCrateName => "empty-crate-name",
            Self::EmptySourceLabel => "empty-source-label",
            Self::EmptySourcePath => "empty-source-path",
            Self::EmptySourceText => "empty-source-text",
            Self::DuplicateSourceLabel => "duplicate-source-label",
            Self::RustParseFailed => "rust-parse-failed",
            Self::MissingRequiredRoot => "missing-required-root",
            Self::SourceInventoryReadFailed => "source-inventory-read-failed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBoundaryAuditError {
    kind: ForgeQueryBoundaryAuditErrorKind,
    source_label: Option<String>,
    message: String,
}

impl ForgeQueryBoundaryAuditError {
    pub(crate) fn new(kind: ForgeQueryBoundaryAuditErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            source_label: None,
            message: message.into(),
        }
    }

    pub(crate) fn for_source(
        kind: ForgeQueryBoundaryAuditErrorKind,
        source_label: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            source_label: Some(source_label.into()),
            message: message.into(),
        }
    }

    pub fn kind(&self) -> ForgeQueryBoundaryAuditErrorKind {
        self.kind
    }

    pub fn source_label(&self) -> Option<&str> {
        self.source_label.as_deref()
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for ForgeQueryBoundaryAuditError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.kind.as_str(), self.message)
    }
}

impl std::error::Error for ForgeQueryBoundaryAuditError {}
