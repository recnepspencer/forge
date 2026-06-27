#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQuerySurfaceMatrixErrorKind {
    EmptyMatrix,
    CurrentPathBuildFailure,
    DuplicateRowIdentity,
    MissingFamilyStageTouchpointRow,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySurfaceMatrixError {
    kind: EvidenceLookupQuerySurfaceMatrixErrorKind,
    message: String,
}

impl EvidenceLookupQuerySurfaceMatrixError {
    pub(crate) fn new(
        kind: EvidenceLookupQuerySurfaceMatrixErrorKind,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupQuerySurfaceMatrixErrorKind {
        self.kind
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
