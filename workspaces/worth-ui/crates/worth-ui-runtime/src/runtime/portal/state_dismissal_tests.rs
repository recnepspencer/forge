use super::{
    state_tests::{
        idempotency, open_request, portal, presented_geometry, semantic_surface, state,
        viewport_bounds,
    },
    UiPortalDismissalCause, UiPortalDismissalIgnoreReason, UiPortalDismissalPreparation,
    UiPortalDismissalTrigger, UiPortalInputShielding, UiPortalLifecyclePosture,
    UiPortalServiceRequest,
};

#[test]
fn escape_and_anchor_loss_dismiss_nested_portals_in_topmost_order() {
    let mut state = state();
    let parent = portal(221, 231);
    let child = portal(222, 232);
    let parent_open = state.prepare(open_request(parent, 241)).unwrap();
    state.commit_published(parent_open).unwrap();
    let geometry = presented_geometry(4);
    let child_open = state
        .prepare(UiPortalServiceRequest::open_nested(
            child,
            idempotency(242),
            geometry,
            viewport_bounds(geometry),
            semantic_surface(),
            parent,
            UiPortalInputShielding::ModalSurface,
        ))
        .unwrap();
    state.commit_published(child_open).unwrap();

    let UiPortalDismissalPreparation::Prepared(dismiss_child) = state
        .prepare_dismissal(UiPortalDismissalTrigger::Escape, None, idempotency(243))
        .unwrap()
    else {
        panic!("Escape must dismiss the topmost nested portal")
    };
    assert_eq!(dismiss_child.portal(), child);
    assert!(dismiss_child.input_shielded());
    state
        .commit_published(dismiss_child.into_transition())
        .unwrap();
    let closed = state
        .last_closed()
        .expect("the owner retains its latest close cause");
    assert_eq!(closed.portal(), child);
    assert_eq!(closed.cause(), UiPortalDismissalCause::Escape);

    let UiPortalDismissalPreparation::Prepared(dismiss_parent) = state
        .prepare_dismissal(
            UiPortalDismissalTrigger::AnchorLoss(parent),
            None,
            idempotency(244),
        )
        .unwrap()
    else {
        panic!("anchor loss must dismiss its remaining portal")
    };
    assert_eq!(dismiss_parent.portal(), parent);
    state
        .commit_published(dismiss_parent.into_transition())
        .unwrap();
    assert_eq!(state.active_count(), 0);
}

#[test]
fn explicit_parent_close_atomically_closes_its_descendant_chain() {
    let mut state = state();
    let parent = portal(225, 235);
    let child = portal(226, 236);
    let parent_open = state.prepare(open_request(parent, 245)).unwrap();
    state.commit_published(parent_open).unwrap();
    let geometry = presented_geometry(5);
    let child_open = state
        .prepare(UiPortalServiceRequest::open_nested(
            child,
            idempotency(246),
            geometry,
            viewport_bounds(geometry),
            semantic_surface(),
            parent,
            UiPortalInputShielding::ContentBounds,
        ))
        .unwrap();
    state.commit_published(child_open).unwrap();

    let close = state
        .prepare(UiPortalServiceRequest::close(
            parent,
            idempotency(247),
            UiPortalDismissalCause::ExplicitOwnerRequest,
            semantic_surface(),
        ))
        .unwrap();
    assert!(state.mounted_projection_inputs(&close, false).is_empty());
    state.commit_published(close).unwrap();
    assert_eq!(state.posture(parent), UiPortalLifecyclePosture::Closed);
    assert_eq!(state.posture(child), UiPortalLifecyclePosture::Closed);
    assert_eq!(state.active_count(), 0);
}

