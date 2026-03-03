//! Error data shapes for the Forge geometry kernel.

pub mod ambiguous_result;
pub mod diagnostic_payload;
pub mod kernel_error;
pub mod merge_error;
pub mod topology_error;

pub use ambiguous_result::AmbiguousResult;
pub use diagnostic_payload::DiagnosticPayload;
pub use kernel_error::{ErrorContext, ErrorScope, KernelError, SuggestedFix};
pub use merge_error::{
    MergeError, PersistentResolutionIncompatibility, PersistentResolutionOriginKind,
    PersistentResolutionRole,
};
pub use topology_error::TopologyError;
