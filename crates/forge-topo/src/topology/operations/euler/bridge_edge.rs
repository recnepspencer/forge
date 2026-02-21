//! BridgeEdge — splice an inner loop (hole) into the outer boundary.
//!
//! DOMAIN: Given a face with an outer loop and an inner loop (hole),
//! insert two zero-width half-edges connecting a vertex on the outer
//! boundary to a vertex on the inner boundary. This absorbs the inner
//! loop into the outer loop, allowing the standard DCEL to represent
//! faces with holes.
//!
//! INVARIANTS:
//! - `outer_he` must originate from a vertex on the face's outer loop
//! - `inner_he` must originate from a vertex on one of the face's inner loops
//! - Creates 2 new halfedges, removes 1 inner loop
//! - Euler formula: E+1 (2 half-edges = 1 edge), L-1 (net: same V-E+F+L)
//!
//! DEPENDENCIES: `arena` (entity storage), `lineage` (provenance)

use forge_core::KernelError;

use crate::arena::HalfEdgeData;
use crate::handles::{FaceId, HalfEdgeId, LoopId};
use crate::lineage::{Lineage, OpSignature};
use crate::state::MutableDraft;
use crate::EulerOperator;

/// Splice an inner loop into the outer boundary via two bridge half-edges.
///
/// `outer_he` originates from the target vertex on the outer loop.
/// `inner_he` originates from the splice vertex on the inner loop.
/// After execution, the inner loop is absorbed: the outer loop
/// traverses into the hole and back via zero-width bridge edges.
#[derive(Debug)]
pub struct BridgeEdge {
    /// Half-edge on the outer loop starting at the bridge target vertex.
    pub outer_he: HalfEdgeId,
    /// Half-edge on the inner loop starting at the bridge source vertex.
    pub inner_he: HalfEdgeId,
    /// The face containing both loops.
    pub face: FaceId,
}

/// Output of the BridgeEdge operator.
pub struct BridgeEdgeOutput {
    /// Bridge half-edge from outer vertex into the inner loop.
    pub he_into_hole: HalfEdgeId,
    /// Bridge half-edge from inner vertex back to the outer loop.
    pub he_out_of_hole: HalfEdgeId,
}

impl EulerOperator for BridgeEdge {
    type Output = BridgeEdgeOutput;

    fn execute(&self, draft: &mut MutableDraft, sig: &OpSignature) -> Result<Self::Output, KernelError> {
        validate_bridge_preconditions(draft, self.face, self.outer_he, self.inner_he)?;

        let outer_prev = draft.arena().get_half_edge(self.outer_he)?.prev();
        let inner_prev = draft.arena().get_half_edge(self.inner_he)?.prev();

        let outer_vertex = draft.arena().get_half_edge(self.outer_he)?.origin();
        let inner_vertex = draft.arena().get_half_edge(self.inner_he)?.origin();

        let face_lineage = draft.arena().get_face(self.face)?.lineage().cloned();
        let bridge_in_lineage = Lineage::derive_from(&face_lineage, sig.clone());
        let bridge_out_lineage = Lineage::derive_from(&face_lineage, sig.clone());

        let inner_loop_id = find_inner_loop_containing(draft, self.face, self.inner_he)?;

        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);

        let (he_into_hole, he_out_of_hole) = draft.arena_mut().insert_half_edge_pair(
            HalfEdgeData::with_lineage(
                placeholder_he,
                self.inner_he,
                outer_prev,
                self.face,
                outer_vertex,
                Some(bridge_in_lineage),
            ),
            HalfEdgeData::with_lineage(
                placeholder_he,
                self.outer_he,
                inner_prev,
                self.face,
                inner_vertex,
                Some(bridge_out_lineage),
            ),
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(outer_prev)?.set_next(he_into_hole);
        arena.get_half_edge_mut(self.inner_he)?.set_prev(he_into_hole);
        arena.get_half_edge_mut(inner_prev)?.set_next(he_out_of_hole);
        arena.get_half_edge_mut(self.outer_he)?.set_prev(he_out_of_hole);

        let outer_loop_id = arena.get_face(self.face)?.outer_loop();
        arena.get_loop_mut(outer_loop_id)?.set_half_edge(he_into_hole);

        arena.get_face_mut(self.face)?.remove_inner_loop(inner_loop_id);
        arena.remove_loop(inner_loop_id)?;

        Ok(BridgeEdgeOutput {
            he_into_hole,
            he_out_of_hole,
        })
    }

