use crate::runtime::worker_host::WorkerRuntimeShell;

use crate::runtime::tests::support::*;

#[test]
fn worker_runtime_shell_denies_callback_definition_envelope_publication() {
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    compatibility_runtime
        .define_source(SourceSpec {
            id: "counter".to_owned(),
            initial: SignalValue::Number(1.0),
            produces_aspects: None,
        })
        .unwrap();
    compatibility_runtime
        .define_web_computed_native_callback(
            "callbackDouble".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(2.0),
                    captured_read_ids: vec!["counter".to_owned()],
                    captured_host_capability_reads: Vec::new(),
                    runtime_read_breadth: 1,
                    return_serialization_breadth: 1,
                })
            }),
        )
        .unwrap();

    let definition_envelope = compatibility_runtime.export_definitions().unwrap();
    let mut worker_shell = WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap();
    let err = worker_shell
        .publish_definition_envelope(definition_envelope)
        .unwrap_err();

    assert_eq!(
        err.code,
        "workerRuntimePublicationRequiresPortableDefinitions"
    );
    assert!(err.message.contains("callbackDouble"));
}
