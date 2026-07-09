use super::super::support::*;

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

#[cfg(feature = "profile-extended")]
#[test]
fn extended_profile_accepts_aspect_slot_fifteen() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "sensor".to_owned(),
            initial: SignalValue::Number(10.0),
            produces_aspects: Some(vec![15]),
        })
        .unwrap();

    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "sensor".to_owned(),
            value: SignalValue::Number(15.0),
            aspect: None,
            aspects: Some(vec![15]),
        }])
        .unwrap();

    let versions = runtime.read_versions(vec!["sensor".to_owned()]).unwrap();
    assert_eq!(versions[0].aspect_versions.len(), 1);
    assert_eq!(versions[0].aspect_versions[0].aspect, 15);
    assert_eq!(versions[0].aspect_versions[0].version, 2);
}
