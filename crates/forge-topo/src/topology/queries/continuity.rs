//! Edge continuity queries.
//!
//! DOMAIN: Read-only continuity measurements for topological edges.
//!
//! INVARIANTS:
//! - Returns `None` for boundary or non-manifold edges
//! - Uses topology traversal only; geometry comes from caller-provided positions
//! - Deterministic traversal order

use forge_core::KernelError;
use forge_math::linalg::{dot, cross, normalize_checked};

use crate::arena::TopologyArena;
use crate::handles::{EdgeId, FaceId, VertexId};

use super::traverse::{FaceEdgeIterator, radial_valence};

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

/// Check whether a manifold edge is G1-continuous under the given angular threshold.
pub fn is_edge_g1_continuous(
    arena: &TopologyArena,
    position_fn: &dyn Fn(VertexId) -> Option<[f64; 3]>,
    edge: EdgeId,
    angle_threshold: f64,
) -> Result<bool, KernelError> {
    let Some(angle) = edge_dihedral_angle(arena, position_fn, edge)? else {
        return Ok(false);
    };

    Ok(angle.abs() <= angle_threshold)
}

fn face_normal_from_outer_loop(
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::traverse::FaceEdgeIterator;
    use std::collections::BTreeMap;

    #[test]
    fn edge_dihedral_angle_returns_none_for_boundary_edge() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();
        let positions = BTreeMap::from([(mvf.vertex.index(), [0.0, 0.0, 0.0])]);
        let position_fn = |vertex: VertexId| positions.get(&vertex.index()).copied();

        let angle = edge_dihedral_angle(state.arena(), &position_fn, mvf.edge).unwrap();
        assert_eq!(angle, None);
    }

    #[test]
    fn coplanar_split_edge_is_g1_continuous() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(&mut draft, SplitEdge { edge: mvf.half_edge, parameter: 0.25 }).unwrap().into_value();
        let se2 = apply_op(&mut draft, SplitEdge { edge: se1.he_mb, parameter: 0.5 }).unwrap().into_value();
        let _se3 = apply_op(&mut draft, SplitEdge { edge: se2.he_mb, parameter: 0.75 }).unwrap().into_value();
        let outer_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face).unwrap()
            .map(|r| r.unwrap())
            .collect();
        let va = draft.arena().get_half_edge(outer_edges[0]).unwrap().origin();
        let vb = draft.arena().get_half_edge(outer_edges[2]).unwrap().origin();
        let mef = apply_op(&mut draft, MakeEdgeFace { face: mvf.face, vertex_a: va, vertex_b: vb }).unwrap().into_value();

        let mut positions = BTreeMap::new();
        let v0 = draft.arena().get_half_edge(outer_edges[0]).unwrap().origin();
        let v1 = draft.arena().get_half_edge(outer_edges[1]).unwrap().origin();
        let v2 = draft.arena().get_half_edge(outer_edges[2]).unwrap().origin();
        let v3 = draft.arena().get_half_edge(outer_edges[3]).unwrap().origin();
        positions.insert(v0.index(), [0.0, 0.0, 0.0]);
        positions.insert(v1.index(), [1.0, 0.0, 0.0]);
        positions.insert(v2.index(), [1.0, 1.0, 0.0]);
        positions.insert(v3.index(), [0.0, 1.0, 0.0]);

        let state = draft.commit().unwrap();
        let position_fn = |vertex: VertexId| positions.get(&vertex.index()).copied();

        let angle = edge_dihedral_angle(state.arena(), &position_fn, mef.edge).unwrap().unwrap();
        let g1 = is_edge_g1_continuous(state.arena(), &position_fn, mef.edge, 1e-9).unwrap();

        assert!(angle.abs() <= 1e-9);
        assert!(g1);
    }
}
