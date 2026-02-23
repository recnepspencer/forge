//! Data shapes for arena-stored topology entities.
//!
//! DOMAIN: Defines the per-entity data structs (Face, HalfEdge, Vertex, Loop,
//! Shell, Edge) and the generational `Slot` wrapper.
//!
//! DEPENDENCIES: `handles` (typed IDs), `lineage` (inline provenance)

use serde::{Deserialize, Serialize};

use crate::handles::{FaceId, HalfEdgeId, VertexId, LoopId, ShellId, EdgeId};
use crate::lineage::Lineage;

/// A slot in the arena that may be occupied or vacant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Slot<T: Clone> {
    /// The current generation of this slot.
    pub(crate) generation: u32,
    /// The current version of the data in this slot (increments on mutation).
    pub(crate) version: u32,
    /// The data, if the slot is occupied.
    pub(crate) data: Option<T>,
}

impl<T: Clone> Slot<T> {
    /// Create a new empty slot at generation 0.
    pub(crate) fn empty() -> Self {
        Self {
            generation: 0,
            version: 0,
            data: None,
        }
    }

    /// Occupy this slot with data, returning the current generation.
    /// Resets version to 0.
    pub(crate) fn occupy(&mut self, data: T) -> u32 {
        self.data = Some(data);
        self.version = 0;
        self.generation
    }
}

/// Data stored for each face.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceData {
    outer_loop: LoopId,
    inner_loops: Vec<LoopId>,
    shell: ShellId,
    lineage: Option<Lineage>,
}

impl FaceData {
    /// Construct a new face with the given outer loop and shell.
    pub fn new(outer_loop: LoopId, shell: ShellId) -> Self {
        Self { outer_loop, inner_loops: Vec::new(), shell, lineage: None }
    }

    /// Construct a new face with lineage.
    pub fn with_lineage(outer_loop: LoopId, shell: ShellId, lineage: Option<Lineage>) -> Self {
        Self { outer_loop, inner_loops: Vec::new(), shell, lineage }
    }

    /// The outer boundary loop of this face.
    pub fn outer_loop(&self) -> LoopId { self.outer_loop }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// The shell this face belongs to.
    pub fn shell(&self) -> ShellId { self.shell }

    /// Set the outer boundary loop.
    pub fn set_outer_loop(&mut self, id: LoopId) { self.outer_loop = id; }

    /// Set the shell this face belongs to.
    pub fn set_shell(&mut self, id: ShellId) { self.shell = id; }

    /// Inner loops (holes) on this face.
    pub fn inner_loops(&self) -> &[LoopId] { &self.inner_loops }

    /// Add an inner loop (hole boundary) to this face.
    pub fn add_inner_loop(&mut self, id: LoopId) { self.inner_loops.push(id); }

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
    pub fn inner_loop_count(&self) -> usize { self.inner_loops.len() }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }
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
    lineage: Option<Lineage>,
    /// Whether this halfedge is a synthetic zero-width bridge inserted by the
    /// `BridgeEdge` operator. Bridge halfedges absorb an inner loop into the
    /// outer loop and are not geometric boundaries. Fillet and offset algorithms
    /// must skip or treat these differently (concave vs. convex rolling ball).
    is_bridge: bool,
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
        Self { radial_next, next, prev, face, origin, edge, lineage: None, is_bridge: false }
    }

    /// Construct a new halfedge with lineage.
    pub fn with_lineage(
        radial_next: HalfEdgeId,
        next: HalfEdgeId,
        prev: HalfEdgeId,
        face: FaceId,
        origin: VertexId,
        edge: EdgeId,
        lineage: Option<Lineage>,
    ) -> Self {
        Self { radial_next, next, prev, face, origin, edge, lineage, is_bridge: false }
    }

    /// Next halfedge in the radial ring around the same geometric edge.
    ///
    /// For manifold edges, `radial_next(radial_next(he)) == he` (pair).
    /// For boundary edges, `radial_next == self` (self-radial).
    /// For non-manifold edges, the ring has 3+ halfedges.
    pub fn radial_next(&self) -> HalfEdgeId { self.radial_next }

    /// The next halfedge in the face loop.
    pub fn next(&self) -> HalfEdgeId { self.next }

    /// The previous halfedge in the face loop.
    pub fn prev(&self) -> HalfEdgeId { self.prev }

    /// The face this halfedge borders.
    pub fn face(&self) -> FaceId { self.face }

    /// The origin vertex.
    pub fn origin(&self) -> VertexId { self.origin }

    /// The owning undirected edge.
    pub fn edge(&self) -> EdgeId { self.edge }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// Whether this is a synthetic bridge halfedge (inserted by `BridgeEdge`).
    pub fn is_bridge(&self) -> bool { self.is_bridge }

    /// Set the next halfedge in the radial ring.
    pub fn set_radial_next(&mut self, id: HalfEdgeId) { self.radial_next = id; }

    /// Set the next halfedge.
    pub fn set_next(&mut self, id: HalfEdgeId) { self.next = id; }

    /// Set the previous halfedge.
    pub fn set_prev(&mut self, id: HalfEdgeId) { self.prev = id; }

    /// Set the face this halfedge borders.
    pub fn set_face(&mut self, id: FaceId) { self.face = id; }

    /// Set the origin vertex.
    pub fn set_origin(&mut self, id: VertexId) { self.origin = id; }

    /// Set the owning undirected edge.
    pub fn set_edge(&mut self, id: EdgeId) { self.edge = id; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }

    /// Mark this halfedge as a synthetic bridge (from `BridgeEdge`).
    pub fn set_bridge(&mut self, value: bool) { self.is_bridge = value; }
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
    lineage: Option<Lineage>,
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
        Self { outgoing, lineage: None, provenance: None, birth_parameter: None }
    }

    /// Construct a new vertex with lineage.
    pub fn with_lineage(outgoing: HalfEdgeId, lineage: Option<Lineage>) -> Self {
        Self { outgoing, lineage, provenance: None, birth_parameter: None }
    }

    /// One outgoing halfedge (for traversal entry).
    pub fn outgoing(&self) -> HalfEdgeId { self.outgoing }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// The 3-plane intersection provenance (sorted plane indices).
    pub fn provenance(&self) -> Option<&[usize; 3]> { self.provenance.as_ref() }

    /// The birth parameter `t` at which this vertex was inserted during `SplitEdge`.
    pub fn birth_parameter(&self) -> Option<f64> { self.birth_parameter }

    /// Set the outgoing halfedge.
    pub fn set_outgoing(&mut self, id: HalfEdgeId) { self.outgoing = id; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }

    /// Set the 3-plane intersection provenance.
    pub fn set_provenance(&mut self, provenance: Option<[usize; 3]>) { self.provenance = provenance; }

    /// Set the curve birth parameter (stored during `SplitEdge`).
    pub fn set_birth_parameter(&mut self, t: Option<f64>) { self.birth_parameter = t; }
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
    pub fn half_edge(&self) -> HalfEdgeId { self.half_edge }

    /// The face this loop belongs to.
    pub fn face(&self) -> FaceId { self.face }

    /// Set the entry halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) { self.half_edge = id; }

    /// Set the owning face.
    pub fn set_face(&mut self, id: FaceId) { self.face = id; }
}

