use forge_spec::facade::{
    MakeEdgeFaceMutation, MakeVertexFaceMutation, SpecState, SplitEdgeMutation,
};

use crate::projection::facade::{ProjectedTopologyQueries, ProjectionBuilder};

#[test]
fn loop_half_edges_walks_deterministic_cycle() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let original_face = projected.faces()[0].spec_id;
    let original_face = projected
        .resolve(original_face)
        .expect("original face should resolve");
    let original_face = match original_face {
        crate::projection::facade::ProjectedEntityRef::Face(face) => face,
        other => panic!("expected face ref, got {other:?}"),
    };

    let loop_ids = projected.face_loops(original_face);
    assert_eq!(loop_ids.len(), 1);

    let half_edges = projected
        .loop_half_edges(loop_ids[0])
        .expect("loop should close");
    assert_eq!(half_edges.len(), 2);
}

#[test]
fn face_half_edges_collects_boundary_in_loop_order() {
    let projected = project_seed_plus_split_edge_plus_mef();

    let face_half_edges = projected
        .face_half_edges(crate::projection::facade::ProjectedFaceId::new(0))
        .expect("face should have a valid loop");
    assert_eq!(face_half_edges.len(), 2);
}

#[test]
fn edge_faces_reports_both_faces_for_split_edge_face_pair() {
    let projected = project_seed_plus_split_edge_plus_mef();

    let shared_edge = projected
        .edges()
        .iter()
        .enumerate()
        .map(|(index, _)| crate::projection::facade::ProjectedEdgeId::new(index as u32))
        .find(|edge| projected.edge_faces(*edge).len() == 2)
        .expect("one projected edge should separate the two faces");
    let faces = projected.edge_faces(shared_edge);
    assert_eq!(faces.len(), 2);
}

#[test]
fn vertex_outgoing_half_edges_is_deterministic() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let outgoing = projected.vertex_outgoing_half_edges(crate::projection::facade::ProjectedVertexId::new(0));
    assert_eq!(outgoing.len(), 2);
    assert!(outgoing[0].raw() < outgoing[1].raw());
}

#[test]
fn shell_faces_returns_faces_in_projection_order() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let faces = projected.shell_faces(crate::projection::facade::ProjectedShellId::new(0));
    assert_eq!(faces.len(), 2);
    assert_eq!(faces[0].raw(), 0);
    assert_eq!(faces[1].raw(), 1);
}

#[test]
fn face_edges_deduplicates_shared_boundary_edges() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let face_edges = projected
        .face_edges(crate::projection::facade::ProjectedFaceId::new(0))
        .expect("face edges should resolve");
    assert_eq!(face_edges.len(), 2);
}

#[test]
fn radial_half_edges_and_valence_match_shared_edge_ring() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let shared_edge = projected
        .edges()
        .iter()
        .enumerate()
        .map(|(index, _)| crate::projection::facade::ProjectedEdgeId::new(index as u32))
        .find(|edge| projected.edge_faces(*edge).len() == 2)
        .expect("one projected edge should separate the two faces");

    let representative = projected.edge(shared_edge).half_edge;
    let ring = projected.radial_half_edges(representative);
    assert_eq!(ring.len(), 2);
    assert_eq!(projected.radial_valence(shared_edge), 2);
    assert!(!projected.is_boundary_edge(shared_edge));
}

#[test]
fn vertex_faces_deduplicates_incident_faces() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let faces = projected.vertex_faces(crate::projection::facade::ProjectedVertexId::new(0));
    assert_eq!(faces.len(), 2);
}

fn project_seed_plus_split_edge_plus_mef() -> crate::projection::facade::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    draft
        .execute(MakeEdgeFaceMutation {
            face: seed.value.face,
            vertex_a: seed.value.vertex,
            vertex_b: split.value.new_vertex,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    ProjectionBuilder::build(&state).expect("projection should succeed")
}
