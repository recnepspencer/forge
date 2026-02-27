//! Topological classification and neighborhood queries.
//!
//! DOMAIN: Read-only predicates and adjacency helpers for faces, loops,
//! edges, and vertices.
//!
//! INVARIANTS:
//! - Deterministic output ordering via `BTreeSet`
//! - No floating-point comparisons
//! - Corruption-safe traversal delegates to `traverse` iterators

use std::collections::BTreeSet;

use forge_core::KernelError;

use crate::arena::TopologyArena;
use crate::handles::{FaceId, HalfEdgeId, LoopId, VertexId};

use super::traverse::{
    face_loops, is_boundary_edge, radial_valence, vertex_neighborhood_orbits, FaceAllEdgesIterator,
    VertexRingIterator,
};

/// Return all distinct faces adjacent to `face` across shared geometric edges.
pub fn face_adjacent_faces(
    arena: &TopologyArena,
    face: FaceId,
) -> Result<Vec<FaceId>, KernelError> {
    arena.get_face(face)?;

    let mut adjacent = BTreeSet::new();

    for he_res in FaceAllEdgesIterator::new(arena, face)? {
        let he_id = he_res?;
        let he_data = arena.get_half_edge(he_id)?;
        let radial = he_data.radial_next();
        let radial_face = arena.get_half_edge(radial)?.face();
        if radial_face != face {
            adjacent.insert(radial_face);
        }
    }

    Ok(adjacent.into_iter().collect())
}

/// Return all distinct faces incident to a vertex.
pub fn vertex_faces(arena: &TopologyArena, vertex: VertexId) -> Result<Vec<FaceId>, KernelError> {
    arena.get_vertex(vertex)?;

    let mut faces = BTreeSet::new();
    for he_res in VertexRingIterator::new(arena, vertex)? {
        let he_id = he_res?;
        let face_id = arena.get_half_edge(he_id)?.face();
        faces.insert(face_id);
    }

    Ok(faces.into_iter().collect())
}

/// True when the radial valence of the edge is exactly 2.
pub fn is_manifold_edge(arena: &TopologyArena, he: HalfEdgeId) -> Result<bool, KernelError> {
    Ok(radial_valence(arena, he)? == 2)
}

/// True when the radial valence of the edge is greater than 2.
pub fn is_non_manifold_edge(arena: &TopologyArena, he: HalfEdgeId) -> Result<bool, KernelError> {
    Ok(radial_valence(arena, he)? > 2)
}

/// Alias for boundary-edge classification (valence 1).
pub fn is_laminar_edge(arena: &TopologyArena, he: HalfEdgeId) -> Result<bool, KernelError> {
    is_boundary_edge(arena, he)
}

/// True when all incident halfedges belong to a single connected umbrella.
pub fn vertex_is_manifold(arena: &TopologyArena, vertex: VertexId) -> Result<bool, KernelError> {
    let orbits = vertex_neighborhood_orbits(arena, vertex)?;
    Ok(orbits.len() <= 1)
}

/// True when `loop_id` is the face outer loop.
pub fn is_outer_loop(
    arena: &TopologyArena,
    face: FaceId,
    loop_id: LoopId,
) -> Result<bool, KernelError> {
    Ok(arena.get_face(face)?.outer_loop() == loop_id)
}

