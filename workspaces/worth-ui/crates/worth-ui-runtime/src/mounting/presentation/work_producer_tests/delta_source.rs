use worth_ui_host_contract::UiMountedFrameIdentity;

use super::{rect_spec, MountedPresentationWorld};
use crate::mounting::presentation::work_producer::{
    SuccessorIssueRequest, UiMountedPresentationState, UiMountedPresentationWorkProductionDenial,
};

#[test]
fn stale_successor_affinity_is_denied_before_work_issuance() {
    let world = MountedPresentationWorld::new();
    let predecessor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let successor = world.projection(
        UiMountedFrameIdentity::mint_unbound().unwrap(),
        [rect_spec(world.first_instance, 0.0)],
    );
    let predecessor_state =
        UiMountedPresentationState::from_projection(&predecessor, world.requirement, None);
    let stale_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(UiMountedFrameIdentity::mint_unbound().unwrap()),
    );
    let current_state = UiMountedPresentationState::from_projection(
        &successor,
        world.requirement,
        Some(predecessor.frame()),
    );
    let lease = super::super::UiMountedPresentationLeaseGate::default()
        .claim()
        .unwrap();

    assert!(matches!(
        predecessor_state.issue_successor(SuccessorIssueRequest::new(
            &stale_state,
            &[],
            &[],
            &lease,
        )),
        Err(UiMountedPresentationWorkProductionDenial::StalePredecessor)
    ));
    assert!(predecessor_state
        .issue_successor(SuccessorIssueRequest::new(&current_state, &[], &[], &lease,))
        .is_ok());
    assert!(matches!(
        predecessor_state.issue_successor(
            SuccessorIssueRequest::new(&current_state, &[], &[], &lease,)
                .with_source_predecessor(Some(UiMountedFrameIdentity::mint_unbound().unwrap()))
        ),
        Err(UiMountedPresentationWorkProductionDenial::StalePredecessor)
    ));
}
