use super::support::*;

#[test]
fn callback_snapshot_restore_recovers_dependency_shape_for_branchy_callbacks() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "enabled".to_owned(),
            initial: SignalValue::Bool(true),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_source(SourceSpec {
            id: "name".to_owned(),
            initial: SignalValue::String("Ada".to_owned()),
            produces_aspects: None,
        })
        .unwrap();

    let enabled_state = Rc::new(RefCell::new(true));
    let enabled_for_callback = enabled_state.clone();
    let name_state = Rc::new(RefCell::new(String::from("Ada")));
    let name_for_callback = name_state.clone();
    runtime
        .define_web_computed_native_callback(
            "label".to_owned(),
            Box::new(move || {
                if *enabled_for_callback.borrow() {
                    Ok(compute_callbacks::ComputeCallbackInvocationResult {
                        value: SignalValue::String(name_for_callback.borrow().clone()),
                        captured_read_ids: vec!["name".to_owned(), "enabled".to_owned()],
                        runtime_read_breadth: 2,
                        return_serialization_breadth: 1,
                    })
                } else {
                    Ok(compute_callbacks::ComputeCallbackInvocationResult {
                        value: SignalValue::String("disabled".to_owned()),
                        captured_read_ids: vec!["enabled".to_owned()],
                        runtime_read_breadth: 1,
                        return_serialization_breadth: 1,
                    })
                }
            }),
        )
        .unwrap();

    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("Ada".to_owned())
    );
    let enabled_snapshot = runtime.snapshot().unwrap();

    *enabled_state.borrow_mut() = false;
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "enabled".to_owned(),
            value: SignalValue::Bool(false),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("disabled".to_owned())
    );
    let disabled_snapshot = runtime.snapshot().unwrap();

    *name_state.borrow_mut() = String::from("Grace");
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "name".to_owned(),
            value: SignalValue::String("Grace".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("disabled".to_owned())
    );

    runtime.restore_snapshot(enabled_snapshot).unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("Ada".to_owned())
    );
    *enabled_state.borrow_mut() = true;
    *name_state.borrow_mut() = String::from("Ada");
    *name_state.borrow_mut() = String::from("Hopper");
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "name".to_owned(),
            value: SignalValue::String("Hopper".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("Hopper".to_owned())
    );

    runtime.restore_snapshot(disabled_snapshot).unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("disabled".to_owned())
    );
    *enabled_state.borrow_mut() = false;
    *name_state.borrow_mut() = String::from("Lin");
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "name".to_owned(),
            value: SignalValue::String("Lin".to_owned()),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    assert_eq!(
        runtime.read_value("label").unwrap(),
        SignalValue::String("disabled".to_owned())
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
