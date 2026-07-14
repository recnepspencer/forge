use super::super::support::*;

#[test]
fn merge_preserves_non_overlapping_source_edits_when_recipe_materializes_combined_object() {
    let mut runtime = runtime_with_gear_dimensions_model();

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
                record.source_node.clone(),
                format!("{:?}", record.target_node),
                record.action.clone(),
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
    assert_eq!(result.source_branch, feature_branch.id.0);
    assert_eq!(result.target_branch, main_branch.id.0);
}

#[test]
fn merge_keeps_target_only_source_value_when_combined_recipe_was_not_read_after_edits() {
    let mut runtime = runtime_with_gear_dimensions_model();

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
                record.source_node.clone(),
                format!("{:?}", record.target_node),
                record.action.clone(),
            ))
            .collect::<Vec<_>>()
    );
}

fn runtime_with_gear_dimensions_model() -> RuntimeCore {
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

    runtime
}
