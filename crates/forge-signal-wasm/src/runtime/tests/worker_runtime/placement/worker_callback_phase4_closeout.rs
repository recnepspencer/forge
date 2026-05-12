use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{DefinitionEnvelopeCallbackReattachment, WorkerRuntimeShell};

fn shell() -> WorkerRuntimeShell {
    WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap()
}

fn callback_definition_envelope() -> crate::runtime::adapters::RuntimeDefinitionEnvelope {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "hostedCallback".to_owned(),
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
    runtime.export_definitions().unwrap()
}

fn callback_runtime_envelope() -> crate::runtime::adapters::RuntimeEnvelope {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime
        .define_web_computed_native_callback(
            "hostedCallback".to_owned(),
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
    runtime.export_runtime_envelope().unwrap()
}

fn source_only_definition_envelope() -> crate::runtime::adapters::RuntimeDefinitionEnvelope {
    let mut runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    runtime.export_definitions().unwrap()
}

fn reattachment(callback_id: &str, value: f64) -> DefinitionEnvelopeCallbackReattachment {
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
    DefinitionEnvelopeCallbackReattachment {
        callback_id: callback_id.to_owned(),
        token,
        invocation,
    }
}

#[test]
fn worker_callback_phase4_closeout_certifies_current_worker_retained_evidence() {
    let envelope = callback_definition_envelope();
    let mut worker_shell = shell();
    worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            envelope.clone(),
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();
    let denied_import_envelope = worker_shell.export_worker_runtime_envelope().unwrap();
    worker_shell
        .admit_worker_runtime_envelope_import(denied_import_envelope)
        .unwrap();

    let package = worker_shell
        .certify_worker_callback_phase4_closeout()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "workerCallbackPhase4CloseoutCertification"
    );
    assert_eq!(
        package.closeout_gate_mode,
        "PublicationReattachmentWithPortableImportDenial"
    );
    assert_eq!(package.covered_suite_count, 3);
    assert_eq!(package.runtime_envelope_import_outcome, "Denied");
    assert_eq!(
        package.definition_publication_outcome,
        "AdmittedWithReattachments"
    );
    assert_eq!(package.fallback_count, 0);
    assert_eq!(package.published_reattached_callback_count, 1);
    assert_eq!(package.imported_reattached_callback_count, 0);
    assert_digest_shape(&package.runtime_envelope_import_digest);
    assert_digest_shape(&package.definition_publication_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn worker_callback_phase4_closeout_rejects_non_callback_publication_evidence() {
    let mut worker_shell = shell();
    worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            source_only_definition_envelope(),
            Vec::new(),
        )
        .unwrap();
    worker_shell
        .admit_worker_runtime_envelope_import(callback_runtime_envelope())
        .unwrap();

    let error = worker_shell
        .certify_worker_callback_phase4_closeout()
        .unwrap_err();

    assert!(error.message.contains("reattachment publication"));
}

#[test]
fn worker_callback_phase4_closeout_rejects_missing_worker_retained_evidence() {
    let worker_shell = shell();

    let error = worker_shell
        .certify_worker_callback_phase4_closeout()
        .unwrap_err();

    assert!(error.message.contains("import evidence"));
}

#[test]
fn worker_callback_phase4_closeout_rejects_stale_worker_truth_evidence() {
    let envelope = callback_definition_envelope();
    let mut worker_shell = shell();
    worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            envelope.clone(),
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();
    let denied_import_envelope = worker_shell.export_worker_runtime_envelope().unwrap();
    worker_shell
        .admit_worker_runtime_envelope_import(denied_import_envelope)
        .unwrap();
    worker_shell
        .apply_committed_transaction(vec![TransactionOp::Set {
            id: "base".to_owned(),
            value: SignalValue::Number(5.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let error = worker_shell
        .certify_worker_callback_phase4_closeout()
        .unwrap_err();

    assert!(error.message.contains("import evidence"));
}
