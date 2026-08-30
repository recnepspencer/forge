use crate::domain_computation::primary_graph::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryApplicationContinuationDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    BasisUnavailable,
    RetentionCapacityExhausted,
    RetentionIdentityExhausted,
    SnapshotIdentityExhausted,
    ExpiredBasis,
    BasisReleaseFailed,
    PredicateIndexUnavailable,
    PredicateLookupOverflow,
    ResultLimitExceeded,
    CardinalityMismatch,
    TraversalUnavailable,
    ContinuationIndexUnavailable,
    ContinuationBoundaryRejected,
    ContinuationPageWidthInvalid,
    ContinuationGenerationChanged,
    ProjectionUnavailable,
    Projection(super::super::WorthQueryApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryApplicationContinuationDenial {
    kind: WorthQueryApplicationContinuationDenialKind,
    authorization_denial: Option<Box<WorthQueryOperationAuthorizationDenial>>,
    subject: String,
}

impl WorthQueryApplicationContinuationDenial {
    pub(super) fn new(
        kind: WorthQueryApplicationContinuationDenialKind,
        subject: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            authorization_denial: None,
            subject: subject.into(),
        }
    }

    pub(super) fn from_authorization(denial: WorthQueryOperationAuthorizationDenial) -> Self {
        Self {
            kind: WorthQueryApplicationContinuationDenialKind::Authorization(denial.kind()),
            subject: denial.subject().to_string(),
            authorization_denial: Some(Box::new(denial)),
        }
    }

    pub const fn kind(&self) -> WorthQueryApplicationContinuationDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn authorization_denial(&self) -> Option<&WorthQueryOperationAuthorizationDenial> {
        self.authorization_denial.as_deref()
    }
}

impl std::fmt::Display for WorthQueryApplicationContinuationDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application-query continuation denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryApplicationContinuationDenial {}
