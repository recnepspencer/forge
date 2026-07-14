//! Convenience-only typed wrapper around `SignalGraph`.
//!
//! This module optimizes for approachability and small examples, not kernel-grade
//! execution performance or fully static contracts. Heavyweight/runtime-critical
//! integrations should prefer the prepared/runtime APIs directly.

mod compute;
mod observation;
mod runtime;
mod signal;

pub use compute::SignalContext;
pub use runtime::SignalApp;
pub use signal::{ComputedSignal, InputSignal, Signal};

#[cfg(test)]
pub use compute::SignalContext as ComputeContext;
#[cfg(test)]
pub use runtime::SignalApp as ReactiveGraph;
