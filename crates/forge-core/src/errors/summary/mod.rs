//! Serializable error-summary types for audit artifacts and machine-readable logs.
//!
//! These summaries are intended for audit/replay records and other serialized
//! outputs where callers need typed failure semantics without relying on
//! `Display` strings or carrying full runtime state.

pub mod diagnostic_payload_summary;
pub mod error_summary;
pub mod merge_error_summary;
pub mod topology_error_summary;

pub use diagnostic_payload_summary::DiagnosticPayloadSummary;
pub use error_summary::{ErrorCategory, ErrorSummary, KernelErrorSummary, SourceErrorSummary};
pub use merge_error_summary::MergeErrorSummary;
pub use topology_error_summary::TopologyErrorSummary;
