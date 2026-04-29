use super::support::*;

#[test]
fn signals_phase3_watch_and_nuke_follow_committed_delivery_semantics() {
    let signals = build_signals();
    build_phase3_graph(&signals);

    let notices = Arc::new(Mutex::new(Vec::<WebObservationNotice>::new()));
    let notices_clone = notices.clone();
    let handle: DisposableHandle = signals
        .watch_for_test("panel", move |notice| {
            notices_clone
                .lock()
                .expect("watch notices mutex poisoned")
                .push(notice);
        })
        .unwrap();

    set_signal_value(&signals, "count", 4.0);

    let notices_locked = notices.lock().expect("watch notices mutex poisoned");
    assert_eq!(notices_locked.len(), 1);
    assert_eq!(notices_locked[0].signal_id, "panel");
    assert!(notices_locked[0].meaningful_change);
    drop(notices_locked);

    assert!(signals.nuke(handle));

    set_signal_value(&signals, "count", 9.0);

    assert_eq!(
        notices.lock().expect("watch notices mutex poisoned").len(),
        1
    );
    assert!(
        signals
            .core
            .borrow()
            .latest_observation()
            .unwrap()
            .is_some(),
        "latest observation should still record the committed boundary"
    );
}

#[test]
fn signals_phase3_effect_and_failed_transaction_do_not_create_illegal_delivery() {
    let signals = build_signals();
    build_phase3_graph(&signals);

    let hits = Arc::new(Mutex::new(0usize));
    let hits_clone = hits.clone();
    let handle = signals
        .effect_for_test("panel", move || {
            *hits_clone.lock().expect("effect hits mutex poisoned") += 1;
        })
        .unwrap();

    set_signal_value(&signals, "count", 3.0);
    assert_eq!(*hits.lock().expect("effect hits mutex poisoned"), 1);

    let failed = signals.core.borrow_mut().apply_transaction(vec![
        crate::recipe::model::TransactionOp::Set {
            id: "missing".to_owned(),
            value: SignalValue::Number(5.0),
            aspect: None,
            aspects: None,
        },
    ]);
    assert!(failed.is_err());
    assert_eq!(*hits.lock().expect("effect hits mutex poisoned"), 1);

    assert!(signals.nuke(handle));
}

#[test]
fn signals_phase4_latest_observation_stays_visible_and_nuked_handles_do_not_resurrect_after_branch_churn(
) {
    let signals = build_signals();
    build_phase3_graph(&signals);

    let notices = Arc::new(Mutex::new(Vec::<WebObservationNotice>::new()));
    let notices_clone = notices.clone();
    let handle = signals
        .watch_for_test("panel", move |notice| {
            notices_clone
                .lock()
                .expect("phase4 watch notices mutex poisoned")
                .push(notice);
        })
        .unwrap();

    set_signal_value(&signals, "count", 2.0);
    assert_eq!(
        notices
            .lock()
            .expect("phase4 watch notices mutex poisoned")
            .len(),
        1
    );

    let latest = signals
        .core
        .borrow()
        .latest_observation()
        .unwrap()
        .expect("latest observation should exist after committed watch delivery");
    assert_eq!(latest.observation.boundary_events.len(), 1);
    assert!(latest.observation.boundary_events[0].meaningful_change);
    assert_eq!(latest.observation.boundary_events[0].matched_nodes.len(), 1);

    assert!(signals.nuke(handle));

    let main_branch_id = signals.core.borrow().current_branch().id.0;
    let branch = signals
        .core
        .borrow_mut()
        .create_branch("phase4-observation-branch".to_owned())
        .unwrap();
    signals
        .core
        .borrow_mut()
        .switch_branch(branch.id.0)
        .unwrap();
    set_signal_value(&signals, "count", 7.0);
    signals
        .core
        .borrow_mut()
        .switch_branch(main_branch_id)
        .unwrap();
    set_signal_value(&signals, "count", 8.0);

    assert_eq!(
        notices
            .lock()
            .expect("phase4 watch notices mutex poisoned")
            .len(),
        1,
        "nuked watch handle must not resurrect across branch churn"
    );
}
