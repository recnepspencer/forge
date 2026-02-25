//! Data shapes for the geometry store.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use forge_core::{KernelError, ToleranceProvider};
use forge_math::{MathError, GeometrySource, PlaneCoefficients};
use forge_math::arithmetic::Rational;
use forge_geom::Plane;
use forge_geom::{SurfaceData, SurfaceKind, CurveGeom, Coedge};
use forge_topo::handles::{FaceId, VertexId, HalfEdgeId, EdgeId, SurfaceRef, CurveRef, CoedgeRef};
use forge_topo::arena::TopologyArena;

/// Exact 3D position backed by rational coordinates with a cached f64 approximation.
///
/// Vertex positions derived from 3-plane intersection are stored exactly.
/// The f64 cache is derived from the rationals at construction time and
/// is used for BVH, AABB, and rendering — never for topology decisions.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExactPosition {
    /// Exact rational coordinates.
    exact: [Rational; 3],
    /// Cached f64 approximation (derived from exact).
    approx: [f64; 3],
    /// Whether this position was computed via genuine exact arithmetic
    /// (e.g. intersect_three_planes_exact) or promoted from f64.
    /// Only exact positions should be used with classify_point_exact.
    is_exact: bool,
    /// Indices of the 3 planes defining this vertex, if known symbolically.
    symbolic_planes: Option<[usize; 3]>,
}

impl ExactPosition {
    /// Create from exact rational coordinates (genuine exact arithmetic).
    ///
    /// If the rational-to-f64 conversion overflows (producing ±inf or NaN),
    /// the f64 approximation is clamped to 0.0. The exact rationals remain
    /// intact for topology decisions via `classify_point_exact`.
    pub fn from_exact(exact: [Rational; 3]) -> Self {
        let raw = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let approx = [
            if raw[0].is_finite() { raw[0] } else { 0.0 },
            if raw[1].is_finite() { raw[1] } else { 0.0 },
            if raw[2].is_finite() { raw[2] } else { 0.0 },
        ];
        Self { exact, approx, is_exact: true, symbolic_planes: None }
    }

    /// Create from exact rationals with an explicit f64 fallback.
    ///
    /// When the rational-to-f64 conversion overflows (producing ±inf or NaN),
    /// use the provided fallback instead. The exact rationals are still stored
    /// for topology decisions via `classify_point_exact`.
    pub fn from_exact_with_fallback(exact: [Rational; 3], fallback: [f64; 3]) -> Self {
        let approx = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite() {
            approx
        } else {
            fallback
        };
        Self { exact, approx: safe_approx, is_exact: true, symbolic_planes: None }
    }

    /// Create from f64 coordinates (lossless IEEE754 → Rational conversion).
    /// NOT marked as exact — f64-promoted positions may not satisfy
    /// rational plane equations exactly.
    pub fn from_f64(pos: [f64; 3]) -> Self {
        let exact = [
            Rational::try_from_f64(pos[0]).unwrap_or_else(|_| Rational::zero()),
            Rational::try_from_f64(pos[1]).unwrap_or_else(|_| Rational::zero()),
            Rational::try_from_f64(pos[2]).unwrap_or_else(|_| Rational::zero()),
        ];
        Self { exact, approx: pos, is_exact: false, symbolic_planes: None }
    }

    /// The cached f64 approximation.
    pub fn approx(&self) -> &[f64; 3] {
        &self.approx
    }

    /// The exact rational coordinates (only meaningful when `is_exact()` is true).
    pub fn exact(&self) -> &[Rational; 3] {
        &self.exact
    }

    /// Whether this position was computed via genuine exact arithmetic.
    pub fn is_exact(&self) -> bool {
        self.is_exact
    }

    /// Create from exact rationals, preserving the symbolic planes that defined it.
    pub fn from_symbolic(exact: [Rational; 3], fallback: [f64; 3], planes: [usize; 3]) -> Self {
        let approx = [
            exact[0].to_f64_approx(),
            exact[1].to_f64_approx(),
            exact[2].to_f64_approx(),
        ];
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite() {
            approx
        } else {
            fallback
        };
        Self { exact, approx: safe_approx, is_exact: true, symbolic_planes: Some(planes) }
    }

