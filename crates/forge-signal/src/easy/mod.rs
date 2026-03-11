//! Convenience-only typed wrapper around `SignalGraph`.
//!
//! This module optimizes for approachability and small examples, not kernel-grade
//! execution performance or fully static contracts. Heavyweight/runtime-critical
//! integrations should prefer the prepared/runtime APIs directly.

mod compute;
mod runtime;
mod signal;

pub use compute::ComputeContext;
pub use runtime::ReactiveGraph;
pub use signal::{ComputedSignal, InputSignal, Signal};
