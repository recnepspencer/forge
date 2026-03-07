use crate::facade::*;

fn build_face_with_hole_draft() -> (SpecDraft, MakeFaceFromVerticesOutput, SpecNodeId) {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let face = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap()
        .value;
    let h0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let h2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let hole = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face: face.face,
            vertices: vec![h0, h1, h2],
        })
        .unwrap()
        .value;
    (draft, face, hole.loop_id)
}

#[test]
fn make_face_kill_ring_hole_mutation_promotes_inner_loop_to_new_face() {
    let (mut draft, face, loop_id) = build_face_with_hole_draft();
    let result = draft
        .execute(MakeFaceKillRingHoleMutation { loop_id })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 26);
    assert_eq!(state.graph().iter_relations().count(), 39);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(face.face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceInnerLoop)
            .count(),
        0
    );
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.new_face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceOuterLoop)
            .count(),
        1
    );
}

#[test]
fn kill_face_make_ring_hole_mutation_demotes_face_back_to_inner_loop() {
    let (mut draft, face, loop_id) = build_face_with_hole_draft();
    let promoted = draft
        .execute(MakeFaceKillRingHoleMutation { loop_id })
        .unwrap()
        .value;

    let result = draft
        .execute(KillFaceMakeRingHoleMutation {
            face_to_kill: promoted.new_face,
            target_face: face.face,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 25);
    assert_eq!(state.graph().iter_relations().count(), 38);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(face.face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceInnerLoop)
            .count(),
        1
    );
}
