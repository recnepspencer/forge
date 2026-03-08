use forge_spec::facade::{
    MakeEmptyShellMutation, MakeFaceFromVerticesMutation, MakeFaceInShellFromVerticesMutation,
    MakeIsolatedVertexMutation, MakeSolidMutation, SpecShellKind, SpecState,
};

use crate::boundary_editing::make_face_from_vertices::MakeFaceFromVertices;
use crate::boundary_editing::make_face_in_shell_from_vertices::MakeFaceInShellFromVertices;
use crate::entity_lifecycle::make_isolated_vertex::MakeIsolatedVertex;
use crate::lifecycle::shell::MakeEmptyShell;
use crate::lifecycle::solid::MakeSolid;
use crate::projection::facade::{ProjectionBuilder, compute_projected_topology_hash};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};
use crate::b_rep::ShellKind;

#[test]
fn projected_make_face_from_vertices_matches_legacy_structural_signature() {
    let legacy = build_legacy_make_face_from_vertices_state();
    let projected = ProjectionBuilder::build(&build_spec_make_face_from_vertices_state())
        .expect("spec-state MFFV projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_make_face_in_shell_from_vertices_matches_legacy_structural_signature() {
    let legacy = build_legacy_make_face_in_shell_from_vertices_state();
    let projected = ProjectionBuilder::build(&build_spec_make_face_in_shell_from_vertices_state())
        .expect("spec-state MFIS projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

fn build_spec_make_face_from_vertices_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    draft
        .execute(MakeFaceFromVerticesMutation {
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_make_face_in_shell_from_vertices_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    let v0 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v1 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    let v2 = draft.execute(MakeIsolatedVertexMutation).unwrap().value.vertex;
    draft
        .execute(MakeFaceInShellFromVerticesMutation {
            shell: shell.value.shell,
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_face_from_vertices_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let v0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    draft
        .execute(MakeFaceFromVertices {
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_face_in_shell_from_vertices_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft
        .execute(MakeSolid)
        .unwrap()
        .into_value();
    let shell = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let v0 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v1 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    let v2 = draft.execute(MakeIsolatedVertex).unwrap().into_value().vertex;
    draft
        .execute(MakeFaceInShellFromVertices {
            shell: shell.shell,
            vertices: vec![v0, v1, v2],
        })
        .unwrap();
    draft.commit().unwrap()
}
