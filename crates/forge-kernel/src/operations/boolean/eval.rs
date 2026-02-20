//! Evaluation logic for Boolean operations.
//!
//! Includes vertex match key computation for robust deduplication.
//! Vertices are matched by their exact rational 3D position.
//! This key is TRANSIENT (used only during the boolean phase).
//! The vertex's permanent identity remains its `VertexId` + `Lineage`.

use forge_geom::Plane;
use forge_math::arithmetic::Rational;

/// Transient geometric match key for cross-solid vertex deduplication.
///
/// Keyed by the exact rational `[x, y, z]` position of the vertex.
/// Because `intersect_three_planes_exact` always reduces its result
/// to canonical form (GCD-reduced numerator/denominator), two vertices
/// at the same physical point produce bitwise-identical `Rational` values
/// and therefore identical `VertexMatchKey`s — regardless of which solid
/// they came from, which PlaneTable they used, or how many planes meet
/// at that corner.
///
/// This is NOT the vertex's permanent identity — that remains the
/// `VertexId` and its `Lineage`. This key is used strictly during
/// the boolean assembly phase. When a match is found, lineages are
/// merged (D1 compliance) rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VertexMatchKey {
    /// Exact rational coordinates.
    pos: [Rational; 3],
}


impl VertexMatchKey {
    /// Create a match key from an exact rational vertex position.
    ///
    /// `BigRational` is always stored in canonical (GCD-reduced) form,
    /// so two keys are equal iff they represent the same point.
    pub fn from_exact_position(x: Rational, y: Rational, z: Rational) -> Self {
        Self { pos: [x, y, z] }
    }

    /// Access the exact rational coordinates.
    pub fn position(&self) -> &[Rational; 3] {
        &self.pos
    }
}


/// Check if two planes have parallel normals (same or opposite direction).
///
/// Delegates to `forge_geom::primitives::plane::are_parallel_exact`, which
/// uses exact rational cross product. No tolerance, no magic numbers — D3 compliant.
pub fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    forge_geom::primitives::plane::are_parallel_exact(a, b)
}

/// Compute the centroid of a face.
///
/// Used for heuristic classification (e.g. containment check).
pub fn compute_face_centroid(
    arena: &forge_topo::arena::TopologyArena,
    geom: &crate::geometry_store::GeometryStore,
    face: forge_topo::handles::FaceId,
) -> Result<[f64; 3], forge_core::KernelError> {
    let mut vertices = Vec::new();
    let edges: Vec<_> = forge_topo::traverse::FaceEdgeIterator::new(arena, face)?
        .collect::<Result<Vec<_>, _>>()?;
    for he in edges {
        let v = arena.get_half_edge(he)?.origin();
        if let Some(pos) = geom.get_vertex_position(v) {
            vertices.push(*pos);
        }
    }
    
    forge_geom::primitives::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        forge_core::KernelError::InvalidInput {
            message: format!("Face {:?} has degenerate geometry (no vertices/area)", face),
            context: None,
        }
    })
}
