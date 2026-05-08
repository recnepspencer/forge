use crate::runtime::tests::support::*;
use crate::runtime::worker_host::{
    WorkerMainThreadHostedCallbackOutcome, WorkerMainThreadHostedCallbackResult, WorkerRuntimeShell,
};

fn shell() -> WorkerRuntimeShell {
    WorkerRuntimeShell::new(RuntimePolicySpec::default()).unwrap()
}

fn signal_tracked_invocation(value: f64) -> compute_callbacks::ComputeCallbackInvocationResult {
    compute_callbacks::ComputeCallbackInvocationResult {
        value: SignalValue::Number(value),
        captured_read_ids: vec!["base".to_owned()],
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 1,
        return_serialization_breadth: 1,
    }
}

fn hosted_callback_shell() -> WorkerRuntimeShell {
    let mut shell = shell();
    shell
        .publish_graph(
            crate::runtime::worker_host::WorkerPortableGraphPublication {
                policy: RuntimePolicySpec::default(),
                sources: vec![SourceSpec {
                    id: "base".to_owned(),
                    initial: SignalValue::Number(2.0),
                    produces_aspects: None,
                }],
                recipes: Vec::new(),
                output_ids: Vec::new(),
            },
        )
        .unwrap();
    shell
        .define_main_thread_hosted_callback_for_test(
            "hostedCallback".to_owned(),
            Box::new(|| Ok(signal_tracked_invocation(3.0))),
        )
        .unwrap();
    shell
}

#[test]
fn main_thread_hosted_callback_result_readmits_through_worker_truth() {
    let mut shell = hosted_callback_shell();

    let request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    let result = WorkerMainThreadHostedCallbackResult {
        request_digest: request.request_digest.clone(),
        callback_id: "hostedCallback".to_owned(),
        outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
        artifact_identity: "hosted-callback-result-1".to_owned(),
        value: Some(SignalValue::Number(11.0)),
        captured_read_ids: vec!["base".to_owned()],
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 1,
        return_serialization_breadth: 1,
    };

    let report = shell
        .admit_main_thread_hosted_callback_result(request, result)
        .unwrap();

    assert_eq!(report.envelope_family, "mainThreadHostedCallbackExecution");
    assert_eq!(report.callback_id, "hostedCallback");
    assert_eq!(
        report.callback_execution_artifact,
        "mainThreadHostedCallbackCompleted"
    );
    assert_eq!(report.runtime_admitted_result_count, 1);
    assert_eq!(report.runtime_mutation_breadth, 1);
    assert!(!report.host_result_is_authoritative);
    assert!(report.ambient_graph_read_denied);
    assert_eq!(report.fallback_count, 0);
    assert_eq!(
        shell.peek_value("hostedCallback").unwrap(),
        SignalValue::Number(11.0)
    );
}

#[test]
fn main_thread_hosted_callback_execution_certification_binds_placement_and_result_evidence() {
    let mut shell = hosted_callback_shell();
    let mut request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    let mut report = shell
        .admit_main_thread_hosted_callback_result(
            request.clone(),
            WorkerMainThreadHostedCallbackResult {
                request_digest: request.request_digest.clone(),
                callback_id: "hostedCallback".to_owned(),
                outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
                artifact_identity: "hosted-callback-result-1".to_owned(),
                value: Some(SignalValue::Number(11.0)),
                captured_read_ids: vec!["base".to_owned()],
                captured_host_capability_reads: Vec::new(),
                runtime_read_breadth: 1,
                return_serialization_breadth: 1,
            },
        )
        .unwrap();
    request.callback_id = "callerTamperedCallback".to_owned();
    report.result_digest = "caller-tampered-result".to_owned();

    let package = shell
        .certify_main_thread_hosted_callback_execution()
        .unwrap();

    assert_eq!(
        package.certification_family,
        "mainThreadHostedCallbackExecutionCertification"
    );
    assert_eq!(package.covered_suite_count, 1);
    assert_eq!(package.callback_id, "hostedCallback");
    assert_eq!(package.runtime_admitted_result_count, 1);
    assert_eq!(package.runtime_mutation_breadth, 1);
    assert!(package.ambient_graph_read_denied);
    assert!(!package.host_result_is_authoritative);
    assert_eq!(package.fallback_count, 0);
    assert_digest_shape(&package.placement_digest);
    assert_digest_shape(&package.denial_digest);
    assert_digest_shape(&package.fallback_digest);
    assert_digest_shape(&package.capability_availability_digest);
    assert_digest_shape(&package.replay_import_compatibility_digest);
    assert_digest_shape(&package.placement_identity_digest);
    assert_digest_shape(&package.hosted_execution_digest);
    assert_digest_shape(&package.certification_digest);
}

