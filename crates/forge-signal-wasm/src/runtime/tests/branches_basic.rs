use super::support::*;

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
