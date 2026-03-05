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
    #[serde(alias = "outgoing")]
    primary_disk: HalfEdgeId,
}

impl VertexData {
    /// Construct a new vertex with the given primary disk entry.
    pub fn new(primary_disk: HalfEdgeId) -> Self {
        Self {
            primary_disk,
        }
    }

    /// Representative halfedge for this vertex's primary disk.
    pub fn primary_disk(&self) -> HalfEdgeId {
        self.primary_disk
    }

    /// Set the primary disk entry.
    pub fn set_primary_disk(&mut self, id: HalfEdgeId) {
        self.primary_disk = id;
    }
}

