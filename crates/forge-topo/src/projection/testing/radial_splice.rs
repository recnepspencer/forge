use forge_spec::facade::{
    MakeVertexFaceMutation, RelationKind, SewEdgeMutation, SpecDraft, SpecNodeId, SpecNodeKind,
    SpecState, SplitEdgeMutation, UnsewEdgeMutation,
};

use crate::b_rep::ShellKind;
use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::operations::entity_lifecycle::split_edge::SplitEdge;
use crate::operations::non_manifold::sew_edge::SewEdge;
use crate::operations::non_manifold::unsew_edge::UnsewEdge;
use crate::projection::facade::{compute_projected_topology_hash, ProjectionBuilder};
use crate::transactions::facade::{compute_arena_topology_hash, TopologyState};

#[test]
fn projected_sew_edge_matches_legacy_structural_signature() {
    let legacy_hash = build_legacy_sew_hash();
    let projected = ProjectionBuilder::build(&build_spec_sew_state())
        .expect("spec-state SewEdge projection should succeed");

    assert_eq!(legacy_hash, compute_projected_topology_hash(&projected));
}

#[test]
fn projected_unsew_edge_matches_legacy_structural_signature() {
    let legacy = build_legacy_unsew_state();
    let projected = ProjectionBuilder::build(&build_spec_unsew_state())
        .expect("spec-state UnsewEdge projection should succeed");

    assert_eq!(
        compute_arena_topology_hash(legacy.arena()),
        compute_projected_topology_hash(&projected)
    );
}

#[test]
fn projected_high_valence_radial_ring_builds_from_spec_truth() {
    let projected = ProjectionBuilder::build(&build_spec_high_valence_sew_state())
        .expect("high-valence spec projection should succeed");

    let ring = collect_projected_radial_ring(
        &projected,
        crate::projection::data::ProjectedHalfEdgeId::new(0),
    );
    assert_eq!(ring.len(), 3);
}

#[test]
fn projected_high_valence_unsew_detaches_single_use() {
    let projected = ProjectionBuilder::build(&build_spec_high_valence_unsew_state())
        .expect("high-valence spec unsew projection should succeed");

    let remaining = collect_projected_radial_ring(
        &projected,
        crate::projection::data::ProjectedHalfEdgeId::new(0),
    );
    assert_eq!(remaining.len(), 2);
}

fn build_spec_sew_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_spec_unsew_state() -> SpecState {
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
    draft
        .execute(UnsewEdgeMutation {
            half_edge_a: seed.half_edge,
            half_edge_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_spec_high_valence_sew_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_spec_high_valence_unsew_state() -> SpecState {
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
    draft
        .execute(UnsewEdgeMutation {
            half_edge_a: fixture.seed_ab,
            half_edge_b: fixture.third_ba,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_sew_hash() -> u128 {
    let mut draft = TopologyState::empty().into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let split = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    draft
        .execute(SewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    compute_arena_topology_hash(draft.arena())
}

fn build_legacy_unsew_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let mvf = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let split = draft
        .execute(SplitEdge {
            edge: mvf.half_edge,
        })
        .unwrap()
        .into_value();
    draft
        .execute(SewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    draft
        .execute(UnsewEdge {
            he_a: mvf.half_edge,
            he_b: split.he_mb,
        })
        .unwrap();
    draft.commit().unwrap()
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
    let face = draft
        .create_node(SpecNodeKind::Face, None, &format!("{role}-face"))
        .unwrap();
    let loop_id = draft
        .create_node(SpecNodeKind::Loop, None, &format!("{role}-loop"))
        .unwrap();
    let edge_ab = draft
        .create_node(SpecNodeKind::Edge, None, &format!("{role}-edge-ab"))
        .unwrap();
    let edge_ba = draft
        .create_node(SpecNodeKind::Edge, None, &format!("{role}-edge-ba"))
        .unwrap();
    let he_ab = draft
        .create_node(
            SpecNodeKind::HalfEdge,
            None,
            &format!("{role}-half-edge-ab"),
        )
        .unwrap();
    let he_ba = draft
        .create_node(
            SpecNodeKind::HalfEdge,
            None,
            &format!("{role}-half-edge-ba"),
        )
        .unwrap();

    draft
        .add_relation(
            RelationKind::ShellOwnsFace,
            shell,
            face,
            0,
            &format!("{role}-shell-face"),
        )
        .unwrap();
    draft
        .add_relation(
            RelationKind::FaceOuterLoop,
            face,
            loop_id,
            0,
            &format!("{role}-face-loop"),
        )
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

    for (kind, source, target, relation_role) in [
        (
            RelationKind::HalfEdgeNext,
            he_ab,
            he_ba,
            format!("{role}-ab-next"),
        ),
        (
            RelationKind::HalfEdgeNext,
            he_ba,
            he_ab,
            format!("{role}-ba-next"),
        ),
        (
            RelationKind::HalfEdgeRadialNext,
            he_ab,
            he_ab,
            format!("{role}-ab-radial"),
        ),
        (
            RelationKind::HalfEdgeRadialNext,
            he_ba,
            he_ba,
            format!("{role}-ba-radial"),
        ),
        (
            RelationKind::HalfEdgeUsesEdge,
            he_ab,
            edge_ab,
            format!("{role}-ab-edge"),
        ),
        (
            RelationKind::HalfEdgeUsesEdge,
            he_ba,
            edge_ba,
            format!("{role}-ba-edge"),
        ),
        (
            RelationKind::HalfEdgeOriginVertex,
            he_ab,
            vertex_a,
            format!("{role}-ab-origin"),
        ),
        (
            RelationKind::HalfEdgeOriginVertex,
            he_ba,
            vertex_b,
            format!("{role}-ba-origin"),
        ),
        (
            RelationKind::HalfEdgeBoundsFace,
            he_ab,
            face,
            format!("{role}-ab-face"),
        ),
        (
            RelationKind::HalfEdgeBoundsFace,
            he_ba,
            face,
            format!("{role}-ba-face"),
        ),
    ] {
        draft
            .add_relation(kind, source, target, 0, &relation_role)
            .unwrap();
    }

    (he_ab, he_ba)
}

fn collect_projected_radial_ring(
    projected: &crate::projection::data::ProjectedTopology,
    start: crate::projection::data::ProjectedHalfEdgeId,
) -> Vec<crate::projection::data::ProjectedHalfEdgeId> {
    let mut ring = Vec::new();
    let mut current = start;
    let max_steps = projected.half_edge_count().max(1);

    for _ in 0..max_steps {
        ring.push(current);
        let next = projected.half_edge(current).radial_next;
        if next == start {
            return ring;
        }
        current = next;
    }

    panic!("projected radial ring did not close in test fixture");
}