    fn signature(&self) -> OpSignature {
        OpSignature::new("bridge_edge")
    }
}

/// Validate that the bridge preconditions hold.
fn validate_bridge_preconditions(
    draft: &MutableDraft,
    face: FaceId,
    outer_he: HalfEdgeId,
    inner_he: HalfEdgeId,
) -> Result<(), KernelError> {
    let outer_face = draft.arena().get_half_edge(outer_he)?.face();
    let inner_face = draft.arena().get_half_edge(inner_he)?.face();

    if outer_face != face {
        return Err(KernelError::InvalidInput {
            message: format!(
                "BridgeEdge: outer_he {} belongs to face {}, expected {}",
                outer_he.index(), outer_face.index(), face.index()
            ),
            context: None,
        });
    }

    if inner_face != face {
        return Err(KernelError::InvalidInput {
            message: format!(
                "BridgeEdge: inner_he {} belongs to face {}, expected {}",
                inner_he.index(), inner_face.index(), face.index()
            ),
            context: None,
        });
    }

    Ok(())
}

/// Find which inner loop of `face` contains `inner_he`.
fn find_inner_loop_containing(
    draft: &MutableDraft,
    face: FaceId,
    inner_he: HalfEdgeId,
) -> Result<LoopId, KernelError> {
    let face_data = draft.arena().get_face(face)?;
    for &loop_id in face_data.inner_loops() {
        let loop_data = draft.arena().get_loop(loop_id)?;
        let start = loop_data.half_edge();
        let mut current = start;
        let mut steps = 0usize;

        loop {
            if current == inner_he {
                return Ok(loop_id);
            }
            current = draft.arena().get_half_edge(current)?.next();
            steps += 1;
            if current == start || steps > 100_000 {
                break;
            }
        }
    }

    Err(KernelError::InvalidInput {
        message: format!(
            "BridgeEdge: inner_he {} not found in any inner loop of face {}",
            inner_he.index(), face.index()
        ),
        context: None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::arena::{FaceData, LoopData, HalfEdgeData, VertexData};
    use crate::handles::VertexId;
    use crate::state::TopologyState;
    use crate::operator::apply_op;

    /// Build a face with an outer triangle (v0→v1→v2) and inner triangle hole (v3→v4→v5).
    fn build_face_with_hole(
        draft: &mut MutableDraft,
    ) -> (FaceId, HalfEdgeId, HalfEdgeId, LoopId, [VertexId; 6]) {
        let placeholder_he = HalfEdgeId::new(u32::MAX, 0);
        let placeholder_loop = LoopId::new(u32::MAX, 0);
        let placeholder_face = FaceId::new(u32::MAX, 0);

        let arena = draft.arena_mut();

        let face = arena.insert_face(FaceData::new(placeholder_loop));
        let outer_loop = arena.insert_loop(LoopData::new(placeholder_he, face));
        arena.get_face_mut(face).unwrap().set_outer_loop(outer_loop);

        let v0 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v1 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v2 = arena.insert_vertex(VertexData::new(placeholder_he));

        let (he01, _he10) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v0),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v1),
        );
        let (he12, _he21) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v1),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v2),
        );
        let (he20, _he02) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v2),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v0),
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he01).unwrap().set_next(he12);
        arena.get_half_edge_mut(he01).unwrap().set_prev(he20);
        arena.get_half_edge_mut(he12).unwrap().set_next(he20);
        arena.get_half_edge_mut(he12).unwrap().set_prev(he01);
        arena.get_half_edge_mut(he20).unwrap().set_next(he01);
        arena.get_half_edge_mut(he20).unwrap().set_prev(he12);

        arena.get_loop_mut(outer_loop).unwrap().set_half_edge(he01);
        arena.get_vertex_mut(v0).unwrap().set_outgoing(he01);
        arena.get_vertex_mut(v1).unwrap().set_outgoing(he12);
        arena.get_vertex_mut(v2).unwrap().set_outgoing(he20);

        let v3 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v4 = arena.insert_vertex(VertexData::new(placeholder_he));
        let v5 = arena.insert_vertex(VertexData::new(placeholder_he));

        let (he34, _he43) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v3),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v4),
        );
        let (he45, _he54) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v4),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v5),
        );
        let (he53, _he35) = arena.insert_half_edge_pair(
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, face, v5),
            HalfEdgeData::new(placeholder_he, placeholder_he, placeholder_he, placeholder_face, v3),
        );

        let arena = draft.arena_mut();
        arena.get_half_edge_mut(he34).unwrap().set_next(he45);
        arena.get_half_edge_mut(he34).unwrap().set_prev(he53);
        arena.get_half_edge_mut(he45).unwrap().set_next(he53);
        arena.get_half_edge_mut(he45).unwrap().set_prev(he34);
        arena.get_half_edge_mut(he53).unwrap().set_next(he34);
        arena.get_half_edge_mut(he53).unwrap().set_prev(he45);

        arena.get_vertex_mut(v3).unwrap().set_outgoing(he34);
        arena.get_vertex_mut(v4).unwrap().set_outgoing(he45);
        arena.get_vertex_mut(v5).unwrap().set_outgoing(he53);

        let inner_loop = arena.insert_loop(LoopData::new(he34, face));
        arena.get_face_mut(face).unwrap().add_inner_loop(inner_loop);

        (face, he01, he34, inner_loop, [v0, v1, v2, v3, v4, v5])
    }

    #[test]
    fn bridge_edge_absorbs_inner_loop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) =
            build_face_with_hole(&mut draft);

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

        let result = apply_op(
            &mut draft,
            BridgeEdge { outer_he, inner_he, face },
        )
        .unwrap()
        .into_value();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let he_in = result.he_into_hole;
        let he_out = result.he_out_of_hole;
        assert_eq!(
            draft.arena().get_half_edge(he_in).unwrap().twin(),
            he_out,
        );
        assert_eq!(
            draft.arena().get_half_edge(he_out).unwrap().twin(),
            he_in,
        );
    }

    #[test]
    fn bridge_edge_creates_single_loop_traversal() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) =
            build_face_with_hole(&mut draft);

        let _result = apply_op(
            &mut draft,
            BridgeEdge { outer_he, inner_he, face },
        )
        .unwrap()
        .into_value();

        let outer_loop = draft.arena().get_face(face).unwrap().outer_loop();
        let start_he = draft.arena().get_loop(outer_loop).unwrap().half_edge();
        let mut current = start_he;
        let mut count = 0usize;

        loop {
            count += 1;
            assert!(count <= 100, "Infinite loop detected in traversal");
            current = draft.arena().get_half_edge(current).unwrap().next();
            if current == start_he {
                break;
            }
        }

        assert_eq!(count, 8, "Outer triangle (3) + inner triangle (3) + 2 bridge edges = 8");
    }

    #[test]
    fn bridge_edge_rejects_wrong_face() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) =
            build_face_with_hole(&mut draft);

        let wrong_face = FaceId::new(999, 0);
        let result = apply_op(
            &mut draft,
            BridgeEdge { outer_he, inner_he, face: wrong_face },
        );

        assert!(result.is_err());
        let _ = face;
    }
}
