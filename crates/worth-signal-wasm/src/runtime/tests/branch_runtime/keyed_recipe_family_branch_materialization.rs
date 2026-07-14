use super::super::support::*;

#[test]
fn keyed_recipe_family_handles_survive_branch_switches_with_divergent_materialization() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(8.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "gearToothIndex".to_owned(),
            initial: SignalValue::Number(0.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "gearToothModel".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Signal {
                    id: "gearTeeth".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "gearToothIndex".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
            ],
            expr: Expr::Object {
                fields: vec![
                    (
                        "index".to_owned(),
                        Expr::Read {
                            id: "gearToothIndex".to_owned(),
                        },
                    ),
                    (
                        "toothCount".to_owned(),
                        Expr::Read {
                            id: "gearTeeth".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();

    runtime
        .set_keyed_values(
            "gearToothIndex",
            (0..8)
                .map(|index| KeyedSetValue {
                    key: format!("tooth-{index}"),
                    value: SignalValue::Number(index as f64),
                    aspect: None,
                    aspects: None,
                })
                .collect(),
        )
        .unwrap();
    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    runtime
        .set_keyed_values(
            "gearToothIndex",
            (0..32)
                .map(|index| KeyedSetValue {
                    key: format!("tooth-{index}"),
                    value: SignalValue::Number(index as f64),
                    aspect: None,
                    aspects: None,
                })
                .collect(),
        )
        .unwrap();
    let feature_tooth = runtime
        .read_keyed_value("gearToothModel", "tooth-31")
        .unwrap();
    assert_eq!(
        feature_tooth,
        SignalValue::Object(vec![
            ("index".to_owned(), SignalValue::Number(31.0)),
            ("toothCount".to_owned(), SignalValue::Number(32.0)),
        ])
    );

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(8.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    let main_tooth = runtime
        .read_keyed_value("gearToothModel", "tooth-0")
        .unwrap();
    assert_eq!(
        main_tooth,
        SignalValue::Object(vec![
            ("index".to_owned(), SignalValue::Number(0.0)),
            ("toothCount".to_owned(), SignalValue::Number(8.0)),
        ])
    );

    runtime.switch_branch(feature_branch.id.0).unwrap();
    let feature_tooth_again = runtime
        .read_keyed_value("gearToothModel", "tooth-31")
        .unwrap();
    assert_eq!(feature_tooth_again, feature_tooth);
}
