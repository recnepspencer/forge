mod branches;
mod lifecycle;
mod merge_runtime;
mod snapshotting;

pub(in crate::logic::transaction::runtime) use branches::{
    BranchAncestryState, BranchManager, BranchState, SnapshotBranchState,
};
