//! Data shapes for mesh entities: Face, HalfEdge, Vertex, Loop, Edge.
//!
//! DOMAIN: Defines the per-entity data structs for the core halfedge mesh.
//!
//! DEPENDENCIES: `handles` (typed IDs)

use serde::{Deserialize, Serialize};

use crate::handles::{
    CoedgeRef, CurveRef, EdgeId, FaceId, HalfEdgeId, LoopId, ShellId,
    SurfaceRef, VertexId,
};

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    outer_loop: LoopId,
    inner_loops: Vec<LoopId>,
    shell: ShellId,
    /// Opaque reference to this face's parametric surface in the `GeometryStore`.
    /// `None` for planar faces (the surface is an implicit plane defined by
    /// the face-plane association). `Some` for curved surfaces (Phase 4+).
    surface: Option<SurfaceRef>,
}

impl FaceData {
    /// Construct a new face with the given outer loop and shell.
    pub fn new(outer_loop: LoopId, shell: ShellId) -> Self {
        Self {
            outer_loop,
            inner_loops: Vec::new(),
            shell,
            surface: None,
        }
    }

    /// The outer boundary loop of this face.
    pub fn outer_loop(&self) -> LoopId {
        self.outer_loop
    }



    /// The shell this face belongs to.
    pub fn shell(&self) -> ShellId {
        self.shell
    }

    /// Set the outer boundary loop.
    pub fn set_outer_loop(&mut self, id: LoopId) {
        self.outer_loop = id;
    }

    /// Set the shell this face belongs to.
    pub fn set_shell(&mut self, id: ShellId) {
        self.shell = id;
    }

    /// Inner loops (holes) on this face.
    pub fn inner_loops(&self) -> &[LoopId] {
        &self.inner_loops
    }

    /// Add an inner loop (hole boundary) to this face.
    pub fn add_inner_loop(&mut self, id: LoopId) {
        self.inner_loops.push(id);
    }

    /// Remove an inner loop from this face.
    ///
    /// Returns `true` if the loop was found and removed, `false` otherwise.
    pub fn remove_inner_loop(&mut self, id: LoopId) -> bool {
        if let Some(pos) = self.inner_loops.iter().position(|&l| l == id) {
            self.inner_loops.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of inner loops (rings) on this face.
    pub fn inner_loop_count(&self) -> usize {
        self.inner_loops.len()
    }



    /// Opaque reference to this face's parametric surface (None = planar).
    pub fn surface_ref(&self) -> Option<SurfaceRef> {
        self.surface
    }

    /// Set the surface reference (populated by the kernel for curved faces).
    pub fn set_surface_ref(&mut self, r: Option<SurfaceRef>) {
        self.surface = r;
    }
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

/// Serde default for the `direction` field (true = aligned).
fn default_direction() -> bool {
    true
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

/// Data stored for each undirected edge — owns a representative halfedge.
///
/// All halfedges around this geometric edge form a radial ring linked
/// via `radial_next`. The representative halfedge provides an entry point.
/// Edge-level attributes (fillet radius, crease angle, seam) live here.
///
/// Geometric data (3D curve + tolerance tube) lives in `forge-geom::CurveGeom`
/// and is referenced via the opaque `curve` handle. `EdgeData` never owns
/// or compares `f64` values (Doctrine D3).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    half_edge: HalfEdgeId,
    /// Opaque reference to the edge's 3D curve in `forge-geom::CurveGeom`.
    /// `None` for planar edges (the edge is an implicit plane-plane intersection).
    /// `Some` for curved edges, populated by the kernel in Phase 4+.
    pub curve: Option<CurveRef>,
}

impl EdgeData {
    /// Construct a new edge from one halfedge of the pair.
    pub fn new(half_edge: HalfEdgeId) -> Self {
        Self {
            half_edge,
            curve: None,
        }
    }

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId {
        self.half_edge
    }

    /// The opaque curve reference for this edge (None = planar).
    pub fn curve_ref(&self) -> Option<CurveRef> {
        self.curve
    }

    /// Set the representative halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) {
        self.half_edge = id;
    }

    /// Set the curve reference (populated by the kernel for curved edges).
    pub fn set_curve_ref(&mut self, id: Option<CurveRef>) {
        self.curve = id;
    }
}
