use crate::facade::*;

#[test]
fn make_solid_mutation_builds_empty_container_hierarchy() {
    let mut draft = SpecState::empty().into_draft();
    let result = draft.execute(MakeSolidMutation).unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 3);
    assert_eq!(state.graph().iter_relations().count(), 2);
}

#[test]
fn destroy_body_mutation_removes_empty_container_hierarchy() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    draft
        .execute(DestroyBodyMutation {
            body: solid.value.body,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 0);
    assert_eq!(state.graph().iter_relations().count(), 0);
}

#[test]
fn make_lump_region_mutation_adds_second_container_branch() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let result = draft
        .execute(MakeLumpRegionMutation {
            body: solid.value.body,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 5);
    assert_eq!(state.graph().iter_relations().count(), 4);
}

#[test]
fn destroy_lump_mutation_removes_only_empty_container_branch() {
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
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 3);
    assert_eq!(state.graph().iter_relations().count(), 2);
}

#[test]
fn make_empty_shell_mutation_creates_shell_without_faces() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let result = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(result.touched_domains, vec![TouchedDomain::Topology]);
    assert_eq!(state.graph().iter_nodes().count(), 4);
    assert_eq!(state.graph().iter_relations().count(), 3);
}

#[test]
fn destroy_shell_mutation_removes_empty_shell() {
    let mut draft = SpecState::empty().into_draft();
    let solid = draft.execute(MakeSolidMutation).unwrap();
    let shell = draft
        .execute(MakeEmptyShellMutation {
            region: solid.value.region,
        })
        .unwrap();
    draft
        .execute(DestroyShellMutation {
            shell: shell.value.shell,
        })
        .unwrap();
    let state = draft.commit().unwrap();

    assert_eq!(state.graph().iter_nodes().count(), 3);
    assert_eq!(state.graph().iter_relations().count(), 2);
}
