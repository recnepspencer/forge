use super::adapters::RuntimeEnvelope;
use super::core::RuntimeCore;
use super::policy::{RuntimePolicyPreset, RuntimePolicySpec};
use crate::expression::model::{Expr, IdentitySpec, SignalValue};
use crate::recipe::model::{
    KeyedRecipeFamilySpec, KeyedSetValue, KeyedSourceFamilySpec, RecipeFamilyReadSpec,
    RecipeReadSpec, RecipeSpec, SourceSpec, TransactionOp,
};

fn number(value: f64) -> Expr {
    Expr::Value {
        value: SignalValue::Number(value),
    }
}

fn read(id: &str) -> Expr {
    Expr::Read { id: id.to_owned() }
}

fn build_adversarial_merge_runtime(policy: RuntimePolicySpec) -> (RuntimeCore, u64, u64, String) {
    let mut runtime = RuntimeCore::new(policy).unwrap();
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
            initial: SignalValue::Number(0.22),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearInnerRadius".to_owned(),
            initial: SignalValue::Number(0.28),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "lightIntensity".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "gearTopologyModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTeeth".to_owned()),
                RecipeReadSpec::LegacyId("gearThickness".to_owned()),
                RecipeReadSpec::LegacyId("gearInnerRadius".to_owned()),
            ],
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
                    (
                        "innerRadius".to_owned(),
                        Expr::Read {
                            id: "gearInnerRadius".to_owned(),
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
        .define_recipe(RecipeSpec {
            id: "hudModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTopologyModel".to_owned()),
                RecipeReadSpec::LegacyId("lightIntensity".to_owned()),
            ],
            expr: Expr::Object {
                fields: vec![
                    (
                        "gear".to_owned(),
                        Expr::Read {
                            id: "gearTopologyModel".to_owned(),
                        },
                    ),
                    (
                        "light".to_owned(),
                        Expr::Read {
                            id: "lightIntensity".to_owned(),
                        },
                    ),
                ],
            },
            when: None,
            identity: Some(IdentitySpec::Exact),
            produces_aspects: None,
        })
        .unwrap();

    let _ = runtime.read_value("hudModel").unwrap();
    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![
            TransactionOp::Set {
                id: "gearTeeth".to_owned(),
                value: SignalValue::Number(22.0),
                aspect: None,
                aspects: None,
            },
            TransactionOp::Set {
                id: "lightIntensity".to_owned(),
                value: SignalValue::Number(1.78),
                aspect: None,
                aspects: None,
            },
        ])
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![
            TransactionOp::Set {
                id: "gearThickness".to_owned(),
                value: SignalValue::Number(0.42),
                aspect: None,
                aspects: None,
            },
            TransactionOp::Set {
                id: "gearInnerRadius".to_owned(),
                value: SignalValue::Number(0.36),
                aspect: None,
                aspects: None,
            },
        ])
        .unwrap();

    (
        runtime,
        main_branch.id.0,
        feature_branch.id.0,
        main_branch.name,
    )
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

