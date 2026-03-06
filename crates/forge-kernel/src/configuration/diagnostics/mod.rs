//! Diagnostics vertical slice.
//!
//! DOMAIN: Observability knobs — fingerprint detail, trace verbosity,
//! performance profiling, geometry validation depth, determinism mode,
//! and debug geometry export. Independent of correctness validation.

mod diagnostics_section;

pub use diagnostics_section::DiagnosticsSection;
pub use diagnostics_section::{FingerprintDetail, GeometryValidationDepth, TraceVerbosity};
