mod facade;
mod proof;
mod query_native;
mod types;

pub use facade::{WorthTopologyEditApplicationMode, WorthTopologyEditBatch};
pub use proof::{
    WorthNamingEditContinuityMatrix, WorthRejectedEditScopeReport, WorthRejectedEditScopeRow,
    WorthTopologyEditDigest, WorthTopologyEditRejectionClass,
};
pub(crate) use query_native::WorthTopologyQueryEditRunner;
pub use query_native::{WorthTopologyQueryEditExecution, WorthTopologyQueryEditExecutionError};
pub use types::{
    WorthBoundaryMembershipKind, WorthLoopEndpointKind, WorthLoopSuccessorKind,
    WorthShellOrWireMembershipKind, WorthTopologyDerivedRegion, WorthTopologyEditAction,
    WorthTopologyEditChangedScope, WorthTopologyEditContract, WorthTopologyEditFamily,
    WorthTopologyEditNamingOutcome, WorthTopologyEditNamingReport, WorthTopologyEditNamingRow,
    WorthTopologyEditNamingScope,
};

#[cfg(test)]
mod contract_tests;
