//! Boundary segment provenance payload.

use serde::{Deserialize, Serialize};

use super::snapshot_handle_ref::SnapshotHandleRef;

/// Serializable provenance payload for a boundary segment used in certification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundarySegmentProvenance {
    /// Deterministic transport ID used by low-level certifier payloads/joins.
    pub transport_hash: u64,
    /// Start vertex (snapshot-scoped generational handle).
    pub start_vertex_snapshot: SnapshotHandleRef,
    /// End vertex (snapshot-scoped generational handle).
    pub end_vertex_snapshot: SnapshotHandleRef,
    /// Optional source halfedge for traceability back to topology use.
    pub source_halfedge_snapshot: Option<SnapshotHandleRef>,
    /// Optional source edge entity.
    pub source_edge_snapshot: Option<SnapshotHandleRef>,
    /// Optional source face entity.
    pub source_face_snapshot: Option<SnapshotHandleRef>,
    /// Whether the segment direction is semantically significant.
    pub directed: bool,
}
