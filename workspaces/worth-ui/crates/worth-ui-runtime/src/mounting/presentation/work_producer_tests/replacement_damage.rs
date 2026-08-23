use worth_ui_host_contract::{
    UiMountedFrameIdentity, UiMountedPresentationWorkView, UiMountedRgba8,
};

use super::{rect_spec, MountedPresentationWorld};
use crate::mounting::presentation::work_producer::{
    SuccessorIssueRequest, UiMountedPresentationState,
};

#[test]
fn one_replacement_carries_one_change_and_exact_predecessor_successor_damage() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let mut changed = rect_spec(world.first_instance, 0.0);
    changed.color = UiMountedRgba8::new(242, 204, 96, 255);
    changed.x = 12.0;
    changed.clip_x = 12.0;
    let successor = world.projection(UiMountedFrameIdentity::mint_unbound().unwrap(), [changed]);
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
    let initial_work = predecessor_state.issue_initial(&lease, &predecessor);
    let UiMountedPresentationWorkView::Initial(initial) = initial_work.view() else {
        panic!("predecessor projection must issue initial work");
    };
    assert_eq!(initial.affinity().predecessor(), None);
    assert_eq!(initial.affinity().successor(), predecessor.frame());
    let work = predecessor_state
        .issue_successor(SuccessorIssueRequest::new(
            &successor_state,
            &[world.first_instance],
            &[],
            &lease,
        ))
        .unwrap();
    let UiMountedPresentationWorkView::Delta(delta) = work.view() else {
        panic!("changed retained command must produce delta work");
    };
    assert_eq!(delta.affinity().predecessor(), Some(predecessor.frame()));
    assert_eq!(delta.affinity().successor(), successor.frame());
    assert_eq!(delta.changes().len(), 1);
    assert_eq!(delta.damage().len(), 2);
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 0.0));
    assert!(delta
        .damage()
        .iter()
        .any(|damage| damage.bounds().x() == 12.0));
    assert!(delta.order_integrity().admits(&[
        worth_ui_host_contract::UiMountedPaintOrderIdentity::for_command(change_identity(
            &delta.changes()[0]
        ),),
    ]));
    let unchanged_projection =
        world.projection(UiMountedFrameIdentity::mint_unbound().unwrap(), [changed]);
    let unchanged_state = UiMountedPresentationState::from_projection(
        &unchanged_projection,
        world.requirement,
        Some(successor.frame()),
    );
    let unchanged_work = successor_state
        .issue_successor(SuccessorIssueRequest::new(
            &unchanged_state,
            &[],
            &[],
            &lease,
        ))
        .unwrap();
    let UiMountedPresentationWorkView::Unchanged(unchanged) = unchanged_work.view() else {
        panic!("frame-only progression must issue unchanged work");
    };
    assert_eq!(unchanged.affinity().predecessor(), Some(successor.frame()));
    assert_eq!(
        unchanged.affinity().successor(),
        unchanged_projection.frame()
    );
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P1-AFFINITY-01\":3}}");
}

fn change_identity(
    change: &worth_ui_host_contract::UiMountedPaintCommandChange,
) -> worth_ui_host_contract::UiMountedPaintCommandIdentity {
    match change {
        worth_ui_host_contract::UiMountedPaintCommandChange::Insert(command)
        | worth_ui_host_contract::UiMountedPaintCommandChange::Replace {
            successor: command, ..
        } => command.identity(),
        worth_ui_host_contract::UiMountedPaintCommandChange::Remove(identity) => *identity,
    }
}
