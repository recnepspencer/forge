use forge_spec::facade::{
    CloneBodyMutation, DemoteShellMutation, ExtractLumpMutation, ExtractShellMutation,
    MakeEmptyShellMutation, MakeFaceVertexMutation, MakeLumpRegionMutation, MakeSolidMutation,
    MakeVertexFaceMutation, MergeBodiesMutation, MergeLumpsMutation, MergeShellsMutation,
    PromoteShellMutation, RehomeLumpMutation, RehomeShellMutation, RelationKind, SpecNodeKind,
    SpecShellKind, SpecShellOrientation, SpecState, SplitBodyMutation, SplitLumpMutation,
    SplitShellMutation,
};

use crate::b_rep::{EdgeData, FaceData, HalfEdgeData, LoopData, RegionData, ShellKind, VertexData};
use crate::handles::{EdgeId, FaceId, HalfEdgeId, LoopId, ShellId, VertexId};
use crate::operations::entity_lifecycle::make_face_vertex::MakeFaceVertex;
use crate::operations::entity_lifecycle::make_vertex_face::MakeVertexFace;
use crate::operations::lifecycle::body_ops::CloneBody;
use crate::operations::lifecycle::body_ops::{MergeBodies, SplitBody};
use crate::operations::lifecycle::lump_ops::{ExtractLump, MergeLumps, RehomeLump, SplitLump};
use crate::operations::lifecycle::shell::MakeEmptyShell;
use crate::operations::lifecycle::shell_ops::{
    DemoteShell, ExtractShell, MergeShells, PromoteShell, RehomeShell, SplitShell,
};
use crate::operations::lifecycle::solid::MakeSolid;
use crate::projection::facade::{compute_projected_topology_hash, ProjectionBuilder};
use crate::transactions::facade::{compute_arena_topology_hash, TopologyState};

