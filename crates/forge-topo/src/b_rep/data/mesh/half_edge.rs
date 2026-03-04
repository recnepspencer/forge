//! Data shape for the HalfEdge entity.
//!
//! DOMAIN: A directed use of a geometric edge, bounding one face.
//!
//! Connectivity only — metadata (bridge flags, coedge info, direction)
//! lives in slot-parallel side-car vectors on `TopologyArena`.

use serde::{Deserialize, Serialize};

use crate::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};

/// Data stored for each halfedge — 6 connectivity pointers, nothing else.
///
/// # Radial-Edge Structure
///
/// Each geometric edge is shared by a ring of halfedges linked via
/// `radial_next`. For manifold edges (the common case), the ring has
/// exactly 2 halfedges: `radial_next(radial_next(he)) == he`. For
/// non-manifold edges (3+ faces sharing an edge), the ring is longer.
/// For boundary edges (open shells), `radial_next == self`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalfEdgeData {
    /// Next halfedge in the radial ring around the same geometric edge.
    radial_next: HalfEdgeId,
    next: HalfEdgeId,
    prev: HalfEdgeId,
    face: FaceId,
    origin: VertexId,
    edge: EdgeId,
}

impl HalfEdgeData {
    /// Construct a new halfedge with all connectivity fields.
    pub fn new(
        radial_next: HalfEdgeId,
        next: HalfEdgeId,
        prev: HalfEdgeId,
        face: FaceId,
        origin: VertexId,
        edge: EdgeId,
    ) -> Self {
        Self {
            radial_next,
            next,
            prev,
            face,
            origin,
            edge,
        }
    }

    /// Next halfedge in the radial ring around the same geometric edge.
    ///
    /// For manifold edges, `radial_next(radial_next(he)) == he` (pair).
    /// For boundary edges, `radial_next == self` (self-radial).
    /// For non-manifold edges, the ring has 3+ halfedges.
    pub fn radial_next(&self) -> HalfEdgeId {
        self.radial_next
    }

    /// The next halfedge in the face loop.
    pub fn next(&self) -> HalfEdgeId {
        self.next
    }

    /// The previous halfedge in the face loop.
    pub fn prev(&self) -> HalfEdgeId {
        self.prev
    }

    /// The face this halfedge borders.
    pub fn face(&self) -> FaceId {
        self.face
    }

    /// The origin vertex.
    pub fn origin(&self) -> VertexId {
        self.origin
    }

    /// The owning undirected edge.
    pub fn edge(&self) -> EdgeId {
        self.edge
    }

    /// Set the next halfedge in the radial ring.
    pub fn set_radial_next(&mut self, id: HalfEdgeId) {
        self.radial_next = id;
    }

    /// Set the next halfedge.
    pub fn set_next(&mut self, id: HalfEdgeId) {
        self.next = id;
    }

    /// Set the previous halfedge.
    pub fn set_prev(&mut self, id: HalfEdgeId) {
        self.prev = id;
    }

    /// Set the face this halfedge borders.
    pub fn set_face(&mut self, id: FaceId) {
        self.face = id;
    }

    /// Set the origin vertex.
    pub fn set_origin(&mut self, id: VertexId) {
        self.origin = id;
    }

    /// Set the owning undirected edge.
    pub fn set_edge(&mut self, id: EdgeId) {
        self.edge = id;
    }
}