#[test]
fn exported_envelope_can_seed_a_fresh_runtime() {
    let mut original = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    original
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(7.0),
            produces_aspects: None,
        })
        .unwrap();
    original
        .define_recipe(RecipeSpec {
            id: "plusOne".to_owned(),
            reads: vec![RecipeReadSpec::LegacyId("base".to_owned())],
            expr: Expr::Sum {
                args: vec![read("base"), number(1.0)],
            },
            when: None,
            identity: None,
            produces_aspects: None,
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
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "tax".to_owned(),
            initial: SignalValue::Number(0.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "total".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Keyed {
                    family_id: "price".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "tax".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
            ],
            expr: Expr::Sum {
                args: vec![read("price"), read("tax")],
            },
            when: None,
            identity: None,
            produces_aspects: None,
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
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source_family(KeyedSourceFamilySpec {
            family_id: "pixelBase".to_owned(),
            initial: SignalValue::Number(0.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_keyed_recipe_family(KeyedRecipeFamilySpec {
            family_id: "pixel".to_owned(),
            reads: vec![
                RecipeFamilyReadSpec::Signal {
                    id: "exposure".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
                RecipeFamilyReadSpec::Keyed {
                    family_id: "pixelBase".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec::default(),
                },
            ],
            expr: Expr::Sum {
                args: vec![read("pixelBase"), read("exposure")],
            },
            when: None,
            identity: None,
            produces_aspects: None,
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
            produces_aspects: None,
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
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "counter".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
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
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "gearThickness".to_owned(),
            initial: SignalValue::Number(0.22),
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(20.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    runtime.switch_branch(main_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearThickness".to_owned(),
            value: SignalValue::Number(0.31),
            aspect: None,
            aspects: None,
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
    runtime
        .define_recipe(RecipeSpec {
            id: "gearDimensionsModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTeeth".to_owned()),
                RecipeReadSpec::LegacyId("gearThickness".to_owned()),
            ],
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
            produces_aspects: None,
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
            aspect: None,
            aspects: None,
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
            aspect: None,
            aspects: None,
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
    runtime
        .define_recipe(RecipeSpec {
            id: "gearDimensionsModel".to_owned(),
            reads: vec![
                RecipeReadSpec::LegacyId("gearTeeth".to_owned()),
                RecipeReadSpec::LegacyId("gearThickness".to_owned()),
            ],
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
            produces_aspects: None,
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("what-if".to_owned()).unwrap();

    runtime.switch_branch(feature_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
            aspect: None,
            aspects: None,
        }])
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

#[test]
fn replay_parity_proof_distinguishes_equivalent_and_divergent_branch_states() {
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

    let main = runtime.current_branch();
    let twin = runtime.create_branch("twin".to_owned()).unwrap();

    let parity = runtime.replay_parity_proof(main.id.0, twin.id.0).unwrap();
    assert_eq!(
        parity.proof_schema_version,
        forge_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert!(parity.parity);
    assert_eq!(parity.expected_state_digest, parity.replayed_state_digest);

    runtime.switch_branch(twin.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(32.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let divergent = runtime.replay_parity_proof(main.id.0, twin.id.0).unwrap();
    assert!(!divergent.parity);
    assert_ne!(
        divergent.expected_state_digest,
        divergent.replayed_state_digest
    );
}

#[test]
fn adversarial_merge_proof_envelopes_and_rebuild_state_remain_consistent() {
    let (mut runtime, main_branch_id, feature_branch_id, main_branch_name) =
        build_adversarial_merge_runtime(RuntimePolicySpec::default());

    let plan_envelope = runtime
        .plan_merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    assert_eq!(
        plan_envelope.proof.proof_schema_version,
        forge_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert_eq!(plan_envelope.plan.source_branch_id().0, feature_branch_id);
    assert_eq!(plan_envelope.plan.target_branch_id().0, main_branch_id);

    let result_envelope = runtime
        .merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    assert_eq!(
        result_envelope.proof.proof_schema_version,
        forge_signal::facade::adapters::MERGE_PROOF_SCHEMA_VERSION
    );
    assert_eq!(
        plan_envelope.proof.selected_strategy_digest,
        result_envelope.proof.selected_strategy_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_merge_base_digest,
        result_envelope.proof.selected_merge_base_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_conflict_policy_digest,
        result_envelope.proof.selected_conflict_policy_digest
    );
    assert_eq!(
        plan_envelope.proof.selected_conflict_isolation_digest,
        result_envelope.proof.selected_conflict_isolation_digest
    );
    assert_eq!(result_envelope.result.source_branch.0, feature_branch_id);
    assert_eq!(result_envelope.result.target_branch.0, main_branch_id);

    runtime.switch_branch(main_branch_id).unwrap();
    assert_eq!(
        runtime.read_value("gearTeeth").unwrap(),
        SignalValue::Number(22.0)
    );
    assert_eq!(
        runtime.read_value("gearThickness").unwrap(),
        SignalValue::Number(0.42)
    );
    assert_eq!(
        runtime.read_value("gearInnerRadius").unwrap(),
        SignalValue::Number(0.36)
    );
    assert_eq!(
        runtime.read_value("lightIntensity").unwrap(),
        SignalValue::Number(1.78)
    );

    let merged_proof = runtime.branch_state_proof(main_branch_id).unwrap();
    let envelope = runtime.export_runtime_envelope().unwrap();
    let mut rebuilt = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    rebuilt.replace_runtime_envelope(envelope).unwrap();
    let rebuilt_main_branch = rebuilt
        .branches()
        .into_iter()
        .find(|branch| branch.name == main_branch_name)
        .expect("rebuilt runtime should preserve the merged target branch by name");
    rebuilt.switch_branch(rebuilt_main_branch.id.0).unwrap();
    let rebuilt_proof = rebuilt
        .branch_state_proof(rebuilt_main_branch.id.0)
        .unwrap();

    assert_eq!(
        merged_proof.proof_schema_version,
        rebuilt_proof.proof_schema_version
    );
    assert_eq!(merged_proof.state_digest, rebuilt_proof.state_digest);
    assert_eq!(
        rebuilt.read_value("gearTopologyModel").unwrap(),
        SignalValue::Object(vec![
            ("teeth".to_owned(), SignalValue::Number(22.0)),
            ("thickness".to_owned(), SignalValue::Number(0.42)),
            ("innerRadius".to_owned(), SignalValue::Number(0.36)),
        ])
    );
    assert_eq!(
        rebuilt.read_value("hudModel").unwrap(),
        SignalValue::Object(vec![
            (
                "gear".to_owned(),
                SignalValue::Object(vec![
                    ("teeth".to_owned(), SignalValue::Number(22.0)),
                    ("thickness".to_owned(), SignalValue::Number(0.42)),
                    ("innerRadius".to_owned(), SignalValue::Number(0.36)),
                ]),
            ),
            ("light".to_owned(), SignalValue::Number(1.78)),
        ])
    );
}

#[test]
fn diagnostics_tier_changes_richness_only_not_merge_truth() {
    let development = RuntimePolicySpec {
        preset: RuntimePolicyPreset::WebDevelopment,
    };
    let kernel = RuntimePolicySpec {
        preset: RuntimePolicyPreset::Kernel,
    };

    let (mut development_runtime, development_main, development_feature, _) =
        build_adversarial_merge_runtime(development);
    let (mut kernel_runtime, kernel_main, kernel_feature, _) =
        build_adversarial_merge_runtime(kernel);

    let development_plan = development_runtime
        .plan_merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_plan = kernel_runtime
        .plan_merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();
    assert_eq!(
        development_plan.proof.plan_digest,
        kernel_plan.proof.plan_digest
    );
    assert_eq!(
        development_plan.proof.semantics_digest,
        kernel_plan.proof.semantics_digest
    );

    let development_result = development_runtime
        .merge_branches_with_proof(development_feature, development_main)
        .unwrap();
    let kernel_result = kernel_runtime
        .merge_branches_with_proof(kernel_feature, kernel_main)
        .unwrap();

    assert_eq!(
        development_result.proof.result_digest,
        kernel_result.proof.result_digest
    );
    assert_eq!(
        development_result.result.selected_semantics,
        kernel_result.result.selected_semantics
    );

    let development_state = development_runtime
        .branch_state_proof(development_main)
        .unwrap();
    let kernel_state = kernel_runtime.branch_state_proof(kernel_main).unwrap();
    assert_eq!(development_state.state_digest, kernel_state.state_digest);
}

#[test]
fn aspect_filtered_reads_ignore_irrelevant_aspect_updates() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![1, 2]),
        })
        .unwrap();
    runtime
        .define_recipe(RecipeSpec {
            id: "display".to_owned(),
            reads: vec![RecipeReadSpec::Signal(
                crate::recipe::model::RecipeReadSignalSpec {
                    id: "sensor".to_owned(),
                    scope: None,
                    aspects: crate::recipe::model::AspectSelectionSpec {
                        aspect: Some(1),
                        aspects: None,
                    },
                },
            )],
            expr: read("sensor"),
            when: None,
            identity: None,
            produces_aspects: None,
        })
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(10.0)
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        1
    );

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(99.0),
            aspect: None,
            aspects: Some(vec![2]),
        }])
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(10.0),
        "display should not recompute when only an unread aspect changes"
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        1,
        "unread aspect churn must not advance the derived node version"
    );

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(42.0),
            aspect: None,
            aspects: Some(vec![1]),
        }])
        .unwrap();

    assert_eq!(
        runtime.read_value("display").unwrap(),
        SignalValue::Number(42.0)
    );
    assert_eq!(
        runtime.read_versions(vec!["display".to_owned()]).unwrap()[0].version,
        2
    );
}

#[test]
fn multi_aspect_versions_survive_snapshot_round_trip() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![1, 2]),
        })
        .unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(15.0),
            aspect: None,
            aspects: Some(vec![2]),
        }])
        .unwrap();

    let before = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(before[0].aspect_versions.len(), 2);
    assert_eq!(before[0].aspect_versions[0].aspect, 1);
    assert_eq!(before[0].aspect_versions[0].version, 1);
    assert_eq!(before[0].aspect_versions[1].aspect, 2);
    assert_eq!(before[0].aspect_versions[1].version, 2);

    let snapshot = runtime.snapshot().unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(25.0),
            aspect: None,
            aspects: Some(vec![1]),
        }])
        .unwrap();

    runtime.restore_snapshot(snapshot).unwrap();

    let restored = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(restored[0].aspect_versions, before[0].aspect_versions);
}

