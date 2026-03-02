//! Causal replay and diagnosis tooling (P3).
//!
//! DOMAIN: Post-hoc analysis of decision logs. Checkpoint diffing identifies
//! exactly when a divergence was introduced (P3.1). Delta-debug binary search
//! finds the minimal failure-inducing step (P3.2). Divergence scanning detects
//! where f64 fast-paths disagreed with exact arithmetic.

pub mod checkpoint_diff;
pub mod delta_debug;
pub mod divergence;

pub use checkpoint_diff::{diff_decision_logs, CheckpointLog, DecisionChange, DecisionDelta};
pub use delta_debug::{delta_debug, DeltaDebugResult};
pub use divergence::{scan_for_divergences, DivergenceDetail, DivergenceReport};
