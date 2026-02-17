//! Evaluation logic for Boolean operations.
//!
//! Includes vertex match key computation for robust deduplication.
//! Every vertex is matched by exactly 3 sorted plane IDs —
//! the 3 planes whose intersection defines the point in 3D space.
//! This key is TRANSIENT (used only during the boolean phase).
//! The vertex's permanent identity remains its `VertexId` + `Lineage`.

use forge_geom::plane::Plane;

/// Transient geometric match key for cross-solid vertex deduplication.
///
/// A point in 3D is defined by the intersection of exactly 3 non-parallel
/// planes. This struct stores those 3 plane indices (from the global
/// `PlaneTable`) in sorted order, forming a canonical hash key.
///
/// This is NOT the vertex's permanent identity — that remains the
/// `VertexId` and its `Lineage`. This key is used strictly during
/// the boolean assembly phase to find geometrically coincident vertices
/// across target and tool solids. When a match is found, lineages are
/// merged (D1 compliance) rather than silently dropped.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct VertexMatchKey {
    /// Always sorted: planes[0] < planes[1] < planes[2].
    planes: [usize; 3],
}

impl VertexMatchKey {
    /// Create a match key from exactly 3 plane indices.
    ///
    /// The indices are sorted to ensure canonical representation:
    /// `VertexMatchKey::from_planes(a, b, c) == VertexMatchKey::from_planes(c, a, b)`.
    pub fn from_planes(p0: usize, p1: usize, p2: usize) -> Self {
        let mut planes = [p0, p1, p2];
        planes.sort_unstable();
        Self { planes }
    }

    /// Access the sorted plane indices.
    pub fn planes(&self) -> &[usize; 3] {
        &self.planes
    }
}

/// Check if two planes are parallel (or anti-parallel).
///
/// Uses a strict dot-product check.
pub fn planes_are_parallel(a: &Plane, b: &Plane) -> bool {
    let n1 = a.raw_normal();
    let n2 = b.raw_normal();
    let dot = n1[0] * n2[0] + n1[1] * n2[1] + n1[2] * n2[2];
    dot.abs() > 0.9999999999
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
    let edges = forge_topo::traverse::face_edges(arena, face)?;
    for he in edges {
        let v = arena.get_half_edge(he)?.origin;
        if let Some(pos) = geom.get_vertex_position(v) {
            vertices.push(*pos);
        }
    }
    
    forge_geom::polygon::compute_polygon_centroid(&vertices).ok_or_else(|| {
        forge_core::KernelError::InvalidInput {
            message: format!("Face {:?} has degenerate geometry (no vertices/area)", face),
            context: None,
        }
    })
}