    /// Retrieve the symbolic bounding planes if they form a precise 3-plane intersection.
    pub fn symbolic_planes(&self) -> Option<&[usize; 3]> {
        self.symbolic_planes.as_ref()
    }
}

/// Side-car geometry storage mapping topology handles to geometric data.
///
/// The topology arena stores only structural connectivity. This store holds
/// the geometric meaning: face planes, vertex positions, and — new in the
/// tolerant topology model — per-vertex tolerance spheres.
///
/// Implements `ToleranceProvider` so that `forge-topo` functions
/// (`classify_point_on_face`, `validate_geometric_invariants`) can query
/// per-entity tolerances without owning any `f64` data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeometryState {
    /// Map from face handle to the plane the face lies on.
    pub(crate) face_planes: HashMap<u64, Plane>,
    /// Map from vertex handle to its exact 3D position.
    pub(crate) vertex_positions: HashMap<u64, ExactPosition>,
    /// Per-vertex tolerance spheres (certified uncertainty radii).
    ///
    /// Keyed by packed `(generation << 32 | index)`. When a key is absent the
    /// `ToleranceProvider` implementation returns `global_default()` as a safe
    /// conservative fallback instead of panicking.
    pub(crate) vertex_tolerances: HashMap<u64, f64>,

    // ── Phase 4: Geometry entity arenas ──────────────────────────────────

    /// Parametric surface definitions (indexed by `SurfaceRef`).
    surfaces: Vec<GeomSlot<SurfaceData>>,
    /// 3D edge curve geometries (indexed by `CurveRef`).
    curves: Vec<GeomSlot<CurveGeom>>,
    /// UV trim curves / coedges (indexed by `CoedgeRef`).
    coedges: Vec<GeomSlot<Coedge>>,

    /// Map from face handle → `SurfaceRef` for faces with attached surfaces.
    face_surfaces: HashMap<u64, SurfaceRef>,
    /// Map from halfedge handle → `(CoedgeRef, direction)` for curved halfedges.
    halfedge_coedges: HashMap<u64, (CoedgeRef, bool)>,
    /// Map from edge handle → `CurveRef` for edges with attached curves.
    edge_curves: HashMap<u64, CurveRef>,
}

/// A generational slot in the geometry arenas.
#[derive(Clone, Debug, Serialize, Deserialize)]
struct GeomSlot<T> {
    data: Option<T>,
    generation: u32,
}

impl<T> GeomSlot<T> {
    fn vacant(gen: u32) -> Self {
        Self { data: None, generation: gen }
    }

    fn occupied(data: T, gen: u32) -> Self {
        Self { data: Some(data), generation: gen }
    }
}

