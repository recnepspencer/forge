//! Validation checkpoint enum.

use serde::{Deserialize, Serialize};

/// Checkpoints where invariant validation can be triggered.
///
/// This is a shared contract type used by both the configuration system
/// (to specify which checkpoints are active) and the proof system
/// (to execute validation at those checkpoints).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ValidationCheckpoint {
    /// After MutableDraft::commit() — structural invariants.
    PostCommit,
    /// After every Boolean operation.
    PostBoolean,
    /// After every feature evaluation.
    PostFeature,
    /// After STEP/IGES import healing.
    PostImport,
    /// On explicit request only.
    OnDemand,
}
