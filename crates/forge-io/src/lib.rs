//! # forge-io
//!
//! File format support for the Forge geometry kernel.
//!
//! ## Modules
//!
//! - **json** — Versioned JSON serialization (`save_model`, `load_model`, `diff_models`)
//! - **audit** — Versioned audit-record + trace bundle storage
//!
//! Future formats (STEP, STL, IGES) will be added as sibling directories.

#![forbid(unsafe_code)]

pub mod audit;
pub mod json;

/// Error type for IO operations.
#[derive(Debug)]
pub enum IoError {
    /// Standard IO error.
    Io(std::io::Error),
    /// JSON serialization error.
    Json(serde_json::Error),
    /// Schema version mismatch.
    VersionMismatch {
        /// The version found in the file.
        found: u32,
        /// The maximum version this build supports.
        supported: u32,
    },
}

impl From<std::io::Error> for IoError {
    fn from(e: std::io::Error) -> Self {
        IoError::Io(e)
    }
}

impl From<serde_json::Error> for IoError {
    fn from(e: serde_json::Error) -> Self {
        IoError::Json(e)
    }
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IoError::Io(e) => write!(f, "IO error: {}", e),
            IoError::Json(e) => write!(f, "JSON error: {}", e),
            IoError::VersionMismatch { found, supported } => {
                write!(
                    f,
                    "Schema version {} not supported (max: {})",
                    found, supported
                )
            }
        }
    }
}

pub use audit::{
    append_audit_record_jsonl, build_replay_bridge_record, load_audit_record, save_audit_record,
    write_audit_bundle, AuditBundleFiles, AuditBundleManifest, AuditConventionError,
    AuditFieldLabel, AuditIdentityScope, ReplayBridgeRecord, ReplayCompatibility,
    ReplayWitnessKind, ReplayWitnessRef, VersionedAuditRecord, AUDIT_BUNDLE_MANIFEST_VERSION,
    AUDIT_SCHEMA_VERSION,
};
pub use json::diff::{diff_models, ModelChange};
/// Backwards-compatible re-exports.
pub use json::{load_model, save_model, VersionedModel, SCHEMA_VERSION};
