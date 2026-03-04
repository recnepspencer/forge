//! Data shape for the Vertex entity.
//!
//! DOMAIN: A point in 3D space where edges meet.
//!
//! Connectivity only — provenance data lives in a slot-parallel
//! side-car vector on `TopologyArena`.

use serde::{Deserialize, Serialize};

use crate::handles::HalfEdgeId;

/// Data stored for each vertex — 1 connectivity pointer, nothing else.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VertexData {
    outgoing: HalfEdgeId,
}

impl VertexData {
    /// Construct a new vertex with the given outgoing halfedge.
    pub fn new(outgoing: HalfEdgeId) -> Self {
        Self {
            outgoing,
        }
    }

    /// One outgoing halfedge (for traversal entry).
    pub fn outgoing(&self) -> HalfEdgeId {
        self.outgoing
    }

    /// Set the outgoing halfedge.
    pub fn set_outgoing(&mut self, id: HalfEdgeId) {
        self.outgoing = id;
    }
}

