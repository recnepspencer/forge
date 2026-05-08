use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{DefinitionEnvelopeCallbackReattachment, WorkerRuntimeShell};

fn shell() -> WorkerRuntimeShell {
    WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap()
}

fn callback_definition_envelope() -> crate::runtime::adapters::RuntimeDefinitionEnvelope {
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    compatibility_runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    compatibility_runtime
        .define_web_computed_native_callback(
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
    compatibility_runtime.export_definitions().unwrap()
}

fn two_callback_definition_envelope() -> crate::runtime::adapters::RuntimeDefinitionEnvelope {
    let mut compatibility_runtime = RuntimeCore::new(RuntimePolicySpec::default()).unwrap();
    compatibility_runtime
        .define_source(SourceSpec {
            id: "base".to_owned(),
            initial: SignalValue::Number(2.0),
            produces_aspects: None,
        })
        .unwrap();
    for callback_id in ["firstHostedCallback", "secondHostedCallback"] {
        compatibility_runtime
            .define_web_computed_native_callback(
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
    compatibility_runtime.export_definitions().unwrap()
}

fn reattachment(callback_id: &str, value: f64) -> DefinitionEnvelopeCallbackReattachment {
    reattachment_with_reads(callback_id, value, vec!["base".to_owned()])
}

fn reattachment_with_reads(
    callback_id: &str,
    value: f64,
    captured_read_ids: Vec<String>,
) -> DefinitionEnvelopeCallbackReattachment {
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
    DefinitionEnvelopeCallbackReattachment {
        callback_id: callback_id.to_owned(),
        token,
        invocation,
    }
}

#[test]
fn worker_definition_publication_reattaches_callback_and_resumes_live_truth() {
    let envelope = callback_definition_envelope();
    assert_eq!(envelope.unavailable_callbacks.len(), 1);
    let mut worker_shell = shell();

    let report = worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            envelope,
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();

    assert_eq!(
        report.publication_family,
        "workerDefinitionEnvelopePublication"
    );
    assert_eq!(report.publication_outcome, "AdmittedWithReattachments");
    assert_eq!(report.published_source_count, 1);
    assert_eq!(report.published_recipe_count, 0);
    assert_eq!(report.reattached_callback_count, 1);
    assert_eq!(
        report.reattached_callback_ids,
        vec!["hostedCallback".to_owned()]
    );
    assert_eq!(report.host_capability_transport_count, 1);
    assert_eq!(report.fallback_count, 0);
    assert_eq!(
        worker_shell.read_value("hostedCallback").unwrap(),
        SignalValue::Number(99.0)
    );
    assert_digest_shape(&report.worker_first_truth_digest);
    assert_digest_shape(&report.publication_digest);
}

#[test]
fn worker_definition_publication_plans_recipe_after_reattached_callback() {
    let mut envelope = callback_definition_envelope();
    envelope.recipes.push(RecipeSpec {
        id: "derivedFromCallback".to_owned(),
        reads: vec![RecipeReadSpec::LegacyId("hostedCallback".to_owned())],
        expr: read("hostedCallback"),
        when: None,
        identity: None,
        produces_aspects: None,
    });
    let mut worker_shell = shell();

    worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            envelope,
            vec![reattachment("hostedCallback", 99.0)],
        )
        .unwrap();

    assert_eq!(
        worker_shell.read_value("derivedFromCallback").unwrap(),
        SignalValue::Number(99.0)
    );
}

#[test]
fn worker_definition_publication_admits_multiple_callback_reattachments() {
    let envelope = two_callback_definition_envelope();
    let mut worker_shell = shell();

    let report = worker_shell
        .publish_definition_envelope_with_callback_reattachments(
            envelope,
            vec![
                reattachment("firstHostedCallback", 11.0),
                reattachment("secondHostedCallback", 22.0),
            ],
        )
        .unwrap();

    assert_eq!(report.publication_outcome, "AdmittedWithReattachments");
    assert_eq!(report.reattached_callback_count, 2);
    assert_eq!(
        report.reattached_callback_ids,
        vec![
            "firstHostedCallback".to_owned(),
            "secondHostedCallback".to_owned()
        ]
    );
    assert_eq!(
        worker_shell.read_value("firstHostedCallback").unwrap(),
        SignalValue::Number(11.0)
    );
    assert_eq!(
        worker_shell.read_value("secondHostedCallback").unwrap(),
        SignalValue::Number(22.0)
    );
}

#[test]
fn worker_definition_publication_rejects_missing_callback_reattachment() {
    let envelope = callback_definition_envelope();
    let mut worker_shell = shell();

    let error = worker_shell
        .publish_definition_envelope_with_callback_reattachments(envelope, Vec::new())
        .unwrap_err();

    assert!(error.message.contains("missing callback reattachments"));
}

#[test]
fn worker_definition_publication_rejects_unexpected_callback_reattachment() {
    let envelope = callback_definition_envelope();
    let mut worker_shell = shell();
    let required = reattachment("hostedCallback", 99.0);
    let required_token = required.token;
    let extra = reattachment("extraCallback", 100.0);
    let extra_token = extra.token;

    let error = worker_shell
        .publish_definition_envelope_with_callback_reattachments(envelope, vec![required, extra])
        .unwrap_err();

    assert!(error.message.contains("unexpected callback reattachments"));
    assert!(!compute_callbacks::is_compute_registered(required_token));
    assert!(!compute_callbacks::is_compute_registered(extra_token));
}

#[test]
fn worker_definition_publication_rejects_dependency_widening_reattachment() {
    let envelope = callback_definition_envelope();
    let mut worker_shell = shell();
    let reattachment =
        reattachment_with_reads("hostedCallback", 99.0, vec!["unclosedInput".to_owned()]);
    let token = reattachment.token;

    let error = worker_shell
        .publish_definition_envelope_with_callback_reattachments(envelope, vec![reattachment])
        .unwrap_err();

    assert!(error.message.contains("read frontier"));
    assert!(!compute_callbacks::is_compute_registered(token));
}

#[test]
fn worker_definition_publication_disposes_reattachment_after_publish_failure() {
    let mut envelope = callback_definition_envelope();
    envelope.sources.clear();
    let mut worker_shell = shell();
    let reattachment = reattachment("hostedCallback", 99.0);
    let token = reattachment.token;

    let error = worker_shell
        .publish_definition_envelope_with_callback_reattachments(envelope, vec![reattachment])
        .unwrap_err();

    assert!(error.message.contains("base"));
    assert!(!compute_callbacks::is_compute_registered(token));
}

#[test]
fn worker_definition_publication_preflight_failure_leaves_no_partial_sources() {
    let mut envelope = callback_definition_envelope();
    envelope.unavailable_callbacks[0].current_reads = vec!["unclosedInput".to_owned()];
    let mut worker_shell = shell();
    let reattachment =
        reattachment_with_reads("hostedCallback", 99.0, vec!["unclosedInput".to_owned()]);
    let token = reattachment.token;

    let error = worker_shell
        .publish_definition_envelope_with_callback_reattachments(envelope, vec![reattachment])
        .unwrap_err();

    assert!(error.message.contains("unclosedInput"));
    assert!(worker_shell.read_value("base").is_err());
    assert!(worker_shell.read_value("hostedCallback").is_err());
    assert!(!compute_callbacks::is_compute_registered(token));
}
