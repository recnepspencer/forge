use super::adapters::RuntimeEnvelope;
use super::core::RuntimeCore;
use super::policy::RuntimePolicySpec;
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeFamilyReadSpec, RecipeSpec,
    SourceSpec, TransactionOp,
};

fn number(value: f64) -> Expr {
    Expr::Value {
        value: SignalValue::Number(value),
    }
}

fn read(id: &str) -> Expr {
    Expr::Read { id: id.to_owned() }
}

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
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "operators".to_owned(),
            reads: vec!["items".to_owned(), "profile".to_owned()],
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
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "tax".to_owned(),
            initial: SignalValue::Number(20.0),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "total".to_owned(),
            reads: vec!["price".to_owned(), "tax".to_owned()],
            expr: Expr::Sum {
                args: vec![read("price"), read("tax")],
            },
            when: None,
            identity: None,
        })
        .unwrap();

    let first = runtime.read_value("total").unwrap();
    assert_eq!(first, SignalValue::Number(120.0));

    let summary = runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "price".to_owned(),
            value: SignalValue::Number(110.0),
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
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "right".to_owned(),
            initial: SignalValue::Number(3.0),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "product".to_owned(),
            reads: vec!["left".to_owned(), "right".to_owned()],
            expr: Expr::Multiply {
                args: vec![read("left"), read("right")],
            },
            when: None,
            identity: None,
        })
        .unwrap();

    let _ = runtime.read_value("product").unwrap();
    let envelope = runtime.export_runtime_envelope().unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "left".to_owned(),
            value: SignalValue::Number(10.0),
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

#[test]
fn exported_envelope_can_seed_a_fresh_runtime() {
    let mut original = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    original
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(7.0),
        })
        .unwrap();
    original
        .define_recipe(RecipeSpec {
            id: "plusOne".to_owned(),
            reads: vec!["base".to_owned()],
            expr: Expr::Sum {
                args: vec![read("base"), number(1.0)],
            },
            when: None,
            identity: None,
        })
        .unwrap();
    let _ = original.read_value("plusOne").unwrap();

    let envelope: RuntimeEnvelope = original.export_runtime_envelope().unwrap();

    let mut restored = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    restored.replace_runtime_envelope(envelope).unwrap();

    assert_eq!(
        restored.read_value("plusOne").unwrap(),
        SignalValue::Number(8.0)
    );
    assert!(!restored.replay_for_id("plusOne").unwrap().frames.is_empty());
    assert!(!restored
        .lineage_for_id("plusOne")
        .unwrap()
        .events
        .is_empty());
}

#[test]
fn keyed_families_expand_and_recompute() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "price".to_owned(),
            initial: SignalValue::Number(0.0),
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "tax".to_owned(),
            initial: SignalValue::Number(0.0),
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "total".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Keyed {
                    family_id: "price".to_owned(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "tax".to_owned(),
                },
            ],
            expr: Expr::Sum {
                args: vec![read("price"), read("tax")],
            },
            when: None,
            identity: None,
        })
        .unwrap();

    runtime
        .set_keyed_value("price", "cart-1", SignalValue::Number(100.0))
        .unwrap();
    runtime
        .set_keyed_value("tax", "cart-1", SignalValue::Number(8.0))
        .unwrap();

    let value = runtime.read_keyed_value("total", "cart-1").unwrap();
    assert_eq!(value, SignalValue::Number(108.0));
}

#[test]
fn keyed_families_can_mix_shared_and_keyed_reads() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "exposure".to_owned(),
            initial: SignalValue::Number(3.0),
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "pixelBase".to_owned(),
            initial: SignalValue::Number(0.0),
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "pixel".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Signal {
                    id: "exposure".to_owned(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "pixelBase".to_owned(),
                },
            ],
            expr: Expr::Sum {
                args: vec![read("pixelBase"), read("exposure")],
            },
            when: None,
            identity: None,
        })
        .unwrap();

    runtime
        .set_keyed_value("pixelBase", "10,5", SignalValue::Number(7.0))
        .unwrap();

    let value = runtime.read_keyed_value("pixel", "10,5").unwrap();
    assert_eq!(value, SignalValue::Number(10.0));
}

#[test]
fn branches_can_be_created_and_switched() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();
    assert_ne!(main_branch.id, feature_branch.id);

    runtime.switch_branch(feature_branch.id.0).unwrap();
    let active = runtime.current_branch();
    assert_eq!(active.id, feature_branch.id);

    let replay = runtime.replay_for_branch(feature_branch.id.0).unwrap();
    assert!(!replay.frames.is_empty());
}

#[test]
fn merge_plan_and_result_are_available_through_history_surface() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "counter".to_owned(),
            value: SignalValue::Number(2.0),
        }])
        .unwrap();

    let plan = runtime
        .plan_merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();
    assert_eq!(plan.source_branch_id(), feature_branch.id);
    assert_eq!(plan.target_branch_id(), main_branch.id);

    let result = runtime
        .merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();
    assert_eq!(result.source_branch, feature_branch.id);
    assert_eq!(result.target_branch, main_branch.id);
}

