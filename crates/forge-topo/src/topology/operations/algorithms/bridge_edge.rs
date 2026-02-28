//! BridgeEdge — splice an inner loop (hole) into the outer boundary.
//!
//! DOMAIN: Given a face with an outer loop and an inner loop (hole),
//! insert two zero-width half-edges connecting a vertex on the outer
//! boundary to a vertex on the inner boundary. This absorbs the inner
//! loop into the outer loop, allowing the standard DCEL to represent
//! faces with holes.
//!
//! This is a compound algorithm that invokes the `MakeEdgeKillLoop` Euler
//! Operator and tags the resulting halfedges as bridge edges.
//!
//! DEPENDENCIES: `arena` (entity storage)

use crate::handles::HalfEdgeId;
use crate::operator::apply_op;
use crate::state::MutableDraft;
use crate::topology::operations::boundary_editing::make_edge_kill_loop::MakeEdgeKillLoop;
use forge_core::KernelError;

/// Output of the bridge_edge algorithm.
pub struct BridgeEdgeOutput {
    /// Bridge half-edge from outer vertex into the inner loop.
    pub he_into_hole: HalfEdgeId,
    /// Bridge half-edge from inner vertex back to the outer loop.
    pub he_out_of_hole: HalfEdgeId,
}

/// Splice an inner loop into the outer boundary via two bridge half-edges.
///
/// `outer_he` originates from the target vertex on the outer loop.
/// `inner_he` originates from the splice vertex on the inner loop.
/// After execution, the inner loop is absorbed: the outer loop
/// traverses into the hole and back via zero-width bridge edges.
///
/// Validation (same face, outer vs inner loop membership) is handled
/// by the underlying `MakeEdgeKillLoop` operator.
pub fn bridge_edge(
    draft: &mut MutableDraft,
    outer_he: HalfEdgeId,
    inner_he: HalfEdgeId,
) -> Result<BridgeEdgeOutput, KernelError> {
    let mekl = apply_op(
        draft,
        MakeEdgeKillLoop {
            he_a: outer_he,
            he_b: inner_he,
        },
    )?
    .into_value();

    {
        let arena = draft.arena_mut();
        arena.get_half_edge_mut(mekl.he_ab)?.set_bridge(true);
        arena.get_half_edge_mut(mekl.he_ba)?.set_bridge(true);
    }

    Ok(BridgeEdgeOutput {
        he_into_hole: mekl.he_ab,
        he_out_of_hole: mekl.he_ba,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::TopologyState;
    use crate::testing::build_face_with_hole;

    #[test]
    fn bridge_edge_absorbs_inner_loop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 1);

        let result = bridge_edge(&mut draft, outer_he, inner_he).unwrap();

        assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

        let he_in = result.he_into_hole;
        let he_out = result.he_out_of_hole;
        assert_eq!(
            draft.arena().get_half_edge(he_in).unwrap().radial_next(),
            he_out,
        );
        assert_eq!(
            draft.arena().get_half_edge(he_out).unwrap().radial_next(),
            he_in,
        );

        assert!(draft.arena().get_half_edge(he_in).unwrap().is_bridge());
        assert!(draft.arena().get_half_edge(he_out).unwrap().is_bridge());
    }

    #[test]
    fn bridge_edge_creates_single_loop_traversal() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (face, outer_he, inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

        let _result = bridge_edge(&mut draft, outer_he, inner_he).unwrap();

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

        assert_eq!(
            count, 8,
            "Outer triangle (3) + inner triangle (3) + 2 bridge edges = 8"
        );
    }

    #[test]
    fn bridge_edge_rejects_both_on_outer_loop() {
        let state = TopologyState::empty();
        let mut draft = state.into_mutation();

        let (_face, outer_he, _inner_he, _inner_loop, _verts) = build_face_with_hole(&mut draft);

        let outer_he_next = draft.arena().get_half_edge(outer_he).unwrap().next();
        let result = bridge_edge(&mut draft, outer_he, outer_he_next);

        assert!(
            result.is_err(),
            "Both halfedges on the outer loop must be rejected"
        );
    }
}
