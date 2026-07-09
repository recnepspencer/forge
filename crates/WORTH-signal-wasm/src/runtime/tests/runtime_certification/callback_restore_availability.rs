use super::super::support::*;

#[test]
fn callback_snapshot_restore_denies_missing_callback_registrations() {
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

    let snapshot = runtime.snapshot().unwrap();
    assert!(runtime
        .dispose_web_computed_callback_for_test("doubled")
        .unwrap());

    let err = runtime.restore_snapshot(snapshot).unwrap_err();
    assert_eq!(err.code, "computeCallbackUnavailableForRestore");
    assert!(err.message.contains("doubled"));
    let summary = runtime.web_performance_summary();
    assert_eq!(summary.compute_callback_missing_unavailability_count, 1);
}