impl GeometryState {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            face_planes: HashMap::new(),
            vertex_positions: HashMap::new(),
            vertex_tolerances: HashMap::new(),
            surfaces: Vec::new(),
            curves: Vec::new(),
            coedges: Vec::new(),
            face_surfaces: HashMap::new(),
            halfedge_coedges: HashMap::new(),
            edge_curves: HashMap::new(),
        }
    }

    /// Associate a face with a plane.
    pub fn set_face_plane(&mut self, face: FaceId, plane: Plane) {
        self.face_planes.insert(pack_handle(face.index(), face.generation()), plane);
    }

    /// Retrieve the plane for a face.
    pub fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.face_planes.get(&pack_handle(face.index(), face.generation()))
    }

    /// Remove the plane binding for a killed face (spec §6.7 cleanup).
    ///
    /// Returns the plane that was removed, or `None` if no binding existed.
    pub fn remove_face_plane(&mut self, face: FaceId) -> Option<Plane> {
        self.face_planes.remove(&pack_handle(face.index(), face.generation()))
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_face_plane_raw(&mut self, packed_key: u64, plane: Plane) {
        self.face_planes.insert(packed_key, plane);
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_face_plane_raw(&mut self, packed_key: u64) {
        self.face_planes.remove(&packed_key);
    }

    /// Associate a vertex with an f64 position (promoted to exact rational internally).
    pub fn set_vertex_position(&mut self, vertex: VertexId, position: [f64; 3]) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            ExactPosition::from_f64(position),
        );
    }

    /// Internal: commit a raw handle position directly.
    pub(crate) fn _set_vertex_position_raw(&mut self, packed_key: u64, pos: ExactPosition) {
        self.vertex_positions.insert(packed_key, pos);
    }

    /// Internal: remove a raw handle position directly.
    pub(crate) fn _remove_vertex_position_raw(&mut self, packed_key: u64) {
        self.vertex_positions.remove(&packed_key);
    }

    /// Associate a vertex with an exact rational position.
    pub fn set_vertex_position_exact(&mut self, vertex: VertexId, exact: [Rational; 3]) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            ExactPosition::from_exact(exact),
        );
    }

    /// Associate a vertex with exact rational coordinates and an f64 fallback.
    ///
    /// When the rational-to-f64 conversion overflows, the fallback is used
    /// for AABB/BVH/rendering while the exact rationals drive topology.
    pub fn set_vertex_position_exact_with_fallback(
        &mut self,
        vertex: VertexId,
        exact: [Rational; 3],
        fallback: [f64; 3],
    ) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            ExactPosition::from_exact_with_fallback(exact, fallback),
        );
    }

    /// Associate a vertex with exact rational coordinates, an f64 fallback, and its defining planes.
    pub fn set_vertex_position_symbolic(
        &mut self,
        vertex: VertexId,
        exact: [Rational; 3],
        fallback: [f64; 3],
        planes: [usize; 3],
    ) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            ExactPosition::from_symbolic(exact, fallback, planes),
        );
    }
    ///
    /// Uses exact Rational arithmetic for plane transforms and vertex positions,
    /// preserving all precision through the transformation. The f64 approximations
    /// are recomputed from the exact values after transformation.
    pub fn transform(&mut self, space: &forge_geom::spatial::local_space::LocalCoordinateSpace) {
        for plane in self.face_planes.values_mut() {
            *plane = space.transform_plane_exact(plane);
        }
        for pos in self.vertex_positions.values_mut() {
            let local_exact = space.to_local_exact(pos.exact());
            let local_approx = space.to_local(pos.approx);
            *pos = ExactPosition::from_exact_with_fallback(local_exact, local_approx);
        }
    }

    /// Transform all geometry from a local coordinate space back to world (exact Rational).
    ///
    /// Uses exact Rational arithmetic for plane transforms and vertex positions,
    /// preserving all precision through the inverse transformation.
    pub fn inverse_transform(&mut self, space: &forge_geom::spatial::local_space::LocalCoordinateSpace) {
        for plane in self.face_planes.values_mut() {
            *plane = space.inverse_transform_plane_exact(plane);
        }
        for pos in self.vertex_positions.values_mut() {
            let world_exact = space.from_local_exact(pos.exact());
            let world_approx = space.from_local(pos.approx);
            *pos = ExactPosition::from_exact_with_fallback(world_exact, world_approx);
        }
    }

    /// Retrieve the f64 approximation for a vertex position.
    ///
    /// This is the primary API for downstream consumers (BVH, AABB, rendering,
    /// stitch, copy). Returns the cached f64 from the ExactPosition.
    pub fn get_vertex_position(&self, vertex: VertexId) -> Option<&[f64; 3]> {
        self.vertex_positions
            .get(&pack_handle(vertex.index(), vertex.generation()))
            .map(|ep| ep.approx())
    }

    /// Retrieve the exact rational position for a vertex.
    ///
    /// Returns `None` for vertices whose positions were only f64-promoted,
    /// since those may not satisfy rational plane equations exactly.
    /// Only returns `Some` for vertices computed via genuine exact arithmetic
    /// (e.g. `intersect_three_planes_exact`).
    pub fn get_vertex_position_exact(&self, vertex: VertexId) -> Option<&[Rational; 3]> {
        self.vertex_positions
            .get(&pack_handle(vertex.index(), vertex.generation()))
            .filter(|ep| ep.is_exact())
            .map(|ep| ep.exact())
    }

    /// Retrieve the symbolic plane indices for a vertex if it is defined as a pure intersection.
    pub fn get_vertex_symbolic_planes(&self, vertex: VertexId) -> Option<&[usize; 3]> {
        self.vertex_positions
            .get(&pack_handle(vertex.index(), vertex.generation()))
            .and_then(|ep| ep.symbolic_planes())
    }

    /// Number of face-plane associations.
    pub fn face_plane_count(&self) -> usize {
        self.face_planes.len()
    }

    /// Number of vertex-position associations.
    pub fn vertex_position_count(&self) -> usize {
        self.vertex_positions.len()
    }

    /// Iterate over all vertex f64 positions.
    ///
    /// Used by `QuantizedSpace::build` to compute the combined AABB.
    pub fn iter_vertex_positions(&self) -> impl Iterator<Item = &[f64; 3]> {
        self.vertex_positions.values().map(|ep| ep.approx())
    }

    /// Compute the model bounding-box diagonal (mm) from all vertex positions.
    ///
    /// Iterates all stored vertex approximations to find the axis-aligned bounding
    /// box, then returns `‖bbox_max − bbox_min‖`. Returns `0.0` for an empty store.
    /// This drives the ISO 10303-42 scale-aware tolerance formula.
    pub fn compute_model_scale(&self) -> f64 {
        let mut min = [f64::MAX; 3];
        let mut max = [f64::MIN; 3];
        let mut any = false;

        for pos in self.iter_vertex_positions() {
            any = true;
            for i in 0..3 {
                if pos[i] < min[i] { min[i] = pos[i]; }
                if pos[i] > max[i] { max[i] = pos[i]; }
            }
        }

        if !any {
            return 0.0;
        }
        let dx = max[0] - min[0];
        let dy = max[1] - min[1];
        let dz = max[2] - min[2];
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    /// Whether a face has a planar surface (or no surface attached yet).
    ///
    /// Returns `false` for faces with an attached non-planar `SurfaceRef`.
    /// Returns `true` for faces with no surface (planar-only phase) or
    /// faces with an attached `SurfaceKind::Plane`.
    pub fn face_is_planar(&self, face: FaceId) -> bool {
        let key = pack_handle(face.index(), face.generation());
        match self.face_surfaces.get(&key) {
            None => true,
            Some(surface_ref) => {
                match self.get_surface(*surface_ref) {
                    Ok(data) => matches!(data.kind, SurfaceKind::Plane { .. }),
                    Err(_) => true,
                }
            }
        }
    }

    // ── Phase 4: Surface CRUD ────────────────────────────────────────────

    /// Insert a new surface, returning its `SurfaceRef` handle.
    pub fn insert_surface(&mut self, data: SurfaceData) -> SurfaceRef {
        let index = self.surfaces.len() as u32;
        self.surfaces.push(GeomSlot::occupied(data, 0));
        SurfaceRef::from_raw_parts(index, 0)
    }

    /// Retrieve a surface by its handle.
    pub fn get_surface(&self, r: SurfaceRef) -> Result<&SurfaceData, KernelError> {
        let slot = self.surfaces.get(r.index() as usize)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("SurfaceRef {} out of range", r),
                context: None,
            })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale SurfaceRef {} (arena gen {})", r, slot.generation),
                context: None,
            });
        }
        slot.data.as_ref().ok_or_else(|| KernelError::InternalError {
            message: format!("SurfaceRef {} points to a vacant slot", r),
            context: None,
        })
    }

    /// Remove a surface, returning its data.
    pub fn remove_surface(&mut self, r: SurfaceRef) -> Result<SurfaceData, KernelError> {
        let slot = self.surfaces.get_mut(r.index() as usize)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("SurfaceRef {} out of range", r),
                context: None,
            })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale SurfaceRef {}", r),
                context: None,
            });
        }
        let data = slot.data.take().ok_or_else(|| KernelError::InternalError {
            message: format!("SurfaceRef {} already vacant", r),
            context: None,
        })?;
        slot.generation += 1;
        Ok(data)
    }

    // ── Phase 4: Curve CRUD ──────────────────────────────────────────────

    /// Insert a new curve geometry, returning its `CurveRef` handle.
    pub fn insert_curve(&mut self, data: CurveGeom) -> CurveRef {
        let index = self.curves.len() as u32;
        self.curves.push(GeomSlot::occupied(data, 0));
        CurveRef::from_raw_parts(index, 0)
    }

    /// Retrieve a curve geometry by its handle.
    pub fn get_curve(&self, r: CurveRef) -> Result<&CurveGeom, KernelError> {
        let slot = self.curves.get(r.index() as usize)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("CurveRef {} out of range", r),
                context: None,
            })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CurveRef {}", r),
                context: None,
            });
        }
        slot.data.as_ref().ok_or_else(|| KernelError::InternalError {
            message: format!("CurveRef {} points to a vacant slot", r),
            context: None,
        })
    }

    /// Remove a curve geometry, returning its data.
    pub fn remove_curve(&mut self, r: CurveRef) -> Result<CurveGeom, KernelError> {
        let slot = self.curves.get_mut(r.index() as usize)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("CurveRef {} out of range", r),
                context: None,
            })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CurveRef {}", r),
                context: None,
            });
        }
        let data = slot.data.take().ok_or_else(|| KernelError::InternalError {
            message: format!("CurveRef {} already vacant", r),
            context: None,
        })?;
        slot.generation += 1;
        Ok(data)
    }

    // ── Phase 4: Coedge CRUD ─────────────────────────────────────────────

    /// Insert a new coedge (UV trim curve), returning its `CoedgeRef` handle.
    pub fn insert_coedge(&mut self, data: Coedge) -> CoedgeRef {
        let index = self.coedges.len() as u32;
        self.coedges.push(GeomSlot::occupied(data, 0));
        CoedgeRef::from_raw_parts(index, 0)
    }

    /// Retrieve a coedge by its handle.
    pub fn get_coedge(&self, r: CoedgeRef) -> Result<&Coedge, KernelError> {
        let slot = self.coedges.get(r.index() as usize)
            .ok_or_else(|| KernelError::InternalError {
                message: format!("CoedgeRef {} out of range", r),
                context: None,
            })?;
        if slot.generation != r.generation() {
            return Err(KernelError::InternalError {
                message: format!("Stale CoedgeRef {}", r),
                context: None,
            });
        }
        slot.data.as_ref().ok_or_else(|| KernelError::InternalError {
            message: format!("CoedgeRef {} points to a vacant slot", r),
            context: None,
        })
    }

    // ── Phase 4: Attachment APIs ──────────────────────────────────────────

    /// Attach a surface to a face. Does NOT go through Euler operators
    /// (geometry attachment ≠ topology change).
    pub fn attach_surface_to_face(&mut self, face: FaceId, surface: SurfaceRef) {
        let key = pack_handle(face.index(), face.generation());
        self.face_surfaces.insert(key, surface);
    }

    /// Attach a coedge + direction to a halfedge.
    pub fn attach_coedge_to_halfedge(&mut self, he: HalfEdgeId, coedge: CoedgeRef, direction: bool) {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedges.insert(key, (coedge, direction));
    }

    /// Attach a curve to an edge.
    pub fn attach_curve_to_edge(&mut self, edge: EdgeId, curve: CurveRef) {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curves.insert(key, curve);
    }

    /// Retrieve the surface attached to a face, if any.
    pub fn get_face_surface(&self, face: FaceId) -> Option<SurfaceRef> {
        let key = pack_handle(face.index(), face.generation());
        self.face_surfaces.get(&key).copied()
    }

    /// Retrieve the coedge + direction attached to a halfedge, if any.
    pub fn get_halfedge_coedge(&self, he: HalfEdgeId) -> Option<(CoedgeRef, bool)> {
        let key = pack_handle(he.index(), he.generation());
        self.halfedge_coedges.get(&key).copied()
    }

    /// Retrieve the curve attached to an edge, if any.
    pub fn get_edge_curve(&self, edge: EdgeId) -> Option<CurveRef> {
        let key = pack_handle(edge.index(), edge.generation());
        self.edge_curves.get(&key).copied()
    }

    /// Number of active surfaces.
    pub fn surface_count(&self) -> usize {
        self.surfaces.iter().filter(|s| s.data.is_some()).count()
    }

    /// Number of active curves.
    pub fn curve_count(&self) -> usize {
        self.curves.iter().filter(|s| s.data.is_some()).count()
    }

    /// Number of active coedges.
    pub fn coedge_count(&self) -> usize {
        self.coedges.iter().filter(|s| s.data.is_some()).count()
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_face_surface_raw(&mut self, packed_key: u64, surface: SurfaceRef) {
        self.face_surfaces.insert(packed_key, surface);
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_face_surface_raw(&mut self, packed_key: u64) {
        self.face_surfaces.remove(&packed_key);
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_halfedge_coedge_raw(&mut self, packed_key: u64, coedge: CoedgeRef, direction: bool) {
        self.halfedge_coedges.insert(packed_key, (coedge, direction));
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_halfedge_coedge_raw(&mut self, packed_key: u64) {
        self.halfedge_coedges.remove(&packed_key);
    }

    /// Internal: commit a raw handle mapping directly.
    pub(crate) fn _set_edge_curve_raw(&mut self, packed_key: u64, curve: CurveRef) {
        self.edge_curves.insert(packed_key, curve);
    }

    /// Internal: remove a raw handle mapping directly.
    pub(crate) fn _remove_edge_curve_raw(&mut self, packed_key: u64) {
        self.edge_curves.remove(&packed_key);
    }

    // ── Phase 4: Validation ──────────────────────────────────────────────

    pub fn validate_geometry_bindings(&self, arena: &TopologyArena) -> Result<(), KernelError> {
        // Core topology bindings
        for &key in self.face_planes.keys() {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_face(FaceId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling face_plane binding for FaceId {}:{}", index, gen),
                    context: None,
                });
            }
        }

        for &key in self.vertex_positions.keys() {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_vertex(VertexId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling vertex_position binding for VertexId {}:{}", index, gen),
                    context: None,
                });
            }
        }

        for &key in self.vertex_tolerances.keys() {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_vertex(VertexId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling vertex_tolerance binding for VertexId {}:{}", index, gen),
                    context: None,
                });
            }
        }

        // Phase 4: Curved entity bindings
        for (&key, &surface_ref) in &self.face_surfaces {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_face(FaceId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling face_surface topology handle for FaceId {}:{}", index, gen),
                    context: None,
                });
            }
            if self.get_surface(surface_ref).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("FaceId {}:{} has a dangling SurfaceRef {}", index, gen, surface_ref),
                    context: None,
                });
            }
        }

        for (&key, &(coedge_ref, _)) in &self.halfedge_coedges {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_half_edge(HalfEdgeId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling halfedge_coedge topology handle for HalfEdgeId {}:{}", index, gen),
                    context: None,
                });
            }
            if self.get_coedge(coedge_ref).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("HalfEdgeId {}:{} has a dangling CoedgeRef {}", index, gen, coedge_ref),
                    context: None,
                });
            }
        }

        for (&key, &curve_ref) in &self.edge_curves {
            let index = (key & 0xFFFF_FFFF) as u32;
            let gen = (key >> 32) as u32;
            if arena.get_edge(EdgeId::from_raw_parts(index, gen)).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("Dangling edge_curve topology handle for EdgeId {}:{}", index, gen),
                    context: None,
                });
            }
            if self.get_curve(curve_ref).is_err() {
                return Err(KernelError::InternalError {
                    message: format!("EdgeId {}:{} has a dangling CurveRef {}", index, gen, curve_ref),
                    context: None,
                });
            }
        }

        Ok(())
    }

    /// Insert a new vertex with a position derived from its provenance.
    ///
    /// Dispatches on `provenance` to compute the certified tolerance:
    /// - `ThreePlaneIntersection` → uses `global_default()` (conservative pre-SSI)
    /// - `EdgeSplit` → `VertexGeom::split_tolerance(origin, target)` caller must supply
    ///   pre-computed inherited tolerance via the `tolerance` parameter
    /// - `Imported` → `healing_tolerance` from the provenance record
    /// - `Coalesced` → `VertexGeom::coalesced_tolerance(a, b)` caller must supply
    ///   pre-computed RSS via the `tolerance` parameter
    ///
    /// For `ThreePlaneIntersection` and other variants where the caller must
    /// supply the tolerance, pass it in the `tolerance` parameter. The helper
    /// will always use `Imported::healing_tolerance` when available.
    pub fn insert_vertex_with_provenance(
        &mut self,
        vertex: VertexId,
        position: [f64; 3],
        tolerance: f64,
        provenance: &forge_geom::primitives::vertex_geom::VertexProvenance,
    ) {
        use forge_geom::primitives::vertex_geom::VertexProvenance;
        let certified_tol = match provenance {
            VertexProvenance::Imported { healing_tolerance } => *healing_tolerance,
            _ => tolerance,
        };
        debug_assert!(certified_tol > 0.0, "insert_vertex_with_provenance: tolerance must be > 0.0");
        self.set_vertex_position(vertex, position);
        self.set_vertex_tolerance(vertex, certified_tol);
    }

    /// Set the per-vertex tolerance sphere radius.
    ///
    /// Must be strictly positive. This is the certified uncertainty bound for
    /// the vertex's 3D position — used by `ToleranceProvider::vertex_tolerance`.
    ///
    /// # Panics
    /// Panics in debug builds if `tolerance <= 0.0`.
    pub fn set_vertex_tolerance(&mut self, vertex: VertexId, tolerance: f64) {
        debug_assert!(tolerance > 0.0, "vertex tolerance must be > 0.0, got {}", tolerance);
        self.vertex_tolerances.insert(
            pack_handle(vertex.index(), vertex.generation()),
            tolerance,
        );
    }

    /// Internal: commit a raw handle tolerance directly.
    pub(crate) fn _set_vertex_tolerance_raw(&mut self, packed_key: u64, tolerance: f64) {
        self.vertex_tolerances.insert(packed_key, tolerance);
    }

    /// Internal: remove a raw handle tolerance directly.
    pub(crate) fn _remove_vertex_tolerance_raw(&mut self, packed_key: u64) {
        self.vertex_tolerances.remove(&packed_key);
    }

    /// Retrieve the per-vertex tolerance sphere radius, or `None` if not yet bound.
    ///
    /// A `None` return means the vertex was created by a Euler op but the kernel
    /// has not yet decorated it with geometry. Callers should use `global_default()`
    /// rather than treating `None` as `0.0`.
    pub fn get_vertex_tolerance(&self, vertex: VertexId) -> Option<f64> {
        self.vertex_tolerances.get(&pack_handle(vertex.index(), vertex.generation())).copied()
    }

    /// Check that every VertexId in the provided iterator has a tolerance bound.
    ///
    /// Call this before `MutableDraft::commit()` to catch Euler ops that created
    /// vertices without the kernel decorating them with geometry. Returns the
    /// first unbound vertex found.
    pub fn validate_bindings<I>(&self, vertex_ids: I) -> Result<(), KernelError>
    where
        I: IntoIterator<Item = VertexId>,
    {
        for v in vertex_ids {
            if self.get_vertex_tolerance(v).is_none() || self.get_vertex_position(v).is_none() {
                return Err(KernelError::InternalError {
                    message: format!(
                        "VertexId {}:{} has no geometry binding. \
                         Bind position and tolerance before calling draft.commit().",
                        v.index(), v.generation(),
                    ),
                    context: None,
                });
            }
        }
        Ok(())
    }

    /// Whether all face geometry is planar (no curved surfaces).
    pub fn is_all_planar(&self) -> bool {
        self.surfaces.iter()
            .filter_map(|s| s.data.as_ref())
            .all(|s| matches!(s.kind, SurfaceKind::Plane { .. }))
            && self.face_surfaces.is_empty()
    }
}

