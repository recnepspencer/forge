#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupWorkloadCutoverErrorKind {
    RawEvidenceFallbackDenied,
    ScopeExpansionDenied,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupWorkloadCutoverError {
    kind: EvidenceLookupWorkloadCutoverErrorKind,
    detail: String,
}

impl EvidenceLookupWorkloadCutoverError {
    pub(crate) fn new(
        kind: EvidenceLookupWorkloadCutoverErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupWorkloadCutoverErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
