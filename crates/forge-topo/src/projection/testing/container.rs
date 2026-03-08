use forge_spec::facade::{
    DestroyBodyMutation, DestroyLumpMutation, DestroyShellMutation, MakeEmptyShellMutation,
    MakeLumpRegionMutation, MakeSolidMutation, SpecShellKind, SpecState,
};

use crate::operations::lifecycle::lump::{DestroyLump, MakeLumpRegion};
use crate::operations::lifecycle::shell::{DestroyShell, MakeEmptyShell};
use crate::operations::lifecycle::solid::{DestroyBody, MakeSolid};
use crate::projection::facade::{ProjectionBuilder, compute_projected_topology_hash};
use crate::transactions::facade::{TopologyState, compute_arena_topology_hash};

#[test]
fn projected_make_solid_matches_legacy_container_signature() {
    let legacy = build_legacy_make_solid_state();
    let projected = ProjectionBuilder::build(&build_make_solid_state())
        .expect("spec-state MakeSolid projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
}

#[test]
fn projected_make_solid_plus_destroy_matches_legacy_container_signature() {
    let legacy = build_legacy_make_solid_destroy_body_state();
    let projected = ProjectionBuilder::build(&build_make_solid_destroy_body_state())
        .expect("spec-state MakeSolid+DestroyBody projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
}

#[test]
fn projected_make_lump_region_matches_legacy_container_signature() {
    let legacy = build_legacy_make_lump_region_state();
    let projected = ProjectionBuilder::build(&build_make_lump_region_state())
        .expect("spec-state MakeLumpRegion projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
}

#[test]
fn projected_make_lump_region_plus_destroy_matches_legacy_container_signature() {
    let legacy = build_legacy_make_lump_region_destroy_state();
    let projected = ProjectionBuilder::build(&build_make_lump_region_destroy_state())
        .expect("spec-state MakeLumpRegion+DestroyLump projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
}

#[test]
fn projected_make_empty_shell_matches_legacy_container_signature() {
    let legacy = build_legacy_make_empty_shell_state();
    let projected = ProjectionBuilder::build(&build_make_empty_shell_state())
        .expect("spec-state MakeEmptyShell projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
}

#[test]
fn projected_make_empty_shell_plus_destroy_matches_legacy_container_signature() {
    let legacy = build_legacy_make_empty_shell_destroy_state();
    let projected = ProjectionBuilder::build(&build_make_empty_shell_destroy_state())
        .expect("spec-state MakeEmptyShell+DestroyShell projection should succeed");

    assert_eq!(
        compute_projected_topology_hash(&projected),
        compute_arena_topology_hash(legacy.arena())
    );
    assert_eq!(projected.body_count(), legacy.arena().body_count() as usize);
    assert_eq!(projected.lump_count(), legacy.arena().lump_count() as usize);
    assert_eq!(projected.region_count(), legacy.arena().region_count() as usize);
    assert_eq!(projected.shell_count(), legacy.arena().shell_count() as usize);
}

fn build_make_solid_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    draft.execute(MakeSolidMutation).unwrap();
    draft.commit().unwrap()
}

fn build_make_solid_destroy_body_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(DestroyBodyMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_make_lump_region_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_make_lump_region_destroy_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    draft
        .execute(DestroyLumpMutation {
            lump: extra.value.lump,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_make_empty_shell_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_make_empty_shell_destroy_state() -> SpecState {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    draft
        .execute(DestroyShellMutation {
            shell: shell.value.shell,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_solid_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    draft.execute(MakeSolid).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_solid_destroy_body_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    draft.execute(DestroyBody { body: solid.body }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_lump_region_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    draft.execute(MakeLumpRegion { body: solid.body }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_lump_region_destroy_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let extra = draft.execute(MakeLumpRegion { body: solid.body }).unwrap().into_value();
    draft.execute(DestroyLump { lump: extra.lump }).unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_empty_shell_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: crate::b_rep::ShellKind::Sheet,
        })
        .unwrap();
    draft.commit().unwrap()
}

fn build_legacy_make_empty_shell_destroy_state() -> TopologyState {
    let mut draft = TopologyState::empty().into_mutation();
    let solid = draft.execute(MakeSolid).unwrap().into_value();
    let shell = draft
        .execute(MakeEmptyShell {
            region: solid.region,
            kind: crate::b_rep::ShellKind::Sheet,
        })
        .unwrap()
        .into_value();
    draft.execute(DestroyShell { shell: shell.shell }).unwrap();
    draft.commit().unwrap()
}