/// Orientation of a shell within a solid.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellOrientation {
    /// Material-enclosing shell (outer boundary of a solid).
    Outer,
    /// Void-enclosing shell (inner boundary — a cavity).
    Inner,
}

/// Classification of a shell's topological character.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ShellKind {
    /// Closed watertight shell (every edge has exactly 2 incident faces).
    Solid(ShellOrientation),
    /// Open shell with boundary edges (car body panels, sheet metal).
    Sheet,
    /// Wire body: edges and vertices only, no faces.
    Wire,
}

/// Data stored for each shell — a maximal connected subset of faces.
///
/// Solid shells bound material or voids (cavities). Sheet shells are
/// open surfaces with boundary edges. Wire shells have only edges/vertices.
/// Shell membership is tracked via `FaceData::shell`. The representative
/// face provides a traversal entry point.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellData {
    representative_face: FaceId,
    kind: ShellKind,
    lineage: Option<Lineage>,
}

impl ShellData {
    /// Construct a new shell with the given representative face.
    pub fn new(representative_face: FaceId, kind: ShellKind) -> Self {
        Self { representative_face, kind, lineage: None }
    }

    /// Construct a new shell with lineage.
    pub fn with_lineage(
        representative_face: FaceId,
        kind: ShellKind,
        lineage: Option<Lineage>,
    ) -> Self {
        Self { representative_face, kind, lineage }
    }

    /// One representative face (entry point for shell traversal).
    pub fn representative_face(&self) -> FaceId { self.representative_face }

    /// Shell kind (solid, sheet, or wire).
    pub fn kind(&self) -> ShellKind { self.kind }

    /// Shell orientation for solid shells, `None` for sheet/wire.
    pub fn orientation(&self) -> Option<ShellOrientation> {
        match self.kind {
            ShellKind::Solid(o) => Some(o),
            _ => None,
        }
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// Set the representative face.
    pub fn set_representative_face(&mut self, id: FaceId) { self.representative_face = id; }

    /// Set the shell kind.
    pub fn set_kind(&mut self, kind: ShellKind) { self.kind = kind; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }
}

/// Data stored for each undirected edge — owns a representative halfedge.
///
/// All halfedges around this geometric edge form a radial ring linked
/// via `radial_next`. The representative halfedge provides an entry point.
/// Edge-level attributes (fillet radius, crease angle, seam) live here.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeData {
    half_edge: HalfEdgeId,
    lineage: Option<Lineage>,
}

impl EdgeData {
    /// Construct a new edge from one halfedge of the pair.
    pub fn new(half_edge: HalfEdgeId) -> Self {
        Self { half_edge, lineage: None }
    }

    /// Construct a new edge with lineage.
    pub fn with_lineage(half_edge: HalfEdgeId, lineage: Option<Lineage>) -> Self {
        Self { half_edge, lineage }
    }

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId { self.half_edge }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> { self.lineage.as_ref() }

    /// Set the representative halfedge.
    pub fn set_half_edge(&mut self, id: HalfEdgeId) { self.half_edge = id; }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) { self.lineage = lineage; }
}
