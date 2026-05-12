use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{
    RuntimeEnvelopeCallbackReattachment, WorkerPortableGraphPublication, WorkerRuntimeShell,
};

fn worker_shell() -> WorkerRuntimeShell {
    WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap()
}

fn publish_base_source(shell: &mut WorkerRuntimeShell) {
    shell
        .publish_graph(WorkerPortableGraphPublication {
            policy: RuntimePolicySpec::default(),
            sources: vec![SourceSpec {
                id: "base".to_owned(),
                initial: SignalValue::Number(2.0),
                produces_aspects: None,
            }],
            recipes: Vec::new(),
            output_ids: Vec::new(),
        })
        .unwrap();
}

fn hosted_callback_shell() -> WorkerRuntimeShell {
    let mut shell = worker_shell();
    publish_base_source(&mut shell);
    shell
        .define_main_thread_hosted_callback_for_test(
            "hostedCallback".to_owned(),
            Box::new(|| {
                Ok(compute_callbacks::ComputeCallbackInvocationResult {
                    value: SignalValue::Number(3.0),
                    captured_read_ids: vec!["base".to_owned()],
                    captured_host_capability_reads: vec![
                        compute_callbacks::CapturedHostCapabilityRead {
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
    shell
}

fn reattachment(callback_id: &str, value: f64) -> RuntimeEnvelopeCallbackReattachment {
    let token = compute_callbacks::register_native_compute_result(Box::new(move || {
        Ok(compute_callbacks::ComputeCallbackInvocationResult {
            value: SignalValue::Number(value),
            captured_read_ids: vec!["base".to_owned()],
            captured_host_capability_reads: Vec::new(),
            runtime_read_breadth: 1,
            return_serialization_breadth: 1,
        })
    }));
    let invocation = compute_callbacks::invoke_compute(token).unwrap();
    RuntimeEnvelopeCallbackReattachment {
        callback_id: callback_id.to_owned(),
        token,
        invocation,
    }
}

#[test]
fn worker_import_export_callback_unavailability_certifies_denial_and_reattachment() {
    let mut shell = hosted_callback_shell();
    let envelope = shell.export_worker_runtime_envelope().unwrap();
    shell.certify_worker_callback_capability_export().unwrap();
    shell
        .admit_worker_runtime_envelope_import(envelope.clone())
        .unwrap();
    shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();

    let package = shell
        .certify_worker_import_export_callback_unavailability()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerImportExportCallbackUnavailabilityCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(package.exported_callback_count, 1);
    assert_eq!(package.unavailable_callback_count, 1);
    assert_eq!(package.rejected_callback_count, 1);
    assert_eq!(package.reattached_callback_count, 1);
    assert_eq!(package.host_capability_transport_count, 1);
    assert_eq!(package.fallback_count, 0);
    assert_eq!(
        package.callback_unavailability_artifact,
        "computeCallbackUnavailableForPortableExport"
    );
    assert_eq!(package.unavailable_callbacks[0].id, "hostedCallback");
    assert_eq!(
        shell.read_value("hostedCallback").unwrap(),
        SignalValue::Number(99.0)
    );
    assert_digest_shape(&package.export_digest);
    assert_digest_shape(&package.import_digest);
    assert_digest_shape(&package.capability_reattachment_digest);
    assert_digest_shape(&package.callback_unavailability_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_import_export_callback_unavailability_rejects_missing_portable_denial() {
    let mut shell = hosted_callback_shell();
    let envelope = shell.export_worker_runtime_envelope().unwrap();
    shell.certify_worker_callback_capability_export().unwrap();
    shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();

    let error = shell
        .certify_worker_import_export_callback_unavailability()
        .unwrap_err();

    assert!(error.message.contains("portable import denial evidence"));
}

#[test]
fn worker_import_export_callback_unavailability_rejects_missing_reattachment_import() {
    let mut shell = hosted_callback_shell();
    let envelope = shell.export_worker_runtime_envelope().unwrap();
    shell.certify_worker_callback_capability_export().unwrap();
    shell
        .admit_worker_runtime_envelope_import(envelope)
        .unwrap();

    let error = shell
        .certify_worker_import_export_callback_unavailability()
        .unwrap_err();

    assert!(error.message.contains("reattachment import evidence"));
}

#[test]
fn worker_import_export_callback_unavailability_rejects_export_without_callback_artifacts() {
    let mut shell = worker_shell();
    publish_base_source(&mut shell);
    shell.certify_worker_callback_capability_export().unwrap();

    let error = shell
        .certify_worker_import_export_callback_unavailability()
        .unwrap_err();

    assert!(error
        .message
        .contains("callback-unavailability export artifacts"));
}
