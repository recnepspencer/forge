//! Data shapes for arena-stored topology entities.
//!
//! DOMAIN: Defines the per-entity data structs (Face, HalfEdge, Vertex, Loop,
//! Shell, Edge) and the generational `Slot` wrapper.
//!
//! DEPENDENCIES: `handles` (typed IDs), `lineage` (inline provenance)

use serde::{Deserialize, Serialize};

use crate::handles::{
    BodyId, CoedgeRef, CurveRef, EdgeId, FaceId, HalfEdgeId, LoopId, LumpId, RegionId, ShellId,
    SurfaceRef, VertexId,
};
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
    /// Next vacant slot in the arena free-list.
    #[serde(default)]
    pub(crate) next_free: Option<u32>,
}

impl<T: Clone> Slot<T> {
    /// Create a new empty slot at generation 0.
    pub(crate) fn empty() -> Self {
        Self {
            generation: 0,
            version: 0,
            data: None,
            next_free: None,
        }
    }

    /// Occupy this slot with data, returning the current generation.
    /// Resets version to 0.
    pub(crate) fn occupy(&mut self, data: T) -> u32 {
        self.data = Some(data);
        self.version = 0;
        self.next_free = None;
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
            lineage: None,
            surface: None,
        }
    }

    /// Construct a new face with lineage.
    pub fn with_lineage(outer_loop: LoopId, shell: ShellId, lineage: Option<Lineage>) -> Self {
        Self {
            outer_loop,
            inner_loops: Vec::new(),
            shell,
            lineage,
            surface: None,
        }
    }

    /// The outer boundary loop of this face.
    pub fn outer_loop(&self) -> LoopId {
        self.outer_loop
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
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

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
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
    lineage: Option<Lineage>,
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
            lineage: None,
            is_bridge: false,
            coedge: None,
            direction: true,
        }
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
        Self {
            radial_next,
            next,
            prev,
            face,
            origin,
            edge,
            lineage,
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

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
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

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
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
        Self {
            outgoing,
            lineage: None,
            provenance: None,
            birth_parameter: None,
        }
    }

    /// Construct a new vertex with lineage.
    pub fn with_lineage(outgoing: HalfEdgeId, lineage: Option<Lineage>) -> Self {
        Self {
            outgoing,
            lineage,
            provenance: None,
            birth_parameter: None,
        }
    }

    /// One outgoing halfedge (for traversal entry).
    pub fn outgoing(&self) -> HalfEdgeId {
        self.outgoing
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
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

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
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

/// Data stored for each solid — the top-level topology container.
///
/// A solid owns one or more lumps. Each lump is a connected
/// component of material within this body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BodyData {
    lumps: Vec<LumpId>,
    lineage: Option<Lineage>,
}

impl BodyData {
    /// Construct a new empty solid.
    pub fn new() -> Self {
        Self {
            lumps: Vec::new(),
            lineage: None,
        }
    }

    /// Construct a new solid with lineage.
    pub fn with_lineage(lineage: Option<Lineage>) -> Self {
        Self {
            lumps: Vec::new(),
            lineage,
        }
    }

    /// The lumps belonging to this solid.
    pub fn lumps(&self) -> &[LumpId] {
        &self.lumps
    }

    /// Add a lump to this solid.
    pub fn add_lump(&mut self, id: LumpId) {
        self.lumps.push(id);
    }

    /// Remove a lump from this solid.
    ///
    /// Returns `true` if the lump was found and removed, `false` otherwise.
    pub fn remove_lump(&mut self, id: LumpId) -> bool {
        if let Some(pos) = self.lumps.iter().position(|&l| l == id) {
            self.lumps.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of lumps in this solid.
    pub fn lump_count(&self) -> usize {
        self.lumps.len()
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
    }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
    }

    /// Backward-compatible access: collect all shells across all lumps.
    ///
    /// Callers that previously used `solid.shells()` should migrate to
    /// traversing the Lump/Region hierarchy directly. This method exists
    /// as a migration aid.
    #[deprecated(note = "traverse lumps/regions/shells hierarchy instead")]
    pub fn shells(&self) -> &[LumpId] {
        &self.lumps
    }
}

/// Data stored for each lump — a connected component of material.
///
/// A lump contains one or more regions. Disconnected boolean results
/// produce multiple lumps within a single body.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LumpData {
    regions: Vec<RegionId>,
    body: BodyId,
    lineage: Option<Lineage>,
}

