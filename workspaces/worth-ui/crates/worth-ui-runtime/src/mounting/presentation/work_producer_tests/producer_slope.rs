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
        observed.push(exercise_one_change(retained));
    }
    assert!(observed.iter().all(|cost| cost[..6] == [1, 1, 2, 2, 0, 0]));
    assert!(observed.windows(2).all(|pair| pair[0][6] < pair[1][6]));
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P3-DELTA-SOURCE-01\":1,\"P3-PRODUCER-SLOPE-01\":0}}");
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P3-DELTA-SOURCE-01\":\"successor-rediscovery\",\"P3-PRODUCER-SLOPE-01\":\"complete-successor-scan\"}}"
    );
}

fn exercise_one_change(retained: usize) -> [u64; 7] {
    let world = MountedPresentationWorld::new();
    let instances = (0..retained)
        .map(|_| UiMountedInstanceIdentity::mint_unbound().unwrap())
        .collect::<Vec<_>>();
    let predecessor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let (predecessor, successor) = if retained > 2_048 {
        (
            world.mixed_projection(predecessor_frame, &instances, false),
            world.mixed_projection(successor_frame, &instances, true),
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
                    if index == 0 {
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
