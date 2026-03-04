//! Data shape for the Vertex entity.
//!
//! DOMAIN: A point in 3D space where edges meet.

use serde::{Deserialize, Serialize};

use crate::handles::HalfEdgeId;

/// Data stored for each vertex.
///
/// The optional `provenance` field stores 3 sorted plane indices
/// that define this vertex as a 3-plane intersection. This survives
/// across chained boolean operations so that cross-solid vertex
/// welding can identify geometrically coincident vertices without
/// re-deriving keys from (potentially changed) face adjacency.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    outgoing: HalfEdgeId,
    provenance: Option<[usize; 3]>,
}

impl VertexData {
    /// Construct a new vertex with the given outgoing halfedge.
    pub fn new(outgoing: HalfEdgeId) -> Self {
        Self {
            outgoing,
            provenance: None,
        }
    }

    /// One outgoing halfedge (for traversal entry).
    pub fn outgoing(&self) -> HalfEdgeId {
        self.outgoing
    }

    /// The 3-plane intersection provenance (sorted plane indices).
    pub fn provenance(&self) -> Option<&[usize; 3]> {
        self.provenance.as_ref()
    }


    /// Set the outgoing halfedge.
    pub fn set_outgoing(&mut self, id: HalfEdgeId) {
        self.outgoing = id;
    }

    /// Set the 3-plane intersection provenance.
    pub fn set_provenance(&mut self, provenance: Option<[usize; 3]>) {
        self.provenance = provenance;
    }

}
