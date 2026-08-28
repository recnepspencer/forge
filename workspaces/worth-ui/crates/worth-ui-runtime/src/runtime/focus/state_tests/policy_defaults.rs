use super::*;

#[test]
fn public_focus_defaults_disable_portal_restoration_and_reveal_work() {
    let world = World::new(2);
    let policy = crate::declaration::UiFocusPolicy::workbench()
        .with_scope_restoration(false)
        .with_focus_reveal(false);
    let mut state = UiFocusRuntimeState::new_session_restore_candidate_with_policy(policy);
    state
        .reconcile_mounted_participation(&world.snapshot)
        .unwrap();
    state
        .commit(state.plan(first_request(world.scope)).unwrap())
        .unwrap();
    let portal_owner = worth_ui_host_contract::UiMountedInstanceIdentity::mint_unbound().unwrap();
    let scope = crate::runtime::session::service_proposal::
        UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(portal_owner);
    let proposal =
        crate::runtime::session::service_proposal::UiServiceProposalIdentity::for_test(91);
    let staged = crate::runtime::focus::UiStagedFocusServiceProposal::prepare(
        proposal,
        crate::runtime::focus::UiPortalFocusRequirement::new(scope, portal_owner, true, Vec::new()),
    );

    state
        .stage_portal_proposal(&staged, world.snapshot.clone())
        .unwrap();

    assert!(state
        .staged_portal_reveal_requirement(proposal)
        .unwrap()
        .is_none());
    assert!(state.pending_portal[&proposal].restoration().is_none());
}