/// True when `loop_id` is one of the face inner loops.
pub fn is_inner_loop(
    arena: &TopologyArena,
    face: FaceId,
    loop_id: LoopId,
) -> Result<bool, KernelError> {
    if is_outer_loop(arena, face, loop_id)? {
        return Ok(false);
    }

    let loops = face_loops(arena, face)?;
    Ok(loops
        .into_iter()
        .skip(1)
        .any(|candidate| candidate == loop_id))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::euler::make_edge_face::MakeEdgeFace;
    use crate::euler::make_loop_in_face_from_vertices::MakeLoopInFaceFromVertices;
    use crate::euler::make_vertex_face::MakeVertexFace;
    use crate::euler::split_edge::SplitEdge;
    use crate::operator::apply_op;
    use crate::state::TopologyState;
    use crate::traverse::FaceEdgeIterator;

    #[test]
    fn seed_edge_and_vertex_classification_is_boundary_and_manifold_vertex() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let state = draft.commit().unwrap();

        assert!(is_boundary_edge(state.arena(), mvf.half_edge).unwrap());
        assert!(is_laminar_edge(state.arena(), mvf.half_edge).unwrap());
        assert!(!is_manifold_edge(state.arena(), mvf.half_edge).unwrap());
        assert!(!is_non_manifold_edge(state.arena(), mvf.half_edge).unwrap());
        assert!(vertex_is_manifold(state.arena(), mvf.vertex).unwrap());
        assert_eq!(
            vertex_faces(state.arena(), mvf.vertex).unwrap(),
            vec![mvf.face]
        );
    }

    #[test]
    fn face_adjacency_detects_split_faces() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.25,
            },
        )
        .unwrap()
        .into_value();
        let se2 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se1.he_mb,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se2.he_mb,
                parameter: 0.75,
            },
        )
        .unwrap()
        .into_value();
        let outer_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let va = draft
            .arena()
            .get_half_edge(outer_edges[0])
            .unwrap()
            .origin();
        let vb = draft
            .arena()
            .get_half_edge(outer_edges[2])
            .unwrap()
            .origin();
        let mef = apply_op(
            &mut draft,
            MakeEdgeFace {
                face: mvf.face,
                vertex_a: va,
                vertex_b: vb,
            },
        )
        .unwrap()
        .into_value();
        let state = draft.commit().unwrap();

        assert_eq!(
            face_adjacent_faces(state.arena(), mvf.face).unwrap(),
            vec![mef.new_face]
        );
        assert_eq!(
            face_adjacent_faces(state.arena(), mef.new_face).unwrap(),
            vec![mvf.face]
        );
    }

    #[test]
    fn loop_classification_distinguishes_outer_and_inner() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();
        let mvf = apply_op(&mut draft, MakeVertexFace).unwrap().into_value();
        let se1 = apply_op(
            &mut draft,
            SplitEdge {
                edge: mvf.half_edge,
                parameter: 0.25,
            },
        )
        .unwrap()
        .into_value();
        let se2 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se1.he_mb,
                parameter: 0.5,
            },
        )
        .unwrap()
        .into_value();
        let _se3 = apply_op(
            &mut draft,
            SplitEdge {
                edge: se2.he_mb,
                parameter: 0.75,
            },
        )
        .unwrap()
        .into_value();
        let outer_edges: Vec<_> = FaceEdgeIterator::new(draft.arena(), mvf.face)
            .unwrap()
            .map(|r| r.unwrap())
            .collect();
        let v0 = draft
            .arena()
            .get_half_edge(outer_edges[0])
            .unwrap()
            .origin();
        let v1 = draft
            .arena()
            .get_half_edge(outer_edges[1])
            .unwrap()
            .origin();
        let v2 = draft
            .arena()
            .get_half_edge(outer_edges[2])
            .unwrap()
            .origin();
        let inner = apply_op(
            &mut draft,
            MakeLoopInFaceFromVertices {
                face: mvf.face,
                vertices: vec![v0, v1, v2],
            },
        )
        .unwrap()
        .into_value();
        let state = draft.commit().unwrap();

        assert!(is_outer_loop(state.arena(), mvf.face, mvf.loop_id).unwrap());
        assert!(!is_inner_loop(state.arena(), mvf.face, mvf.loop_id).unwrap());
        assert!(!is_outer_loop(state.arena(), mvf.face, inner.loop_id).unwrap());
        assert!(is_inner_loop(state.arena(), mvf.face, inner.loop_id).unwrap());
    }
}
