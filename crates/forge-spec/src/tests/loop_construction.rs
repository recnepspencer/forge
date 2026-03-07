use crate::facade::*;

#[test]
fn make_loop_in_face_from_vertices_mutation_builds_inner_loop_from_existing_vertices() {
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

    let result = draft
        .execute(MakeLoopInFaceFromVerticesMutation {
            face: face.face,
            vertices: vec![h0, h1, h2],
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
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.loop_id)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::LoopEntryHalfEdge)
            .count(),
        1
    );
}
