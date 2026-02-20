//! Data shapes for the geometry store.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

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
        Self { exact, approx, is_exact: true }
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
        Self { exact, approx: safe_approx, is_exact: true }
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
        Self { exact, approx: pos, is_exact: false }
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
}

/// Side-car geometry storage mapping topology handles to geometric data.
///
/// The topology arena (Architecture Rule 2.3) stores only structural
/// connectivity. This store holds the geometric meaning: which plane
/// each face lies on, and where each vertex sits in 3D space.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeometryStore {
    /// Map from face handle to the plane the face lies on.
    face_planes: HashMap<u64, Plane>,
    /// Map from vertex handle to its exact 3D position.
    vertex_positions: HashMap<u64, ExactPosition>,
}

impl GeometryStore {
    /// Create an empty geometry store.
    pub fn new() -> Self {
        Self {
            face_planes: HashMap::new(),
            vertex_positions: HashMap::new(),
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

    /// Number of face-plane associations.
    pub fn face_plane_count(&self) -> usize {
        self.face_planes.len()
    }

    /// Number of vertex-position associations.
    pub fn vertex_position_count(&self) -> usize {
        self.vertex_positions.len()
    }
}

impl Default for GeometryStore {
    fn default() -> Self {
        Self::new()
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
