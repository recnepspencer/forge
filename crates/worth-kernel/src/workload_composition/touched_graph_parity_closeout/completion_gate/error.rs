#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthTouchedGraphRoadmapCompletionGateErrorKind {
    CurrentCloseoutMatrixUnavailable,
    CurrentReadinessHandoffUnavailable,
    CurrentRepresentativePathUnavailable,
    CurrentPublicCloseoutUnavailable,
    CurrentSourceFirewallCloseoutUnavailable,
    CurrentLiveCoverageLedgerUnavailable,
    MismatchedArchitectureClaim,
    RepresentativePathAuthorityMismatch,
    MissingCoveredFamilyCertification,
    OrdinarySecondOntologyStillReachable,
    SourceFirewallViolation,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTouchedGraphRoadmapCompletionGateError {
    kind: WorthTouchedGraphRoadmapCompletionGateErrorKind,
    detail: String,
}

impl WorthTouchedGraphRoadmapCompletionGateError {
    pub(crate) fn new(
        kind: WorthTouchedGraphRoadmapCompletionGateErrorKind,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> WorthTouchedGraphRoadmapCompletionGateErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
