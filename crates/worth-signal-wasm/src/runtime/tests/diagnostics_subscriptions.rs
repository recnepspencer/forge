use super::support::*;

#[test]
fn diagnostics_subscribers_receive_runtime_owned_notifications_for_direct_transactions() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_web_input("count".to_owned(), SignalValue::Number(1.0), None)
        .unwrap();

    let hits = Rc::new(RefCell::new(0usize));
    let hits_for_callback = hits.clone();
    let token = runtime.register_native_diagnostics_callback(Box::new(move || {
        *hits_for_callback.borrow_mut() += 1;
    }));

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "count".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(*hits.borrow(), 1);
    assert!(runtime.dispose_diagnostics_callback(token));
}

#[test]
fn stale_diagnostics_subscription_tokens_cannot_dispose_reused_slots() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();

    let first = runtime.register_native_diagnostics_callback(Box::new(|| {}));
    assert!(runtime.dispose_diagnostics_callback(first));

    let hits = Rc::new(RefCell::new(0usize));
    let hits_for_callback = hits.clone();
    let second = runtime.register_native_diagnostics_callback(Box::new(move || {
        *hits_for_callback.borrow_mut() += 1;
    }));

    assert_eq!(first.slot, second.slot);
    assert_ne!(first.generation, second.generation);
    assert!(!runtime.dispose_diagnostics_callback(first));

    runtime.notify_diagnostics_subscribers();
    assert_eq!(*hits.borrow(), 1);
    assert!(runtime.dispose_diagnostics_callback(second));
}