impl Default for GeometryState {
    fn default() -> Self {
        Self::new()
    }
}

/// Global default tolerance for planar vertices.
///
/// # Deprecated
///
/// Use `GeometryState::global_default()` (via [`forge_core::ToleranceProvider`])
/// instead. That method returns a scale-aware value derived from the model
/// bounding box following ISO 10303-42 (`1e-7 * max(bbox_diagonal, 1.0)`).
/// This constant remains only for external test code that has not yet migrated.
#[deprecated(
    since = "0.1.0",
    note = "Use GeometryState::global_default() via ToleranceProvider instead"
)]
pub const PLANAR_VERTEX_TOLERANCE: f64 = 1e-10;

impl ToleranceProvider for GeometryState {
    fn vertex_tolerance(&self, vertex_index: u32, vertex_generation: u32) -> f64 {
        let key = pack_handle(vertex_index, vertex_generation);
        self.vertex_tolerances
            .get(&key)
            .copied()
            .unwrap_or_else(|| self.global_default())
    }

    /// Tolerance used in point-on-edge classification.
    ///
    /// Capped at 1e-6 regardless of model scale so that snap-to-boundary
    /// behaviour in `classify_point_in_solid` stays conservative on large models
    /// (a 1000mm panel would otherwise get 1e-4 snap, which is far too aggressive).
    fn edge_tolerance(&self, _edge_index: u32, _edge_generation: u32) -> f64 {
        self.global_default().min(1e-6)
    }