impl LumpData {
    /// Construct a new lump with its parent body.
    pub fn new(body: BodyId) -> Self {
        Self {
            regions: Vec::new(),
            body,
            lineage: None,
        }
    }

    /// Construct a new lump with lineage.
    pub fn with_lineage(body: BodyId, lineage: Option<Lineage>) -> Self {
        Self {
            regions: Vec::new(),
            body,
            lineage,
        }
    }

    /// The regions belonging to this lump.
    pub fn regions(&self) -> &[RegionId] {
        &self.regions
    }

    /// Add a region to this lump.
    pub fn add_region(&mut self, id: RegionId) {
        self.regions.push(id);
    }

    /// Remove a region from this lump.
    pub fn remove_region(&mut self, id: RegionId) -> bool {
        if let Some(pos) = self.regions.iter().position(|&r| r == id) {
            self.regions.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Number of regions in this lump.
    pub fn region_count(&self) -> usize {
        self.regions.len()
    }

    /// The body this lump belongs to.
    pub fn body(&self) -> BodyId {
        self.body
    }

    /// Set the parent body.
    pub fn set_body(&mut self, id: BodyId) {
        self.body = id;
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
    }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
    }
}

/// Data stored for each region — a 3D volume bounded by shells.
///
/// A region contains exactly one outer shell (material boundary) and
/// zero or more inner shells (cavities/voids). This is the topological
/// encoding of a 3-manifold with boundary.
///
/// The outer/inner distinction is enforced at the type level:
/// `set_outer_shell` must be called exactly once, and `add_inner_shell`
/// adds cavity walls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionData {
    outer_shell: Option<ShellId>,
    inner_shells: Vec<ShellId>,
    lump: LumpId,
    lineage: Option<Lineage>,
}

impl RegionData {
    /// Construct a new region with its parent lump.
    ///
    /// The outer shell must be set via `set_outer_shell` before commit.
    pub fn new(lump: LumpId) -> Self {
        Self {
            outer_shell: None,
            inner_shells: Vec::new(),
            lump,
            lineage: None,
        }
    }

    /// Construct a new region with lineage.
    pub fn with_lineage(lump: LumpId, lineage: Option<Lineage>) -> Self {
        Self {
            outer_shell: None,
            inner_shells: Vec::new(),
            lump,
            lineage,
        }
    }

    /// The outer shell bounding this region's material (if set).
    pub fn outer_shell(&self) -> Option<ShellId> {
        self.outer_shell
    }

    /// Inner shells (cavity/void walls) within this region.
    pub fn inner_shells(&self) -> &[ShellId] {
        &self.inner_shells
    }

    /// All shells in this region: outer first (if present), then inner.
    ///
    /// Prefer `outer_shell()` and `inner_shells()` for type-safe access.
    pub fn shells(&self) -> Vec<ShellId> {
        let mut result = Vec::with_capacity(1 + self.inner_shells.len());
        if let Some(outer) = self.outer_shell {
            result.push(outer);
        }
        result.extend_from_slice(&self.inner_shells);
        result
    }

    /// Add a shell to this region. If no outer shell is set, the first
    /// shell added becomes the outer shell. Subsequent shells are inner.
    pub fn add_shell(&mut self, id: ShellId) {
        if self.outer_shell.is_none() {
            self.outer_shell = Some(id);
        } else {
            self.inner_shells.push(id);
        }
    }

