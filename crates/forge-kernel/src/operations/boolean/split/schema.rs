//! Data shapes for the face-splitting phase.
//!
//! DOMAIN: Planes, vertex provenance, and split-phase results.
//! DEPENDENCIES: forge_geom::Plane, VertexMatchKey, GeometryStore.
//! INVARIANTS: PlaneTable uses exact equality; LocalVertexDedup is per-solid.

use std::collections::BTreeMap;

use forge_geom::primitives::plane::Plane;
use forge_math::arithmetic::Rational;
use forge_topo::handles::{FaceId, HalfEdgeId, VertexId};
use forge_topo::state::TopologyState;

use crate::geometry_store::GeometryStore;
use crate::core::ToleranceConfig;
use crate::operations::boolean::eval::VertexMatchKey;

/// Read-only configuration for the split phase.
///
/// Groups immutable lookup tables that every split function reads.
/// Separating from mutable bookkeeping (dedup, edge_cut_map) prevents
/// borrow conflicts when a function needs `&SplitConfig` while another
/// holds `&mut LocalVertexDedup`.
pub struct SplitConfig<'a> {
    pub plane_table: &'a PlaneTable,
    pub face_plane_map: &'a BTreeMap<FaceId, usize>,
    pub tolerance: &'a ToleranceConfig,
}

/// A centralized table of unique planes in the operation.
///
/// Used to assign stable IDs to planes for provenance tracking.
pub struct PlaneTable {
    planes: Vec<Plane>,
}

impl PlaneTable {
    pub fn new() -> Self {
        Self { planes: Vec::new() }
    }

    /// Intern a plane, returning its index.
    ///
    /// Uses exact rational equality — no tolerances, no scale sensitivity.
    pub fn intern(&mut self, plane: &Plane) -> usize {
        for (i, p) in self.planes.iter().enumerate() {
            if forge_geom::primitives::plane::exact_eq(p, plane) {
                return i;
            }
        }
        let idx = self.planes.len();
        let (a, b, c, d) = plane.exact_coefficients();
        eprintln!("PLANE_INTERN_EXACT: idx={} n_approx=[{:.6},{:.6},{:.6}] d_approx={:.6} a={} b={} c={} d={}",
            idx, plane.raw_normal()[0], plane.raw_normal()[1], plane.raw_normal()[2], plane.raw_offset(),
            a, b, c, d);
        self.planes.push(plane.clone());
        idx
    }

    pub fn get(&self, index: usize) -> &Plane {
        &self.planes[index]
    }
}

/// Output of the split phase for both solids.
pub struct SplitPhaseResult {
    pub target_topology: TopologyState,
    pub target_geometry: GeometryStore,
    pub tool_topology: TopologyState,
    pub tool_geometry: GeometryStore,
    pub split_count: usize,
    pub target_provenance: BTreeMap<VertexId, VertexMatchKey>,
    pub tool_provenance: BTreeMap<VertexId, VertexMatchKey>,
}

impl SplitPhaseResult {
    pub fn split_count(&self) -> usize { self.split_count }

    pub fn into_parts(self) -> (
        TopologyState, GeometryStore,
        TopologyState, GeometryStore,
        BTreeMap<VertexId, VertexMatchKey>,
        BTreeMap<VertexId, VertexMatchKey>,
    ) {
        (
            self.target_topology,
            self.target_geometry,
            self.tool_topology,
            self.tool_geometry,
            self.target_provenance,
            self.tool_provenance,
        )
    }
}

/// Maps an undirected edge (sorted vertex index pair) to the cut plane index that created it.
///
/// Used to resolve provenance for edges between coplanar sub-faces.
pub type EdgeCutMap = BTreeMap<(u32, u32), usize>;

/// Create a canonical (sorted) edge key from two vertex IDs.
pub fn make_edge_key(v1: VertexId, v2: VertexId) -> (u32, u32) {
    let a = v1.index();
    let b = v2.index();
    if a <= b { (a, b) } else { (b, a) }
}

/// Deduplication map for a single solid's vertices.
pub struct LocalVertexDedup {
    /// VertexId → MatchKey (forward provenance map)
    pub provenance: BTreeMap<VertexId, VertexMatchKey>,
    /// MatchKey → VertexId (reverse lookup for finding existing vertices)
    lookup: BTreeMap<VertexMatchKey, VertexId>,
}

impl LocalVertexDedup {
    pub fn new() -> Self {
        Self {
            provenance: BTreeMap::new(),
            lookup: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, vid: VertexId, prov: VertexMatchKey) {
        self.provenance.insert(vid, prov.clone());
        self.lookup.insert(prov, vid);
    }

    pub fn find_by_provenance(&self, prov: &VertexMatchKey) -> Option<VertexId> {
        self.lookup.get(prov).copied()
    }
}

/// Shared registry of canonical intersection positions.
///
/// Each 3-plane intersection is computed once and stored here.
/// Both `split_solid` calls reference the same registry so the same
/// geometric point always gets the same position — zero floating-point
/// divergence between solids.
pub struct SharedVertexRegistry {
    positions: BTreeMap<VertexMatchKey, [f64; 3]>,
}

impl SharedVertexRegistry {
    pub fn new() -> Self {
        Self { positions: BTreeMap::new() }
    }

    /// Register a position for a 3-plane key.
    ///
    /// If the key already exists, returns the previously stored (canonical) position.
    /// If new, stores and returns the provided position.
    pub fn canonical_position(&mut self, key: &VertexMatchKey, computed: [f64; 3]) -> [f64; 3] {
        *self.positions.entry(key.clone()).or_insert(computed)
    }
}

/// A point where the cut plane intersects a face edge.
#[derive(Debug)]
pub enum CutPoint {
    Existing(VertexId),
    NewOnEdge {
        half_edge: HalfEdgeId,
        provenance: VertexMatchKey,
        position: [f64; 3],
        exact_position: Option<[Rational; 3]>,
    },
}
