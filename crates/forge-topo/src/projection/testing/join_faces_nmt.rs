use forge_spec::facade::{
    JoinFacesNmtMutation, RelationKind, SpecDraft, SpecNodeId, SpecNodeKind, SpecState,
};

use crate::projection::facade::{ProjectedTopologyQueries, ProjectionBuilder, ProjectedEntityRef};

#[test]
fn projected_join_faces_nmt_builds_slit_and_preserves_protected_ring() {
    let (spec, fixture, output) = build_spec_join_faces_nmt_state();
    let projected =
        ProjectionBuilder::build(&spec).expect("spec-state JoinFacesNmt projection should succeed");

    let surviving_face = match projected.resolve(output.surviving_face) {
        Some(ProjectedEntityRef::Face(face)) => face,
        other => panic!("expected surviving face resolution, got {:?}", other),
    };
    let slit_edge = match projected.resolve(output.slit_edge) {
        Some(ProjectedEntityRef::Edge(edge)) => edge,
        other => panic!("expected slit edge resolution, got {:?}", other),
    };
    let shared_edge = match projected.resolve(fixture.shared_edge) {
        Some(ProjectedEntityRef::Edge(edge)) => edge,
        other => panic!("expected shared edge resolution, got {:?}", other),
    };

    assert_eq!(projected.radial_valence(shared_edge), fixture.valence - 2);
    assert_eq!(projected.radial_valence(slit_edge), 2);
    assert_eq!(projected.face_loops(surviving_face).len(), 2);
    assert!(projected.resolve(fixture.removed_face).is_none());
}

struct JoinFacesNmtFixture {
    valence: usize,
    shared_edge: SpecNodeId,
    removed_face: SpecNodeId,
}

fn build_spec_join_faces_nmt_state() -> (
    SpecState,
    JoinFacesNmtFixture,
    forge_spec::facade::JoinFacesNmtOutput,
) {
    let mut draft = SpecState::empty().into_draft();
    let fixture = build_antiparallel_valence_fixture(&mut draft, 4);
    let output = draft
        .execute(JoinFacesNmtMutation {
            half_edge_survive: fixture.shared_half_edges[0],
            half_edge_kill: fixture.shared_half_edges[1],
        })
        .unwrap()
        .value;
    (draft.commit().unwrap(), fixture.fixture, output)
}

struct AntiparallelFixtureState {
    shared_half_edges: Vec<SpecNodeId>,
    fixture: JoinFacesNmtFixture,
}

fn build_antiparallel_valence_fixture(
    draft: &mut SpecDraft,
    valence: usize,
) -> AntiparallelFixtureState {
    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    let lump = draft.create_node(SpecNodeKind::Lump, None, "lump").unwrap();
    let region = draft.create_node(SpecNodeKind::Region, None, "region").unwrap();
    let shell = draft.create_shell(forge_spec::facade::SpecShellKind::Sheet, "shell").unwrap();

    draft
        .add_relation(RelationKind::BodyOwnsLump, body, lump, 0, "body-lump")
        .unwrap();
    draft
        .add_relation(RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region")
        .unwrap();
    draft
        .add_relation(RelationKind::RegionOwnsShell, region, shell, 0, "region-shell")
        .unwrap();

    let vertex_a = draft.create_node(SpecNodeKind::Vertex, None, "vertex-a").unwrap();
    let vertex_b = draft.create_node(SpecNodeKind::Vertex, None, "vertex-b").unwrap();
    let shared_edge = draft.create_node(SpecNodeKind::Edge, None, "shared-edge").unwrap();

    let mut shared_half_edges = Vec::with_capacity(valence);
    let mut removed_face = None;

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
            .add_relation(RelationKind::ShellOwnsFace, shell, face, 0, &format!("shell-face-{index}"))
            .unwrap();
        draft
            .add_relation(RelationKind::FaceOuterLoop, face, loop_id, 0, &format!("face-loop-{index}"))
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

        if index == 1 {
            removed_face = Some(face);
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

    AntiparallelFixtureState {
        shared_half_edges,
        fixture: JoinFacesNmtFixture {
            valence,
            shared_edge,
            removed_face: removed_face.unwrap(),
        },
    }
}