    /// Scale-aware global default following ISO 10303-42.
    ///
    /// Returns `1e-7 * max(bbox_diagonal, 1.0)`, floored at `1e-13`.
    /// Unknown vertices (not yet decorated by the kernel) fall back to this
    /// value rather than panicking or returning 0.
    fn global_default(&self) -> f64 {
        let scale = self.compute_model_scale().max(1.0);
        (scale * 1e-7).max(1e-13)
    }
}

impl GeometrySource for GeometryState {
    /// Find a plane by face index, scanning packed generational keys.
    ///
    /// `GeometrySource` passes only the bare face index. We scan all keys
    /// and match on the low-32-bit index word. Multiple live entries sharing
    /// the same index indicate an ABA generation collision — returned as
    /// `Err(InvalidInput)` rather than silently resolving to either value.
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        let mut found: Option<PlaneCoefficients> = None;
        for (&key, plane) in &self.face_planes {
            let stored_index = (key & 0xFFFF_FFFF) as usize;
            if stored_index == index {
                let n = plane.normal();
                let coeff = PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset())?;
                if found.is_some() {
                    return Err(MathError::InvalidInput(format!(
                        "Ambiguous plane lookup: multiple live generations for face index {} \
                         (ABA generation collision in GeometryState)",
                        index
                    )));
                }
                found = Some(coeff);
            }
        }
        found.ok_or_else(|| MathError::InvalidInput(
            format!("No plane found for face index {}", index),
        ))
    }
}

/// Pack a (index, generation) pair into a single u64 key for HashMap lookup.
pub(crate) fn pack_handle(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}
