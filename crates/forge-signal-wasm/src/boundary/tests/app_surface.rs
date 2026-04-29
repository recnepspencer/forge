use super::support::*;

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
fn signals_phase2_explicit_spec_aliases_match_legacy_ast_surface() {
    let signals = build_signals();

    let count = signals
        .input_for_test("count", SignalValue::Number(2.0))
        .unwrap();

    let doubled = signals
        .computed_spec_for_test(
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
        .output_spec_for_test(
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
