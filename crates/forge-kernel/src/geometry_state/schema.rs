//! Data shapes for the geometry store.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use forge_core::{KernelError, ToleranceProvider};
use crate::geom_facade::Plane;
use forge_math::arithmetic::Rational;
use forge_math::{GeometrySource, MathError, PlaneCoefficients};
use forge_topo::arena::TopologyArena;
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};

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
        Self {
            exact,
            approx,
            is_exact: true,
            symbolic_planes: None,
        }
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
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite()
        {
            approx
        } else {
            fallback
        };
        Self {
            exact,
            approx: safe_approx,
            is_exact: true,
            symbolic_planes: None,
        }
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
        Self {
            exact,
            approx: pos,
            is_exact: false,
            symbolic_planes: None,
        }
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
        let safe_approx = if approx[0].is_finite() && approx[1].is_finite() && approx[2].is_finite()
        {
            approx
        } else {
            fallback
        };
        Self {
            exact,
            approx: safe_approx,
            is_exact: true,
            symbolic_planes: Some(planes),
        }
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
}

impl GeometryState {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            face_planes: HashMap::new(),
            vertex_positions: HashMap::new(),
        }
    }

    /// Associate a face with a plane.
    pub fn set_face_plane(&mut self, face: FaceId, plane: Plane) {
        self.face_planes
            .insert(pack_handle(face.index(), face.generation()), plane);
    }

    /// Retrieve the plane for a face.
    pub fn get_face_plane(&self, face: FaceId) -> Option<&Plane> {
        self.face_planes
            .get(&pack_handle(face.index(), face.generation()))
    }

    /// Remove the plane binding for a killed face (spec §6.7 cleanup).
    ///
    /// Returns the plane that was removed, or `None` if no binding existed.
    pub fn remove_face_plane(&mut self, face: FaceId) -> Option<Plane> {
        self.face_planes
            .remove(&pack_handle(face.index(), face.generation()))
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
    /// Uses exact Rational arithmetic for plane transforms and vertex positions,
    /// preserving all precision through the transformation. The f64 approximations
    /// are recomputed from the exact values after transformation.
    pub fn transform(&mut self, space: &crate::geom_facade::LocalCoordinateSpace) {
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
    pub fn inverse_transform(
        &mut self,
        space: &crate::geom_facade::LocalCoordinateSpace,
    ) {
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
        self.vertex_positions
            .values()
            .map(|ep: &ExactPosition| ep.approx())
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
                if pos[i] < min[i] {
                    min[i] = pos[i];
                }
                if pos[i] > max[i] {
                    max[i] = pos[i];
                }
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

    // B-Rep entity queries moved to BrepState.

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
            if arena
                .get_vertex(VertexId::from_raw_parts(index, gen))
                .is_err()
            {
                return Err(KernelError::InternalError {
                    message: format!(
                        "Dangling vertex_position binding for VertexId {}:{}",
                        index, gen
                    ),
                    context: None,
                });
            }
        }

        // Removed loop over vertex_tolerances

        // Phase 4: Curved entity bindings moved to BrepState.

        Ok(())
    }

    // Local tolerance accessors removed: tolerances now live in AttributeStore.

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
            if self.get_vertex_position(v).is_none() {
                return Err(KernelError::InternalError {
                    message: format!(
                        "VertexId {}:{} has no geometry binding. \
                         Bind position and tolerance before calling draft.commit().",
                        v.index(),
                        v.generation(),
                    ),
                    context: None,
                });
            }
        }
        Ok(())
    }

    // is_all_planar moved to BrepState
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
    fn vertex_tolerance(&self, _vertex_index: u32, _vertex_generation: u32) -> f64 {
        // Vertex tolerances have moved to the AttributeStore in forge-topo.
        // GeometryState alone can only provide the global default now.
        // If exact per-vertex tolerance is required, query AttributeStore.
        self.global_default()
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
        found.ok_or_else(|| {
            MathError::InvalidInput(format!("No plane found for face index {}", index))
        })
    }
}

/// Pack a (index, generation) pair into a single u64 key for HashMap lookup.
pub(crate) fn pack_handle(index: u32, generation: u32) -> u64 {
    (u64::from(generation) << 32) | u64::from(index)
}
