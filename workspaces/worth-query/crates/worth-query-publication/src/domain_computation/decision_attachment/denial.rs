#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryDomainEvidenceAdmissionDenialKind {
    StaleExecutionBinding,
    MissingRequiredMaterial,
    UndeclaredMaterial,
    MissingRequiredCounter,
    UndeclaredCounter,
    DuplicateCounter,
    CounterMovedBackward,
    CounterAggregateMismatch,
    ProviderCertificationMissing,
    MissingDecisionSummary,
    UndeclaredDecisionSummary,
    DuplicateDecisionSummary,
    InvalidDecisionSummary,
    DecisionSidecarMismatch,
    MissingCandidateSearchSummary,
    UnexpectedCandidateSearchSummary,
    CandidateSearchOverclaim,
    CandidateSidecarMismatch,
    MissingTransformationSummary,
    UnexpectedTransformationSummary,
    TransformationSummaryMismatch,
    TransformationSidecarMismatch,
    InvalidPortableValue,
    LedgerRegression,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainEvidenceAdmissionDenial {
    kind: WorthQueryDomainEvidenceAdmissionDenialKind,
    subject: String,
}

impl WorthQueryDomainEvidenceAdmissionDenial {
    pub(super) fn new(
        kind: WorthQueryDomainEvidenceAdmissionDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryDomainEvidenceAdmissionDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}
