use crate::facade::*;

fn build_face_with_hole_draft() -> (SpecDraft, SpecNodeId, SpecNodeId, SpecNodeId) {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let v1 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let v2 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value
        .face;
    let h0 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let h1 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let h2 = draft
        .execute(MakeIsolatedVertexMutation)
        .unwrap()
        .value
        .vertex;
    let loop_output = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap()
        .value;
    (draft, face, loop_output.half_edges[0], loop_output.loop_id)
}

#[test]
fn make_edge_kill_loop_mutation_absorbs_inner_loop() {
    let (mut draft, face, inner_half_edge, killed_loop) = build_face_with_hole_draft();
    let outer_half_edge = draft
        .single_outgoing_target(face, RelationKind::FaceOuterLoop)
        .and_then(|loop_id| draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge))
        .unwrap();

    let result = draft
        .execute(MakeEdgeKillLoopMutation {
            half_edge_a: outer_half_edge,
            half_edge_b: inner_half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 27);
    assert_eq!(state.graph().iter_relations().count(), 46);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceInnerLoop)
            .count(),
        0
    );
    assert!(state.graph().node(killed_loop).is_none());
}

#[test]
fn kill_edge_make_loop_mutation_restores_inner_loop() {
    let (mut draft, face, inner_half_edge, _killed_loop) = build_face_with_hole_draft();
    let outer_half_edge = draft
        .single_outgoing_target(face, RelationKind::FaceOuterLoop)
        .and_then(|loop_id| draft.single_outgoing_target(loop_id, RelationKind::LoopEntryHalfEdge))
        .unwrap();

    let bridge = draft
        .execute(MakeEdgeKillLoopMutation {
            half_edge_a: outer_half_edge,
            half_edge_b: inner_half_edge,
        })
        .unwrap()
        .value;

    let result = draft
        .execute(KillEdgeMakeLoopMutation {
            half_edge: bridge.half_edge_ab,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 25);
    assert_eq!(state.graph().iter_relations().count(), 38);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceInnerLoop)
            .count(),
        1
    );
}
