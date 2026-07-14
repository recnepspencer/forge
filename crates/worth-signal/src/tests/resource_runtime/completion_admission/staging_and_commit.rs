use super::*;

#[test]
fn resource_completion_duplicate_after_commit_is_retired_without_second_commit() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let raw = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let admitted_completion = runtime
        .admit_resource_completion(raw.clone())
        .admitted_completion()
        .expect("first matching completion should admit");
    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage")
        .staged_effect();

    runtime
        .commit_staged_resource_completion(staged)
        .expect("first completion should commit");
    let duplicate = runtime.admit_resource_completion(raw);

    assert_eq!(
        duplicate
            .denied_completion()
            .expect("duplicate completion should be denied")
            .class(),
        CompletionDenialClass::Retired
    );
    assert!(duplicate.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        1
    );
}

#[test]
fn resource_completion_staging_rejects_admitted_proof_after_cancellation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted_request.handle(),
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit before cancellation");

    runtime
        .cancel_resource_request(
            admitted_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should apply");
    let err = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect_err("admitted completion proof should not stage after lifecycle changes");

    assert!(err.to_string().contains("non-active request"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_staging_count,
        0
    );
}

#[test]
fn resource_completion_admission_denies_unknown_request_without_lifecycle_mutation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should exist")
        .payload_contract_digest()
        .clone();

    let report = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        ResourceRequestId::new(999),
        ResourceGeneration::new(1),
        ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
        ResourceAttemptId::ZERO,
        digest,
        32,
    ));

    let denied = report
        .denied_completion()
        .expect("unknown request should produce a retained denial");
    assert_eq!(denied.class(), CompletionDenialClass::UnknownRequest);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime.resource_runtime_summary().denied_completion_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}
