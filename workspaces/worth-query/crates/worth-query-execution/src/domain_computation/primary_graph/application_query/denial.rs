use worth_query_admission::facade::{
    application_query::WorthQueryApplicationQueryParameterDenialKind,
    graph_read_access::WorthQueryGraphReadPlanReviewDenialKind,
};
use worth_query_installation::facade::WorthQueryApplicationQueryInstallationDenialKind;

use crate::domain_computation::primary_graph::WorthQueryOperationAuthorizationDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationQueryAdmissionDenialKind {
    InstalledQuery(WorthQueryApplicationQueryInstallationDenialKind),
    ForeignPrincipal,
    ForeignScope,
    StalePrincipal,
    StaleScope,
    ScopeTypeMismatch,
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    BasisUnsupported,
    ForeignBasis,
    StaleBasis,
    WrongProviderBasis,
    ExpiredBasis,
    BasisUnavailable,
    TruthViewUnavailable,
    ForeignHistoricalReceipt,
    ForeignPreviewSession,
    StalePreviewSession,
    RuntimeSupportUnavailable,
    ForeignContinuation,
    StaleContinuation,
    ContinuationParameterMismatch,
    ContinuationScopeMismatch,
    ContinuationProviderMismatch,
    ContinuationPageWidthUnsupported,
    LaneUnsupported,
    DisclosureGovernanceRequired,
    Parameter(WorthQueryApplicationQueryParameterDenialKind),
    WorkLimitExceeded,
    CanonicalWorkDenied,
    GraphReadPlan(WorthQueryGraphReadPlanReviewDenialKind),
    ExecutionShapeUnsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationQueryAdmissionDenial {
    kind: WorthQueryApplicationQueryAdmissionDenialKind,
    subject: String,
}

impl WorthQueryApplicationQueryAdmissionDenial {
    pub(super) fn new(
        kind: WorthQueryApplicationQueryAdmissionDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            subject: subject.into(),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationQueryAdmissionDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }
}

impl std::fmt::Display for WorthQueryApplicationQueryAdmissionDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application query admission denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryApplicationQueryAdmissionDenial {}
