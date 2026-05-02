use super::support::*;
use crate::runtime::adapters::RuntimeEnvelope;
use crate::runtime::summaries::RuntimeSnapshotEnvelope;

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
                    captured_host_capability_reads: vec![
                        crate::runtime::compute_callbacks::CapturedHostCapabilityRead {
                            family: "visibility".to_owned(),
                            registration_id: "visibility".to_owned(),
                            compatibility: "LiveOnly".to_owned(),
                        },
                    ],
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
    assert!(latest_flow["flow"].is_object());
    assert!(latest_flow["flow"]["change"].is_object());
    assert!(latest_flow["flow"]["apply"]["report"].is_object());
    assert!(
        latest_flow["flow"]["observation"].is_null()
            || latest_flow["flow"]["observation"].is_object()
    );
    assert!(latest_flow["callbackNodes"].is_array());
    assert_eq!(latest_flow["callbackNodes"][0]["id"], "doubled");
    assert_eq!(
        latest_flow["callbackNodes"][0]["purityPosture"],
        "signalTracked"
    );
    assert_eq!(
        latest_flow["callbackNodes"][0]["hostCapabilityReads"][0]["family"],
        "visibility"
    );
    assert_eq!(
        latest_flow["callbackNodes"][0]["hostCapabilityReads"][0]["compatibility"],
        "LiveOnly"
    );

    let latest_observation: JsonValue =
        serde_json::to_value(signals.core.borrow().latest_observation().unwrap().unwrap()).unwrap();
    assert!(latest_observation["observation"].is_object());
    assert!(latest_observation["observation"]["boundary_events"].is_array());
    assert_eq!(
        latest_observation["observation"]["boundary_events"][0]["outcome"],
        "Delivered"
    );
    assert!(latest_observation["callbackNodes"].is_array());
    assert_eq!(latest_observation["callbackNodes"][0]["id"], "doubled");
    assert_eq!(
        latest_observation["callbackNodes"][0]["hostCapabilityReads"][0]["registrationId"],
        "visibility"
    );
    assert_eq!(
        notices
            .lock()
            .expect("phase7 callback notices mutex poisoned")
            .len(),
        1
    );

    let history_now: JsonValue =
        serde_json::to_value(signals.core.borrow().execution_history_now().unwrap()).unwrap();
    assert!(history_now["history"].is_object());
    assert!(history_now["history"]["traced_node_count"].is_number());
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
fn signals_phase7_runtime_envelope_boundary_exports_typed_snapshot_artifacts() {
    let signals = build_signals();
    build_phase3_graph(&signals);
    set_signal_value(&signals, "count", 4.0);

    let adapters = signals.adapters_compat();
    let envelope: RuntimeEnvelope = adapters.export_runtime_envelope_for_test().unwrap();

    assert!(envelope.definitions.unavailable_callbacks.is_empty());
    assert_eq!(envelope.snapshot.state.sources.len(), 1);
    assert_eq!(envelope.snapshot.state.recipes.len(), 2);

    let restored = build_signals();
    restored
        .adapters_compat()
        .replace_runtime_envelope_for_test(envelope)
        .unwrap();

    assert_eq!(
        restored.read_for_test("double").unwrap(),
        SignalValue::Number(8.0)
    );
    assert_eq!(
        restored.read_for_test("panel").unwrap(),
        serde_json::from_value(serde_json::json!({
            "count": 4.0,
            "double": 8.0,
        }))
        .unwrap()
    );
}

#[test]
fn signals_phase7_runtime_envelope_boundary_rejects_callback_unavailable_imports_with_typed_error()
{
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
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();
    let _ = signals.core.borrow_mut().read_value("doubled").unwrap();

    let envelope = signals
        .adapters_compat()
        .export_runtime_envelope_for_test()
        .unwrap();
    let restored = build_signals();
    let err: ForgeSignalJsError = restored
        .adapters_compat()
        .replace_runtime_envelope_for_test(envelope)
        .unwrap_err();

    assert_eq!(
        err.code,
        "computeCallbackUnavailableForRuntimeEnvelopeImport"
    );
    assert!(err
        .message
        .contains("callback-backed nodes without live callback registrations"));
    assert_eq!(err.context.as_deref(), Some("doubled"));
}

#[test]
fn signals_phase7_history_snapshot_boundary_serializes_structured_snapshot_artifacts() {
    let signals = build_signals();
    build_phase3_graph(&signals);
    set_signal_value(&signals, "count", 3.0);

    let history = signals.history_compat();
    let envelope: RuntimeSnapshotEnvelope = history.snapshot_for_test().unwrap();
    assert_eq!(envelope.snapshot.meta.branch_id.0, 0);
    assert_eq!(envelope.state.sources.len(), 1);

    let branch_envelope: RuntimeSnapshotEnvelope =
        history.branch_snapshot_envelope_for_test(0).unwrap();
    assert_eq!(branch_envelope.snapshot.meta.branch_id.0, 0);
    assert_eq!(branch_envelope.state.recipes.len(), 2);
}
