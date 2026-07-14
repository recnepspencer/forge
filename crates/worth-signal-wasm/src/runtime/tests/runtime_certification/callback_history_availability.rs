use super::super::support::*;

#[test]
fn callback_history_surfaces_report_callback_availability() {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "count".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "doubled".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(2.0),
                    captured_read_ids: vec!["count".to_owned()],
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let _ = runtime.read_value("doubled").unwrap();
    runtime
        .apply_transaction(vec![TransactionOp::Set {
            id: "count".to_owned(),
            value: SignalValue::Number(2.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let replay = runtime.replay_for_id("doubled").unwrap();
    assert!(replay.frames.iter().any(|frame| frame
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));
    let branch_replay = runtime
        .replay_for_branch(runtime.current_branch().id.0)
        .unwrap();
    assert!(branch_replay.frames.iter().any(|frame| frame
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));
    let lineage = runtime.lineage_for_id("doubled").unwrap();
    assert!(lineage.events.iter().any(|event| event
        .callback
        .as_ref()
        .map(|callback| callback.registered)
        == Some(true)));

    assert!(runtime
        .dispose_web_computed_callback_for_test("doubled")
        .unwrap());

    let replay = runtime.replay_for_id("doubled").unwrap();
    assert!(replay.frames.iter().any(|frame| {
        frame.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
    let branch_replay = runtime
        .replay_for_branch(runtime.current_branch().id.0)
        .unwrap();
    assert!(branch_replay.frames.iter().any(|frame| {
        frame.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
    let lineage = runtime.lineage_for_id("doubled").unwrap();
    assert!(lineage.events.iter().any(|event| {
        event.callback.as_ref().map(|callback| {
            !callback.registered
                && callback.unavailable_reason.as_deref()
                    == Some("computeCallbackUnavailableForReplay")
        }) == Some(true)
    }));
}
