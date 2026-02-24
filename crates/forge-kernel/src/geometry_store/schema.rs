//! Data shapes for the geometry store.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use forge_core::{KernelError, ToleranceProvider};
use forge_math::{MathError, GeometrySource, PlaneCoefficients};
use forge_math::arithmetic::Rational;
use forge_geom::Plane;
use forge_topo::handles::{FaceId, VertexId};

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
pub struct GeometryStore {
    /// Map from face handle to the plane the face lies on.
    face_planes: HashMap<u64, Plane>,
    /// Map from vertex handle to its exact 3D position.
    vertex_positions: HashMap<u64, ExactPosition>,
    /// Per-vertex tolerance spheres (certified uncertainty radii).
    ///
    /// Keyed by packed `(generation << 32 | index)`. When a key is absent the
    /// `ToleranceProvider` implementation returns `global_default()` as a safe
    /// conservative fallback instead of panicking.
    vertex_tolerances: HashMap<u64, f64>,
}

impl GeometryStore {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            face_planes: HashMap::new(),
            vertex_positions: HashMap::new(),
            vertex_tolerances: HashMap::new(),
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

    /// Associate a vertex with an f64 position (promoted to exact rational internally).
    pub fn set_vertex_position(&mut self, vertex: VertexId, position: [f64; 3]) {
        self.vertex_positions.insert(
            pack_handle(vertex.index(), vertex.generation()),
            ExactPosition::from_f64(position),
        );
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

    /// Whether a face is planar (always true for the current BSP-only kernel).
    ///
    /// Returns `false` for future NURBS faces so that `measure_gap` can
    /// guard against projecting onto non-planar supporting planes.
    pub fn face_is_planar(&self, _face: FaceId) -> bool {
        true
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
        true
    }
}

impl Default for GeometryStore {
    fn default() -> Self {
        Self::new()
    }
}

/// Global default tolerance for planar vertices.
///
/// # Deprecated
///
/// Use `GeometryStore::global_default()` (via [`forge_core::ToleranceProvider`])
/// instead. That method returns a scale-aware value derived from the model
/// bounding box following ISO 10303-42 (`1e-7 * max(bbox_diagonal, 1.0)`).
/// This constant remains only for external test code that has not yet migrated.
#[deprecated(
    since = "0.1.0",
    note = "Use GeometryStore::global_default() via ToleranceProvider instead"
)]
pub const PLANAR_VERTEX_TOLERANCE: f64 = 1e-10;

impl ToleranceProvider for GeometryStore {
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

impl GeometrySource for GeometryStore {
    fn get_plane(&self, index: usize) -> Result<PlaneCoefficients, MathError> {
        for (&key, plane) in &self.face_planes {
            let stored_index = (key & 0xFFFF_FFFF) as usize;
            if stored_index == index {
                let n = plane.normal();
                return PlaneCoefficients::try_new(n[0], n[1], n[2], plane.offset());
            }
        }
        Err(MathError::InvalidInput(
            format!("No plane found for face index {}", index),
        ))
    }
}

/// Pack a (index, generation) pair into a single u64 key for HashMap lookup.
fn pack_handle(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}
