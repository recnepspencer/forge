use forge_spec::facade::{
    KillEdgeVertexMutation, KillFaceVertexMutation, KillShellFaceMutation,
    KillVertexFaceMutation, MakeEdgeFaceMutation, MakeEdgeVertexMutation,
    MakeFaceVertexMutation, MakeShellFaceMutation, MakeVertexFaceMutation, RelationKind,
    SpecNodeKind, SpecState, SplitEdgeMutation,
};

use crate::entity_lifecycle::make_edge_face::MakeEdgeFace;
use crate::entity_lifecycle::make_edge_vertex::MakeEdgeVertex;
use crate::entity_lifecycle::kill_shell_face::KillShellFace;
use crate::entity_lifecycle::kill_edge_vertex::KillEdgeVertex;
use crate::entity_lifecycle::kill_face_vertex::KillFaceVertex;
use crate::entity_lifecycle::kill_vertex_face::KillVertexFace;
use crate::entity_lifecycle::make_face_vertex::MakeFaceVertex;
use crate::entity_lifecycle::make_shell_face::MakeShellFace;
use crate::entity_lifecycle::split_edge::SplitEdge;
use crate::projection::facade::{
    ProjectedBodyId, ProjectedEntityRef, ProjectedFaceId, ProjectedHalfEdgeId, ProjectedLoopId,
    ProjectedLumpId, ProjectedRegionId, ProjectedShellId, ProjectionBuilder,
    compute_projected_topology_hash,
};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};
use crate::{b_rep::ShellKind, entity_lifecycle::make_vertex_face::MakeVertexFace};

#[test]
fn projects_minimal_topology_into_dense_handles() {
    let state = build_single_halfedge_state();
    let projected = ProjectionBuilder::build(&state).expect("projection should succeed");

    assert_eq!(projected.body_count(), 1);
    assert_eq!(projected.lump_count(), 1);
    assert_eq!(projected.region_count(), 1);
    assert_eq!(projected.shell_count(), 1);
    assert_eq!(projected.face_count(), 1);
    assert_eq!(projected.loop_count(), 1);
    assert_eq!(projected.half_edge_count(), 1);
    assert_eq!(projected.edge_count(), 1);
    assert_eq!(projected.vertex_count(), 1);

    let body = &projected.bodies()[0];
    let lump = &projected.lumps()[0];
    let region = &projected.regions()[0];
    let shell = &projected.shells()[0];
    let face = &projected.faces()[0];
    let loop_data = &projected.loops()[0];
    let half_edge = &projected.half_edges()[0];
    let edge = &projected.edges()[0];
    let vertex = &projected.vertices()[0];

    assert_eq!(body.lumps, vec![ProjectedLumpId::new(0)]);
    assert_eq!(lump.body, ProjectedBodyId::new(0));
    assert_eq!(lump.regions, vec![ProjectedRegionId::new(0)]);
    assert_eq!(region.lump, ProjectedLumpId::new(0));
    assert_eq!(region.shells, vec![ProjectedShellId::new(0)]);
    assert_eq!(shell.region, ProjectedRegionId::new(0));
    assert_eq!(shell.faces, vec![ProjectedFaceId::new(0)]);
    assert_eq!(face.shell, ProjectedShellId::new(0));
    assert_eq!(face.outer_loop, ProjectedLoopId::new(0));
    assert!(face.inner_loops.is_empty());
    assert_eq!(loop_data.face, ProjectedFaceId::new(0));
    assert_eq!(loop_data.half_edge, ProjectedHalfEdgeId::new(0));
    assert_eq!(edge.half_edge, ProjectedHalfEdgeId::new(0));
    assert_eq!(half_edge.next, half_edge.prev);
    assert_eq!(half_edge.next, ProjectedHalfEdgeId::new(0));
    assert_eq!(vertex.primary_half_edge, Some(ProjectedHalfEdgeId::new(0)));

    match projected.resolve(face.spec_id) {
        Some(ProjectedEntityRef::Face(id)) => assert_eq!(id, ProjectedFaceId::new(0)),
        other => panic!("unexpected projected face ref: {other:?}"),
    }
}

#[test]
fn projection_rejects_duplicate_halfedge_predecessors() {
    let state = build_duplicate_prev_state();
    let error = ProjectionBuilder::build(&state).expect_err("projection must reject ambiguous prev");
    assert!(error.to_string().contains("multiple projected predecessors"));
}

