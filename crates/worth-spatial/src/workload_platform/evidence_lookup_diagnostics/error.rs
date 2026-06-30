#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupDiagnosticsErrorKind {
    EmptyDiagnosticRows,
    MissingFamilyStageWitness,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupDiagnosticsError {
    kind: EvidenceLookupDiagnosticsErrorKind,
    detail: String,
}

impl EvidenceLookupDiagnosticsError {
    pub(crate) fn new(kind: EvidenceLookupDiagnosticsErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> EvidenceLookupDiagnosticsErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
