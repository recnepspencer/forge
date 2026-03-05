//! Validation checkpoint enum.

use serde::{Deserialize, Serialize};

/// Checkpoints where invariant validation can be triggered.
///
/// This is a shared contract type used by both the configuration system
/// (to specify which checkpoints are active) and the proof system
/// (to execute validation at those checkpoints).
///
/// `#[repr(u8)]` enables array indexing for O(1) bitmask lookups
/// in `GroupPolicyRuntime`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum ValidationCheckpoint {
    /// After every individual topology operator (within `execute()`).
    PerOp = 0,
    /// After MutableDraft::commit() — structural invariants.
    PostCommit = 1,
    /// After every Boolean operation.
    PostBoolean = 2,
    /// After every feature evaluation.
    PostFeature = 3,
    /// After STEP/IGES import healing.
    PostImport = 4,
    /// On explicit request only.
    OnDemand = 5,
}

impl ValidationCheckpoint {
    /// Number of currently defined checkpoints.
    /// Used for fixed-size arrays in `GroupPolicyRuntime`.
    pub const COUNT: usize = 6;
}
