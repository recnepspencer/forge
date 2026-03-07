use crate::facade::*;

#[test]
fn node_ids_are_deterministic_for_same_namespace_sequence_and_role() {
    let mut a = DeterministicIdAllocator::new(7, 0);
    let mut b = DeterministicIdAllocator::new(7, 0);
    assert_eq!(
        a.mint_node_id(SpecNodeKind::Feature, "root"),
        b.mint_node_id(SpecNodeKind::Feature, "root")
    );
}

#[test]
fn state_serialization_is_deterministic() {
    let state_a = SpecState::empty();
    let state_b = SpecState::empty();
    assert_eq!(state_a.canonical_bytes(), state_b.canonical_bytes());
    assert_eq!(state_a.spec_hash(), state_b.spec_hash());
}

#[test]
fn make_vertex_face_mutation_builds_minimal_seed_topology() {
    let mut draft = SpecState::empty().into_draft();
    let result = draft.execute(MakeVertexFaceMutation).unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 9);
    assert_eq!(state.graph().iter_relations().count(), 11);

    let face = state.graph().node(result.value.face).unwrap();
    assert_eq!(face.kind, SpecNodeKind::Face);

    let outgoing = state
        .graph()
        .outgoing_relations(result.value.half_edge);
    assert_eq!(outgoing.len(), 5);
}

#[test]
fn make_edge_vertex_mutation_rewires_seed_loop() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let result = draft
        .execute(MakeEdgeVertexMutation {
            anchor: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 13);
    assert_eq!(state.graph().iter_relations().count(), 21);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(seed.value.half_edge)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::HalfEdgeNext)
            .count(),
        1
    );
}

#[test]
fn kill_edge_vertex_mutation_removes_restricted_wire_edge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let mev = draft
        .execute(MakeEdgeVertexMutation {
            anchor: seed.value.half_edge,
        })
        .unwrap();
    draft
        .execute(KillEdgeVertexMutation {
            half_edge: mev.value.he_out,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 9);
    assert_eq!(state.graph().iter_relations().count(), 11);
}

#[test]
fn make_face_vertex_mutation_creates_disjoint_face_seed_in_shell() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let result = draft
        .execute(MakeFaceVertexMutation {
            shell: seed.value.shell,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 14);
    assert_eq!(state.graph().iter_relations().count(), 19);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(seed.value.shell)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::ShellOwnsFace)
            .count(),
        2
    );
}

#[test]
fn kill_face_vertex_mutation_removes_only_disjoint_face_seed() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let face_seed = draft
        .execute(MakeFaceVertexMutation {
            shell: seed.value.shell,
        })
        .unwrap();
    draft
        .execute(KillFaceVertexMutation {
            face: face_seed.value.face,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 9);
    assert_eq!(state.graph().iter_relations().count(), 11);
}

#[test]
fn split_edge_mutation_splits_seed_self_loop() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let result = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 12);
    assert_eq!(state.graph().iter_relations().count(), 16);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.he_am)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::HalfEdgeNext)
            .count(),
        1
    );
}

#[test]
fn kill_vertex_edge_mutation_restores_seed_after_split_edge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    draft
        .execute(KillVertexEdgeMutation {
            vertex: split.value.new_vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 9);
    assert_eq!(state.graph().iter_relations().count(), 11);
}

#[test]
fn make_edge_face_mutation_splits_seed_face_after_split_edge() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    let result = draft
        .execute(MakeEdgeFaceMutation {
            face: seed.value.face,
            vertex_a: seed.value.vertex,
            vertex_b: split.value.new_vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 17);
    assert_eq!(state.graph().iter_relations().count(), 29);

    let outgoing_ab = state.graph().outgoing_relations(result.value.half_edge_ab);
    let outgoing_ba = state.graph().outgoing_relations(result.value.half_edge_ba);

    assert_eq!(
        outgoing_ab
            .iter()
            .filter(|relation| relation.kind == RelationKind::HalfEdgeNext)
            .count(),
        1
    );
    assert_eq!(
        outgoing_ba
            .iter()
            .filter(|relation| relation.kind == RelationKind::HalfEdgeNext)
            .count(),
        1
    );
    assert_eq!(
        state
            .graph()
            .incoming_relations(result.value.new_face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::ShellOwnsFace)
            .count(),
        1
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
fn make_shell_face_mutation_creates_disjoint_shell_seed_in_region() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let result = draft
        .execute(MakeShellFaceMutation {
            region: seed.value.region,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 15);
    assert_eq!(state.graph().iter_relations().count(), 20);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(seed.value.region)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::RegionOwnsShell)
            .count(),
        2
    );
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.shell)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::ShellOwnsFace)
            .count(),
        1
    );
}

#[test]
fn kill_shell_face_mutation_removes_only_disjoint_shell_seed() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let shell_seed = draft
        .execute(MakeShellFaceMutation {
            region: seed.value.region,
        })
        .unwrap();
    draft
        .execute(KillShellFaceMutation {
            face: shell_seed.value.face,
            vertex: shell_seed.value.vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 9);
    assert_eq!(state.graph().iter_relations().count(), 11);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(seed.value.region)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::RegionOwnsShell)
            .count(),
        1
    );
}

#[test]
fn kill_vertex_face_mutation_removes_exact_seed_chain() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(KillVertexFaceMutation {
            face: seed.value.face,
            vertex: seed.value.vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 0);
    assert_eq!(state.graph().iter_relations().count(), 0);
}
