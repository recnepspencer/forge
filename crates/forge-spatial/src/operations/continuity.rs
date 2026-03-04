//! Edge continuity queries.
//!
//! DOMAIN: Read-only continuity measurements for topological edges.
//!
//! INVARIANTS:
//! - Returns `None` for boundary or non-manifold edges
//! - Uses topology traversal only; geometry comes from caller-provided positions
//! - Deterministic traversal order

use forge_core::KernelError;
use forge_core::ToleranceProvider;
use forge_math::linalg::{cross, dot, normalize_checked};

use forge_topo::b_rep::TopologyArena;
use forge_topo::handles::{EdgeId, FaceId, HalfEdgeId, VertexId};
use forge_topo::traverse::{radial_valence, FaceEdgeIterator};

/// Compute the signed dihedral angle (radians) for a manifold edge.
///
/// Returns `Ok(None)` for boundary or non-manifold edges, or when face normals are degenerate.
pub fn edge_dihedral_angle(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    edge: EdgeId,
) -> Result<Option<f64>, KernelError> {
    let edge_data = arena.get_edge(edge)?;
    let he_a = edge_data.half_edge();

    if radial_valence(arena, he_a)? != 2 {
        return Ok(None);
    }

    let he_a_data = arena.get_half_edge(he_a)?;
    let he_b = he_a_data.radial_next();
    let he_b_data = arena.get_half_edge(he_b)?;

    let face_a = he_a_data.face();
    let face_b = he_b_data.face();
    if face_a == face_b {
        return Ok(None);
    }

    let Some(normal_a) = face_normal_from_outer_loop(arena, position_fn, face_a)? else {
        return Ok(None);
    };
    let Some(normal_b) = face_normal_from_outer_loop(arena, position_fn, face_b)? else {
        return Ok(None);
    };

    let origin = he_a_data.origin();
    let dest = arena.get_half_edge(he_a_data.next())?.origin();
    let Some(p0) = position_fn(origin) else {
        return Ok(None);
    };
    let Some(p1) = position_fn(dest) else {
        return Ok(None);
    };

    let edge_vec = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
    let Some(edge_dir) = normalize_checked(edge_vec) else {
        return Ok(None);
    };

    let cross_n = cross(normal_a, normal_b);
    let sin_term = dot(edge_dir, cross_n);
    let cos_term = dot(normal_a, normal_b).clamp(-1.0, 1.0);
    Ok(Some(sin_term.atan2(cos_term)))
}

/// Check whether a manifold edge is G1-continuous under the tolerance provider's threshold.
pub fn is_edge_g1_continuous(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    edge: EdgeId,
    tol: &dyn ToleranceProvider,
) -> Result<bool, KernelError> {
    let angle_threshold = tol.global_default();
    let Some(angle) = edge_dihedral_angle(arena, position_fn, edge)? else {
        return Ok(false);
    };

    Ok(angle.abs() <= angle_threshold)
}

/// Compute the face normal from the outer loop (Newell's method).
pub fn face_normal_from_outer_loop(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    face: FaceId,
) -> Result<Option<[f64; 3]>, KernelError> {
    let mut vertices = Vec::new();
    for he_res in FaceEdgeIterator::new(arena, face)? {
        let he_id = he_res?;
        let vertex_id = arena.get_half_edge(he_id)?.origin();
        let Some(point) = position_fn(vertex_id) else {
            continue;
        };
        vertices.push(point);
    }

    if vertices.len() < 3 {
        return Ok(None);
    }

    let mut nx = 0.0;
    let mut ny = 0.0;
    let mut nz = 0.0;
    let count = vertices.len();
    for i in 0..count {
        let current = vertices[i];
        let next = vertices[(i + 1) % count];
        nx += (current[1] - next[1]) * (current[2] + next[2]);
        ny += (current[2] - next[2]) * (current[0] + next[0]);
        nz += (current[0] - next[0]) * (current[1] + next[1]);
    }

    normalize_checked([nx, ny, nz]).map_or(Ok(None), |normal| Ok(Some(normal)))
}

/// Find a G1-continuous chain of edges starting from `start_edge`.
///
/// Uses the tolerance provider for both the dihedral angle threshold
/// and the degeneracy tolerance.
pub fn find_g1_chain(
    arena: &TopologyArena,
    start_edge: HalfEdgeId,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    tol: &dyn ToleranceProvider,
) -> Result<Vec<HalfEdgeId>, KernelError> {
    let angle_threshold = tol.global_default();
    let cos_threshold = angle_threshold.cos();
    let max_iter = arena.half_edge_count().max(1);
    let mut chain = vec![start_edge];
    let mut current = start_edge;

    for _ in 0..max_iter {
        let he_data = arena.get_half_edge(current)?;

        let twin_id = he_data.radial_next();
        let twin_data = arena.get_half_edge(twin_id)?;
        let candidate = twin_data.next();

        if candidate == start_edge {
            break;
        }

        let candidate_data = arena.get_half_edge(candidate)?;
        if candidate_data.is_bridge() {
            break;
        }

        let face_a = he_data.face();
        let face_b = candidate_data.face();

        if face_a == face_b {
            break;
        }

        let normal_a = face_normal_from_outer_loop(arena, position_fn, face_a)?;
        let normal_b = face_normal_from_outer_loop(arena, position_fn, face_b)?;

        if let (Some(na), Some(nb)) = (normal_a, normal_b) {
            let dot = na[0] * nb[0] + na[1] * nb[1] + na[2] * nb[2];
            if dot < cos_threshold {
                break;
            }
        } else {
            break;
        }

        chain.push(candidate);
        current = candidate;
    }

    Ok(chain)
}