#[test]
fn main_thread_hosted_callback_execution_certification_rejects_cleared_runtime_evidence() {
    let mut shell = hosted_callback_shell();
    let request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    shell
        .admit_main_thread_hosted_callback_result(
            request.clone(),
            WorkerMainThreadHostedCallbackResult {
                request_digest: request.request_digest.clone(),
                callback_id: "hostedCallback".to_owned(),
                outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
                artifact_identity: "hosted-callback-result-1".to_owned(),
                value: Some(SignalValue::Number(11.0)),
                captured_read_ids: vec!["base".to_owned()],
                captured_host_capability_reads: Vec::new(),
                runtime_read_breadth: 1,
                return_serialization_breadth: 1,
            },
        )
        .unwrap();

    shell
        .apply_committed_transaction(vec![TransactionOp::Set {
            id: "base".to_owned(),
            value: SignalValue::Number(12.0),
            aspect: None,
            aspects: None,
        }])
        .unwrap();

    let error = shell
        .certify_main_thread_hosted_callback_execution()
        .unwrap_err();

    assert!(error.message.contains("retained request evidence"));
}

#[test]
fn main_thread_hosted_callback_rejects_ambient_graph_read_result() {
    let mut shell = hosted_callback_shell();
    let request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    let result = WorkerMainThreadHostedCallbackResult {
        request_digest: request.request_digest.clone(),
        callback_id: "hostedCallback".to_owned(),
        outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
        artifact_identity: "ambient-read-attempt".to_owned(),
        value: Some(SignalValue::Number(11.0)),
        captured_read_ids: vec!["base".to_owned(), "unclosedDerived".to_owned()],
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 2,
        return_serialization_breadth: 1,
    };

    let error = shell
        .admit_main_thread_hosted_callback_result(request, result)
        .unwrap_err();

    assert!(error
        .message
        .contains("closed worker-issued input frontier"));
    assert_eq!(
        shell.peek_value("hostedCallback").unwrap(),
        SignalValue::Number(3.0)
    );
}

#[test]
fn main_thread_hosted_callback_denial_report_does_not_mutate_runtime_truth() {
    let mut shell = hosted_callback_shell();
    let request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    let result = WorkerMainThreadHostedCallbackResult {
        request_digest: request.request_digest.clone(),
        callback_id: "hostedCallback".to_owned(),
        outcome: WorkerMainThreadHostedCallbackOutcome::Denied,
        artifact_identity: "hosted-callback-denied".to_owned(),
        value: None,
        captured_read_ids: Vec::new(),
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 0,
        return_serialization_breadth: 0,
    };

    let report = shell
        .admit_main_thread_hosted_callback_result(request, result)
        .unwrap();

    assert_eq!(
        report.callback_execution_artifact,
        "mainThreadHostedCallbackDenied"
    );
    assert_eq!(report.runtime_admitted_result_count, 0);
    assert_eq!(report.runtime_mutation_breadth, 0);
    assert!(report.worker_readmission_required);
    assert_eq!(
        shell.peek_value("hostedCallback").unwrap(),
        SignalValue::Number(3.0)
    );
}

#[test]
fn main_thread_hosted_callback_rejects_mismatched_request_digest() {
    let mut shell = hosted_callback_shell();
    let request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    let result = WorkerMainThreadHostedCallbackResult {
        request_digest: "wrong-request".to_owned(),
        callback_id: "hostedCallback".to_owned(),
        outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
        artifact_identity: "hosted-callback-result-1".to_owned(),
        value: Some(SignalValue::Number(11.0)),
        captured_read_ids: vec!["base".to_owned()],
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 1,
        return_serialization_breadth: 1,
    };

    let error = shell
        .admit_main_thread_hosted_callback_result(request, result)
        .unwrap_err();

    assert!(error.message.contains("issued request digest"));
}

#[test]
fn main_thread_hosted_callback_rejects_tampered_request_frontier() {
    let mut shell = hosted_callback_shell();
    let mut request = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap();
    request.closed_input_ids.push("unissuedInput".to_owned());
    request.closed_input_count = request.closed_input_ids.len() as u64;
    let result = WorkerMainThreadHostedCallbackResult {
        request_digest: request.request_digest.clone(),
        callback_id: "hostedCallback".to_owned(),
        outcome: WorkerMainThreadHostedCallbackOutcome::Completed,
        artifact_identity: "hosted-callback-result-1".to_owned(),
        value: Some(SignalValue::Number(11.0)),
        captured_read_ids: request.closed_input_ids.clone(),
        captured_host_capability_reads: Vec::new(),
        runtime_read_breadth: 2,
        return_serialization_breadth: 1,
    };

    let error = shell
        .admit_main_thread_hosted_callback_result(request, result)
        .unwrap_err();

    assert!(error.message.contains("request digest"));
}

#[test]
fn main_thread_hosted_callback_request_requires_declared_hosted_callback() {
    let mut shell = shell();
    shell
        .publish_graph(
            crate::runtime::worker_host::WorkerPortableGraphPublication {
                policy: RuntimePolicySpec::default(),
                sources: vec![SourceSpec {
                    id: "base".to_owned(),
                    initial: SignalValue::Number(2.0),
                    produces_aspects: None,
                }],
                recipes: vec![RecipeSpec {
                    id: "portableDerived".to_owned(),
                    reads: vec![RecipeReadSpec::LegacyId("base".to_owned())],
                    expr: read("base"),
                    when: None,
                    identity: None,
                    produces_aspects: None,
                }],
                output_ids: Vec::new(),
            },
        )
        .unwrap();
    let error = shell
        .issue_main_thread_hosted_callback_request("hostedCallback")
        .unwrap_err();
    assert!(error.message.contains("unknown"));
}
