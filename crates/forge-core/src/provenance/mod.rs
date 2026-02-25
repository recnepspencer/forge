//! Serializable provenance payloads for tracing and audit artifacts.

mod schema;

#[cfg(test)]
mod tests;

pub use schema::{
    SnapshotHandleRef,
    BoundarySegmentProvenance,
    MergeStepProvenance,
    ProvenanceValidationError,
    SelectorOrigin,
    hash_directed_snapshot_segment_transport,
    hash_undirected_snapshot_segment_transport,
};
