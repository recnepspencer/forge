use super::*;

#[test]
fn prepared_reconciliation_is_effect_free_until_exact_commit() {
    let world = World::new(2);
    let mut state = UiFocusRuntimeState::new_session_restore_candidate();
    state
        .reconcile_mounted_participation(&world.snapshot)
        .unwrap();
    let current = state
        .commit(
            state
                .plan(UiFocusRequest::Direct {
                    scope: world.scope,
                    participant: world.identities[1].0,
                    incarnation: world.identities[1].1,
                    cause: UiFocusCause::Direct,
                })
                .unwrap(),
        )
        .unwrap()
        .current()
        .unwrap();
    let successor = world.with_reincarnated_participant(1);

    let prepared = state
        .prepare_mounted_reconciliation(&successor.snapshot)
        .unwrap();
    assert_eq!(state.inspect().current(), Some(current));

    let committed = state.commit_mounted_reconciliation(prepared).unwrap();
    let fallback = committed.transition().unwrap();
    assert_eq!(fallback.cause(), UiFocusCause::RebindFallback);
    assert_eq!(
        fallback.current().unwrap().participant(),
        successor.identities[0].0
    );
    assert_eq!(committed.participants_installed(), 2);
    assert_eq!(committed.mounted_nodes_visited(), 2);
}
