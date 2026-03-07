use crate::facade::*;

#[test]
fn duplicate_single_cardinality_relation_is_rejected() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let loop_node = draft.create_node(SpecNodeKind::Loop, None, "loop").unwrap();
    let he_a = draft.create_node(SpecNodeKind::HalfEdge, None, "he-a").unwrap();
    let he_b = draft.create_node(SpecNodeKind::HalfEdge, None, "he-b").unwrap();
    draft
        .add_relation(RelationKind::LoopEntryHalfEdge, loop_node, he_a, 0, "entry-a")
        .unwrap();
    let err = draft
        .add_relation(RelationKind::LoopEntryHalfEdge, loop_node, he_b, 0, "entry-b")
        .unwrap_err();
    assert!(format!("{err}").contains("already has an outgoing LoopEntryHalfEdge relation"));
}

#[test]
fn missing_required_halfedge_relations_fail_commit_validation() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let _ = draft.create_node(SpecNodeKind::HalfEdge, None, "dangling-he").unwrap();
    let err = draft.commit().unwrap_err();
    assert!(format!("{err}").contains("requires exactly one outgoing HalfEdgeNext relation"));
}

#[test]
fn invalid_relation_kinds_are_rejected() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex").unwrap();
    let err = draft
        .add_relation(RelationKind::BodyOwnsLump, body, vertex, 0, "invalid")
        .unwrap_err();
    assert!(format!("{err}").contains("is not valid from Body to Vertex"));
}

#[test]
fn ordered_face_inner_loop_ordinals_must_be_unique() {
    let state = SpecState::empty();
    let mut draft = state.into_draft();
    let face = draft.create_node(SpecNodeKind::Face, None, "face").unwrap();
    let outer = draft.create_node(SpecNodeKind::Loop, None, "outer").unwrap();
    let inner_a = draft.create_node(SpecNodeKind::Loop, None, "inner-a").unwrap();
    let inner_b = draft.create_node(SpecNodeKind::Loop, None, "inner-b").unwrap();
    let he_outer = draft.create_node(SpecNodeKind::HalfEdge, None, "outer-he").unwrap();
    let he_inner_a = draft.create_node(SpecNodeKind::HalfEdge, None, "inner-he-a").unwrap();
    let he_inner_b = draft.create_node(SpecNodeKind::HalfEdge, None, "inner-he-b").unwrap();
    let edge = draft.create_node(SpecNodeKind::Edge, None, "edge").unwrap();
    let vertex = draft.create_node(SpecNodeKind::Vertex, None, "vertex").unwrap();

    for he in [he_outer, he_inner_a, he_inner_b] {
        draft.add_relation(RelationKind::HalfEdgeNext, he, he, 0, "next").unwrap();
        draft.add_relation(RelationKind::HalfEdgeRadialNext, he, he, 0, "radial").unwrap();
        draft.add_relation(RelationKind::HalfEdgeUsesEdge, he, edge, 0, "edge").unwrap();
        draft.add_relation(RelationKind::HalfEdgeOriginVertex, he, vertex, 0, "origin").unwrap();
        draft.add_relation(RelationKind::HalfEdgeBoundsFace, he, face, 0, "face").unwrap();
    }

    draft.add_relation(RelationKind::FaceOuterLoop, face, outer, 0, "outer").unwrap();
    draft.add_relation(RelationKind::LoopEntryHalfEdge, outer, he_outer, 0, "outer-entry").unwrap();
    draft.add_relation(RelationKind::LoopEntryHalfEdge, inner_a, he_inner_a, 0, "inner-entry-a").unwrap();
    draft.add_relation(RelationKind::LoopEntryHalfEdge, inner_b, he_inner_b, 0, "inner-entry-b").unwrap();
    draft.add_relation(RelationKind::FaceInnerLoop, face, inner_a, 0, "inner-a").unwrap();
    let err = draft
        .add_relation(RelationKind::FaceInnerLoop, face, inner_b, 0, "inner-b")
        .unwrap_err();
    assert!(format!("{err}").contains("already has inner-loop ordinal 0"));
}