#[test]
fn projected_minimal_seed_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_state();
    let projected = ProjectionBuilder::build(&build_single_halfedge_state())
        .expect("spec-state seed projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_mev_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_mev_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_mev_state())
        .expect("spec-state MVF+MEV projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_split_edge_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_split_edge_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_split_edge_state())
        .expect("spec-state MVF+SplitEdge projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_split_edge_plus_mef_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_split_edge_mef_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_split_edge_plus_mef_state())
        .expect("spec-state MVF+SplitEdge+MEF projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_shell_face_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_shell_face_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_shell_face_state())
        .expect("spec-state MVF+MSF projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_shell_face_plus_kill_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_shell_face_plus_kill_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_shell_face_plus_kill_state())
        .expect("spec-state MVF+MSF+KSF projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_kill_vertex_face_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_kvf_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_kill_vertex_face_state())
        .expect("spec-state MVF+KVF projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_edge_vertex_plus_kill_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_mev_plus_kev_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_mev_plus_kev_state())
        .expect("spec-state MVF+MEV+KEV projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_face_vertex_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_mfv_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_mfv_state())
        .expect("spec-state MVF+MFV projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

#[test]
fn projected_seed_plus_face_vertex_plus_kill_matches_legacy_arena_structural_signature() {
    let legacy = build_legacy_mvf_plus_mfv_plus_kfv_state();
    let projected = ProjectionBuilder::build(&build_seed_plus_mfv_plus_kfv_state())
        .expect("spec-state MVF+MFV+KFV projection should succeed");

    let legacy_hash = compute_arena_topology_hash(legacy.arena());
    let projected_hash = compute_projected_topology_hash(&projected);

    assert_eq!(legacy_hash, projected_hash);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
    assert_eq!(projected.face_count(), legacy.arena().face_count() as usize);
    assert_eq!(projected.loop_count(), legacy.arena().loop_count() as usize);
    assert_eq!(projected.half_edge_count(), legacy.arena().half_edge_count() as usize);
    assert_eq!(projected.edge_count(), legacy.arena().edge_count() as usize);
    assert_eq!(projected.vertex_count(), legacy.arena().vertex_count() as usize);
}

fn build_single_halfedge_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeVertexFaceMutation).unwrap();
    draft.commit().unwrap()
}

fn build_duplicate_prev_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let body = draft.create_node(SpecNodeKind::Body, None, "body").unwrap();
    let lump = draft.create_node(SpecNodeKind::Lump, None, "lump").unwrap();
    let region = draft.create_node(SpecNodeKind::Region, None, "region").unwrap();
    let shell = draft.create_node(SpecNodeKind::Shell, None, "shell").unwrap();
    let face = draft.create_node(SpecNodeKind::Face, None, "face").unwrap();
    let loop_id = draft.create_node(SpecNodeKind::Loop, None, "loop").unwrap();
    let he_a = draft.create_node(SpecNodeKind::HalfEdge, None, "hea").unwrap();
    let he_b = draft.create_node(SpecNodeKind::HalfEdge, None, "heb").unwrap();
    let edge_a = draft.create_node(SpecNodeKind::Edge, None, "edgea").unwrap();
    let edge_b = draft.create_node(SpecNodeKind::Edge, None, "edgeb").unwrap();
    let vertex_a = draft.create_node(SpecNodeKind::Vertex, None, "vertexa").unwrap();
    let vertex_b = draft.create_node(SpecNodeKind::Vertex, None, "vertexb").unwrap();

    add(&mut draft, RelationKind::BodyOwnsLump, body, lump, 0, "body-lump");
    add(&mut draft, RelationKind::LumpOwnsRegion, lump, region, 0, "lump-region");
    add(&mut draft, RelationKind::RegionOwnsShell, region, shell, 0, "region-shell");
    add(&mut draft, RelationKind::ShellOwnsFace, shell, face, 0, "shell-face");
    add(&mut draft, RelationKind::FaceOuterLoop, face, loop_id, 0, "face-loop");
    add(&mut draft, RelationKind::LoopEntryHalfEdge, loop_id, he_a, 0, "loop-he");

    add(&mut draft, RelationKind::HalfEdgeNext, he_a, he_a, 0, "hea-next");
    add(&mut draft, RelationKind::HalfEdgeNext, he_b, he_a, 0, "heb-next");
    add(&mut draft, RelationKind::HalfEdgeRadialNext, he_a, he_a, 0, "hea-radial");
    add(&mut draft, RelationKind::HalfEdgeRadialNext, he_b, he_b, 0, "heb-radial");
    add(&mut draft, RelationKind::HalfEdgeUsesEdge, he_a, edge_a, 0, "hea-edge");
    add(&mut draft, RelationKind::HalfEdgeUsesEdge, he_b, edge_b, 0, "heb-edge");
    add(&mut draft, RelationKind::HalfEdgeOriginVertex, he_a, vertex_a, 0, "hea-vertex");
    add(&mut draft, RelationKind::HalfEdgeOriginVertex, he_b, vertex_b, 0, "heb-vertex");
    add(&mut draft, RelationKind::HalfEdgeBoundsFace, he_a, face, 0, "hea-face");
    add(&mut draft, RelationKind::HalfEdgeBoundsFace, he_b, face, 0, "heb-face");

    draft.commit().unwrap()
}

fn build_seed_plus_mev_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(MakeEdgeVertexMutation {
            anchor: seed.value.half_edge,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_seed_plus_split_edge_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(SplitEdgeMutation {
            half_edge: seed.value.half_edge,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_seed_plus_split_edge_plus_mef_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_seed_plus_shell_face_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(MakeShellFaceMutation {
            region: seed.value.region,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_seed_plus_shell_face_plus_kill_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_seed_plus_kill_vertex_face_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(KillVertexFaceMutation {
            face: seed.value.face,
            vertex: seed.value.vertex,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_seed_plus_mev_plus_kev_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_seed_plus_mfv_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(MakeFaceVertexMutation {
            shell: seed.value.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_seed_plus_mfv_plus_kfv_state() -> SpecState {
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
    draft.commit().unwrap()
}

fn build_legacy_mvf_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed");
    draft.commit().expect("legacy draft commit should succeed")
}

fn build_legacy_mvf_mev_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    draft
        .execute(MakeEdgeVertex {
            anchor: seed.half_edge,
        })
        .expect("legacy MEV should succeed");
    draft.commit().expect("legacy MVF+MEV commit should succeed")
}

fn build_legacy_mvf_plus_mev_plus_kev_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    let mev = draft
        .execute(MakeEdgeVertex {
            anchor: seed.half_edge,
        })
        .expect("legacy MEV should succeed")
        .into_value();
    draft
        .execute(KillEdgeVertex { edge: mev.he_out })
        .expect("legacy KEV should succeed");
    draft
        .commit()
        .expect("legacy MVF+MEV+KEV commit should succeed")
}

fn build_legacy_mvf_plus_mfv_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    draft
        .execute(MakeFaceVertex { shell: seed.shell })
        .expect("legacy MFV should succeed");
    draft.commit().expect("legacy MVF+MFV commit should succeed")
}

fn build_legacy_mvf_plus_mfv_plus_kfv_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    let face_seed = draft
        .execute(MakeFaceVertex { shell: seed.shell })
        .expect("legacy MFV should succeed")
        .into_value();
    draft
        .execute(KillFaceVertex {
            face: face_seed.face,
        })
        .expect("legacy KFV should succeed");
    draft
        .commit()
        .expect("legacy MVF+MFV+KFV commit should succeed")
}

fn build_legacy_mvf_split_edge_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    draft
        .execute(SplitEdge {
            edge: seed.half_edge,
        })
        .expect("legacy SplitEdge should succeed");
    draft
        .commit()
        .expect("legacy MVF+SplitEdge commit should succeed")
}

fn build_legacy_mvf_split_edge_mef_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    let split = draft
        .execute(SplitEdge {
            edge: seed.half_edge,
        })
        .expect("legacy SplitEdge should succeed")
        .into_value();
    draft
        .execute(MakeEdgeFace {
            vertex_a: seed.vertex,
            vertex_b: split.new_vertex,
            face: seed.face,
        })
        .expect("legacy MEF should succeed");
    draft
        .commit()
        .expect("legacy MVF+SplitEdge+MEF commit should succeed")
}

fn build_legacy_mvf_plus_shell_face_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    draft
        .execute(MakeShellFace {
            region: draft.arena().get_shell(seed.shell).expect("shell must exist").region(),
            kind: ShellKind::Sheet,
        })
        .expect("legacy MakeShellFace should succeed");
    draft
        .commit()
        .expect("legacy MVF+MSF commit should succeed")
}

fn build_legacy_mvf_plus_shell_face_plus_kill_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    let region = draft.arena().get_shell(seed.shell).expect("shell must exist").region();
    let shell_seed = draft
        .execute(MakeShellFace {
            region,
            kind: ShellKind::Sheet,
        })
        .expect("legacy MakeShellFace should succeed")
        .into_value();
    draft
        .execute(KillShellFace {
            face: shell_seed.face,
            vertex: shell_seed.vertex,
        })
        .expect("legacy KillShellFace should succeed");
    draft
        .commit()
        .expect("legacy MVF+MSF+KSF commit should succeed")
}

fn build_legacy_mvf_plus_kvf_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .expect("legacy MVF should succeed")
        .into_value();
    draft
        .execute(KillVertexFace {
            face: seed.face,
            vertex: seed.vertex,
        })
        .expect("legacy KillVertexFace should succeed");
    draft
        .commit()
        .expect("legacy MVF+KVF commit should succeed")
}

fn add(
    draft: &mut forge_spec::facade::SpecDraft,
    kind: RelationKind,
    source: forge_spec::facade::SpecNodeId,
    target: forge_spec::facade::SpecNodeId,
    ordinal: u32,
    role: &str,
) {
    draft.add_relation(kind, source, target, ordinal, role).unwrap();
}
