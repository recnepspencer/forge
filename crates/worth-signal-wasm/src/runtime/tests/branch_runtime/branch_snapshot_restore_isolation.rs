use super::super::support::*;

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
