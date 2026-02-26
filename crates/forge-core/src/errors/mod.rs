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
    ErrorScope, SuggestedFix, ErrorContext,
    KernelError, TopologyError,
    AmbiguousResult, DiagnosticPayload,
    MergeError, PersistentResolutionRole, PersistentResolutionIncompatibility,
    PersistentResolutionOriginKind,
};
pub use summary::{
    ErrorCategory, ErrorSummary, SourceErrorSummary,
    KernelErrorSummary, MergeErrorSummary, TopologyErrorSummary,
    DiagnosticPayloadSummary,
};
