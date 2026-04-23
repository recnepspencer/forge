use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use crate::expression::model::{Expr, SignalValue};
use crate::recipe::model::RecipeReadSpec;
use crate::runtime::core::{new_shared_core, WebSignalKind};
use crate::runtime::policy::RuntimePolicySpec;
use crate::runtime::web_callbacks::WebObservationNotice;

use super::signals_model::{ComputedSpec, OutputSpec};
use super::types::{DisposableHandle, InputSignal, Signals, SignalsTransaction};

fn build_signals() -> Signals {
    Signals {
        core: new_shared_core(RuntimePolicySpec::default()).unwrap(),
    }
}

fn build_phase3_graph(signals: &Signals) {
    let _count = signals
        .input_for_test("count", SignalValue::Number(1.0))
        .unwrap();
    let _double = signals
        .computed_for_test(
            "double",
            ComputedSpec {
                reads: vec![RecipeReadSpec::LegacyId("count".to_owned())],
                expr: Expr::Multiply {
                    args: vec![
                        Expr::Read {
                            id: "count".to_owned(),
                        },
                        Expr::Value {
                            value: SignalValue::Number(2.0),
                        },
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();
    let _panel = signals
        .output_for_test(
            "panel",
            OutputSpec {
                reads: vec![
                    RecipeReadSpec::LegacyId("count".to_owned()),
                    RecipeReadSpec::LegacyId("double".to_owned()),
                ],
                expr: Expr::Object {
                    fields: vec![
                        (
                            "count".to_owned(),
                            Expr::Read {
                                id: "count".to_owned(),
                            },
                        ),
                        (
                            "double".to_owned(),
                            Expr::Read {
                                id: "double".to_owned(),
                            },
                        ),
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();
}

fn set_signal_value(signals: &Signals, id: &str, value: f64) {
    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .ops
        .borrow_mut()
        .push(crate::recipe::model::TransactionOp::Set {
            id: id.to_owned(),
            value: SignalValue::Number(value),
            aspect: None,
            aspects: None,
        });
    signals.apply_transaction_for_test(&builder).unwrap();
}

#[test]
fn signals_phase2_input_computed_output_transaction_surface_round_trips_values() {
    let signals = build_signals();

    let count = signals
        .input_for_test("count", SignalValue::Number(2.0))
        .unwrap();

    let doubled = signals
        .computed_for_test(
            "doubled",
            ComputedSpec {
                reads: vec![RecipeReadSpec::LegacyId("count".to_owned())],
                expr: Expr::Multiply {
                    args: vec![
                        Expr::Read {
                            id: "count".to_owned(),
                        },
                        Expr::Value {
                            value: SignalValue::Number(2.0),
                        },
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();

    let panel = signals
        .output_for_test(
            "panel",
            OutputSpec {
                reads: vec![
                    RecipeReadSpec::LegacyId("count".to_owned()),
                    RecipeReadSpec::LegacyId("doubled".to_owned()),
                ],
                expr: Expr::Object {
                    fields: vec![
                        (
                            "count".to_owned(),
                            Expr::Read {
                                id: "count".to_owned(),
                            },
                        ),
                        (
                            "doubled".to_owned(),
                            Expr::Read {
                                id: "doubled".to_owned(),
                            },
                        ),
                    ],
                },
                when: None,
                identity: None,
                produces_aspects: None,
            },
        )
        .unwrap();

    assert_eq!(count.read_for_test().unwrap(), SignalValue::Number(2.0));
    assert_eq!(doubled.read_for_test().unwrap(), SignalValue::Number(4.0));
    assert_eq!(
        panel.read_for_test().unwrap(),
        SignalValue::Object(vec![
            ("count".to_owned(), SignalValue::Number(2.0)),
            ("doubled".to_owned(), SignalValue::Number(4.0)),
        ])
    );

    assert_eq!(
        signals.core.borrow().web_signal_kind("panel"),
        Some(WebSignalKind::Output)
    );

    let builder = SignalsTransaction {
        core: signals.core.clone(),
        ops: Rc::new(RefCell::new(Vec::new())),
    };
    builder
        .set_for_test(&count, SignalValue::Number(5.0))
        .unwrap();
    signals.apply_transaction_for_test(&builder).unwrap();

    assert_eq!(doubled.read_for_test().unwrap(), SignalValue::Number(10.0));
    assert_eq!(
        panel.read_for_test().unwrap(),
        SignalValue::Object(vec![
            ("count".to_owned(), SignalValue::Number(5.0)),
            ("doubled".to_owned(), SignalValue::Number(10.0)),
        ])
    );

    assert_eq!(
        signals.core.borrow_mut().read_value("panel").unwrap(),
        SignalValue::Object(vec![
            ("count".to_owned(), SignalValue::Number(5.0)),
            ("doubled".to_owned(), SignalValue::Number(10.0)),
        ])
    );
}

#[test]
fn signals_phase2_core_tracks_distinct_web_signal_kinds() {
    let mut core = crate::runtime::core::RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    core.define_web_input("count".to_owned(), SignalValue::Number(1.0), None)
        .unwrap();
    core.define_web_computed(
        "double".to_owned(),
        ComputedSpec {
            reads: vec![RecipeReadSpec::LegacyId("count".to_owned())],
            expr: Expr::Multiply {
                args: vec![
                    Expr::Read {
                        id: "count".to_owned(),
                    },
                    Expr::Value {
                        value: SignalValue::Number(2.0),
                    },
                ],
            },
            when: None,
            identity: None,
            produces_aspects: None,
        }
        .into_recipe("double".to_owned()),
    )
    .unwrap();
    core.define_web_output(
        "panel".to_owned(),
        OutputSpec {
            reads: vec![RecipeReadSpec::LegacyId("double".to_owned())],
            expr: Expr::Read {
                id: "double".to_owned(),
            },
            when: None,
            identity: None,
            produces_aspects: None,
        }
        .into_recipe("panel".to_owned()),
    )
    .unwrap();

    assert_eq!(core.web_signal_kind("count"), Some(WebSignalKind::Input));
    assert_eq!(
        core.web_signal_kind("double"),
        Some(WebSignalKind::Computed)
    );
    assert_eq!(core.web_signal_kind("panel"), Some(WebSignalKind::Output));
}

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
    assert_eq!(latest.boundary_events.len(), 1);
    assert!(latest.boundary_events[0].meaningful_change);
    assert_eq!(latest.boundary_events[0].matched_nodes.len(), 1);

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
            Some(super::signals_model::InputOptions {
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
