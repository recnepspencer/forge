//! Error taxonomy for the Forge geometry kernel.
//!
//! DOMAIN: Shared error types that every Forge crate speaks.
//! `KernelError` is the primary error type, `TopologyError` captures
//! structural invariant violations, and `ErrorContext` provides
//! machine-actionable diagnostic context.
//!
//! DEPENDENCIES: forge-math (MathError conversion), serde
//!
//! STRUCTURE:
//!   data/    — Type definitions (enums, structs)
//!   logic/   — Behavioral impls (Display, From, methods)
//!   summary/ — Serializable audit-artifact projections

pub(crate) mod data;
mod logic;
pub(crate) mod summary;

#[cfg(test)]
mod tests;

pub use data::{
    AmbiguousResult, DiagnosticPayload, ErrorContext, ErrorScope, KernelError, MergeError,
    PersistentResolutionIncompatibility, PersistentResolutionOriginKind, PersistentResolutionRole,
    SuggestedFix, TopologyError,
};
pub use summary::{
    DiagnosticPayloadSummary, ErrorCategory, ErrorSummary, KernelErrorSummary, MergeErrorSummary,
    SourceErrorSummary, TopologyErrorSummary,
};
