use super::support::*;

#[test]
fn collection_and_object_operators_work_on_runtime_values() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "items".to_owned(),
            initial: SignalValue::Array(vec![
                SignalValue::String("alpha".to_owned()),
                SignalValue::String("beta".to_owned()),
                SignalValue::String("gamma".to_owned()),
            ]),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "profile".to_owned(),
            initial: SignalValue::Object(vec![
                ("name".to_owned(), SignalValue::String("forge".to_owned())),
                ("role".to_owned(), SignalValue::String("signal".to_owned())),
                ("tier".to_owned(), SignalValue::String("runtime".to_owned())),
            ]),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "operators".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("items".to_owned()),
                RecipeReadSpec::LegacyId("profile".to_owned()),
            ],
            expr: Expr::Object {
                fields: vec![
                    (
                        "first".to_owned(),
                        Expr::First {
                            target: Box::new(read("items")),
                        },
                    ),
                    (
                        "last".to_owned(),
                        Expr::Last {
                            target: Box::new(read("items")),
                        },
                    ),
                    (
                        "middle".to_owned(),
                        Expr::Slice {
                            target: Box::new(read("items")),
                            start: Box::new(number(1.0)),
                            end: Some(Box::new(number(2.0))),
                        },
                    ),
                    (
                        "joined".to_owned(),
                        Expr::Join {
                            target: Box::new(read("items")),
                            separator: Box::new(Expr::Value {
                                value: SignalValue::String(",".to_owned()),
                            }),
                        },
                    ),
                    (
                        "flat".to_owned(),
                        Expr::Flatten {
                            target: Box::new(Expr::Array {
                                items: vec![
                                    Expr::Array {
                                        items: vec![number(1.0), number(2.0)],
                                    },
                                    Expr::Array {
                                        items: vec![number(3.0)],
                                    },
                                ],
                            }),
                        },
                    ),
                    (
                        "picked".to_owned(),
                        Expr::Pick {
                            target: Box::new(read("profile")),
                            fields: vec!["name".to_owned(), "tier".to_owned()],
                        },
                    ),
                    (
                        "omitted".to_owned(),
                        Expr::Omit {
                            target: Box::new(read("profile")),
                            fields: vec!["role".to_owned()],
                        },
                    ),
                ],
            },
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();

    let value = runtime.read_value("operators").unwrap();
    assert_eq!(
        value,
        SignalValue::Object(vec![
            ("first".to_owned(), SignalValue::String("alpha".to_owned())),
            ("last".to_owned(), SignalValue::String("gamma".to_owned())),
            (
                "middle".to_owned(),
                SignalValue::Array(vec![SignalValue::String("beta".to_owned())])
            ),
            (
                "joined".to_owned(),
                SignalValue::String("alpha,beta,gamma".to_owned())
            ),
            (
                "flat".to_owned(),
                SignalValue::Array(vec![
                    SignalValue::Number(1.0),
                    SignalValue::Number(2.0),
                    SignalValue::Number(3.0)
                ])
            ),
            (
                "picked".to_owned(),
                SignalValue::Object(vec![
                    ("name".to_owned(), SignalValue::String("forge".to_owned())),
                    ("tier".to_owned(), SignalValue::String("runtime".to_owned()))
                ])
            ),
            (
                "omitted".to_owned(),
                SignalValue::Object(vec![
                    ("name".to_owned(), SignalValue::String("forge".to_owned())),
                    ("tier".to_owned(), SignalValue::String("runtime".to_owned()))
                ])
            )
        ])
    );
}

#[test]
fn transaction_updates_recipe_values_and_versions() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "price".to_owned(),
            initial: SignalValue::Number(100.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "tax".to_owned(),
            initial: SignalValue::Number(20.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "total".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("price".to_owned()),
                RecipeReadSpec::LegacyId("tax".to_owned()),
            ],
            expr: Expr::Sum {
                args: vec![read("price"), read("tax")],
            },
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();

    let first = runtime.read_value("total").unwrap();
    assert_eq!(first, SignalValue::Number(120.0));

    let summary = runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "price".to_owned(),
            value: SignalValue::Number(110.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert!(summary.nodes_evaluated >= 1);

    let second = runtime.read_value("total").unwrap();
    assert_eq!(second, SignalValue::Number(130.0));

    let versions = runtime.read_versions(vec!["total".to_owned()]).unwrap();
    assert_eq!(versions[0].version, 2);
}

#[test]
fn snapshot_envelope_round_trip_restores_runtime_state() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "left".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "right".to_owned(),
            initial: SignalValue::Number(3.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "product".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("left".to_owned()),
                RecipeReadSpec::LegacyId("right".to_owned()),
            ],
            expr: Expr::Multiply {
                args: vec![read("left"), read("right")],
            },
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();

    let _ = runtime.read_value("product").unwrap();
    let envelope = runtime.export_runtime_envelope().unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "left".to_owned(),
            value: SignalValue::Number(10.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("product").unwrap(),
        SignalValue::Number(30.0)
    );

    runtime.replace_runtime_envelope(envelope).unwrap();
    assert_eq!(
        runtime.read_value("product").unwrap(),
        SignalValue::Number(6.0)
    );
}
