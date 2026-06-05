mod basis;
mod basis_canonical;
mod branches;
mod fork;
mod fork_snapshot;
mod lifecycle;
mod merge_runtime;
mod snapshotting;

pub use basis::{
    bridge_signal_branch_basis_trust_boundary, BoundaryBridgedSignalBranchBasisArtifact,
    SignalBranchBasis, SignalBranchBasisArtifact, SignalBranchBasisAuthority,
    SignalBranchBasisDenial, SignalBranchBasisIdentity, SignalBranchBasisReadmissionAuthority,
    SignalBranchBasisReady, SignalBranchBasisValidationOutcome, SignalBranchHeadPosture,
    SignalBranchRestorePosture, StaleSignalBranchBasisArtifact, SIGNAL_BRANCH_BASIS_SCHEMA_VERSION,
};
pub use basis_canonical::SignalBranchBasisCompactExplanation;
pub(in crate::logic::transaction::runtime) use branches::{
    BranchAncestryState, BranchManager, BranchState, SnapshotBranchState,
};
pub use fork::{
    SignalBranchForkDenial, SignalBranchForkReceipt, SignalBranchForkRequest,
    SignalBranchForkRequestBasis,
};
