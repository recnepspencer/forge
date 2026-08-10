use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedPresentationWorkView,
    UiMountedRgba8,
};

use super::super::work_producer::UiMountedPresentationState;
use super::world::{rect_spec, MountedPresentationWorld};

#[test]
fn one_changed_command_has_constant_producer_work_at_retained_scale() {
    let mut observed = Vec::new();
    for retained in [1, 32, 2_048] {
        observed.push(exercise_one_change(retained));
    }
    assert!(observed.iter().all(|cost| *cost == observed[0]));
    assert_eq!(observed[0], (1, 1, 2, 2, 0, 0));
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P3-PRODUCER-SLOPE-01\":{}}}",
        observed.len()
    );
}

fn exercise_one_change(retained: usize) -> (u64, u64, u64, u64, u64, u64) {
    let world = MountedPresentationWorld::new();
    let instances = (0..retained)
        .map(|_| UiMountedInstanceIdentity::mint_unbound().unwrap())
        .collect::<Vec<_>>();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        instances
            .iter()
            .enumerate()
            .map(|(index, instance)| rect_spec(*instance, index as f32 * 40.0)),
    );
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        instances.iter().enumerate().map(|(index, instance)| {
            let mut spec = rect_spec(*instance, index as f32 * 40.0);
            if index == 0 {
                spec.color = UiMountedRgba8::new(242, 204, 96, 255);
            }
            spec
        }),
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let successor_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();
    let work = predecessor_state
        .issue_successor(&successor_state, &instances[..1], false, &lease)
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("one changed command must issue delta work");
    };
    assert_eq!(delta.changes().len(), 1);
    let cost = delta.production_cost();
    (
        cost.source_instances(),
        cost.commands_considered(),
        cost.command_index_lookups(),
        cost.order_lookups(),
        cost.retained_command_scans(),
        cost.retained_command_clones(),
    )
}
