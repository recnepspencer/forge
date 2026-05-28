pub(crate) mod application;
mod contracts;
mod facade;
mod local_rewrites;
mod naming_continuity;
mod rejection_locality;
mod replay;

pub(crate) use application::topology_relation_dependency_path;
pub(crate) use application::TopologyOperatorRunner;
pub use application::{TopologyOperatorExecution, TopologyOperatorExecutionError};
pub use contracts::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyEditAction, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditDerivedFallbackPolicy, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditNamingReport, TopologyEditNamingRow, TopologyEditNamingScope,
};
pub use facade::{TopologyEditApplicationMode, TopologyEditBatch};
pub use naming_continuity::NamingEditContinuityMatrix;
pub use rejection_locality::{
    RejectedEditScopeReport, RejectedEditScopeRow, TopologyEditRejectionClass,
};
pub use replay::{TopologyEditDigest, TopologyOperatorDigest};




