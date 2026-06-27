#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupQueryConsumerKitErrorKind {
    QuerySurfaceMatrix,
    EvidenceReport,
    SupportPinning,
    BoundaryAudit,
    ResidueAudit,
    MissingSupportPinRuntimeFamily,
    EmptyCloseout,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryConsumerKitError {
    kind: EvidenceLookupQueryConsumerKitErrorKind,
    detail: String,
}

impl EvidenceLookupQueryConsumerKitError {
    pub(crate) fn new(
        kind: EvidenceLookupQueryConsumerKitErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupQueryConsumerKitErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
