use super::support::*;

#[test]
fn signals_phase7_diagnostics_surfaces_expose_callback_metadata() {
    let signals = build_signals();

    let count_state = Rc::new(RefCell::new(1.0));
    let count_state_for_callback = count_state.clone();
    signals
        .input_for_test("count", SignalValue::Number(1.0))
        .unwrap();
    signals
        .core
        .borrow_mut()
        .define_web_computed_native_callback(
            "doubled".to_owned(),
            Box::new(move || {
                Ok(ComputeCallbackInvocationResult {
                    value: SignalValue::Number(*count_state_for_callback.borrow() * 2.0),
                    captured_read_ids: vec!["count".to_owned()],
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let notices = Arc::new(Mutex::new(Vec::<WebObservationNotice>::new()));
    let notices_clone = notices.clone();
    let _handle = signals
        .watch_for_test("doubled", move |notice| {
            notices_clone
                .lock()
                .expect("phase7 callback notices mutex poisoned")
                .push(notice);
        })
        .unwrap();

    let _ = signals.core.borrow_mut().read_value("doubled").unwrap();
    *count_state.borrow_mut() = 2.0;
    signals
        .core
        .borrow_mut()
        .apply_transaction(vec![crate::recipe::model::TransactionOp::Set {
            id: "count".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let latest_flow: JsonValue =
        serde_json::to_value(signals.core.borrow().latest_flow().unwrap().unwrap()).unwrap();
    assert!(latest_flow["callbackNodes"].is_array());

    let latest_observation: JsonValue =
        serde_json::to_value(signals.core.borrow().latest_observation().unwrap().unwrap()).unwrap();
    assert!(latest_observation["callbackNodes"].is_array());
    assert_eq!(
        notices
            .lock()
            .expect("phase7 callback notices mutex poisoned")
            .len(),
        1
    );

    let history_now: JsonValue =
        serde_json::to_value(signals.core.borrow().execution_history_now().unwrap()).unwrap();
    assert!(history_now["callbackNodes"].is_array());
}

#[test]
fn signals_phase7_output_callback_surface_defers_with_typed_error() {
    let signals = build_signals();

    let error: ForgeSignalJsError = signals.output_callback_deferred_error_for_test("panel");
    assert_eq!(error.code, "outputCallbackDeferred");
    assert!(error.message.contains("intentionally deferred"));
    assert_eq!(error.context.as_deref(), Some("panel"));
}

#[test]
fn signals_phase7_runtime_envelope_boundary_defers_until_portable_snapshot_artifact_exists() {
    let signals = build_signals();
    let adapters = signals.adapters_compat();

    let error = adapters.runtime_envelope_js_boundary_deferred_error_for_test();
    assert_eq!(error.code, "runtimeEnvelopeJsBoundaryDeferred");
    assert!(error
        .message
        .contains("self-describing portable snapshot artifact"));
}
