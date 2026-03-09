use forge_spec::facade::{RelationKind, SpecNodeKind, SpecState};

use crate::projection::facade::{
    validate_projected_no_cross_disk_coedges, validate_projected_vertex_disk,
    validate_projected_vertex_disk_partition, validate_projected_vertex_outgoing,
    ProjectedEntityRef, ProjectedHalfEdgeId, ProjectedTopology, ProjectedTopologyQueries,
    ProjectedVertexId, ProjectionBuilder,
};

#[test]
fn projected_vertex_disk_accepts_split_vertex_components() {
    let fixture = build_split_vertex_projection();

    assert!(validate_projected_vertex_disk(&fixture.projection).is_ok());
    assert_eq!(
        fixture
            .projection
            .vertex_disk_components(fixture.shared_vertex)
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn projected_vertex_outgoing_rejects_wrong_primary_halfedge_origin() {
    let mut fixture = build_split_vertex_projection();
    fixture.projection.vertices[fixture.shared_vertex.index()].primary_half_edge =
        Some(fixture.foreign_half_edge);

    let error = validate_projected_vertex_outgoing(&fixture.projection).unwrap_err();
    assert!(format!("{error}").contains("projected_vertex_outgoing"));
}

#[test]
fn projected_vertex_disk_partition_accepts_split_vertex_components() {
    let fixture = build_split_vertex_projection();

    assert!(validate_projected_vertex_disk_partition(&fixture.projection).is_ok());
}

#[test]
fn projected_vertex_disk_cross_disk_check_accepts_split_vertex_components() {
    let fixture = build_split_vertex_projection();

    assert!(validate_projected_no_cross_disk_coedges(&fixture.projection).is_ok());
}

#[test]
fn projected_vertex_outgoing_rejects_missing_primary_halfedge() {
    let mut fixture = build_split_vertex_projection();
    fixture.projection.vertices[fixture.shared_vertex.index()].primary_half_edge = None;

    let error = validate_projected_vertex_outgoing(&fixture.projection).unwrap_err();
    assert!(format!("{error}").contains("projected_vertex_outgoing"));
}

struct SplitVertexFixture {
    projection: ProjectedTopology,
    shared_vertex: ProjectedVertexId,
    foreign_half_edge: ProjectedHalfEdgeId,
}

fn build_split_vertex_projection() -> SplitVertexFixture {
    let mut draft = SpecState::empty().into_draft();

    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    let lump = draft.create_node(SpecNodeKind::Lump, None, "lump").unwrap();
    let region = draft
        .create_node(SpecNodeKind::Region, None, "region")
        .unwrap();
    let shell = draft
        .create_shell(forge_spec::facade::SpecShellKind::Sheet, "shell")
        .unwrap();

    draft
        .add_relation(RelationKind::BodyOwnsLump, body, lump, 0, "body-lump")
        .unwrap();
    draft
        .add_relation(RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")
        .unwrap();
    draft
        .add_relation(
            RelationKind::RegionOwnsShell,
            region,
            shell,
            0,
            "region-shell",
        )
        .unwrap();

    let shared_vertex = draft
        .create_node(SpecNodeKind::Vertex, None, "shared-vertex")
        .unwrap();
    let endpoint_a = draft
        .create_node(SpecNodeKind::Vertex, None, "endpoint-a")
        .unwrap();
    let endpoint_b = draft
        .create_node(SpecNodeKind::Vertex, None, "endpoint-b")
        .unwrap();

    build_two_half_edge_face(&mut draft, shell, shared_vertex, endpoint_a, "a");
    build_two_half_edge_face(&mut draft, shell, shared_vertex, endpoint_b, "b");

    let projection = ProjectionBuilder::build(&draft.commit().unwrap()).unwrap();
    let shared_vertex = match projection.resolve(shared_vertex) {
        Some(ProjectedEntityRef::Vertex(vertex)) => vertex,
        other => panic!("expected shared vertex projection, got {:?}", other),
    };
    let endpoint_a = match projection.resolve(endpoint_a) {
        Some(ProjectedEntityRef::Vertex(vertex)) => vertex,
        other => panic!("expected endpoint-a projection, got {:?}", other),
    };
    let foreign_half_edge = projection.vertex_outgoing_half_edges(endpoint_a)[0];

    SplitVertexFixture {
        projection,
        shared_vertex,
        foreign_half_edge,
    }
}

fn build_two_half_edge_face(
    draft: &mut forge_spec::facade::SpecDraft,
    shell: forge_spec::facade::SpecNodeId,
    shared_vertex: forge_spec::facade::SpecNodeId,
    endpoint: forge_spec::facade::SpecNodeId,
    label: &str,
) {
    let face = draft
        .create_node(SpecNodeKind::Face, None, &format!("face-{label}"))
        .unwrap();
    let loop_id = draft
        .create_node(SpecNodeKind::Loop, None, &format!("loop-{label}"))
        .unwrap();
    let edge_out = draft
        .create_node(SpecNodeKind::Edge, None, &format!("edge-out-{label}"))
        .unwrap();
    let edge_back = draft
        .create_node(SpecNodeKind::Edge, None, &format!("edge-back-{label}"))
        .unwrap();
    let he_out = draft
        .create_node(SpecNodeKind::HalfEdge, None, &format!("he-out-{label}"))
        .unwrap();
    let he_back = draft
        .create_node(SpecNodeKind::HalfEdge, None, &format!("he-back-{label}"))
        .unwrap();

    draft
        .add_relation(
            RelationKind::ShellOwnsFace,
            shell,
            face,
            0,
            &format!("shell-face-{label}"),
        )
        .unwrap();
    draft
        .add_relation(
            RelationKind::FaceOuterLoop,
            face,
            loop_id,
            0,
            &format!("face-loop-{label}"),
        )
        .unwrap();
    draft
        .add_relation(
            RelationKind::LoopEntryHalfEdge,
            loop_id,
            he_out,
            0,
            &format!("entry-{label}"),
        )
        .unwrap();

    for (kind, source, target, role) in [
        (
            RelationKind::HalfEdgeNext,
            he_out,
            he_back,
            format!("out-next-{label}"),
        ),
        (
            RelationKind::HalfEdgeNext,
            he_back,
            he_out,
            format!("back-next-{label}"),
        ),
        (
            RelationKind::HalfEdgeRadialNext,
            he_out,
            he_out,
            format!("out-radial-{label}"),
        ),
        (
            RelationKind::HalfEdgeRadialNext,
            he_back,
            he_back,
            format!("back-radial-{label}"),
        ),
        (
            RelationKind::HalfEdgeUsesEdge,
            he_out,
            edge_out,
            format!("out-edge-{label}"),
        ),
        (
            RelationKind::HalfEdgeUsesEdge,
            he_back,
            edge_back,
            format!("back-edge-{label}"),
        ),
        (
            RelationKind::HalfEdgeOriginVertex,
            he_out,
            shared_vertex,
            format!("out-origin-{label}"),
        ),
        (
            RelationKind::HalfEdgeOriginVertex,
            he_back,
            endpoint,
            format!("back-origin-{label}"),
        ),
        (
            RelationKind::HalfEdgeBoundsFace,
            he_out,
            face,
            format!("out-face-{label}"),
        ),
        (
            RelationKind::HalfEdgeBoundsFace,
            he_back,
            face,
            format!("back-face-{label}"),
        ),
    ] {
        draft.add_relation(kind, source, target, 0, &role).unwrap();
    }
}
