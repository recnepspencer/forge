#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReadinessHandoffErrorKind {
    CurrentRepresentativePathUnavailable,
    CurrentCoverageInventoryUnavailable,
    CurrentLiveCoverageLedgerUnavailable,
    CurrentSelectedRouteUnavailable,
    PlannerSemanticGraphUnavailable,
    SchemaContractRejected,
    MissingRepresentativeFamilyCoverage,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReadinessHandoffError {
    kind: ReadinessHandoffErrorKind,
    detail: String,
}

impl ReadinessHandoffError {
    pub(crate) fn new(kind: ReadinessHandoffErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            detail: detail.into(),
        }
    }

    pub const fn kind(&self) -> ReadinessHandoffErrorKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}