#[test]
fn outside_press_respects_bounds_and_duplicate_dismissal_coalesces() {
    let mut state = state();
    let portal = portal(251, 261);
    let opened = state.prepare(open_request(portal, 271)).unwrap();
    let bounds = opened.placement().unwrap().bounds().components();
    state.commit_published(opened).unwrap();
    let inside = [bounds[0] + 1.0, bounds[1] + 1.0].map(f32::to_bits);
    assert!(matches!(
        state
            .prepare_dismissal(
                UiPortalDismissalTrigger::OutsidePress {
                    viewport_point_bits: inside
                },
                None,
                idempotency(272),
            )
            .unwrap(),
        UiPortalDismissalPreparation::Ignored(UiPortalDismissalIgnoreReason::InsideTopmostPortal)
    ));
    let sampled_bounds = worth_ui_host_contract::UiMountedCanonicalBox::canonicalize(
        worth_ui_host_contract::UiMountedCanonicalBoxInput {
            x: bounds[0] + 100.0,
            y: bounds[1] + 100.0,
            width: bounds[2],
            height: bounds[3],
            coordinate_space: worth_ui_host_contract::UiMountedCoordinateSpace::Viewport,
        },
    )
    .unwrap();
    let UiPortalDismissalPreparation::Prepared(dismissal) = state
        .prepare_dismissal(
            UiPortalDismissalTrigger::OutsidePress {
                viewport_point_bits: inside,
            },
            Some(sampled_bounds),
            idempotency(273),
        )
        .unwrap()
    else {
        panic!("outside press must prepare dismissal")
    };
    state.commit_published(dismissal.into_transition()).unwrap();
    let revision = state.revision();
    assert!(matches!(
        state
            .prepare_dismissal(
                UiPortalDismissalTrigger::OutsidePress {
                    viewport_point_bits: inside
                },
                None,
                idempotency(273),
            )
            .unwrap(),
        UiPortalDismissalPreparation::Ignored(UiPortalDismissalIgnoreReason::NoMatchingPortal)
    ));
    assert_eq!(state.revision(), revision);
}

#[test]
fn modal_policy_shields_input_and_disables_outside_press_dismissal() {
    let mut state = super::UiPortalRuntimeState::new_with_policy(
        crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate,
        crate::declaration::UiPortalPolicy::modal_dialog(),
    );
    let portal = portal(281, 291);
    let opened = state.prepare(open_request(portal, 301)).unwrap();
    assert_eq!(
        opened.placement().unwrap().shielding(),
        UiPortalInputShielding::ModalSurface
    );
    state.commit_published(opened).unwrap();

    assert!(matches!(
        state
            .prepare_dismissal(
                UiPortalDismissalTrigger::OutsidePress {
                    viewport_point_bits: [0.0_f32.to_bits(), 0.0_f32.to_bits()],
                },
                None,
                idempotency(302),
            )
            .unwrap(),
        UiPortalDismissalPreparation::Ignored(UiPortalDismissalIgnoreReason::NoMatchingPortal)
    ));
    assert_eq!(state.active_count(), 1);
}

#[test]
fn accepted_selection_and_anchor_loss_respect_the_declared_policy() {
    let policy = crate::declaration::UiPortalPolicy::dropdown()
        .with_accepted_selection_dismissal(false)
        .with_anchor_loss_dismissal(false);
    let mut state = super::UiPortalRuntimeState::new_with_policy(
        crate::runtime::UiServiceStatePersistencePosture::SessionRestoreCandidate,
        policy,
    );
    let portal = portal(311, 321);
    let opened = state.prepare(open_request(portal, 331)).unwrap();
    state.commit_published(opened).unwrap();

    for trigger in [
        UiPortalDismissalTrigger::AcceptedSelection,
        UiPortalDismissalTrigger::AnchorLoss(portal),
    ] {
        assert!(matches!(
            state
                .prepare_dismissal(trigger, None, idempotency(332))
                .unwrap(),
            UiPortalDismissalPreparation::Ignored(UiPortalDismissalIgnoreReason::NoMatchingPortal)
        ));
    }
    assert_eq!(state.active_count(), 1);
}
