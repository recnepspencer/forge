use crate::facade::*;

#[test]
fn make_isolated_vertex_mutation_creates_topology_vertex_without_relations() {
    let mut draft = SpecState::empty().into_draft();
    let result = draft.execute(MakeIsolatedVertexMutation).unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 1);
    assert_eq!(state.graph().iter_relations().count(), 0);
    assert_eq!(
        state.graph().node(result.value.vertex).unwrap().kind,
        SpecNodeKind::Vertex
    );
}

#[test]
fn make_face_in_shell_from_vertices_mutation_builds_face_cycle_from_existing_vertices() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
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

    let result = draft
        .execute(MakeFaceInShellFromVerticesMutation {
            shell: shell.value.shell,
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 15);
    assert_eq!(state.graph().iter_relations().count(), 21);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(shell.value.shell)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::ShellOwnsFace)
            .count(),
        1
    );
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.face)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::FaceOuterLoop)
            .count(),
        1
    );
}

#[test]
fn make_face_from_vertices_mutation_creates_container_and_face_from_existing_vertices() {
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

    let result = draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 15);
    assert_eq!(state.graph().iter_relations().count(), 21);
    assert_eq!(
        state
            .graph()
            .outgoing_relations(result.value.body)
            .into_iter()
            .filter(|relation| relation.kind == RelationKind::BodyOwnsLump)
            .count(),
        1
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
