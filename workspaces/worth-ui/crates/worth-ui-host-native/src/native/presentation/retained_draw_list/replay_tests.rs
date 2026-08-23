use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedLogicalDamage, UiMountedPaintCommandChange,
    UiMountedPaintOrderEdit, UiMountedPresentationDelta, UiMountedPresentationDeltaInput,
    UiMountedRgba8,
};

use super::DrawListWorld;
use crate::native::presentation::UiNativeRetainedDrawList;

#[test]
fn disjoint_regions_retain_separate_local_replay_lists() {
    let world = DrawListWorld::new();
    let predecessor = UiMountedFrameIdentity::mint_unbound().unwrap();
    let rows = [
        world.rect(
            predecessor,
            world.first,
            0.0,
            UiMountedRgba8::new(10, 20, 30, 255),
        ),
        world.rect(
            predecessor,
            world.second,
            80.0,
            UiMountedRgba8::new(40, 50, 60, 255),
        ),
    ];
    let initial = world.initial(predecessor, rows);
    let expected = initial
        .commands()
        .iter()
        .map(|command| command.identity())
        .collect::<Vec<_>>();
    let mut retained = UiNativeRetainedDrawList::initial(&initial, &[]).unwrap();
    let delta = UiMountedPresentationDelta::from_inert_mechanics(UiMountedPresentationDeltaInput {
        predecessor,
        successor: UiMountedFrameIdentity::mint_unbound().unwrap(),
        surface: world.surface,
        binding: world.binding,
        content: world.content,
        baseline: world.requirement.baseline(),
        changes: Vec::new(),
        nodes: Vec::new(),
        order: Vec::new(),
        order_integrity: initial.order_integrity(),
        damage: rows
            .iter()
            .map(|row| UiMountedLogicalDamage::from_runtime_mounting(row.bounds()))
            .collect(),
        auxiliary: None,
        production_cost: Default::default(),
    });
    let plan = retained.apply_delta(&delta).unwrap();
    assert_eq!(plan.regions.len(), 2);
    assert_eq!(plan.regions[0].replay.as_ref(), [expected[0]]);
    assert_eq!(plan.regions[1].replay.as_ref(), [expected[1]]);
    assert_eq!(plan.counters.damage_region_command_checks, 2);
    assert_eq!(plan.counters.replayed_commands, 2);
}

#[test]
fn removing_the_top_command_replays_the_vacated_underlying_command() {
    let world = DrawListWorld::new();
    let predecessor = UiMountedFrameIdentity::mint_unbound().unwrap();
    let lower = world.rect(
        predecessor,
        world.first,
        0.0,
        UiMountedRgba8::new(10, 20, 30, 255),
    );
    let upper = world.rect(
        predecessor,
        world.second,
        0.0,
        UiMountedRgba8::new(40, 50, 60, 255),
    );
    let initial = world.initial(predecessor, [lower, upper]);
    let lower_identity = initial.commands()[0].identity();
    let upper_identity = initial.commands()[1].identity();
    let mut retained = UiNativeRetainedDrawList::initial(&initial, &[]).unwrap();
    let delta = UiMountedPresentationDelta::from_inert_mechanics(UiMountedPresentationDeltaInput {
        predecessor,
        successor: UiMountedFrameIdentity::mint_unbound().unwrap(),
        surface: world.surface,
        binding: world.binding,
        content: world.content,
        baseline: world.requirement.baseline(),
        changes: vec![UiMountedPaintCommandChange::Remove(upper_identity)],
        nodes: Vec::new(),
        order: vec![UiMountedPaintOrderEdit::remove(
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(upper_identity),
        )],
        order_integrity: worth_ui_host_contract::UiMountedPaintOrderIntegrity::for_order(&[
            worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(lower_identity),
        ]),
        damage: vec![UiMountedLogicalDamage::from_runtime_mounting(
            upper.bounds(),
        )],
        auxiliary: None,
        production_cost: Default::default(),
    });
    let plan = retained.apply_delta(&delta).unwrap();
    assert_eq!(plan.regions.len(), 1);
    assert_eq!(plan.regions[0].replay.as_ref(), [lower_identity]);
    assert_eq!(plan.counters.replayed_commands, 1);
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-DAMAGE-REPLAY-01\":\"omitted-vacated-replay\"}}"
    );
}
