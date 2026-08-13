//! Bank kinds for application-query execution lanes.

use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationContinuationDenialKind as QueryContinuation,
    WorthQueryApplicationLiveOpenDenialKind as QueryLive,
    WorthQueryApplicationOneShotDenialKind as QueryOneShot,
    WorthQueryApplicationPreviewSessionDenialKind as QueryPreview,
    WorthQueryApplicationProjectionDenialKind as QueryProjection,
    WorthQueryBoundedLaneDenialKind as QueryBounded,
};

use crate::{BankAuthorizationDenial, BankAuthorizationDenialKind, BankEntityResolutionDenialKind};

use super::admission::{admission, BankApplicationQueryAdmissionDenialKind};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankApplicationPreviewSessionDenialKind {
    Cancelled,
    DeadlineExceeded,
    CurrentTruthUnavailable,
    SessionIdentityExhausted,
    BridgeRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankApplicationProjectionDenialKind {
    FieldNotProjected,
    FieldContractMismatch,
    FieldTypeMismatch,
    FieldOmitted,
    RelationNotProjected,
    RelationContractMismatch,
    RelationCardinalityMismatch,
    RelationOmitted,
    DomainProjectionRejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankApplicationOneShotDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    Authorization(BankAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    BasisUnavailable,
    ExpiredBasis,
    BasisReleaseFailed,
    PredicateIndexUnavailable,
    PredicateLookupOverflow,
    ResultLimitExceeded,
    CardinalityMismatch,
    TraversalUnavailable,
    ProjectionUnavailable,
    Projection(BankApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankBoundedLaneDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    StaleBasisScope(BankEntityResolutionDenialKind),
    Authorization(BankAuthorizationDenialKind),
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
    Projection(BankApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankApplicationContinuationDenialKind {
    ForeignPlan,
    StaleInstalledQuery,
    StalePrincipal,
    StaleScope,
    Authorization(BankAuthorizationDenialKind),
    Cancelled,
    DeadlineExceeded,
    BasisUnavailable,
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
    Projection(BankApplicationProjectionDenialKind),
    ResultBufferLimitExceeded,
    WorkLimitExceeded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankApplicationLiveOpenDenialKind {
    LiveContractMissing,
    BindingMismatch,
    BufferCapacityExceedsInstalled,
    WorkLimitExceedsInstalled,
    Admission(BankApplicationQueryAdmissionDenialKind),
    AuthorizationDenied(BankAuthorizationDenialKind),
    ScopeIdentityUnavailable,
    BasisReleaseFailed,
    ProviderVersionUnavailable,
    BridgeBasisRejected,
}

pub(super) const fn preview(kind: QueryPreview) -> BankApplicationPreviewSessionDenialKind {
    use BankApplicationPreviewSessionDenialKind as Bank;
    match kind {
        QueryPreview::Cancelled => Bank::Cancelled,
        QueryPreview::DeadlineExceeded => Bank::DeadlineExceeded,
        QueryPreview::CurrentTruthUnavailable => Bank::CurrentTruthUnavailable,
        QueryPreview::SessionIdentityExhausted => Bank::SessionIdentityExhausted,
        QueryPreview::BridgeRejected => Bank::BridgeRejected,
    }
}

pub(super) const fn one_shot(kind: QueryOneShot) -> BankApplicationOneShotDenialKind {
    use BankApplicationOneShotDenialKind as Bank;
    match kind {
        QueryOneShot::ForeignPlan => Bank::ForeignPlan,
        QueryOneShot::StaleInstalledQuery => Bank::StaleInstalledQuery,
        QueryOneShot::StalePrincipal => Bank::StalePrincipal,
        QueryOneShot::StaleScope => Bank::StaleScope,
        QueryOneShot::Authorization(kind) => Bank::Authorization(authorization(kind)),
        QueryOneShot::Cancelled => Bank::Cancelled,
        QueryOneShot::DeadlineExceeded => Bank::DeadlineExceeded,
        QueryOneShot::BasisUnavailable => Bank::BasisUnavailable,
        QueryOneShot::ExpiredBasis => Bank::ExpiredBasis,
        QueryOneShot::BasisReleaseFailed => Bank::BasisReleaseFailed,
        QueryOneShot::PredicateIndexUnavailable => Bank::PredicateIndexUnavailable,
        QueryOneShot::PredicateLookupOverflow => Bank::PredicateLookupOverflow,
        QueryOneShot::ResultLimitExceeded => Bank::ResultLimitExceeded,
        QueryOneShot::CardinalityMismatch => Bank::CardinalityMismatch,
        QueryOneShot::TraversalUnavailable => Bank::TraversalUnavailable,
        QueryOneShot::ProjectionUnavailable => Bank::ProjectionUnavailable,
        QueryOneShot::Projection(kind) => Bank::Projection(projection(kind)),
        QueryOneShot::ResultBufferLimitExceeded => Bank::ResultBufferLimitExceeded,
        QueryOneShot::WorkLimitExceeded => Bank::WorkLimitExceeded,
    }
}

pub(super) const fn bounded(kind: QueryBounded) -> BankBoundedLaneDenialKind {
    use BankBoundedLaneDenialKind as Bank;
    match kind {
        QueryBounded::ForeignPlan => Bank::ForeignPlan,
        QueryBounded::StaleInstalledQuery => Bank::StaleInstalledQuery,
        QueryBounded::StalePrincipal => Bank::StalePrincipal,
        QueryBounded::StaleScope => Bank::StaleScope,
        QueryBounded::StaleBasisScope(kind) => {
            Bank::StaleBasisScope(crate::BankEntityResolutionDenial::from_query(kind).kind())
        }
        QueryBounded::Authorization(kind) => Bank::Authorization(authorization(kind)),
        QueryBounded::Cancelled => Bank::Cancelled,
        QueryBounded::DeadlineExceeded => Bank::DeadlineExceeded,
        QueryBounded::StalePreviewSession => Bank::StalePreviewSession,
        QueryBounded::BasisUnavailable => Bank::BasisUnavailable,
        QueryBounded::ExpiredBasis => Bank::ExpiredBasis,
        QueryBounded::BasisReleaseFailed => Bank::BasisReleaseFailed,
        QueryBounded::PredicateIndexUnavailable => Bank::PredicateIndexUnavailable,
        QueryBounded::PredicateLookupOverflow => Bank::PredicateLookupOverflow,
        QueryBounded::ResultLimitExceeded => Bank::ResultLimitExceeded,
        QueryBounded::CardinalityMismatch => Bank::CardinalityMismatch,
        QueryBounded::TraversalUnavailable => Bank::TraversalUnavailable,
        QueryBounded::ProjectionUnavailable => Bank::ProjectionUnavailable,
        QueryBounded::Projection(kind) => Bank::Projection(projection(kind)),
        QueryBounded::ResultBufferLimitExceeded => Bank::ResultBufferLimitExceeded,
        QueryBounded::WorkLimitExceeded => Bank::WorkLimitExceeded,
    }
}

pub(super) const fn continuation(kind: QueryContinuation) -> BankApplicationContinuationDenialKind {
    use BankApplicationContinuationDenialKind as Bank;
    match kind {
        QueryContinuation::ForeignPlan => Bank::ForeignPlan,
        QueryContinuation::StaleInstalledQuery => Bank::StaleInstalledQuery,
        QueryContinuation::StalePrincipal => Bank::StalePrincipal,
        QueryContinuation::StaleScope => Bank::StaleScope,
        QueryContinuation::Authorization(kind) => Bank::Authorization(authorization(kind)),
        QueryContinuation::Cancelled => Bank::Cancelled,
        QueryContinuation::DeadlineExceeded => Bank::DeadlineExceeded,
        QueryContinuation::BasisUnavailable => Bank::BasisUnavailable,
        QueryContinuation::ExpiredBasis => Bank::ExpiredBasis,
        QueryContinuation::BasisReleaseFailed => Bank::BasisReleaseFailed,
        QueryContinuation::PredicateIndexUnavailable => Bank::PredicateIndexUnavailable,
        QueryContinuation::PredicateLookupOverflow => Bank::PredicateLookupOverflow,
        QueryContinuation::ResultLimitExceeded => Bank::ResultLimitExceeded,
        QueryContinuation::CardinalityMismatch => Bank::CardinalityMismatch,
        QueryContinuation::TraversalUnavailable => Bank::TraversalUnavailable,
        QueryContinuation::ContinuationIndexUnavailable => Bank::ContinuationIndexUnavailable,
        QueryContinuation::ContinuationBoundaryRejected => Bank::ContinuationBoundaryRejected,
        QueryContinuation::ContinuationPageWidthInvalid => Bank::ContinuationPageWidthInvalid,
        QueryContinuation::ContinuationGenerationChanged => Bank::ContinuationGenerationChanged,
        QueryContinuation::ProjectionUnavailable => Bank::ProjectionUnavailable,
        QueryContinuation::Projection(kind) => Bank::Projection(projection(kind)),
        QueryContinuation::ResultBufferLimitExceeded => Bank::ResultBufferLimitExceeded,
        QueryContinuation::WorkLimitExceeded => Bank::WorkLimitExceeded,
    }
}

pub(super) const fn live(kind: QueryLive) -> BankApplicationLiveOpenDenialKind {
    use BankApplicationLiveOpenDenialKind as Bank;
    match kind {
        QueryLive::LiveContractMissing => Bank::LiveContractMissing,
        QueryLive::BindingMismatch => Bank::BindingMismatch,
        QueryLive::BufferCapacityExceedsInstalled => Bank::BufferCapacityExceedsInstalled,
        QueryLive::WorkLimitExceedsInstalled => Bank::WorkLimitExceedsInstalled,
        QueryLive::Admission(kind) => Bank::Admission(admission(kind)),
        QueryLive::AuthorizationDenied(kind) => Bank::AuthorizationDenied(authorization(kind)),
        QueryLive::ScopeIdentityUnavailable => Bank::ScopeIdentityUnavailable,
        QueryLive::BasisReleaseFailed => Bank::BasisReleaseFailed,
        QueryLive::ProviderVersionUnavailable => Bank::ProviderVersionUnavailable,
        QueryLive::BridgeBasisRejected => Bank::BridgeBasisRejected,
    }
}

const fn projection(kind: QueryProjection) -> BankApplicationProjectionDenialKind {
    use BankApplicationProjectionDenialKind as Bank;
    match kind {
        QueryProjection::FieldNotProjected => Bank::FieldNotProjected,
        QueryProjection::FieldContractMismatch => Bank::FieldContractMismatch,
        QueryProjection::FieldTypeMismatch => Bank::FieldTypeMismatch,
        QueryProjection::FieldOmitted => Bank::FieldOmitted,
        QueryProjection::RelationNotProjected => Bank::RelationNotProjected,
        QueryProjection::RelationContractMismatch => Bank::RelationContractMismatch,
        QueryProjection::RelationCardinalityMismatch => Bank::RelationCardinalityMismatch,
        QueryProjection::RelationOmitted => Bank::RelationOmitted,
        QueryProjection::DomainProjectionRejected => Bank::DomainProjectionRejected,
    }
}

const fn authorization(
    kind: worth_query_host::facade::primary_graph::WorthQueryOperationAuthorizationDenialKind,
) -> BankAuthorizationDenialKind {
    BankAuthorizationDenial::from_kind(kind, 0).kind()
}
