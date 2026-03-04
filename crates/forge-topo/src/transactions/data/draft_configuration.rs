//! Configuration for mutable draft transactions.

use crate::validators::validate::ValidationLevel;

/// Configuration for a mutable draft transaction.
///
/// Controls opt-in features like per-operation structural hashing,
/// deterministic seeding, and invariant validation behavior.
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
    /// Debug override: run ALL invariant validators after every `execute()`
    /// call, regardless of operator contracts.
    ///
    /// Catches misclassified `Unrelated`/`Preserves` that should be `MayBreak`.
    /// Expensive — use only in dev/CI.
    ///
    /// Default: `false`.
    pub validate_all_invariants_per_op: bool,
    /// Macro-op suppression: skip all per-op invariant checks.
    ///
    /// For massive compound operations (booleans, imports) where even
    /// cheap per-op validation adds unacceptable overhead. Defers all
    /// checks to commit-time validation.
    ///
    /// Default: `false`.
    pub suppress_per_op_validation: bool,
}

impl Default for DraftConfig {
    fn default() -> Self {
        Self {
            per_op_hashing: false,
            deterministic_seed: 0,
            validation_level: ValidationLevel::default(),
            validate_all_invariants_per_op: false,
            suppress_per_op_validation: false,
        }
    }
}
