//! Constructor and validation logic for `BoundarySegmentProvenance`.

use crate::provenance::data::{
    BoundarySegmentProvenance, ProvenanceValidationError, SnapshotHandleRef,
};
use crate::provenance::logic::transport_hash::{
    hash_directed_snapshot_segment_transport, hash_undirected_snapshot_segment_transport,
};
use crate::tracing::EntityKind;

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
