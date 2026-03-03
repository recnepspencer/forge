//! Public API surface for the provenance domain.
//!
//! External components depend ONLY on this facade.

pub use super::data::{
    BoundarySegmentProvenance, MergeStepProvenance, ProvenanceValidationError, SelectorOrigin,
    SnapshotHandleRef,
};
pub use super::logic::transport_hash::{
    hash_directed_snapshot_segment_transport, hash_undirected_snapshot_segment_transport,
};
