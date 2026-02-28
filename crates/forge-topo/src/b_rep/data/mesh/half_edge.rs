//! Data shape for the HalfEdge entity.
//!
//! DOMAIN: A directed use of a geometric edge, bounding one face.

use serde::{Deserialize, Serialize};

use crate::handles::{CoedgeRef, EdgeId, FaceId, HalfEdgeId, VertexId};

/// Serde default for the `direction` field (true = aligned).
fn default_direction() -> bool {
    true
}

/// Data stored for each halfedge.
///
/// Radial_next, next, and prev are all explicit pointers.
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
    /// Whether this halfedge is a synthetic zero-width bridge inserted by the
    /// `BridgeEdge` operator. Bridge halfedges absorb an inner loop into the
    /// outer loop and are not geometric boundaries. Fillet and offset algorithms
    /// must skip or treat these differently (concave vs. convex rolling ball).
    is_bridge: bool,
    /// Opaque reference to this halfedge's UV trim curve (coedge) in the
    /// `GeometryStore`. `None` for planar halfedges where the coedge is a
    /// trivial straight line in UV space. `Some` for curved surfaces (Phase 4+).
    coedge: Option<CoedgeRef>,
    /// Whether this coedge's parametric direction is aligned with the parent
    /// Edge's 3D curve direction. `true` = same direction, `false` = reversed.
    /// This is the "sense" in STEP terminology (`ORIENTED_EDGE.orientation`).
    /// Defaults to `true` for planar geometry where coedges don't exist yet.
    #[serde(default = "default_direction")]
    direction: bool,
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
            is_bridge: false,
            coedge: None,
            direction: true,
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

    /// Whether this is a synthetic bridge halfedge (inserted by `BridgeEdge`).
    pub fn is_bridge(&self) -> bool {
        self.is_bridge
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

    /// Mark this halfedge as a synthetic bridge (from `BridgeEdge`).
    pub fn set_bridge(&mut self, value: bool) {
        self.is_bridge = value;
    }

    /// Opaque reference to this halfedge's UV trim curve (None = planar).
    pub fn coedge_ref(&self) -> Option<CoedgeRef> {
        self.coedge
    }

    /// Set the coedge reference (populated by the kernel for curved halfedges).
    pub fn set_coedge_ref(&mut self, r: Option<CoedgeRef>) {
        self.coedge = r;
    }

    /// Whether this coedge's direction is aligned with the parent Edge's 3D curve.
    pub fn direction(&self) -> bool {
        self.direction
    }

    /// Set the coedge direction sense (true = aligned with Edge curve).
    pub fn set_direction(&mut self, d: bool) {
        self.direction = d;
    }
}
