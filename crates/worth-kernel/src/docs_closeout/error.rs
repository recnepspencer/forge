use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WorthDocsCloseoutErrorKind {
    Io,
    InvalidMetadata,
    MissingMetadata,
    MissingHeading,
    MissingDoc,
    DuplicateOwnership,
    TopologyDrift,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WorthDocsCloseoutError {
    kind: WorthDocsCloseoutErrorKind,
    path: Option<PathBuf>,
    detail: String,
}

impl WorthDocsCloseoutError {
    pub fn new(
        kind: WorthDocsCloseoutErrorKind,
        path: Option<PathBuf>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            path,
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> &WorthDocsCloseoutErrorKind {
        &self.kind
    }

    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl From<std::io::Error> for WorthDocsCloseoutError {
    fn from(error: std::io::Error) -> Self {
        Self::new(WorthDocsCloseoutErrorKind::Io, None, error.to_string())
    }
}
