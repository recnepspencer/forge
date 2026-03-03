//! Public API surface for the errors domain.
//!
//! External components depend ONLY on this facade.

pub use super::data::{
    AmbiguousResult, DiagnosticPayload, ErrorContext, ErrorScope, KernelError, MergeError,
    PersistentResolutionIncompatibility, PersistentResolutionOriginKind, PersistentResolutionRole,
    SuggestedFix, TopologyError,
};
pub use super::summary::{
    DiagnosticPayloadSummary, ErrorCategory, ErrorSummary, KernelErrorSummary, MergeErrorSummary,
    SourceErrorSummary, TopologyErrorSummary,
};
