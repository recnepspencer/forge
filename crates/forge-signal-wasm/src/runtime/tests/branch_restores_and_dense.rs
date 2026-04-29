use super::support::*;

#[test]
fn restoring_inactive_branch_snapshot_then_editing_other_field_keeps_branch_local_source_values() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(8.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    let main_snapshot = runtime.branch_snapshot_id(main_branch.id.0).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    let feature_snapshot = runtime.branch_snapshot_id(feature_branch.id.0).unwrap();

    runtime
        .restore_branch_snapshot_by_id(main_branch.id.0, main_snapshot)
        .unwrap();
    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearThickness".to_owned(),
            value: SignalValue::Number(0.1),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(8.0),
        "restoring main then editing thickness must not inherit feature teeth"
    );
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.1)
    );

    runtime
        .restore_branch_snapshot_by_id(feature_branch.id.0, feature_snapshot)
        .unwrap();
    runtime.switch_branch(feature_branch.id.0).unwrap();
    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(32.0),
        "feature branch teeth should remain isolated after main restore/edit"
    );
}

#[test]
fn packed_dense_grid_updates_are_readable_through_keyed_family_surface() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "pixel".to_owned(),
            initial: SignalValue::Object(vec![
                ("r".to_owned(), SignalValue::Number(0.0)),
                ("g".to_owned(), SignalValue::Number(0.0)),
                ("b".to_owned(), SignalValue::Number(0.0)),
                ("a".to_owned(), SignalValue::Number(255.0)),
            ]),
            produces_aspects: None,
        })
        .unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::SetPackedGridRgba {
            family_id: "pixel".to_owned(),
            width: 2,
            height: 1,
            rgba: vec![10, 20, 30, 255, 40, 50, 60, 255],
        }])
        .unwrap();

    let left = runtime.read_keyed_value("pixel", "0,0").unwrap();
    let right = runtime.read_keyed_value("pixel", "1,0").unwrap();

    assert_eq!(
        left,
        SignalValue::Object(vec![
            ("r".to_owned(), SignalValue::Number(10.0)),
            ("g".to_owned(), SignalValue::Number(20.0)),
            ("b".to_owned(), SignalValue::Number(30.0)),
            ("a".to_owned(), SignalValue::Number(255.0)),
        ])
    );
    assert_eq!(
        right,
        SignalValue::Object(vec![
            ("r".to_owned(), SignalValue::Number(40.0)),
            ("g".to_owned(), SignalValue::Number(50.0)),
            ("b".to_owned(), SignalValue::Number(60.0)),
            ("a".to_owned(), SignalValue::Number(255.0)),
        ])
    );
}

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
#[test]
fn branch_state_proof_is_versioned_and_stable_for_unchanged_branch_state() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
            produces_aspects: None,
        })
        .unwrap();

    let branch = runtime.current_branch();
    let left = runtime.branch_state_proof(branch.id.0).unwrap();
    let right = runtime.branch_state_proof(branch.id.0).unwrap();

    assert!(left
        .proof_schema_version
        .starts_with(forge_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION));
    assert_eq!(left.proof_schema_version, right.proof_schema_version);
    assert_eq!(left.branch_id, right.branch_id);
    assert_eq!(left.state_digest, right.state_digest);
}