#[test]
fn replay_artifact_proof_reports_typed_mismatch_classes() {
    let (mut runtime, main_branch_id, feature_branch_id, _) =
        build_adversarial_merge_runtime(RuntimePolicySpec::default());

    let expected_plan = runtime
        .plan_merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_result = runtime
        .merge_branches_with_proof(feature_branch_id, main_branch_id)
        .unwrap();
    let expected_state = runtime.branch_state_proof(main_branch_id).unwrap();

    let replayed_branch = runtime
        .create_branch("replayed-divergent".to_owned())
        .unwrap();
    runtime.switch_branch(replayed_branch.id.0).unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "gearTeeth".to_owned(),
            value: SignalValue::Number(7.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let report = runtime
        .replay_artifact_proof(
            forge_signal::facade::adapters::ReplayArtifactProofInput {
                proof_schema_version: expected_result.proof.proof_schema_version.clone(),
                registry_bundle_digest: Some(expected_result.proof.registry_bundle_digest.clone()),
                lowered_strategy_bundle_digest: Some(
                    expected_result.proof.lowered_strategy_bundle_digest.clone(),
                ),
                merge_plan_digest: Some(expected_plan.proof.plan_digest.clone()),
                merge_result_digest: Some(expected_result.proof.result_digest.clone()),
                lineage_digest: Some(expected_result.proof.lineage_digest.clone()),
                branch_state_digest: expected_state.state_digest.clone(),
            },
            replayed_branch.id.0,
        )
        .unwrap();

    assert!(!report.parity);
    assert!(report
        .mismatch_classes
        .contains(&forge_signal::facade::adapters::ReplayMismatchClass::BranchStateDigestMismatch));
}