#[test]
fn merge_preserves_non_overlapping_source_edits_from_both_branches() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.22),
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(20.0),
        }])
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearThickness".to_owned(),
            value: SignalValue::Number(0.31),
        }])
        .unwrap();

    let result = runtime
        .merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(20.0)
    );
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.31)
    );
    assert_eq!(result.source_branch, feature_branch.id);
    assert_eq!(result.target_branch, main_branch.id);
}

#[test]
fn merge_preserves_non_overlapping_source_edits_when_recipe_materializes_combined_object() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(8.0),
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "gearDimensionsModel".to_owned(),
            reads: vec!["gearTeeth".to_owned(), "gearThickness".to_owned()],
            expr: Expr::Object {
                fields: vec![
                    (
                        "teeth".to_owned(),
                        Expr::Read {
                            id: "gearTeeth".to_owned(),
                        },
                    ),
                    (
                        "thickness".to_owned(),
                        Expr::Read {
                            id: "gearThickness".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
        })
        .unwrap();

    let _ = runtime.read_value("gearDimensionsModel").unwrap();
    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("gearDimensionsModel").unwrap(),
        SignalValue::Object(vec![
            ("teeth".to_owned(), SignalValue::Number(32.0)),
            ("thickness".to_owned(), SignalValue::Number(0.42)),
        ])
    );

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearThickness".to_owned(),
            value: SignalValue::Number(0.1),
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("gearDimensionsModel").unwrap(),
        SignalValue::Object(vec![
            ("teeth".to_owned(), SignalValue::Number(8.0)),
            ("thickness".to_owned(), SignalValue::Number(0.1)),
        ])
    );

    let result = runtime
        .merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();

    let debug_records = result
        .records
        .iter()
        .map(|record| {
            (
                format!("{:?}", record.source_node),
                format!("{:?}", record.target_node),
                format!("{:?}", record.action),
            )
        })
        .collect::<Vec<_>>();

    runtime.switch_branch(main_branch.id.0).unwrap();
    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(32.0)
    );
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.1),
        "merge records: {:?}",
        debug_records
    );
    assert_eq!(
        runtime.read_value("gearDimensionsModel").unwrap(),
        SignalValue::Object(vec![
            ("teeth".to_owned(), SignalValue::Number(32.0)),
            ("thickness".to_owned(), SignalValue::Number(0.1)),
        ])
    );
    assert_eq!(result.source_branch, feature_branch.id);
    assert_eq!(result.target_branch, main_branch.id);
}

#[test]
fn merge_with_combined_recipe_but_without_post_edit_recipe_reads_keeps_target_only_source_value() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(8.0),
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "gearDimensionsModel".to_owned(),
            reads: vec!["gearTeeth".to_owned(), "gearThickness".to_owned()],
            expr: Expr::Object {
                fields: vec![
                    (
                        "teeth".to_owned(),
                        Expr::Read {
                            id: "gearTeeth".to_owned(),
                        },
                    ),
                    (
                        "thickness".to_owned(),
                        Expr::Read {
                            id: "gearThickness".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
        }])
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearThickness".to_owned(),
            value: SignalValue::Number(0.1),
        }])
        .unwrap();

    let result = runtime
        .merge_branches(feature_branch.id.0, main_branch.id.0)
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.1),
        "merge records: {:?}",
        result
            .records
            .iter()
            .map(|record| (
                format!("{:?}", record.source_node),
                format!("{:?}", record.target_node),
                format!("{:?}", record.action),
            ))
            .collect::<Vec<_>>()
    );
}

#[test]
fn restoring_inactive_branch_snapshot_then_editing_other_field_keeps_branch_local_source_values() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearTeeth".to_owned(),
            initial: SignalValue::Number(16.0),
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.42),
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(8.0),
        }])
        .unwrap();
    let main_snapshot = runtime.branch_snapshot_id(main_branch.id.0).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
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
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "gearToothIndex".to_owned(),
            initial: SignalValue::Number(0.0),
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "gearToothModel".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Signal {
                    id: "gearTeeth".to_owned(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "gearToothIndex".to_owned(),
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
        })
        .unwrap();

    runtime
        .set_keyed_values(
            "gearToothIndex",
            (0..8)
                .map(|index| KeyedSetValue {
                    key: format!("tooth-{index}"),
                    value: SignalValue::Number(index as f64),
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
        }])
        .unwrap();
    runtime
        .set_keyed_values(
            "gearToothIndex",
            (0..32)
                .map(|index| KeyedSetValue {
                    key: format!("tooth-{index}"),
                    value: SignalValue::Number(index as f64),
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
