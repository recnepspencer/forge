use super::support::*;

#[test]
fn signals_phase4_compatibility_surface_agrees_with_app_first_committed_truth() {
    let signals = build_signals();

    let count = signals
        .input_for_test("count", SignalValue::Number(2.0))
        .unwrap();
    let panel = signals
        .output_for_test(
            "panel",
            OutputSpec {
                reads: vec![RecipeReadSpec::LegacyId("count".to_owned())],
                expr: Expr::Object {
                    fields: vec![(
                        "count".to_owned(),
                        Expr::Read {
                            id: "count".to_owned(),
                        },
                    )],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();

    let app = signals.compatibility_app_public();
    let diagnostics = signals.diagnostics_compat();

    let initial = app.read_for_test("panel").unwrap();
    assert_eq!(initial, panel.read_for_test().unwrap());

    let notices = Arc::new(Mutex::new(Vec::<WebObservationNotice>::new()));
    let notices_clone = notices.clone();
    let _handle = signals
        .watch_for_test("panel", move |notice| {
            notices_clone
                .lock()
                .expect("compat watch notices mutex poisoned")
                .push(notice);
        })
        .unwrap();

    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .set_for_test(&count, SignalValue::Number(6.0))
        .unwrap();
    signals.apply_transaction_for_test(&builder).unwrap();

    let compatibility_value = app.read_for_test("panel").unwrap();
    assert_eq!(compatibility_value, panel.read_for_test().unwrap());

    let latest_observation = diagnostics.latest_observation_for_test().unwrap();
    assert!(
        latest_observation.is_some(),
        "compatibility diagnostics should surface the same committed observation boundary"
    );
    assert_eq!(
        notices
            .lock()
            .expect("compat watch notices mutex poisoned")
            .len(),
        1
    );
}

#[test]
fn signals_phase4_host_callback_failure_does_not_create_partial_committed_truth() {
    let signals = build_signals();
    build_phase3_graph(&signals);

    let _handle = signals
        .watch_for_test("panel", |_notice| {
            panic!("simulated host callback failure");
        })
        .unwrap();

    set_signal_value(&signals, "count", 5.0);

    assert_eq!(
        signals
            .core
            .borrow_mut()
            .read_value("panel")
            .expect("panel value should still commit"),
        SignalValue::Object(vec![
            ("count".to_owned(), SignalValue::Number(5.0)),
            ("double".to_owned(), SignalValue::Number(10.0)),
        ])
    );

    let summary = signals.core.borrow().web_performance_summary();
    assert_eq!(summary.delivered_observation_count, 1);
    assert_eq!(summary.js_callback_failure_count, 1);
}

#[test]
fn signals_phase4_performance_summary_exposes_web_cert_surface() {
    let signals = build_signals();
    build_phase3_graph(&signals);

    let watch_handle = signals.watch_for_test("panel", |_| {}).unwrap();
    let _effect_handle = signals.effect_for_test("panel", || {}).unwrap();

    let compatibility_app = signals.compatibility_app_public();
    let _ = compatibility_app.read_for_test("panel").unwrap();
    let compatibility_runtime = signals.compatibility_runtime_public();
    let _ = compatibility_runtime.read_for_test("panel").unwrap();

    set_signal_value(&signals, "count", 4.0);

    let summary = signals.core.borrow().web_performance_summary();
    assert_eq!(summary.active_handle_count, 2);
    assert_eq!(summary.active_callback_count, 2);
    assert!(summary.matched_watcher_breadth >= 2);
    assert!(summary.delivered_observation_count >= 2);
    assert_eq!(summary.rollback_suppressed_delivery_count, 0);
    assert!(summary.output_serialization_count >= 1);
    assert!(summary.output_serialization_breadth >= 3);
    assert!(summary.js_callback_invocation_count >= 2);
    assert!(summary.compatibility_read_count >= 2);
    assert!(summary.compatibility_read_breadth >= 2);

    assert!(signals.nuke(watch_handle));
    assert_eq!(
        signals
            .core
            .borrow()
            .web_performance_summary()
            .active_handle_count,
        1
    );
}

#[test]
fn app_first_aspect_scoped_transaction_methods_preserve_node_level_observation_semantics() {
    let signals = build_signals();

    signals
        .core
        .borrow_mut()
        .define_web_input(
            "count".to_owned(),
            SignalValue::Number(1.0),
            Some(signals_model::InputOptions {
                produces_aspects: Some(vec![1, 2]),
            }),
        )
        .unwrap();

    let count = InputSignal {
        core: signals.core.clone(),
        id: "count".to_owned(),
    };
    let panel = signals
        .output_for_test(
            "panel",
            OutputSpec {
                reads: vec![RecipeReadSpec::Signal(
                    crate::recipe::model::RecipeReadSignalSpec {
                        id: "count".to_owned(),
                        scope: None,
                        aspects: crate::recipe::model::AspectSelectionSpec {
                            aspect: Some(1),
                            aspects: None,
                        },
                    },
                )],
                expr: Expr::Read {
                    id: "count".to_owned(),
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();

    assert_eq!(panel.read_for_test().unwrap(), SignalValue::Number(1.0));

    let notices = Arc::new(Mutex::new(Vec::<WebObservationNotice>::new()));
    let notices_clone = notices.clone();
    let _watch = signals
        .watch_for_test("panel", move |notice| {
            notices_clone
                .lock()
                .expect("aspect notices mutex poisoned")
                .push(notice);
        })
        .unwrap();

    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .ops
        .borrow_mut()
        .push(crate::recipe::model::TransactionOp::Set {
            id: count.id.clone(),
            value: SignalValue::Number(9.0),
            aspect: None,
            aspects: Some(vec![2]),
        });
    signals.apply_transaction_for_test(&builder).unwrap();

    assert_eq!(
        panel.read_for_test().unwrap(),
        SignalValue::Number(1.0),
        "app-first aspect-targeted writes should not force node-level observers to fire on unread aspects"
    );
    assert!(notices
        .lock()
        .expect("aspect notices mutex poisoned")
        .is_empty());

    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .ops
        .borrow_mut()
        .push(crate::recipe::model::TransactionOp::SetManyWithRegions {
            values: vec![crate::recipe::model::SetValueWithRegions {
                id: count.id.clone(),
                value: SignalValue::Number(11.0),
                changed_regions: Vec::new(),
                aspect: None,
                aspects: Some(vec![1]),
            }],
        });
    signals.apply_transaction_for_test(&builder).unwrap();

    assert_eq!(panel.read_for_test().unwrap(), SignalValue::Number(11.0));
    assert_eq!(
        notices.lock().expect("aspect notices mutex poisoned").len(),
        1
    );
}