#[test]
fn projected_rehome_shell_matches_legacy_signature() {
    let legacy = build_legacy_rehome_shell_state();
    let projected = ProjectionBuilder::build(&build_rehome_shell_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_split_shell_matches_legacy_signature() {
    let legacy = build_legacy_split_shell_state();
    let projected = ProjectionBuilder::build(&build_split_shell_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_merge_shells_matches_legacy_signature() {
    let legacy = build_legacy_merge_shells_state();
    let projected = ProjectionBuilder::build(&build_merge_shells_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_extract_shell_matches_legacy_signature() {
    let legacy = build_legacy_extract_shell_state();
    let projected = ProjectionBuilder::build(&build_extract_shell_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_rehome_lump_matches_legacy_signature() {
    let legacy = build_legacy_rehome_lump_state();
    let projected = ProjectionBuilder::build(&build_rehome_lump_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_extract_lump_matches_legacy_signature() {
    let legacy = build_legacy_extract_lump_state();
    let projected = ProjectionBuilder::build(&build_extract_lump_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_split_lump_matches_legacy_signature() {
    let legacy = build_legacy_split_lump_state();
    let projected = ProjectionBuilder::build(&build_split_lump_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_merge_lumps_matches_legacy_signature() {
    let legacy = build_legacy_merge_lumps_state();
    let projected = ProjectionBuilder::build(&build_merge_lumps_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_split_body_matches_legacy_signature() {
    let legacy = build_legacy_split_body_state();
    let projected = ProjectionBuilder::build(&build_split_body_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_merge_bodies_matches_legacy_signature() {
    let legacy = build_legacy_merge_bodies_state();
    let projected = ProjectionBuilder::build(&build_merge_bodies_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_clone_body_matches_legacy_signature() {
    let legacy = build_legacy_clone_body_state();
    let projected = ProjectionBuilder::build(&build_clone_body_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_promote_shell_matches_legacy_signature() {
    let legacy = build_legacy_promote_shell_state();
    let projected = ProjectionBuilder::build(&build_promote_shell_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

#[test]
fn projected_demote_shell_matches_legacy_signature() {
    let legacy = build_legacy_demote_shell_state();
    let projected = ProjectionBuilder::build(&build_demote_shell_state()).unwrap();
    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
}

fn build_rehome_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    let shell = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    draft
        .execute(RehomeShellMutation {
            shell: shell.value.shell,
            target_region: extra.value.region,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_split_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let extra_face = draft
        .execute(MakeFaceVertexMutation {
            shell: seed.value.shell,
        })
        .unwrap();
    draft
        .execute(SplitShellMutation {
            shell: seed.value.shell,
            faces_to_move: vec![extra_face.value.face],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_merge_shells_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let empty_shell = draft
        .execute(MakeEmptyShellMutation {
            region: seed.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    let second_face = draft
        .execute(MakeFaceVertexMutation {
            shell: empty_shell.value.shell,
        })
        .unwrap();
    let _ = second_face;
    draft
        .execute(MergeShellsMutation {
            target: seed.value.shell,
            source: empty_shell.value.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_rehome_lump_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let a = draft.execute(MakeSolidMutation).unwrap();
    let b = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(RehomeLumpMutation {
            lump: b.value.lump,
            target_body: a.value.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_extract_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let _outer = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    let inner = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Inner),
        })
        .unwrap();
    draft
        .execute(ExtractShellMutation {
            shell: inner.value.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_extract_lump_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft
        .execute(ExtractLumpMutation {
            lump: extra.value.lump,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_split_lump_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let region = draft
        .create_node(SpecNodeKind::Region, None, "region")
        .unwrap();
    draft
        .add_relation(
            RelationKind::LumpOwnsRegion,
            solid.value.lump,
            region,
            0,
            "extra-region",
        )
        .unwrap();
    draft
        .execute(SplitLumpMutation {
            lump: solid.value.lump,
            regions_to_move: vec![region],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_merge_lumps_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft
        .execute(MergeLumpsMutation {
            target: solid.value.lump,
            source: extra.value.lump,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_split_body_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft
        .execute(SplitBodyMutation {
            body: solid.value.body,
            lumps_to_move: vec![extra.value.lump],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_merge_bodies_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let a = draft.execute(MakeSolidMutation).unwrap();
    let b = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(MergeBodiesMutation {
            target: a.value.body,
            source: b.value.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_clone_body_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    draft
        .execute(CloneBodyMutation {
            body: seed.value.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_promote_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let _outer = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    let inner = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Inner),
        })
        .unwrap();
    draft
        .execute(PromoteShellMutation {
            shell: inner.value.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_demote_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();
    draft
        .execute(DemoteShellMutation {
            region: solid.value.region,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_rehome_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft
        .execute(crate::operations::lifecycle::lump::MakeLumpRegion { body: solid.body })
        .unwrap()
        .into_value();
    let shell = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    draft
        .execute(RehomeShell {
            shell: shell.shell,
            target_region: extra.region,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_split_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let extra_face = draft
        .execute(MakeFaceVertex { shell: seed.shell })
        .unwrap()
        .into_value();
    draft
        .execute(SplitShell {
            shell: seed.shell,
            faces_to_move: vec![extra_face.face],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_merge_shells_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    let shell = draft
        .execute(MakeEmptyShell {
            region: seed.region,
            kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    insert_shell_face_seed(&mut draft, shell.shell);
    draft
        .execute(MergeShells {
            target: seed.shell,
            source: shell.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn insert_shell_face_seed(
    draft: &mut crate::transactions::MutableDraft,
    shell: ShellId,
) -> (FaceId, LoopId, HalfEdgeId, EdgeId, VertexId) {
    let placeholder_he = HalfEdgeId::DANGLING;
    let placeholder_loop = LoopId::DANGLING;

    let vertex = draft.insert_vertex(VertexData::new(placeholder_he));
    let face = draft.insert_face(FaceData::new(placeholder_loop, shell));
    let loop_id = draft.insert_loop(LoopData::new(placeholder_he, face));
    let edge = draft.insert_edge(EdgeData::new(placeholder_he));
    let half_edge = draft.insert_half_edge(HalfEdgeData::new(
        placeholder_he,
        placeholder_he,
        placeholder_he,
        face,
        vertex,
        edge,
    ));

    draft
        .arena_mut()
        .set_half_edge_radial_next(half_edge, half_edge)
        .unwrap();
    draft
        .arena_mut()
        .get_half_edge_mut(half_edge)
        .unwrap()
        .set_next(half_edge);
    draft
        .arena_mut()
        .get_half_edge_mut(half_edge)
        .unwrap()
        .set_prev(half_edge);
    draft
        .arena_mut()
        .get_vertex_mut(vertex)
        .unwrap()
        .set_primary_disk(half_edge);
    draft
        .arena_mut()
        .get_face_mut(face)
        .unwrap()
        .loops
        .set_outer(loop_id);
    draft
        .arena_mut()
        .get_loop_mut(loop_id)
        .unwrap()
        .set_half_edge(half_edge);
    draft
        .arena_mut()
        .get_edge_mut(edge)
        .unwrap()
        .set_half_edge(half_edge);
    if draft
        .arena()
        .get_shell(shell)
        .unwrap()
        .representative_face()
        .is_dangling()
    {
        draft
            .arena_mut()
            .get_shell_mut(shell)
            .unwrap()
            .set_representative_face(face);
    }

    (face, loop_id, half_edge, edge, vertex)
}

fn build_legacy_rehome_lump_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let a = draft.execute(MakeSolid).unwrap().into_value();
    let b = draft.execute(MakeSolid).unwrap().into_value();
    draft
        .execute(RehomeLump {
            lump: b.lump,
            target_body: a.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_extract_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let _outer = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Solid(crate::b_rep::ShellOrientation::Outer),
        })
        .unwrap()
        .into_value();
    let inner = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Solid(crate::b_rep::ShellOrientation::Inner),
        })
        .unwrap()
        .into_value();
    draft.execute(ExtractShell { shell: inner.shell }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_extract_lump_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft
        .execute(crate::operations::lifecycle::lump::MakeLumpRegion { body: solid.body })
        .unwrap()
        .into_value();
    draft.execute(ExtractLump { lump: extra.lump }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_split_lump_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let region = draft.insert_region(RegionData::new(solid.lump));
    draft
        .arena_mut()
        .get_lump_mut(solid.lump)
        .unwrap()
        .add_region(region);
    draft
        .execute(SplitLump {
            lump: solid.lump,
            regions_to_move: vec![region],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_merge_lumps_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft
        .execute(crate::operations::lifecycle::lump::MakeLumpRegion { body: solid.body })
        .unwrap()
        .into_value();
    draft
        .execute(MergeLumps {
            target: solid.lump,
            source: extra.lump,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_split_body_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft
        .execute(crate::operations::lifecycle::lump::MakeLumpRegion { body: solid.body })
        .unwrap()
        .into_value();
    draft
        .execute(SplitBody {
            body: solid.body,
            lumps_to_move: vec![extra.lump],
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_merge_bodies_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let a = draft.execute(MakeSolid).unwrap().into_value();
    let b = draft.execute(MakeSolid).unwrap().into_value();
    draft
        .execute(MergeBodies {
            target: a.body,
            source: b.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_clone_body_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let seed = draft
        .execute(MakeVertexFace {
            shell_kind: ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    draft.execute(CloneBody { body: seed.solid }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_promote_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let _outer = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Solid(crate::b_rep::ShellOrientation::Outer),
        })
        .unwrap()
        .into_value();
    let inner = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Solid(crate::b_rep::ShellOrientation::Inner),
        })
        .unwrap()
        .into_value();
    draft.execute(PromoteShell { shell: inner.shell }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_demote_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: ShellKind::Solid(crate::b_rep::ShellOrientation::Outer),
        })
        .unwrap()
        .into_value();
    draft
        .execute(DemoteShell {
            region: solid.region,
        })
        .unwrap();
    draft.commit().unwrap()
}
