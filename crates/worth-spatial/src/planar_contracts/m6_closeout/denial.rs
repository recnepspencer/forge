#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum M6PlanarCloseoutDenialKind {
    MissingPremetabossFamily,
    DuplicatePremetabossFamily,
    MissingLegacyDeletionFamily,
    DuplicateLegacyDeletionFamily,
    MissingQueryBoundaryEvidence,
    QueryBoundaryMismatch,
    BooleanExecutionAlreadyPresent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct M6PlanarCloseoutDenial {
    kind: M6PlanarCloseoutDenialKind,
    reason: String,
}

impl M6PlanarCloseoutDenial {
    pub(crate) fn new(kind: M6PlanarCloseoutDenialKind, reason: impl Into<String>) -> Self {
        Self {
            kind,
            reason: reason.into(),
        }
    }

    pub fn kind(&self) -> M6PlanarCloseoutDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
