use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedPresentationWorkView,
    UiMountedRgba8,
};

use super::super::work_producer::UiMountedPresentationState;
use super::world::{rect_spec, MountedPresentationWorld};

#[test]
fn admitted_sources_leave_only_local_work_inside_delta_issuance() {
    let mut observed = Vec::new();
    for retained in [1, 32, 2_048, 4_096] {
        for changed_index in change_positions(retained) {
            let actual = exercise_one_change(retained, changed_index);
            assert_eq!(actual, expected_local_cost(retained));
            observed.push(actual);
        }
    }
    assert_eq!(observed.len(), 12);
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P3-DELTA-SOURCE-01\":1,\"P3-PRODUCER-SLOPE-01\":0}}");
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-DELTA-SOURCE-01\":\"successor-rediscovery\",\"P3-PRODUCER-SLOPE-01\":\"complete-successor-scan\"}}"
    );
}

fn change_positions(retained: usize) -> [usize; 3] {
    [0, retained / 2, retained.saturating_sub(1)]
}

fn expected_local_cost(retained: usize) -> [u64; 7] {
    [1, 1, 2, 2, 0, 0, u64::try_from(retained * 2).unwrap()]
}

fn exercise_one_change(retained: usize, changed_index: usize) -> [u64; 7] {
    let world = MountedPresentationWorld::new();
    let instances = (0..retained)
        .map(|_| UiMountedInstanceIdentity::mint_unbound().unwrap())
        .collect::<Vec<_>>();
    let predecessor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let (predecessor, successor) = if retained > 2_048 {
        (
            world.mixed_projection(predecessor_frame, &instances, None),
            world.mixed_projection(successor_frame, &instances, Some(changed_index)),
        )
    } else {
        (
            world.projection(
                predecessor_frame,
                instances
                    .iter()
                    .enumerate()
                    .map(|(index, instance)| rect_spec(*instance, index as f32 * 40.0)),
            ),
            world.projection(
                successor_frame,
                instances.iter().enumerate().map(|(index, instance)| {
                    let mut spec = rect_spec(*instance, index as f32 * 40.0);
                    if index == changed_index {
                        spec.color = UiMountedRgba8::new(242, 204, 96, 255);
                    }
                    spec
                }),
            ),
        )
    };
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
        .issue_successor(
            &successor_state,
            &instances[..1],
            &[],
            false,
            Some(predecessor.frame()),
            &lease,
        )
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("one changed command must issue delta work");
    };
    assert_eq!(delta.changes().len(), 1);
    let cost = delta.production_cost();
    [
        cost.source_instances(),
        cost.commands_considered(),
        cost.command_index_lookups(),
        cost.order_lookups(),
        cost.retained_command_scans(),
        cost.retained_command_clones(),
        cost.projection_rows_materialized(),
    ]
}