    /// Set the outer shell explicitly.
    pub fn set_outer_shell(&mut self, id: ShellId) {
        self.outer_shell = Some(id);
    }

    /// Add an inner shell (cavity wall) to this region.
    pub fn add_inner_shell(&mut self, id: ShellId) {
        self.inner_shells.push(id);
    }

    /// Remove a shell from this region.
    ///
    /// If the shell is the outer shell, the outer is cleared (set to `None`).
    /// If the shell is an inner shell, it is removed from the inner list.
    /// Returns `true` if the shell was found and removed, `false` otherwise.
    pub fn remove_shell(&mut self, id: ShellId) -> bool {
        if self.outer_shell == Some(id) {
            self.outer_shell = None;
            return true;
        }
        if let Some(pos) = self.inner_shells.iter().position(|&s| s == id) {
            self.inner_shells.swap_remove(pos);
            true
        } else {
            false
        }
    }

    /// Total number of shells (outer + inner) in this region.
    pub fn shell_count(&self) -> usize {
        (if self.outer_shell.is_some() { 1 } else { 0 }) + self.inner_shells.len()
    }

    /// The lump this region belongs to.
    pub fn lump(&self) -> LumpId {
        self.lump
    }

    /// Set the parent lump.
    pub fn set_lump(&mut self, id: LumpId) {
        self.lump = id;
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
    }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
    }
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
    region: RegionId,
    lineage: Option<Lineage>,
}

impl ShellData {
    /// Construct a new shell with the given representative face and parent region.
    pub fn new(representative_face: FaceId, kind: ShellKind, region: RegionId) -> Self {
        Self {
            representative_face,
            kind,
            region,
            lineage: None,
        }
    }

    /// Construct a new shell with lineage.
    pub fn with_lineage(
        representative_face: FaceId,
        kind: ShellKind,
        region: RegionId,
        lineage: Option<Lineage>,
    ) -> Self {
        Self {
            representative_face,
            kind,
            region,
            lineage,
        }
    }

    /// One representative face (entry point for shell traversal).
    pub fn representative_face(&self) -> FaceId {
        self.representative_face
    }

    /// Shell kind (solid, sheet, or wire).
    pub fn kind(&self) -> ShellKind {
        self.kind
    }

    /// Shell orientation for solid shells, `None` for sheet/wire.
    pub fn orientation(&self) -> Option<ShellOrientation> {
        match self.kind {
            ShellKind::Solid(o) => Some(o),
            _ => None,
        }
    }

    /// The region this shell belongs to.
    pub fn region(&self) -> RegionId {
        self.region
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
    }

    /// Set the representative face.
    pub fn set_representative_face(&mut self, id: FaceId) {
        self.representative_face = id;
    }

    /// Set the shell kind.
    pub fn set_kind(&mut self, kind: ShellKind) {
        self.kind = kind;
    }

    /// Set the region this shell belongs to.
    pub fn set_region(&mut self, id: RegionId) {
        self.region = id;
    }

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
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
    lineage: Option<Lineage>,
}

impl EdgeData {
    /// Construct a new edge from one halfedge of the pair.
    pub fn new(half_edge: HalfEdgeId) -> Self {
        Self {
            half_edge,
            curve: None,
            lineage: None,
        }
    }

    /// Construct a new edge with lineage.
    pub fn with_lineage(half_edge: HalfEdgeId, lineage: Option<Lineage>) -> Self {
        Self {
            half_edge,
            curve: None,
            lineage,
        }
    }

    /// Representative halfedge of the radial ring.
    pub fn half_edge(&self) -> HalfEdgeId {
        self.half_edge
    }

    /// Inline lineage for provenance tracking.
    pub fn lineage(&self) -> Option<&Lineage> {
        self.lineage.as_ref()
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

    /// Set inline lineage.
    pub fn set_lineage(&mut self, lineage: Option<Lineage>) {
        self.lineage = lineage;
    }
}
