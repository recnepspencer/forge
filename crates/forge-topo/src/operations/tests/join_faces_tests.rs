//! Tests for JoinFaces: merging two adjacent faces (inverse of MEF).
//!
//! DOMAIN: JoinFaces removes the shared edge between two faces. The
//! surviving face absorbs the other's topology including inner loops.

use crate::b_rep::ShellKind;
use crate::boundary_editing::join_faces::JoinFaces;
use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::transactions::TopologyState;

/// JoinFaces reduces face count by 1 and leaves a valid surviving face.
#[test]
fn join_faces_merges_two_adjacent_faces() {
    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let se = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: mvf.vertex,
            vertex_b: se.new_vertex,
            face: mvf.face,
        })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().face_count(), 2);

    let jf = draft
        .execute(JoinFaces {
            edge: mef.half_edge_ab,
        })
        .unwrap()
        .into_value();

    assert_eq!(draft.arena().face_count(), 1);
    assert!(draft.arena().get_face(jf.surviving_face).is_ok());
}

// TODO(Phase 3): Re-enable once LineageStore lookup is wired.
// /// JoinFaces records its operation name in the surviving face's lineage.
// #[test]
// fn join_faces_updates_surviving_lineage() { ... }

/// JoinFaces transfers inner loops from the removed face to the survivor.
///
/// Inner loops are manually constructed here because no Euler operator
/// creates inner loops directly. This is intentional: inner loops represent
/// holes cut via boolean operations, which is outside the Euler operator scope.
/// The manual construction is constrained to isolated vertex orbits so it
/// does not corrupt the outer topology.
#[test]
fn join_faces_preserves_inner_loops() {
    use crate::b_rep::{HalfEdgeData, LoopData, VertexData};
    use crate::handles::HalfEdgeId;

    let state = TopologyState::empty();
    let mut draft = state.into_mutation();

    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let v0 = mvf.vertex;
    let face = mvf.face;

    let se1 = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    let v2 = se1.new_vertex;

    let mef = draft
        .execute(MakeEdgeFace {
            vertex_a: v0,
            vertex_b: v2,
            face,
        })
        .unwrap()
        .into_value();

    let target_face = mef.new_face;
    let placeholder_he = HalfEdgeId::DANGLING;

    let (inner_loop, ihe01, ihe12, ihe20) = {
        let arena = draft.arena_mut();

        // Create an exterior face for the twin half-edges.
        // In a valid B-Rep, the twins of an inner-loop's edges border a
        // different face (typically the exterior/void).
        let ext_face = arena.insert_face(crate::b_rep::FaceData::new(
            crate::handles::LoopId::DANGLING,
            crate::handles::ShellId::DANGLING,
        ));
        let ext_loop = arena.insert_loop(LoopData::new(placeholder_he, ext_face));
        arena
            .get_face_mut(ext_face)
            .unwrap()
            .loops
            .set_outer(ext_loop);

        let iv0 = arena.insert_vertex(VertexData::new(placeholder_he));
        let iv1 = arena.insert_vertex(VertexData::new(placeholder_he));
        let iv2 = arena.insert_vertex(VertexData::new(placeholder_he));

        let ie01 = arena.insert_edge(crate::b_rep::EdgeData::new(placeholder_he));
        let ie12 = arena.insert_edge(crate::b_rep::EdgeData::new(placeholder_he));
        let ie20 = arena.insert_edge(crate::b_rep::EdgeData::new(placeholder_he));

        let (ihe01, ihe01_t) = arena.insert_radial_pair(
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                target_face,
                iv0,
                ie01,
            ),
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                ext_face,
                iv1,
                ie01,
            ),
        );
        let (ihe12, ihe12_t) = arena.insert_radial_pair(
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                target_face,
                iv1,
                ie12,
            ),
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                ext_face,
                iv2,
                ie12,
            ),
        );
        let (ihe20, ihe20_t) = arena.insert_radial_pair(
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                target_face,
                iv2,
                ie20,
            ),
            HalfEdgeData::new(
                placeholder_he,
                placeholder_he,
                placeholder_he,
                ext_face,
                iv0,
                ie20,
            ),
        );

        arena.get_edge_mut(ie01).unwrap().set_half_edge(ihe01);
        arena.get_edge_mut(ie12).unwrap().set_half_edge(ihe12);
        arena.get_edge_mut(ie20).unwrap().set_half_edge(ihe20);

        // Inner loop (primary): he01 → he12 → he20 → he01
        arena.get_half_edge_mut(ihe01).unwrap().set_next(ihe12);
        arena.get_half_edge_mut(ihe01).unwrap().set_prev(ihe20);
        arena.get_half_edge_mut(ihe12).unwrap().set_next(ihe20);
        arena.get_half_edge_mut(ihe12).unwrap().set_prev(ihe01);
        arena.get_half_edge_mut(ihe20).unwrap().set_next(ihe01);
        arena.get_half_edge_mut(ihe20).unwrap().set_prev(ihe12);

        // Twin loop (exterior): reversed winding on ext_face
        arena.get_half_edge_mut(ihe01_t).unwrap().set_next(ihe20_t);
        arena.get_half_edge_mut(ihe01_t).unwrap().set_prev(ihe12_t);
        arena.get_half_edge_mut(ihe20_t).unwrap().set_next(ihe12_t);
        arena.get_half_edge_mut(ihe20_t).unwrap().set_prev(ihe01_t);
        arena.get_half_edge_mut(ihe12_t).unwrap().set_next(ihe01_t);
        arena.get_half_edge_mut(ihe12_t).unwrap().set_prev(ihe20_t);

        arena.get_loop_mut(ext_loop).unwrap().set_half_edge(ihe01_t);

        arena.get_vertex_mut(iv0).unwrap().set_primary_disk(ihe01);
        arena.get_vertex_mut(iv1).unwrap().set_primary_disk(ihe12);
        arena.get_vertex_mut(iv2).unwrap().set_primary_disk(ihe20);

        let inner_loop = arena.insert_loop(LoopData::new(ihe01, target_face));
        arena
            .get_face_mut(target_face)
            .unwrap()
            .loops
            .add_inner(inner_loop);
        (inner_loop, ihe01, ihe12, ihe20)
    };

    assert_eq!(
        draft
            .arena()
            .get_face(target_face)
            .unwrap()
            .inner_loop_count(),
        1
    );
    assert_eq!(draft.arena().get_face(face).unwrap().inner_loop_count(), 0);

    let jf = draft
        .execute(JoinFaces {
            edge: mef.half_edge_ab,
        })
        .unwrap()
        .into_value();
    let surviving = jf.surviving_face;

    assert_eq!(
        draft
            .arena()
            .get_face(surviving)
            .unwrap()
            .inner_loop_count(),
        1,
        "Inner loop must survive on the merged face"
    );
    assert_eq!(
        draft.arena().get_loop(inner_loop).unwrap().face(),
        surviving
    );
    assert_eq!(
        draft.arena().get_half_edge(ihe01).unwrap().face(),
        surviving
    );
    assert_eq!(
        draft.arena().get_half_edge(ihe12).unwrap().face(),
        surviving
    );
    assert_eq!(
        draft.arena().get_half_edge(ihe20).unwrap().face(),
        surviving
    );
}
