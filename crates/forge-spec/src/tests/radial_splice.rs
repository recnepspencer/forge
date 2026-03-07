use crate::facade::*;

#[test]
fn sew_edge_mutation_glues_antiparallel_boundaries() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;

    let result = draft
        .execute(SewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Edge)
            .count(),
        1
    );
}

#[test]
fn unsew_edge_mutation_restores_boundary_pair() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap().value;
    let split = draft
        .execute(SplitEdgeMutation {
            half_edge: seed.half_edge,
        })
        .unwrap()
        .value;
    draft
        .execute(SewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();

    let result = draft
        .execute(UnsewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Edge)
            .count(),
        2
    );
}

#[test]
fn sew_edge_mutation_splices_boundary_into_existing_radial_ring() {
    let mut draft = SpecState::empty().into_draft();
    let fixture = build_high_valence_radial_fixture(&mut draft);

    draft
        .execute(SewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.second_ba,
        })
        .unwrap();
    let result = draft
        .execute(SewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.third_ba,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(
        state
            .graph()
            .iter_nodes()
            .filter(|node| node.kind == SpecNodeKind::Edge)
            .count(),
        4
    );
    let ring = collect_radial_ring(state.graph(), fixture.seed_ab);
    assert_eq!(ring.len(), 3);
    assert!(ring.contains(&fixture.seed_ab));
    assert!(ring.contains(&fixture.second_ba));
    assert!(ring.contains(&fixture.third_ba));
}

#[test]
fn unsew_edge_mutation_detaches_one_use_from_high_valence_ring() {
    let mut draft = SpecState::empty().into_draft();
    let fixture = build_high_valence_radial_fixture(&mut draft);

    draft
        .execute(SewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.second_ba,
        })
        .unwrap();
    draft
        .execute(SewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.third_ba,
        })
        .unwrap();

    let result = draft
        .execute(UnsewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.third_ba,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    let remaining_ring = collect_radial_ring(state.graph(), fixture.seed_ab);
    assert_eq!(remaining_ring.len(), 2);
    assert!(!remaining_ring.contains(&fixture.third_ba));
    assert_eq!(
        state
            .graph()
            .outgoing_of_kind(fixture.third_ba, RelationKind::HalfEdgeRadialNext)[0]
            .target,
        fixture.third_ba
    );
}

#[derive(Clone, Copy)]
struct HighValenceFixture {
    seed_ab: SpecNodeId,
    second_ba: SpecNodeId,
    third_ba: SpecNodeId,
}

fn build_high_valence_radial_fixture(draft: &mut SpecDraft) -> HighValenceFixture {
    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    let lump = draft.create_node(SpecNodeKind::Lump, None, "lump").unwrap();
    let region = draft.create_node(SpecNodeKind::Region, None, "region").unwrap();
    let shell = draft.create_node(SpecNodeKind::Shell, None, "shell").unwrap();

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

    let seed = create_boundary_pair_face(draft, shell, vertex_a, vertex_b, "seed");
    let second = create_boundary_pair_face(draft, shell, vertex_a, vertex_b, "second");
    let third = create_boundary_pair_face(draft, shell, vertex_a, vertex_b, "third");

    HighValenceFixture {
        seed_ab: seed.0,
        second_ba: second.1,
        third_ba: third.1,
    }
}

fn create_boundary_pair_face(
    draft: &mut SpecDraft,
    shell: SpecNodeId,
    vertex_a: SpecNodeId,
    vertex_b: SpecNodeId,
    role: &str,
) -> (SpecNodeId, SpecNodeId) {
    let face = draft.create_node(SpecNodeKind::Face, None, &format!("{role}-face")).unwrap();
    let loop_id = draft.create_node(SpecNodeKind::Loop, None, &format!("{role}-loop")).unwrap();
    let edge_ab = draft.create_node(SpecNodeKind::Edge, None, &format!("{role}-edge-ab")).unwrap();
    let edge_ba = draft.create_node(SpecNodeKind::Edge, None, &format!("{role}-edge-ba")).unwrap();
    let he_ab = draft
        .create_node(SpecNodeKind::HalfEdge, None, &format!("{role}-half-edge-ab"))
        .unwrap();
    let he_ba = draft
        .create_node(SpecNodeKind::HalfEdge, None, &format!("{role}-half-edge-ba"))
        .unwrap();

    draft
        .add_relation(RelationKind::ShellOwnsFace, shell, face, 0, &format!("{role}-shell-face"))
        .unwrap();
    draft
        .add_relation(RelationKind::FaceOuterLoop, face, loop_id, 0, &format!("{role}-face-loop"))
        .unwrap();
    draft
        .add_relation(
            RelationKind::LoopEntryHalfEdge,
            loop_id,
            he_ab,
            0,
            &format!("{role}-loop-entry"),
        )
        .unwrap();

    for (source, target, relation_role) in [
        (he_ab, he_ba, format!("{role}-ab-next")),
        (he_ba, he_ab, format!("{role}-ba-next")),
        (he_ab, he_ab, format!("{role}-ab-radial")),
        (he_ba, he_ba, format!("{role}-ba-radial")),
        (he_ab, edge_ab, format!("{role}-ab-edge")),
        (he_ba, edge_ba, format!("{role}-ba-edge")),
        (he_ab, vertex_a, format!("{role}-ab-origin")),
        (he_ba, vertex_b, format!("{role}-ba-origin")),
        (he_ab, face, format!("{role}-ab-face")),
        (he_ba, face, format!("{role}-ba-face")),
    ] {
        let kind = match relation_role.split('-').last().unwrap() {
            "next" => RelationKind::HalfEdgeNext,
            "radial" => RelationKind::HalfEdgeRadialNext,
            "edge" => RelationKind::HalfEdgeUsesEdge,
            "origin" => RelationKind::HalfEdgeOriginVertex,
            "face" => RelationKind::HalfEdgeBoundsFace,
            _ => unreachable!(),
        };
        draft
            .add_relation(kind, source, target, 0, &relation_role)
            .unwrap();
    }

    (he_ab, he_ba)
}

fn collect_radial_ring(graph: &SpecGraph, start: SpecNodeId) -> Vec<SpecNodeId> {
    let mut ring = Vec::new();
    let mut current = start;
    let max_steps = graph.iter_nodes().count().max(1);

    for _ in 0..max_steps {
        ring.push(current);
        let next = graph
            .outgoing_of_kind(current, RelationKind::HalfEdgeRadialNext)
            .first()
            .unwrap()
            .target;
        if next == start {
            return ring;
        }
        current = next;
    }

    panic!("radial ring did not close in test fixture");
}
