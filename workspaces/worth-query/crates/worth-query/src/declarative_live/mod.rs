mod branch_comparison;
mod canonicalization;
mod predicates;
mod request;
mod session;
mod writeback;

pub use branch_comparison::{
    DeclarativeBranchCompareArtifact, DeclarativeBranchCompareChangeFamily,
    DeclarativeBranchCompareFieldDelta, DeclarativeBranchCompareIdentityClass,
    DeclarativeBranchCompareInputRow, DeclarativeBranchCompareRow, DeclarativeBranchCompareValue,
};
pub use predicates::{
    DeclarativeEqualityFilter, DeclarativeNativeComparisonFilter, DeclarativePredicateFilter,
    DeclarativePresenceFilter, DeclarativePresenceFilterKind, DeclarativeSetMembershipFilter,
    DeclarativeStringContainsFilter,
};
pub use request::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, DeclarativeOrderingField,
    DeclarativeProjectionField,
};
pub use session::{DeclarativeLiveQueryError, DeclarativeLiveQuerySession};
pub use writeback::{
    DeclarativeWritebackArtifact, DeclarativeWritebackChange, DeclarativeWritebackIntent,
    DeclarativeWritebackValue,
};

pub(crate) use canonicalization::{
    canonicalize_declarative_request, validate_declared_traversal_contract,
};
#[cfg(test)]
pub use session::declare_runtime_live_query_session;
pub(crate) use session::declare_runtime_live_query_session_from_admitted_read;
pub use session::declare_runtime_live_query_session_with_grouped_baseline;
#[cfg(test)]
pub use writeback::declare_writeback_from_live_session;

#[cfg(test)]
mod tests;
