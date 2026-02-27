//! Error taxonomy for the Forge geometry kernel.
//!
//! DOMAIN: Shared error types that every Forge crate speaks.
//! `KernelError` is the primary error type, `TopologyError` captures
//! structural invariant violations, and `ErrorContext` provides
//! machine-actionable diagnostic context.
//!
//! DEPENDENCIES: forge-math (MathError conversion), serde

mod schema;
mod summary;

#[cfg(test)]
mod tests;

pub use schema::{
    AmbiguousResult, DiagnosticPayload, ErrorContext, ErrorScope, KernelError, MergeError,
    PersistentResolutionIncompatibility, PersistentResolutionOriginKind, PersistentResolutionRole,
    SuggestedFix, TopologyError,
};
pub use summary::{
    DiagnosticPayloadSummary, ErrorCategory, ErrorSummary, KernelErrorSummary, MergeErrorSummary,
    SourceErrorSummary, TopologyErrorSummary,
};
