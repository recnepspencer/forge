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
                        captured_host_capability_reads: Vec::new(),
                        runtime_read_breadth: 2,
                        return_serialization_breadth: 1,
                    })
                } else {
                    Ok(compute_callbacks::ComputeCallbackInvocationResult {
                        value: SignalValue::String("disabled".to_owned()),
                        captured_read_ids: vec!["enabled".to_owned()],
                        captured_host_capability_reads: Vec::new(),
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
    let mut original = RuntimeCore::new(RuntimePolicySpec {
        preset: RuntimePolicyPreset::Forensic,
    })
    .unwrap();
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
fn exact_envelope_import_narrows_owner_authority_to_the_current_head() {
    let mut original = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    original
        .define_source(SourceSpec {
            id: "value".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    let root = original.current_branch();
    let older_snapshot = original.snapshot().unwrap();
    let feature = original
        .create_branch("retained-feature".to_owned())
        .unwrap();
    original.switch_branch(feature.id.0).unwrap();
    let head_before_denial = original.branch_state_proof(feature.id.0).unwrap();
    let history_before_denial = original.recent_history().unwrap().len();
    let indexes_before_denial = original.snapshot_owner_registry_counts();
    let composed_denial = original.export_runtime_envelope().unwrap_err();
    assert!(composed_denial.message.contains("root branch"));
    let non_root_denial = match original.export_exact_runtime_restore_artifact() {
        Ok(_) => panic!("non-root exact export must be denied"),
        Err(error) => error,
    };
    assert!(non_root_denial.message.contains("root branch"));
    let head_after_denial = original.branch_state_proof(feature.id.0).unwrap();
    assert_eq!(
        head_after_denial.snapshot_id,
        head_before_denial.snapshot_id
    );
    assert_eq!(
        head_after_denial.state_digest,
        head_before_denial.state_digest
    );
    assert_eq!(
        original.recent_history().unwrap().len(),
        history_before_denial
    );
    assert_eq!(
        original.snapshot_owner_registry_counts(),
        indexes_before_denial
    );
    original.switch_branch(root.id.0).unwrap();
    original
        .apply_transaction(vec![TransactionOp::Set {
            id: "value".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();
    let artifact = original.export_exact_runtime_restore_artifact().unwrap();

    let mut restored = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    restored.replace_runtime_envelope_exact(artifact).unwrap();

    assert_eq!(restored.branches().len(), 1);
    assert_eq!(
        restored.read_value("value").unwrap(),
        SignalValue::Number(2.0)
    );
    let denial = restored.restore_snapshot(older_snapshot).unwrap_err();
    assert!(denial.message.contains("not admitted by this runtime"));
}
