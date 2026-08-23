use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedInstanceIdentity, UiMountedPresentationWorkView,
    UiMountedRgba8,
};

use super::super::work_producer::UiMountedPresentationState;
use super::{rect_spec, rect_spec_with_clip, MountedPresentationWorld};

#[test]
fn removal_damage_follows_authored_order_after_identity_replacement() {
    let world = MountedPresentationWorld::new();
    let remounted_first = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [
            rect_spec(remounted_first, 0.0),
            rect_spec(world.first_instance, 40.0),
        ],
    );
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        std::iter::empty(),
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
        .issue_successor(super::super::work_producer::SuccessorIssueRequest::new(
            &successor_state,
            &[world.first_instance, remounted_first],
            &[],
            &lease,
        ))
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("removal must produce delta work");
    };
    assert_eq!(
        delta
            .damage()
            .iter()
            .map(|damage| damage.bounds().x())
            .collect::<Vec<_>>(),
        vec![0.0, 40.0]
    );
}

#[test]
fn replacement_damage_is_clipped_to_predecessor_and_successor_visibility() {
    let world = MountedPresentationWorld::new();
    let frame = UiMountedFrameIdentity::mint_unbound().unwrap();
    let predecessor = world.projection(
        frame,
        [rect_spec_with_clip(world.first_instance, 0.0, 4.0, 16.0)],
    );
    let mut successor_spec = rect_spec_with_clip(world.first_instance, 0.0, 8.0, 10.0);
    successor_spec.color = UiMountedRgba8::new(200, 20, 40, 255);
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [successor_spec],
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
        .issue_successor(super::super::work_producer::SuccessorIssueRequest::new(
            &successor_state,
            &[world.first_instance],
            &[],
            &lease,
        ))
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("changed visible rectangle must produce delta work");
    };
    let exact_damage = delta
        .damage()
        .iter()
        .map(|damage| (damage.bounds().x(), damage.bounds().width()))
        .collect::<Vec<_>>();
    assert_eq!(exact_damage, vec![(4.0, 16.0), (8.0, 10.0)]);
    println!(
        "WORTH_UI_LEDGER_COUNTERS={{\"P1-DAMAGE-01\":{}}}",
        exact_damage.len()
    );
}
