use super::*;

fn surface() -> worth_ui_host_contract::UiSemanticSurfaceIdentity {
    worth_ui_host_contract::UiSemanticSurfaceIdentity::mint_unbound().unwrap()
}

fn incarnation(value: u64) -> UiScrollOwnerIncarnation {
    UiScrollOwnerIncarnation::new(value).unwrap()
}

fn bounds(inline: i64, block: i64) -> UiScrollBounds {
    UiScrollBounds::new(inline, block).unwrap()
}

fn register(
    state: &mut UiScrollRuntimeState,
    owner: UiScrollOwnerIdentity,
    incarnation: UiScrollOwnerIncarnation,
    offset: UiScrollOffset,
) {
    state
        .register(UiScrollOwnerRegistration::new(
            owner,
            incarnation,
            UiScrollAxes::Block,
            bounds(0, 500),
            offset,
        ))
        .unwrap();
}

#[test]
fn rebind_rebases_same_anchor_identity_without_treating_equal_bounds_as_proof() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let first = incarnation(3);
    let second = incarnation(4);
    let first_binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let second_binding =
        worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let identity = UiScrollAnchorIdentity::application_item(application_item(7));
    let old_anchor = UiScrollAnchor::new(identity, first_binding, 0, 200).unwrap();
    let new_anchor = UiScrollAnchor::new(identity, second_binding, 0, 260).unwrap();
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                first,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::new(0, 100).unwrap(),
            ),
            Some(old_anchor),
            UiScrollAnchorPolicy::Rebase,
        ))
        .unwrap();
    let receipt = state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                second,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::origin(),
            ),
            Some(new_anchor),
            UiScrollAnchorPolicy::Rebase,
        ))
        .unwrap();
    assert_eq!(
        receipt.outcome(),
        UiScrollAnchorReconciliationOutcome::Rebased
    );
    assert_eq!(receipt.offset(), UiScrollOffset::new(0, 160).unwrap());
}

#[test]
fn exact_numeric_bounds_do_not_preserve_across_anchor_basis_change() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let first = incarnation(5);
    let binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let anchor = |value| {
        UiScrollAnchor::new(
            UiScrollAnchorIdentity::application_item(application_item(value)),
            binding,
            0,
            200,
        )
        .unwrap()
    };
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                first,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::new(0, 100).unwrap(),
            ),
            Some(anchor(8)),
            UiScrollAnchorPolicy::Rebase,
        ))
        .unwrap();
    let receipt = state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                incarnation(6),
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::origin(),
            ),
            Some(anchor(9)),
            UiScrollAnchorPolicy::Preserve,
        ))
        .unwrap();
    assert_eq!(
        receipt.outcome(),
        UiScrollAnchorReconciliationOutcome::Dropped
    );
    assert_eq!(receipt.offset(), UiScrollOffset::origin());
}

#[test]
fn clamp_without_a_new_anchor_retains_the_last_observed_anchor_for_later_rebase() {
    let owner = UiScrollOwnerIdentity::viewport(surface());
    let first = incarnation(7);
    let second = incarnation(8);
    let third = incarnation(9);
    let first_binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let third_binding = worth_ui_host_contract::UiSurfaceBindingGeneration::mint_unbound().unwrap();
    let identity = UiScrollAnchorIdentity::application_item(application_item(10));
    let old_anchor = UiScrollAnchor::new(identity, first_binding, 0, 200).unwrap();
    let new_anchor = UiScrollAnchor::new(identity, third_binding, 0, 260).unwrap();
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                first,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::new(0, 100).unwrap(),
            ),
            Some(old_anchor),
            UiScrollAnchorPolicy::Rebase,
        ))
        .unwrap();

    let clamped = state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                second,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::origin(),
            ),
            None,
            UiScrollAnchorPolicy::Clamp,
        ))
        .unwrap();
    assert_eq!(
        clamped.outcome(),
        UiScrollAnchorReconciliationOutcome::Clamped
    );
    assert_eq!(clamped.offset(), UiScrollOffset::new(0, 100).unwrap());

    let rebased = state
        .reconcile_rebind(UiScrollRebindRequest::new(
            UiScrollOwnerRegistration::new(
                owner,
                third,
                UiScrollAxes::Block,
                bounds(0, 500),
                UiScrollOffset::origin(),
            ),
            Some(new_anchor),
            UiScrollAnchorPolicy::Rebase,
        ))
        .unwrap();
    assert_eq!(
        rebased.outcome(),
        UiScrollAnchorReconciliationOutcome::Rebased
    );
    assert_eq!(rebased.offset(), UiScrollOffset::new(0, 160).unwrap());
}

fn application_item(value: u64) -> crate::runtime::UiApplicationItemKey {
    crate::runtime::UiApplicationItemKey::new(
        crate::runtime::UiApplicationItemKeyFamily::new(core::num::NonZeroU64::new(1).unwrap()),
        core::num::NonZeroU64::new(value).unwrap(),
    )
}

#[test]
fn shutdown_releases_every_scroll_owner() {
    let surface = surface();
    let mut state = UiScrollRuntimeState::new_session_restore_candidate();
    for value in 1..=3 {
        register(
            &mut state,
            UiScrollOwnerIdentity::region(
                surface,
                crate::graph::UiGraphNodeIdentity::new(value),
                value,
            ),
            incarnation(value),
            UiScrollOffset::origin(),
        );
    }
    assert_eq!(state.shutdown(), 3);
    assert_eq!(state.shutdown(), 0);
}
