use super::super::support::*;

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
    assert_eq!(result.source_branch, feature_branch.id.0);
    assert_eq!(result.target_branch, main_branch.id.0);
}
