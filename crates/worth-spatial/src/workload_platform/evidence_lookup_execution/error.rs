#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupExecutionErrorKind {
    PlanIndexDigestMismatch,
    SpatialTouchDigestMismatch,
    StageReceiptDigestMismatch,
    UnexpectedExecutionQueryArtifactFamily,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupExecutionError {
    kind: EvidenceLookupExecutionErrorKind,
    detail: String,
}

impl EvidenceLookupExecutionError {
    pub(crate) fn new(kind: EvidenceLookupExecutionErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupExecutionErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
