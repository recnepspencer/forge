//! Configuration for mutable draft transactions.

use crate::topology::validators::validate::ValidationLevel;

/// Configuration for a mutable draft transaction.
///
/// Controls opt-in features like per-operation structural hashing
/// and deterministic seeding for reproducible operation sequences.
#[derive(Debug, Clone)]
pub struct DraftConfig {
    /// When true, compute and record the arena's structural signature
    /// after every Euler operation. Enables full replay hash trails
    /// at the cost of O(N) per operation.
    ///
    /// Default: `false` (hash is only computed once at commit time).
    pub per_op_hashing: bool,
    /// Base seed for deterministic RNG during this draft.
    ///
    /// Each operation receives `deterministic_seed + op_counter` as its
    /// entry seed in the replay log, producing unique reproducible seeds.
    ///
    /// Default: `0` (no external seed).
    pub deterministic_seed: u64,
    /// Strictness of topology validation at commit time.
    ///
    /// Default: `Full` in Debug, `Minimal` in Release.
    pub validation_level: ValidationLevel,
    /// When true, verify twin/next/prev consistency after every Euler op.
    ///
    /// Expensive — use only in dev/CI. Default: `false`.
    pub per_op_validation: bool,
}

impl Default for DraftConfig {
    fn default() -> Self {
        Self {
            per_op_hashing: false,
            deterministic_seed: 0,
            validation_level: ValidationLevel::default(),
            per_op_validation: false,
        }
    }
}
