//! Canonical truth envelope shapes for bridge-owned committed patch input.

use std::sync::Arc;

use crate::clone_budget::CheapClone;
use crate::identity::{
    BridgeIdentity, CommittedPatchDigestTag, TruthBranchTag, TruthCommitTag, TruthPatchTag,
};
use crate::snapshot::TruthSnapshotIdentity;

mod canonical;
mod producer_and_raw;

pub use canonical::*;
pub use producer_and_raw::*;
