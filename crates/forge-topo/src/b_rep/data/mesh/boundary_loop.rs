//! Data shape for the Loop entity.
//!
//! DOMAIN: A closed cycle of halfedges bounding a face.

use serde::{Deserialize, Serialize};

use crate::handles::{FaceId, HalfEdgeId};

/// Data stored for each loop (boundary of a face).
///
/// Each face has at least one loop (outer boundary).
/// Future: inner loops represent holes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoopData {
    half_edge: HalfEdgeId,
    face: FaceId,
}

impl LoopData {
    /// Construct a new loop.
    pub fn new(half_edge: HalfEdgeId, face: FaceId) -> Self {
        Self { half_edge, face }
    }

    /// One halfedge on this loop (entry point for traversal).
    pub fn half_edge(&self) -> HalfEdgeId {
        self.half_edge
    }

    /// The face this loop belongs to.
    pub fn face(&self) -> FaceId {
        self.face
    }

    /// Set the entry halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) {
        self.half_edge = id;
    }

    /// Set the owning face.
    pub fn set_face(&mut self, id: FaceId) {
        self.face = id;
    }
}
