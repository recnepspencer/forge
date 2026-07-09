use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{
    RuntimeEnvelopeCallbackReattachment, WorkerPortableGraphPublication, WorkerRuntimeShell,
};

fn shell() -> WorkerRuntimeShell {
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
    let mut shell = shell();
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

fn two_hosted_callback_shell() -> WorkerRuntimeShell {
    let mut shell = shell();
    publish_base_source(&mut shell);
    for callback_id in ["firstHostedCallback", "secondHostedCallback"] {
        shell
            .define_main_thread_hosted_callback_for_test(
                callback_id.to_owned(),
                Box::new(|| {
                    Ok(compute_callbacks::ComputeCallbackInvocationResult {
                        value: SignalValue::Number(3.0),
                        captured_read_ids: vec!["base".to_owned()],
                        captured_host_capability_reads: Vec::new(),
                        runtime_read_breadth: 1,
                        return_serialization_breadth: 1,
                    })
                }),
            )
            .unwrap();
    }
    shell
}

fn reattachment(callback_id: &str, value: f64) -> RuntimeEnvelopeCallbackReattachment {
    reattachment_with_reads(callback_id, value, vec!["base".to_owned()])
}

fn reattachment_with_reads(
    callback_id: &str,
    value: f64,
    captured_read_ids: Vec<String>,
) -> RuntimeEnvelopeCallbackReattachment {
    let token = compute_callbacks::register_native_compute_result(Box::new(move || {
        Ok(compute_callbacks::ComputeCallbackInvocationResult {
            value: SignalValue::Number(value),
            captured_read_ids: captured_read_ids.clone(),
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
fn worker_callback_capability_export_certifies_unavailable_portable_callback() {
    let mut shell = hosted_callback_shell();

    let package = shell.certify_worker_callback_capability_export().unwrap();

    assert_eq!(
        package.certification_family,
        "workerCallbackCapabilityExportCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(package.exported_callback_count, 1);
    assert_eq!(package.unavailable_callback_count, 1);
    assert_eq!(package.host_capability_transport_count, 1);
    assert_eq!(package.fallback_count, 0);
    assert_eq!(package.unavailable_callbacks[0].id, "hostedCallback");
    assert_eq!(
        package.unavailable_callbacks[0].reason,
        "computeCallbackUnavailableForPortableExport"
    );
    assert_eq!(
        package.unavailable_callbacks[0].host_capability_transports[0].portable_import_outcome,
        "Denied"
    );
    assert_digest_shape(&package.placement_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.capability_transport_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_runtime_envelope_import_rejects_callback_artifacts_without_fallback() {
    let mut source_shell = hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    assert_eq!(envelope.definitions.unavailable_callbacks.len(), 1);

    let mut target_shell = shell();
    let report = target_shell
        .admit_worker_runtime_envelope_import(envelope)
        .unwrap();

    assert_eq!(report.envelope_family, "workerRuntimeEnvelopeImport");
    assert_eq!(report.import_outcome, "Denied");
    assert_eq!(report.rejected_callback_count, 1);
    assert_eq!(
        report.rejected_callback_ids,
        vec!["hostedCallback".to_owned()]
    );
    assert_eq!(report.host_capability_transport_count, 1);
    assert_eq!(report.fallback_count, 0);
    assert_digest_shape(&report.worker_first_truth_digest);
    assert_digest_shape(&report.import_digest);
}

#[test]
fn worker_runtime_envelope_import_admits_portable_graph_without_callback_artifacts() {
    let mut source_shell = shell();
    source_shell
        .publish_graph(WorkerPortableGraphPublication {
            policy: RuntimePolicySpec::default(),
            sources: vec![SourceSpec {
                id: "base".to_owned(),
                initial: SignalValue::Number(2.0),
                produces_aspects: None,
            }],
            recipes: vec![RecipeSpec {
                id: "derived".to_owned(),
                reads: vec![RecipeReadSpec::LegacyId("base".to_owned())],
                expr: read("base"),
                when: None,
                identity: None,
                produces_aspects: None,
            }],
            output_ids: Vec::new(),
        })
        .unwrap();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    assert!(envelope.definitions.unavailable_callbacks.is_empty());

    let mut target_shell = shell();
    let report = target_shell
        .admit_worker_runtime_envelope_import(envelope)
        .unwrap();

    assert_eq!(report.import_outcome, "Admitted");
    assert_eq!(report.rejected_callback_count, 0);
    assert_eq!(report.fallback_count, 0);
    assert_eq!(
        target_shell.read_value("derived").unwrap(),
        SignalValue::Number(2.0)
    );
}

#[test]
fn worker_runtime_envelope_import_reattaches_callback_and_resumes_live_truth() {
    let mut source_shell = hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    assert_eq!(envelope.definitions.unavailable_callbacks.len(), 1);

    let mut target_shell = shell();
    let report = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();

    assert_eq!(report.import_outcome, "AdmittedWithReattachments");
    assert_eq!(report.reattached_callback_count, 1);
    assert_eq!(
        report.reattached_callback_ids,
        vec!["hostedCallback".to_owned()]
    );
    assert_eq!(report.rejected_callback_count, 0);
    assert_eq!(report.fallback_count, 0);
    assert_eq!(
        target_shell.read_value("hostedCallback").unwrap(),
        SignalValue::Number(99.0)
    );
    assert_digest_shape(&report.import_digest);
}

#[test]
fn worker_runtime_envelope_import_admits_multiple_callback_reattachments() {
    let mut source_shell = two_hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    assert_eq!(envelope.definitions.unavailable_callbacks.len(), 2);

    let mut target_shell = shell();
    let report = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![
                reattachment("firstHostedCallback", 11.0),
                reattachment("secondHostedCallback", 22.0),
            ],
        )
        .unwrap();

    assert_eq!(report.import_outcome, "AdmittedWithReattachments");
    assert_eq!(report.reattached_callback_count, 2);
    assert_eq!(
        report.reattached_callback_ids,
        vec![
            "firstHostedCallback".to_owned(),
            "secondHostedCallback".to_owned()
        ]
    );
    assert_eq!(
        target_shell.read_value("firstHostedCallback").unwrap(),
        SignalValue::Number(11.0)
    );
    assert_eq!(
        target_shell.read_value("secondHostedCallback").unwrap(),
        SignalValue::Number(22.0)
    );
}

#[test]
fn worker_runtime_envelope_import_rejects_dependency_widening_reattachment() {
    let mut source_shell = hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    let mut target_shell = shell();
    let reattachment =
        reattachment_with_reads("hostedCallback", 99.0, vec!["unclosedInput".to_owned()]);
    let token = reattachment.token;

    let error = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![reattachment],
        )
        .unwrap_err();

    assert!(error.message.contains("read frontier"));
    assert!(!compute_callbacks::is_compute_registered(token));
}

#[test]
fn worker_runtime_envelope_import_disposes_reattachment_after_rebuild_failure() {
    let mut source_shell = hosted_callback_shell();
    let mut envelope = source_shell.export_worker_runtime_envelope().unwrap();
    envelope.definitions.sources.clear();
    let mut target_shell = shell();
    let reattachment = reattachment("hostedCallback", 99.0);
    let token = reattachment.token;

    let error = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![reattachment],
        )
        .unwrap_err();

    assert!(error.message.contains("hostedCallback") || error.message.contains("base"));
    assert!(!compute_callbacks::is_compute_registered(token));
}

#[test]
fn worker_runtime_envelope_import_rejects_missing_callback_reattachment() {
    let mut source_shell = hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    let mut target_shell = shell();

    let error = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(envelope, Vec::new())
        .unwrap_err();

    assert!(error.message.contains("missing callback reattachments"));
}

#[test]
fn worker_runtime_envelope_import_rejects_unexpected_callback_reattachment() {
    let mut source_shell = hosted_callback_shell();
    let envelope = source_shell.export_worker_runtime_envelope().unwrap();
    let mut target_shell = shell();
    let required = reattachment("hostedCallback", 99.0);
    let required_token = required.token;
    let extra = reattachment("extraCallback", 100.0);
    let extra_token = extra.token;

    let error = target_shell
        .admit_worker_runtime_envelope_import_with_callback_reattachments(
            envelope,
            vec![required, extra],
        )
        .unwrap_err();

    assert!(error.message.contains("unexpected callback reattachments"));
    assert!(!compute_callbacks::is_compute_registered(required_token));
    assert!(!compute_callbacks::is_compute_registered(extra_token));
}
