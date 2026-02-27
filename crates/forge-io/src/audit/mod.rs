//! Audit artifact storage for operation-level traceability and replay.
//!
//! DOMAIN: Versioned, append-only persistence of audit records and trace bundles.
//! INVARIANTS:
//! - Audit envelopes are explicitly versioned
//! - JSONL append writer never rewrites previous entries
//! - Bundle writer emits a deterministic file layout per operation id

pub mod eval;
pub mod replay_bridge;
pub mod schema;

#[cfg(test)]
mod tests;

pub use eval::{
    append_audit_record_jsonl, load_audit_record, save_audit_record, write_audit_bundle,
};
pub use replay_bridge::{
    build_replay_bridge_record, ReplayBridgeRecord, ReplayCompatibility, ReplayWitnessKind,
    ReplayWitnessRef,
};
pub use schema::{
    AuditBundleFiles, AuditBundleManifest, AuditConventionError, AuditFieldLabel,
    AuditIdentityScope, VersionedAuditRecord, AUDIT_BUNDLE_MANIFEST_VERSION, AUDIT_SCHEMA_VERSION,
};
