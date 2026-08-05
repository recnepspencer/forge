use super::super::{
    authorized_read::WorthQueryAuthorizedApplicationReadDenial,
    read_execution::WorthQueryApplicationReadExecutionDenialKind,
    WorthQueryApplicationProjectionDenialKind,
};
use crate::domain_computation::primary_graph::{
    WorthQueryOperationAuthorizationDenial, WorthQueryOperationAuthorizationDenialKind,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryBoundedLaneDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    StaleBasisScope(crate::domain_computation::primary_graph::WorthQueryEntityResolutionDenialKind),
    Authorization(WorthQueryOperationAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    StalePreviewSession,
    BasisUnavailable,
    ExpiredBasis,
    BasisReleaseFailed,
    PredicateIndexUnavailable,
    PredicateLookupOverflow,
    ResultLimitExceeded,
    CardinalityMismatch,
    TraversalUnavailable,
    ProjectionUnavailable,
    Projection(WorthQueryApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBoundedLaneDenial {
    kind: WorthQueryBoundedLaneDenialKind,
    authorization_denial: Option<Box<WorthQueryOperationAuthorizationDenial>>,
    subject: String,
}

impl WorthQueryBoundedLaneDenial {
    pub const fn kind(&self) -> WorthQueryBoundedLaneDenialKind {
        self.kind
    }

    pub fn subject(&self) -> &str {
        &self.subject
    }

    pub fn authorization_denial(&self) -> Option<&WorthQueryOperationAuthorizationDenial> {
        self.authorization_denial.as_deref()
    }
}

impl std::fmt::Display for WorthQueryBoundedLaneDenial {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "application query bounded lane denied: {:?} ({})",
            self.kind, self.subject
        )
    }
}

impl std::error::Error for WorthQueryBoundedLaneDenial {}

pub(super) fn map_authorized_read_denial(
    value: WorthQueryAuthorizedApplicationReadDenial,
    subject: &str,
) -> WorthQueryBoundedLaneDenial {
    match value {
        WorthQueryAuthorizedApplicationReadDenial::StalePrincipal => {
            denial(WorthQueryBoundedLaneDenialKind::StalePrincipal, subject)
        }
        WorthQueryAuthorizedApplicationReadDenial::StaleScope => {
            denial(WorthQueryBoundedLaneDenialKind::StaleScope, subject)
        }
        WorthQueryAuthorizedApplicationReadDenial::StaleBasisScope(kind) => denial(
            WorthQueryBoundedLaneDenialKind::StaleBasisScope(kind),
            subject,
        ),
        WorthQueryAuthorizedApplicationReadDenial::Authorization(denial) => {
            authorization_denial(denial)
        }
        WorthQueryAuthorizedApplicationReadDenial::Read(read) => {
            denial(map_read_denial(read.kind()), read.subject())
        }
        WorthQueryAuthorizedApplicationReadDenial::Session => {
            denial(WorthQueryBoundedLaneDenialKind::ForeignPlan, subject)
        }
    }
}

fn map_read_denial(
    kind: WorthQueryApplicationReadExecutionDenialKind,
) -> WorthQueryBoundedLaneDenialKind {
    match kind {
        WorthQueryApplicationReadExecutionDenialKind::PredicateIndexUnavailable => {
            WorthQueryBoundedLaneDenialKind::PredicateIndexUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::PredicateLookupOverflow => {
            WorthQueryBoundedLaneDenialKind::PredicateLookupOverflow
        }
        WorthQueryApplicationReadExecutionDenialKind::ResultLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::ResultLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::CardinalityMismatch => {
            WorthQueryBoundedLaneDenialKind::CardinalityMismatch
        }
        WorthQueryApplicationReadExecutionDenialKind::ProjectionUnavailable => {
            WorthQueryBoundedLaneDenialKind::ProjectionUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::ResultBufferLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::ResultBufferLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::WorkLimitExceeded => {
            WorthQueryBoundedLaneDenialKind::WorkLimitExceeded
        }
        WorthQueryApplicationReadExecutionDenialKind::TargetIdentityIndexUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::TargetIdentityLookupOverflow
        | WorthQueryApplicationReadExecutionDenialKind::TargetIdentityNotFound => {
            WorthQueryBoundedLaneDenialKind::ProjectionUnavailable
        }
        WorthQueryApplicationReadExecutionDenialKind::TraversalUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationIndexUnavailable
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationBoundaryRejected
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationGenerationChanged
        | WorthQueryApplicationReadExecutionDenialKind::ContinuationPageWidthInvalid => {
            WorthQueryBoundedLaneDenialKind::TraversalUnavailable
        }
    }
}

pub(super) fn denial(
    kind: WorthQueryBoundedLaneDenialKind,
    subject: impl Into<String>,
) -> WorthQueryBoundedLaneDenial {
    WorthQueryBoundedLaneDenial {
        kind,
        authorization_denial: None,
        subject: subject.into(),
    }
}

fn authorization_denial(
    denial: WorthQueryOperationAuthorizationDenial,
) -> WorthQueryBoundedLaneDenial {
    WorthQueryBoundedLaneDenial {
        kind: WorthQueryBoundedLaneDenialKind::Authorization(denial.kind()),
        subject: denial.subject().to_string(),
        authorization_denial: Some(Box::new(denial)),
    }
}
