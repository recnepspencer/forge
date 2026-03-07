use crate::facade::*;

#[test]
fn join_faces_mutation_merges_adjacent_faces() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    let bridge = draft
        .execute(MakeEdgeFaceMutation {
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
            face: seed.face,
        })
        .unwrap()
        .value;

    let result = draft
        .execute(JoinFacesMutation {
            half_edge: bridge.half_edge_ab,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert!(state.graph().node(result.value.surviving_face).is_some());
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Face)
            .count(),
        1
    );
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Loop)
            .count(),
        1
    );
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Edge)
            .count(),
        2
    );
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::HalfEdge)
            .count(),
        2
    );
}
