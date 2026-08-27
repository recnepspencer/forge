use worth_ui_host_contract::{UiMountedFrameIdentity, UiMountedPresentationWorkView};

use super::{rect_spec, MountedPresentationWorld};
use crate::mounting::presentation::work_producer::{
    SuccessorIssueRequest, UiMountedPresentationState,
};

#[test]
fn unchanged_successor_carries_zero_command_order_and_damage_work() {
    let world = MountedPresentationWorld::new();
    let predecessor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let successor_frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(predecessor_frame, [rect_spec(world.first_instance, 0.0)]);
    let successor = world.projection(successor_frame, [rect_spec(world.first_instance, 0.0)]);
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
        .issue_successor(SuccessorIssueRequest::new(
            &successor_state,
            &[],
            &[],
            &lease,
        ))
        .unwrap();
    let UiMountedPresentationWorkView::Unchanged(unchanged) = work.view() else {
        panic!("frame-only affinity progression must be unchanged work");
    };
    assert_eq!(unchanged.affinity().predecessor(), Some(predecessor_frame));
    assert_eq!(unchanged.affinity().successor(), successor_frame);
    assert_eq!(
        unchanged.affinity().baseline(),
        world.requirement.baseline()
    );
}
