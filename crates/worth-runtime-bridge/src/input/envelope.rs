//! Canonical truth envelope shapes for bridge-owned committed patch input.

use std::sync::Arc;

use worth_foundational::facade::{
    AspectFieldLocator, AspectKey, AspectLocator, AspectMask, CanonicalFieldPath, MutationMask,
    ProjectionMask,
};

use crate::clone_budget::CheapClone;
use crate::identity::{
    BridgeIdentity, CommittedPatchDigestTag, TruthBranchTag, TruthCommitTag, TruthPatchTag,
};
use crate::snapshot::TruthSnapshotIdentity;

mod canonical;
mod construction;
mod core;
mod semantic;
mod target;

pub use canonical::*;
pub use core::*;
pub use semantic::*;
pub use target::*;
