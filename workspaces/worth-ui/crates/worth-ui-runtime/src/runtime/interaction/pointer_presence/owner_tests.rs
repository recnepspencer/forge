use super::*;

fn active_generation() -> crate::runtime::WorthUiActiveApplicationGenerationIdentity {
    let session = crate::runtime::tests::active_application_session_test_support::
        source_backed_component_session();
    let generation = session.active_generation_identity();
    let _ = session.shutdown();
    generation
}

#[test]
fn position_only_motion_does_not_change_presence_revision() {
    let mut owner = UiPointerPresenceOwner::new();
    let generation = active_generation();
    let pointer = UiHostPointerIdentity::new(1);
    let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let target = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let position = UiHostSurfacePosition::viewport_logical(10, 20);
    let presentation = UiHostObservationPresentationBasis::new(
        worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap(),
        binding,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
    );
    let first_receipt =
        worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound().unwrap();
    let first = owner
        .record_mouse_target(
            pointer,
            UiHostObservationSequence::new(1),
            position,
            presentation,
            Some((surface, binding, target, first_receipt)),
            &generation,
        )
        .unwrap();
    assert_eq!(first.owner_revision(), 1);
    assert!(owner
        .record_mouse_target(
            pointer,
            UiHostObservationSequence::new(2),
            UiHostSurfacePosition::viewport_logical(11, 21),
            presentation,
            Some((surface, binding, target, first_receipt)),
            &generation,
        )
        .is_none());
    assert_eq!(owner.appearance_snapshot().owner_revision(), 1);
    let successor = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let successor_receipt =
        worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound().unwrap();
    let changed = owner
        .record_mouse_target(
            pointer,
            UiHostObservationSequence::new(3),
            position,
            presentation,
            Some((surface, binding, successor, successor_receipt)),
            &generation,
        )
        .unwrap();
    assert_eq!(changed.previous(), Some(target));
    assert_eq!(changed.current(), Some(successor));
    assert_eq!(changed.previous_node_receipt(), Some(first_receipt));
    assert_eq!(changed.current_node_receipt(), Some(successor_receipt));
    assert_eq!(changed.owner_revision(), 2);
    assert_eq!(changed.position(), position);
    assert_eq!(changed.presentation(), presentation);
}

#[test]
fn primary_pointer_reselection_changes_revision_but_primary_motion_does_not() {
    let mut owner = UiPointerPresenceOwner::new();
    let generation = active_generation();
    let first = UiHostPointerIdentity::new(1);
    let second = UiHostPointerIdentity::new(2);
    let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let presentation = UiHostObservationPresentationBasis::new(
        worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap(),
        binding,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
    );
    let first_target = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let second_target = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let first_receipt =
        worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound().unwrap();
    let second_receipt =
        worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound().unwrap();
    let position = UiHostSurfacePosition::viewport_logical(10, 20);
    owner
        .record_mouse_target(
            first,
            UiHostObservationSequence::new(1),
            position,
            presentation,
            Some((surface, binding, first_target, first_receipt)),
            &generation,
        )
        .unwrap();
    owner
        .record_mouse_target(
            second,
            UiHostObservationSequence::new(2),
            position,
            presentation,
            Some((surface, binding, second_target, second_receipt)),
            &generation,
        )
        .unwrap();
    let reselected = owner
        .record_mouse_target(
            first,
            UiHostObservationSequence::new(3),
            UiHostSurfacePosition::viewport_logical(11, 21),
            presentation,
            Some((surface, binding, first_target, first_receipt)),
            &generation,
        )
        .unwrap();
    assert_eq!(reselected.owner_revision(), 3);
    assert!(owner
        .record_mouse_target(
            first,
            UiHostObservationSequence::new(4),
            UiHostSurfacePosition::viewport_logical(12, 22),
            presentation,
            Some((surface, binding, first_target, first_receipt)),
            &generation,
        )
        .is_none());
    assert_eq!(owner.appearance_snapshot().owner_revision(), 3);
}

#[test]
fn pre_cutover_owner_transition_is_rejected_by_the_successor_generation() {
    use crate::runtime::tests::active_application_session_test_support::{
        admit_candidate_catalog, component_candidate_submission, source_backed_component_session,
    };

    let mut session = source_backed_component_session();
    let generation = session.active_generation_identity();
    let mut owner = UiPointerPresenceOwner::new();
    let pointer = UiHostPointerIdentity::new(7);
    let surface = UiSemanticSurfaceIdentity::mint_unbound().unwrap();
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let target = UiMountedInstanceIdentity::mint_unbound().unwrap();
    let frame = worth_ui_host_contract::UiMountedFrameIdentity::mint_unbound().unwrap();
    let presentation = UiHostObservationPresentationBasis::new(
        worth_ui_host_contract::UiHostSurfaceIdentity::mint_unbound().unwrap(),
        frame,
        binding,
        worth_ui_host_contract::UiHostPresentationEpoch::issued_by_host(1),
    );
    let transition = owner
        .record_mouse_target(
            pointer,
            UiHostObservationSequence::new(1),
            UiHostSurfacePosition::viewport_logical(10, 20),
            presentation,
            Some((
                surface,
                binding,
                target,
                worth_ui_host_contract::UiMountedNodeReceiptIdentity::mint_unbound().unwrap(),
            )),
            &generation,
        )
        .unwrap();

    let mut prepared = session
        .prepare_replacement(component_candidate_submission(
            &session,
            "pointer-transition-successor",
            "workspace.component.active_session_candidate",
        ))
        .unwrap();
    let catalog = admit_candidate_catalog(&mut prepared);
    let lowered = session.lower_prepared_replacement(*prepared).unwrap();
    let pending = session.stage_prepared_replacement(lowered).unwrap();
    let boundary = session
        .execute_framework_turn(|_| {})
        .unwrap()
        .into_completion()
        .into_execution()
        .unwrap()
        .into_activation_boundary();
    let outcome = session
        .activate_prepared_replacement(pending, catalog, boundary, None)
        .unwrap();
    assert!(outcome.activation().is_some());

    let mut turn = session.begin_observation_turn().unwrap();
    turn.admit_pointer_presence_transition(transition).unwrap();
    let set = turn.seal().unwrap();
    assert!(matches!(
        session.classify_observations(set),
        Err(
            crate::runtime::observation::UiChangeClassificationDenial::ForeignApplicationGeneration
        )
    ));
    let _ = session.shutdown();
}
