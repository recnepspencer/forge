use worth_store_recovery_runtime::PhysicalRecoveryOutcome;

#[test]
fn recovered_session_evidence_is_local_despite_an_unrelated_nonterminal_drop() {
    let dropped_root = super::prepare_ordinary_recovery_root("c8-phase6-unrelated-drop");
    let dropped = super::selected_ordinary_recovery(dropped_root.path());
    let drop_thread = std::thread::spawn(move || drop(dropped));

    let recovered_root = super::prepare_ordinary_recovery_root("c8-phase6-local-terminal");
    let admitted = super::admitted_recovery_with_limits(
        recovered_root.path(),
        super::ordinary_recovery_limits(4_096),
    );
    let expected_session = admitted.session_identity();
    let reopened = admitted
        .discover()
        .unwrap()
        .select()
        .unwrap()
        .plan()
        .unwrap()
        .stage()
        .unwrap()
        .publish()
        .unwrap()
        .reopen()
        .unwrap();
    drop_thread.join().expect("unrelated drop thread");

    let PhysicalRecoveryOutcome::Recovered(handoff) = reopened.finish() else {
        panic!("the consumed local session must retain its recovered terminal")
    };
    assert_eq!(
        handoff.publication_expectation().recovered_root(),
        handoff.core().root()
    );
    assert_eq!(handoff.recovered_session_identity(), expected_session);
}
