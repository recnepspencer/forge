#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticOutcomeKind {
    Accepted,
    Advisory,
    Denied,
    Unsupported,
    Deferred,
    Partial,
    Mismatch,
    Violation,
}

impl FoundationalDiagnosticOutcomeKind {
    pub const fn canonical_name(&self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
            Self::Unsupported => "unsupported",
            Self::Deferred => "deferred",
            Self::Partial => "partial",
            Self::Mismatch => "mismatch",
            Self::Violation => "violation",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalDiagnosticAbsenceCause {
    NotRetained,
    Redacted,
    Unsupported,
    ReconstructionDenied,
    MissingEvidence,
}

impl FoundationalDiagnosticAbsenceCause {
    pub const fn canonical_name(&self) -> &'static str {
        match self {
            Self::NotRetained => "not_retained",
            Self::Redacted => "redacted",
            Self::Unsupported => "unsupported",
            Self::ReconstructionDenied => "reconstruction_denied",
            Self::MissingEvidence => "missing_evidence",
        }
    }
}
