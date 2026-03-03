//! Serializable provenance payloads for tracing and audit artifacts.
//!
//! STRUCTURE:
//!   data/  — Type definitions (structs, enums)
//!   logic/ — Behavioral impls (hashing, validation)

pub(crate) mod data;
pub(crate) mod logic;

#[cfg(test)]
mod tests;

pub use data::{
    BoundarySegmentProvenance, MergeStepProvenance, ProvenanceValidationError, SelectorOrigin,
    SnapshotHandleRef,
};
pub use logic::transport_hash::{
    hash_directed_snapshot_segment_transport, hash_undirected_snapshot_segment_transport,
};
