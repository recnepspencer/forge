use forge_spec::facade::{
    MakeEdgeFaceMutation, MakeVertexFaceMutation, RelationKind, SpecDraft, SpecNodeKind,
    SpecShellKind, SpecShellOrientation, SpecState, SplitEdgeMutation,
};

use crate::projection::facade::{
    validate_projected_broken_boundary, validate_projected_face_adjacency,
    validate_projected_laminar_edges, validate_projected_manifold_edges,
    validate_projected_orientation_consistency, validate_projected_shell_closure,
    validate_projected_shell_consistency, ProjectedEdgeId, ProjectedHalfEdgeId, ProjectedShellData,
    ProjectedShellId, ProjectedTopologyQueries, ProjectionBuilder,
};

#[test]
fn projected_shell_closure_accepts_valid_sewn_state() {
    let projection = build_mef_projection();
    assert!(validate_projected_shell_closure(&projection).is_ok());
}

#[test]
fn projected_manifold_edges_rejects_valence_three_edge() {
    let projection = build_high_valence_projection(3);
    let has_valence_three = (0..projection.edge_count())
        .any(|edge_index| projection.radial_valence(ProjectedEdgeId::new(edge_index as u32)) == 3);
    assert!(has_valence_three);

    let error = validate_projected_manifold_edges(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_manifold_edges"));
}

#[test]
fn projected_broken_boundary_rejects_face_mismatch() {
    let mut projection = build_mef_projection();
    let he = ProjectedHalfEdgeId::new(0);
    projection.half_edges[he.index()].face = crate::projection::data::ProjectedFaceId::new(1);

    let error = validate_projected_broken_boundary(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_broken_boundary"));
}

#[test]
fn projected_face_adjacency_rejects_shell_mismatch() {
    let mut projection = build_mef_projection();
    let new_shell = ProjectedShellId::new(projection.shell_count() as u32);
    projection.shells.push(ProjectedShellData {
        spec_id: projection.shells()[0].spec_id,
        region: projection.shells()[0].region,
        kind: projection.shells()[0].kind,
        faces: vec![crate::projection::data::ProjectedFaceId::new(1)],
    });
    projection.faces[1].shell = new_shell;

    let error = validate_projected_face_adjacency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_face_adjacency_consistency"));
}

#[test]
fn projected_shell_consistency_rejects_boundary_edge_in_solid_shell() {
    let mut projection = build_seed_projection();
    projection.shells[0].kind = SpecShellKind::Solid(SpecShellOrientation::Outer);

    let error = validate_projected_shell_consistency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_shell_consistency"));
}

#[test]
fn projected_laminar_edges_reject_valence_three_edge_in_sheet_shell() {
    let projection = build_high_valence_projection(3);

    let error = validate_projected_laminar_edges(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_laminar_edges"));
}

#[test]
fn projected_orientation_consistency_rejects_parallel_solid_faces() {
    let mut projection = build_mef_projection();
    projection.shells[0].kind = SpecShellKind::Solid(SpecShellOrientation::Outer);

    let shared_edge = (0..projection.edge_count())
        .map(|index| ProjectedEdgeId::new(index as u32))
        .find(|&edge| projection.radial_valence(edge) == 2)
        .unwrap();
    let radial = projection.radial_half_edges(projection.edge(shared_edge).half_edge);
    assert_eq!(radial.len(), 2);

    let first = radial[0];
    let second = radial[1];
    let first_origin = projection.half_edge(first).origin;
    let first_destination = projection
        .half_edge(projection.half_edge(first).next)
        .origin;
    let second_next = projection.half_edge(second).next;

    projection.half_edges[second.index()].origin = first_origin;
    projection.half_edges[second_next.index()].origin = first_destination;

    let error = validate_projected_orientation_consistency(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_orientation_consistency"));
}

#[test]
fn projected_shell_closure_rejects_boundary_edge_in_solid_shell() {
    let mut projection = build_seed_projection();
    projection.shells[0].kind = SpecShellKind::Solid(SpecShellOrientation::Outer);

    let error = validate_projected_shell_closure(&projection).unwrap_err();
    assert!(format!("{error}").contains("projected_shell_consistency"));
}

fn build_mef_projection() -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(MakeEdgeFaceMutation {
            face: seed.face,
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
        })
        .unwrap();
    ProjectionBuilder::build(&draft.commit().unwrap()).unwrap()
}

fn build_seed_projection() -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    ProjectionBuilder::build(&draft.commit().unwrap()).unwrap()
}

fn build_high_valence_projection(valence: usize) -> crate::projection::data::ProjectedTopology {
    let mut draft = SpecState::empty().into_draft();
    let _fixture = build_antiparallel_valence_fixture(&mut draft, valence);
    ProjectionBuilder::build(&draft.commit().unwrap()).unwrap()
}

fn build_antiparallel_valence_fixture(draft: &mut SpecDraft, valence: usize) {
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

    let vertex_a = draft
        .create_node(SpecNodeKind::Vertex, None, "vertex-a")
        .unwrap();
    let vertex_b = draft
        .create_node(SpecNodeKind::Vertex, None, "vertex-b")
        .unwrap();
    let shared_edge = draft
        .create_node(SpecNodeKind::Edge, None, "shared-edge")
        .unwrap();
    let mut shared_half_edges = Vec::with_capacity(valence);

    for index in 0..valence {
        let face = draft
            .create_node(SpecNodeKind::Face, None, &format!("face-{index}"))
            .unwrap();
        let loop_id = draft
            .create_node(SpecNodeKind::Loop, None, &format!("loop-{index}"))
            .unwrap();
        let return_edge = draft
            .create_node(SpecNodeKind::Edge, None, &format!("return-edge-{index}"))
            .unwrap();
        let half_edge_shared = draft
            .create_node(
                SpecNodeKind::HalfEdge,
                None,
                &format!("shared-half-edge-{index}"),
            )
            .unwrap();
        let half_edge_return = draft
            .create_node(
                SpecNodeKind::HalfEdge,
                None,
                &format!("return-half-edge-{index}"),
            )
            .unwrap();

        let (origin, endpoint) = if index % 2 == 0 {
            (vertex_a, vertex_b)
        } else {
            (vertex_b, vertex_a)
        };

        draft
            .add_relation(
                RelationKind::ShellOwnsFace,
                shell,
                face,
                0,
                &format!("shell-face-{index}"),
            )
            .unwrap();
        draft
            .add_relation(
                RelationKind::FaceOuterLoop,
                face,
                loop_id,
                0,
                &format!("face-loop-{index}"),
            )
            .unwrap();
        draft
            .add_relation(
                RelationKind::LoopEntryHalfEdge,
                loop_id,
                half_edge_shared,
                0,
                &format!("loop-entry-{index}"),
            )
            .unwrap();

        for (kind, source, target, role) in [
            (
                RelationKind::HalfEdgeNext,
                half_edge_shared,
                half_edge_return,
                format!("shared-next-{index}"),
            ),
            (
                RelationKind::HalfEdgeNext,
                half_edge_return,
                half_edge_shared,
                format!("return-next-{index}"),
            ),
            (
                RelationKind::HalfEdgeRadialNext,
                half_edge_return,
                half_edge_return,
                format!("return-radial-{index}"),
            ),
            (
                RelationKind::HalfEdgeUsesEdge,
                half_edge_shared,
                shared_edge,
                format!("shared-edge-rel-{index}"),
            ),
            (
                RelationKind::HalfEdgeUsesEdge,
                half_edge_return,
                return_edge,
                format!("return-edge-rel-{index}"),
            ),
            (
                RelationKind::HalfEdgeOriginVertex,
                half_edge_shared,
                origin,
                format!("shared-origin-{index}"),
            ),
            (
                RelationKind::HalfEdgeOriginVertex,
                half_edge_return,
                endpoint,
                format!("return-origin-{index}"),
            ),
            (
                RelationKind::HalfEdgeBoundsFace,
                half_edge_shared,
                face,
                format!("shared-face-{index}"),
            ),
            (
                RelationKind::HalfEdgeBoundsFace,
                half_edge_return,
                face,
                format!("return-face-{index}"),
            ),
        ] {
            draft.add_relation(kind, source, target, 0, &role).unwrap();
        }

        shared_half_edges.push(half_edge_shared);
    }

    for index in 0..shared_half_edges.len() {
        let next = shared_half_edges[(index + 1) % shared_half_edges.len()];
        draft
            .add_relation(
                RelationKind::HalfEdgeRadialNext,
                shared_half_edges[index],
                next,
                0,
                &format!("shared-radial-{index}"),
            )
            .unwrap();
    }
}
