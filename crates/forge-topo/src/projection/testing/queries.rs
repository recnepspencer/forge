use forge_spec::facade::{
    MakeEdgeFaceMutation, MakeEmptyShellMutation, MakeLumpRegionMutation, MakeSolidMutation,
    MakeVertexFaceMutation, SpecNodeId, SpecShellKind, SpecState, SplitEdgeMutation,
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

#[test]
fn hierarchy_queries_follow_container_relationships() {
    let projected = project_container_hierarchy();

    let body = crate::projection::facade::ProjectedBodyId::new(0);
    let lumps = projected.body_lumps(body);
    assert_eq!(lumps.len(), 2);

    let extra_lump = lumps[1];
    assert_eq!(projected.lump_body(extra_lump), body);

    let regions = projected.lump_regions(extra_lump);
    assert_eq!(regions.len(), 1);

    let extra_region = regions[0];
    assert_eq!(projected.region_lump(extra_region), extra_lump);

    let shells = projected.region_shells(extra_region);
    assert_eq!(shells.len(), 1);
    assert_eq!(projected.shell_region(shells[0]), extra_region);
}

#[test]
fn hierarchy_queries_follow_face_and_loop_ownership() {
    let projected = project_seed_plus_split_edge_plus_mef();

    let face = crate::projection::facade::ProjectedFaceId::new(0);
    let shell = projected.face_shell(face);
    let outer_loop = projected.face_outer_loop(face);
    let inner_loops = projected.face_inner_loops(face);

    assert_eq!(shell.raw(), 0);
    assert_eq!(projected.loop_face(outer_loop), face);
    assert!(inner_loops.is_empty());
}

#[test]
fn identity_queries_surface_projected_spec_ids_and_shell_kind() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let shell = crate::projection::facade::ProjectedShellId::new(0);
    let face = crate::projection::facade::ProjectedFaceId::new(0);
    let loop_id = projected.face_outer_loop(face);
    let half_edge = projected.loop_half_edges(loop_id).expect("loop should close")[0];
    let edge = projected.half_edge_edge(half_edge);
    let vertex = projected.half_edge_origin(half_edge);

    assert_eq!(projected.shell_kind(shell), SpecShellKind::Sheet);
    assert!(matches!(
        projected.resolve(projected.shell_spec_id(shell)),
        Some(crate::projection::facade::ProjectedEntityRef::Shell(_))
    ));
    assert!(matches!(
        projected.resolve(projected.face_spec_id(face)),
        Some(crate::projection::facade::ProjectedEntityRef::Face(_))
    ));
    assert!(matches!(
        projected.resolve(projected.loop_spec_id(loop_id)),
        Some(crate::projection::facade::ProjectedEntityRef::Loop(_))
    ));
    assert!(matches!(
        projected.resolve(projected.half_edge_spec_id(half_edge)),
        Some(crate::projection::facade::ProjectedEntityRef::HalfEdge(_))
    ));
    assert!(matches!(
        projected.resolve(projected.edge_spec_id(edge)),
        Some(crate::projection::facade::ProjectedEntityRef::Edge(_))
    ));
    assert!(matches!(
        projected.resolve(projected.vertex_spec_id(vertex)),
        Some(crate::projection::facade::ProjectedEntityRef::Vertex(_))
    ));
}

#[test]
fn adjacency_queries_surface_half_edge_navigation_and_representatives() {
    let projected = project_seed_plus_split_edge_plus_mef();
    let face = crate::projection::facade::ProjectedFaceId::new(0);
    let loop_id = projected.face_outer_loop(face);
    let half_edges = projected.loop_half_edges(loop_id).expect("loop should close");
    let half_edge = half_edges[0];
    let next = projected.half_edge_next(half_edge);
    let prev = projected.half_edge_prev(half_edge);
    let radial_next = projected.half_edge_radial_next(half_edge);
    let edge = projected.half_edge_edge(half_edge);

    assert_eq!(projected.half_edge_face(half_edge), face);
    assert_eq!(next, half_edges[1]);
    assert_eq!(prev, half_edges[1]);
    let origin = projected.half_edge_origin(half_edge);
    let representative = projected.edge_representative_half_edge(edge);
    let primary = projected.vertex_primary_half_edge(origin);

    assert_eq!(projected.half_edge_edge(representative), edge);
    assert!(projected.radial_half_edges(representative).contains(&radial_next));
    assert!(matches!(primary, Some(primary_he) if projected.half_edge_origin(primary_he) == origin));
}

#[test]
fn binding_queries_surface_geometry_and_trim_bindings() {
    let fixture = project_seed_with_bindings();
    let projected = fixture.projected;
    let face = crate::projection::facade::ProjectedFaceId::new(0);
    let loop_id = projected.face_outer_loop(face);
    let half_edge = projected.loop_half_edges(loop_id).expect("loop should close")[0];
    let edge = projected.half_edge_edge(half_edge);
    let vertex = projected.half_edge_origin(half_edge);

    assert_eq!(projected.face_surface_binding(face), Some(fixture.surface));
    assert_eq!(projected.half_edge_coedge_binding(half_edge), Some(fixture.coedge));
    assert_eq!(projected.edge_curve_binding(edge), Some(fixture.curve));
    assert_eq!(projected.vertex_geometry_binding(vertex), Some(fixture.geometry));
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

fn project_container_hierarchy() -> crate::projection::facade::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft
        .execute(MakeEmptyShellMutation {
            region: extra.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    let state = draft.commit().unwrap();
    ProjectionBuilder::build(&state).expect("projection should succeed")
}

struct BindingFixture {
    projected: crate::projection::facade::ProjectedTopology,
    surface: SpecNodeId,
    coedge: SpecNodeId,
    curve: SpecNodeId,
    geometry: SpecNodeId,
}

fn project_seed_with_bindings() -> BindingFixture {
    let mut projected = project_seed_plus_split_edge_plus_mef();
    let face = crate::projection::facade::ProjectedFaceId::new(0);
    let loop_id = projected.face_outer_loop(face);
    let half_edge = projected.loop_half_edges(loop_id).expect("loop should close")[0];
    let edge = projected.half_edge_edge(half_edge);
    let vertex = projected.half_edge_origin(half_edge);
    let surface = SpecNodeId::new(9_101);
    let coedge = SpecNodeId::new(9_102);
    let curve = SpecNodeId::new(9_103);
    let geometry = SpecNodeId::new(9_104);

    projected.faces[face.index()].surface_binding = Some(surface);
    projected.half_edges[half_edge.index()].coedge_binding = Some(coedge);
    projected.edges[edge.index()].curve_binding = Some(curve);
    projected.vertices[vertex.index()].geometry_binding = Some(geometry);

    BindingFixture {
        projected,
        surface,
        coedge,
        curve,
        geometry,
    }
}
