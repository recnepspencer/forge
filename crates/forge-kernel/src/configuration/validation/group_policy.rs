//! Group-level validation policy configuration.
//!
//! DOMAIN: User-facing knobs that control which invariant groups
//! run at which checkpoints. Stable, serializable, lives in
//! `ValidationSection`. Merged with `TopologyContext` at draft
//! creation time to produce `GroupPolicyRuntime`.

use forge_core::{InvariantGroup, ValidatorCost, ValidationCheckpoint};
use serde::{Deserialize, Serialize};

/// User-facing validation policy configuration.
///
/// Controls group-level skip/defer/cost behavior. This is the "config
/// time" half — it doesn't know about topology kind or body state.
/// Those are resolved at draft creation via `GroupPolicyRuntime::resolve()`.
///
/// # Default behavior
///
/// - **Debug builds**: no skips, no deferrals, Expensive allowed everywhere
///   (same as today's `validate_all_invariants_per_op` behavior).
/// - **Release builds**: Semantic-tier groups deferred to PostCommit,
///   PerOp cost ceiling is Cheap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupPolicyConfig {
    /// Groups the user explicitly wants to skip (overrides kind-based defaults).
    /// Example: skip `ShellClosure` on a wire-only workflow.
    pub force_skip: Vec<InvariantGroup>,

    /// Groups the user explicitly wants to run per-op (overrides tier-based deferrals).
    /// Example: force `EulerFormula` per-op for debugging.
    pub force_per_op: Vec<InvariantGroup>,

    /// Per-checkpoint cost ceiling.
    /// Indexed by `[ValidationCheckpoint as u8]`.
    /// Validators with cost above this ceiling are skipped at that checkpoint.
    pub max_cost_by_checkpoint: [ValidatorCost; ValidationCheckpoint::COUNT],
}

impl GroupPolicyConfig {
    /// Bitmask of user-forced skips.
    pub fn force_skip_mask(&self) -> u32 {
        self.force_skip.iter().fold(0u32, |acc, g| acc | g.mask())
    }

    /// Bitmask of user-forced per-op (un-defers these groups).
    pub fn force_per_op_mask(&self) -> u32 {
        self.force_per_op.iter().fold(0u32, |acc, g| acc | g.mask())
    }
}

impl Default for GroupPolicyConfig {
    fn default() -> Self {
        if cfg!(debug_assertions) {
            Self {
                force_skip: vec![],
                force_per_op: vec![],
                max_cost_by_checkpoint: [ValidatorCost::Expensive; ValidationCheckpoint::COUNT],
            }
        } else {
            let mut costs = [ValidatorCost::Expensive; ValidationCheckpoint::COUNT];
            costs[ValidationCheckpoint::PerOp as usize] = ValidatorCost::Cheap;
            Self {
                force_skip: vec![],
                force_per_op: vec![],
                max_cost_by_checkpoint: costs,
            }
        }
    }
}
