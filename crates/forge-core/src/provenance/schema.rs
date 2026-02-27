use serde::{Deserialize, Serialize};

use crate::tracing::EntityKind;

/// Serializable snapshot-scoped handle identity.
///
/// This is explicitly not a persistent identity. It captures a typed
/// `(kind, index, generation)` reference valid within a topology snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotHandleRef {
    pub kind: EntityKind,
    pub index: u32,
    pub generation: u32,
}

impl SnapshotHandleRef {
    pub fn new(kind: EntityKind, index: u32, generation: u32) -> Self {
        Self {
            kind,
            index,
            generation,
        }
    }

    pub fn packed_generational(self) -> u64 {
        ((self.generation as u64) << 32) | (self.index as u64)
    }
}

/// Deterministic transport hash for a directed segment between two snapshot handles.
///
/// Intended as a compact join key / transport identifier for certifier and audit
/// records. This is not a replacement for rich provenance fields.
pub fn hash_directed_snapshot_segment_transport(
    start: SnapshotHandleRef,
    end: SnapshotHandleRef,
) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for word in [
        start.kind as u64,
        start.index as u64,
        start.generation as u64,
        end.kind as u64,
        end.index as u64,
        end.generation as u64,
    ] {
        h ^= word;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Deterministic transport hash for an undirected segment between two snapshot handles.
///
/// Endpoint order is canonicalized so `(a,b)` and `(b,a)` produce the same hash.
pub fn hash_undirected_snapshot_segment_transport(
    a: SnapshotHandleRef,
    b: SnapshotHandleRef,
) -> u64 {
    let (start, end) =
        if (a.kind as u8, a.index, a.generation) <= (b.kind as u8, b.index, b.generation) {
            (a, b)
        } else {
            (b, a)
        };
    hash_directed_snapshot_segment_transport(start, end)
}

/// Invariant validation errors for serialized provenance payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProvenanceValidationError {
    InvalidSegmentEndpointKind {
        field: &'static str,
        kind: EntityKind,
    },
    InvalidSourceKind {
        field: &'static str,
        expected: EntityKind,
        actual: EntityKind,
    },
    TransportHashMismatch {
        expected: u64,
        actual: u64,
        directed: bool,
    },
}

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

impl BoundarySegmentProvenance {
    pub fn new(
        start_vertex_snapshot: SnapshotHandleRef,
        end_vertex_snapshot: SnapshotHandleRef,
    ) -> Self {
        Self {
            transport_hash: hash_directed_snapshot_segment_transport(
                start_vertex_snapshot,
                end_vertex_snapshot,
            ),
            start_vertex_snapshot,
            end_vertex_snapshot,
            source_halfedge_snapshot: None,
            source_edge_snapshot: None,
            source_face_snapshot: None,
            directed: true,
        }
    }

    /// Validate payload invariants for audit/replay use.
    pub fn validate(&self) -> Result<(), ProvenanceValidationError> {
        if self.start_vertex_snapshot.kind != EntityKind::Vertex {
            return Err(ProvenanceValidationError::InvalidSegmentEndpointKind {
                field: "start_vertex_snapshot",
                kind: self.start_vertex_snapshot.kind,
            });
        }
        if self.end_vertex_snapshot.kind != EntityKind::Vertex {
            return Err(ProvenanceValidationError::InvalidSegmentEndpointKind {
                field: "end_vertex_snapshot",
                kind: self.end_vertex_snapshot.kind,
            });
        }
        if let Some(h) = self.source_halfedge_snapshot {
            if h.kind != EntityKind::HalfEdge {
                return Err(ProvenanceValidationError::InvalidSourceKind {
                    field: "source_halfedge_snapshot",
                    expected: EntityKind::HalfEdge,
                    actual: h.kind,
                });
            }
        }
        if let Some(e) = self.source_edge_snapshot {
            if e.kind != EntityKind::Edge {
                return Err(ProvenanceValidationError::InvalidSourceKind {
                    field: "source_edge_snapshot",
                    expected: EntityKind::Edge,
                    actual: e.kind,
                });
            }
        }
        if let Some(f) = self.source_face_snapshot {
            if f.kind != EntityKind::Face {
                return Err(ProvenanceValidationError::InvalidSourceKind {
                    field: "source_face_snapshot",
                    expected: EntityKind::Face,
                    actual: f.kind,
                });
            }
        }

        let expected = if self.directed {
            hash_directed_snapshot_segment_transport(
                self.start_vertex_snapshot,
                self.end_vertex_snapshot,
            )
        } else {
            hash_undirected_snapshot_segment_transport(
                self.start_vertex_snapshot,
                self.end_vertex_snapshot,
            )
        };
        if self.transport_hash != expected {
            return Err(ProvenanceValidationError::TransportHashMismatch {
                expected,
                actual: self.transport_hash,
                directed: self.directed,
            });
        }

        Ok(())
    }
}

/// Origin of a merge-step selector decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectorOrigin {
    AutoDerived,
    UserSelector,
    PolicyResolved,
}

/// Serializable provenance payload for an executed merge step.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeStepProvenance {
    pub step_index: u32,
    pub edge_snapshot: SnapshotHandleRef,
    pub survive_face_snapshot: SnapshotHandleRef,
    pub kill_face_snapshot: SnapshotHandleRef,
    pub selector_origin: SelectorOrigin,
}
