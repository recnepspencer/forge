//! Operation result envelope for the Forge geometry kernel.
//!
//! DOMAIN: Universal return type wrapping every kernel operation's
//! result alongside decision logs, warnings, metrics, and lineage.
//! An AI agent can reconstruct the full state transition from
//! the `OperationResult<T>` envelope alone.
//!
//! DEPENDENCIES: serde

mod schema;

#[cfg(test)]
mod tests;

pub use schema::{KernelWarning, LineageDelta, OperationMetrics, OperationResult};
