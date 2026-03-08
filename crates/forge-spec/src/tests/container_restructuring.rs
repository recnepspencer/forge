use crate::facade::*;
use crate::data::schema::{RelationKind, SpecNodeKind};

#[test]
fn rehome_shell_moves_empty_shell_between_regions() {
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

    let state = draft.commit().unwrap();
    let graph = state.graph();
    let owners: Vec<_> = graph
        .incoming_relations(shell.value.shell)
        .into_iter()
        .filter(|relation| relation.kind == RelationKind::RegionOwnsShell)
        .collect();

    assert_eq!(owners.len(), 1);
    assert_eq!(owners[0].source, extra.value.region);
}

#[test]
fn split_shell_moves_face_subset_into_new_shell() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();
    let extra_face = draft
        .execute(MakeFaceVertexMutation {
            shell: seed.value.shell,
        })
        .unwrap();

    let split = draft
        .execute(SplitShellMutation {
            shell: seed.value.shell,
            faces_to_move: vec![extra_face.value.face],
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();

    assert!(graph.node(split.value.new_shell).is_some());
    assert_eq!(
        graph.outgoing_of_kind(seed.value.shell, RelationKind::ShellOwnsFace)
            .len(),
        1
    );
    assert_eq!(
        graph.outgoing_of_kind(split.value.new_shell, RelationKind::ShellOwnsFace)
            .len(),
        1
    );
}

#[test]
fn merge_shells_absorbs_source_shell() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell_a = draft
        .execute(MakeShellFaceMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();
    let shell_b = draft
        .execute(MakeShellFaceMutation {
            region: solid.value.region,
            kind: SpecShellKind::Sheet,
        })
        .unwrap();

    draft
        .execute(MergeShellsMutation {
            target: shell_a.value.shell,
            source: shell_b.value.shell,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert!(graph.node(shell_b.value.shell).is_none());
    assert_eq!(
        graph.outgoing_of_kind(shell_a.value.shell, RelationKind::ShellOwnsFace)
            .len(),
        2
    );
}

#[test]
fn extract_shell_creates_new_region_for_inner_shell() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let outer = draft
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
    let _ = outer;

    let extracted = draft
        .execute(ExtractShellMutation {
            shell: inner.value.shell,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert_eq!(
        graph.iter_nodes().filter(|node| node.kind == SpecNodeKind::Region).count(),
        2
    );
    assert_eq!(
        graph.outgoing_of_kind(extracted.value.new_region, RelationKind::RegionOwnsShell)
            .len(),
        1
    );
    assert_eq!(
        state.shell_kind(inner.value.shell).unwrap(),
        SpecShellKind::Solid(SpecShellOrientation::Outer)
    );
}

#[test]
fn extract_shell_rejects_outer_solid_shell() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let outer = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
            kind: SpecShellKind::Solid(SpecShellOrientation::Outer),
        })
        .unwrap();

    let result = draft.execute(ExtractShellMutation {
        shell: outer.value.shell,
    });

    assert!(result.is_err());
}

#[test]
fn rehome_lump_moves_lump_and_deletes_empty_source_body() {
    let mut draft = SpecState::empty().into_draft();
    let body_a = draft.execute(MakeSolidMutation).unwrap();
    let body_b = draft.execute(MakeSolidMutation).unwrap();

    draft
        .execute(RehomeLumpMutation {
            lump: body_b.value.lump,
            target_body: body_a.value.body,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert!(graph.node(body_b.value.body).is_none());
    assert_eq!(
        graph.outgoing_of_kind(body_a.value.body, RelationKind::BodyOwnsLump)
            .len(),
        2
    );
}

#[test]
fn extract_lump_creates_new_body_for_existing_lump() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();

    let extracted = draft
        .execute(ExtractLumpMutation {
            lump: extra.value.lump,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert_eq!(
        graph.iter_nodes().filter(|node| node.kind == SpecNodeKind::Body).count(),
        2
    );
    assert_eq!(
        graph.outgoing_of_kind(extracted.value.new_body, RelationKind::BodyOwnsLump)
            .len(),
        1
    );
}

#[test]
fn split_lump_moves_region_subset_into_new_lump() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let region = draft.create_node(SpecNodeKind::Region, None, "region").unwrap();
    draft
        .add_relation(
            RelationKind::LumpOwnsRegion,
            solid.value.lump,
            region,
            0,
            "extra-region",
        )
        .unwrap();

    let split = draft
        .execute(SplitLumpMutation {
            lump: solid.value.lump,
            regions_to_move: vec![region],
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert_eq!(graph.outgoing_of_kind(solid.value.lump, RelationKind::LumpOwnsRegion).len(), 1);
    assert_eq!(graph.outgoing_of_kind(split.value.new_lump, RelationKind::LumpOwnsRegion).len(), 1);
}

#[test]
fn merge_lumps_absorbs_source_lump() {
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

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert!(graph.node(extra.value.lump).is_none());
    assert_eq!(graph.outgoing_of_kind(solid.value.body, RelationKind::BodyOwnsLump).len(), 1);
    assert_eq!(graph.outgoing_of_kind(solid.value.lump, RelationKind::LumpOwnsRegion).len(), 2);
}

#[test]
fn split_body_moves_lump_subset_into_new_body() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let extra = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();

    let split = draft
        .execute(SplitBodyMutation {
            body: solid.value.body,
            lumps_to_move: vec![extra.value.lump],
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert_eq!(graph.outgoing_of_kind(solid.value.body, RelationKind::BodyOwnsLump).len(), 1);
    assert_eq!(graph.outgoing_of_kind(split.value.new_body, RelationKind::BodyOwnsLump).len(), 1);
}

#[test]
fn merge_bodies_absorbs_source_body() {
    let mut draft = SpecState::empty().into_draft();
    let body_a = draft.execute(MakeSolidMutation).unwrap();
    let body_b = draft.execute(MakeSolidMutation).unwrap();

    draft
        .execute(MergeBodiesMutation {
            target: body_a.value.body,
            source: body_b.value.body,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert!(graph.node(body_b.value.body).is_none());
    assert_eq!(graph.outgoing_of_kind(body_a.value.body, RelationKind::BodyOwnsLump).len(), 2);
}

#[test]
fn clone_body_duplicates_seed_topology_subgraph() {
    let mut draft = SpecState::empty().into_draft();
    let seed = draft.execute(MakeVertexFaceMutation).unwrap();

    let cloned = draft
        .execute(CloneBodyMutation {
            body: seed.value.body,
        })
        .unwrap();

    let state = draft.commit().unwrap();
    let graph = state.graph();
    assert_eq!(
        graph.iter_nodes().filter(|node| node.kind == SpecNodeKind::Body).count(),
        2
    );
    assert_eq!(
        graph.iter_nodes().filter(|node| node.kind == SpecNodeKind::Face).count(),
        2
    );
    assert_eq!(
        graph.outgoing_of_kind(cloned.value.cloned_body, RelationKind::BodyOwnsLump)
            .len(),
        1
    );
}

#[test]
fn promote_shell_swaps_inner_and_outer_roles() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let outer = draft
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

    let state = draft.commit().unwrap();
    assert_eq!(
        state.shell_kind(inner.value.shell).unwrap(),
        SpecShellKind::Solid(SpecShellOrientation::Outer)
    );
    assert_eq!(
        state.shell_kind(outer.value.shell).unwrap(),
        SpecShellKind::Solid(SpecShellOrientation::Inner)
    );
}

#[test]
fn demote_shell_reclassifies_outer_shell_as_inner() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let outer = draft
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

    let state = draft.commit().unwrap();
    assert_eq!(
        state.shell_kind(outer.value.shell).unwrap(),
        SpecShellKind::Solid(SpecShellOrientation::Inner)
    );
}
