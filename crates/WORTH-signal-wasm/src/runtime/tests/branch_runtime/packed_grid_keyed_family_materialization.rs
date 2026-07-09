use super::super::support::*;

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
