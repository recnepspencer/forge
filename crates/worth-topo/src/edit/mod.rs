mod facade;
mod proof;
mod query_native;
mod types;

pub use facade::{TopologyEditApplicationMode, TopologyEditBatch};
pub use proof::{
    NamingEditContinuityMatrix, RejectedEditScopeReport, RejectedEditScopeRow, TopologyEditDigest,
    TopologyEditRejectionClass,
};
pub(crate) use query_native::TopologyQueryEditRunner;
pub use query_native::{TopologyQueryEditExecution, TopologyQueryEditExecutionError};
pub use types::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyEditAction, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditFamily, TopologyEditNamingOutcome, TopologyEditNamingReport, TopologyEditNamingRow,
    TopologyEditNamingScope,
};

#[cfg(test)]
mod contract_tests;
