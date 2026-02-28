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
    /// The curve parameter `t` at which this vertex was born during a
    /// `SplitEdge` operation. `None` for vertices not created by splitting.
    /// The geometry layer uses this to locate the vertex on its parent curve
    /// (e.g., a NURBS edge) without re-deriving the parameter from position.
    birth_parameter: Option<f64>,
}

impl VertexData {
    /// Construct a new vertex with the given outgoing halfedge.
    pub fn new(outgoing: HalfEdgeId) -> Self {
        Self {
            outgoing,
            provenance: None,
            birth_parameter: None,
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

    /// The birth parameter `t` at which this vertex was inserted during `SplitEdge`.
    pub fn birth_parameter(&self) -> Option<f64> {
        self.birth_parameter
    }

    /// Set the outgoing halfedge.
    pub fn set_outgoing(&mut self, id: HalfEdgeId) {
        self.outgoing = id;
    }

    /// Set the 3-plane intersection provenance.
    pub fn set_provenance(&mut self, provenance: Option<[usize; 3]>) {
        self.provenance = provenance;
    }

    /// Set the curve birth parameter (stored during `SplitEdge`).
    pub fn set_birth_parameter(&mut self, t: Option<f64>) {
        self.birth_parameter = t;
    }
}
