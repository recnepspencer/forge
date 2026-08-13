use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedPaintCommandChange, UiMountedPresentationWorkView,
};

use super::world::{rect_spec, MountedPresentationWorld};
use crate::mounting::presentation::work_producer::UiMountedPresentationState;

#[test]
fn precise_replacement_carries_vacated_and_successor_bounds() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 96.0)],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let replacement = successor.retained_paint_commands()[0].clone();
    let lease = super::super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    let work = predecessor_state
        .issue_successor(
            &successor_state,
            &[world.first_instance],
            &[UiMountedPaintCommandChange::Replace(replacement)],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("precise replacement must issue delta work");
    };
    let mut xs = delta
        .damage()
        .iter()
        .map(|damage| damage.bounds().x() as u32)
        .collect::<Vec<_>>();
    xs.sort_unstable();
    assert_eq!(xs, [0, 96], "old and new pixels both require replay");
}

#[test]
fn production_state_uses_bounded_direct_lookup_for_first_middle_and_last_collection_rows() {
    let world = MountedPresentationWorld::new();
    let projection = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let mut state =
        UiMountedPresentationState::from_projection(&projection, world.requirement, None);
    let commands = super::super::work_producer::collection_commands_for_test(1_359);
    state.install_command_lookup_probe(&commands);
    for index in [0, commands.len() / 2, commands.len() - 1] {
        let probes = state.command_option_probe(commands[index].identity());
        assert!(
            probes > 0 && probes <= 32,
            "row {index} used {probes} probes"
        );
    }
}
