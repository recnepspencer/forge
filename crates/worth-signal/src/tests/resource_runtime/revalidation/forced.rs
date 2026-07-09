use super::*;

#[test]
fn resource_forced_revalidation_requires_policy_enabled_active_handle_proof() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let active = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let proof = runtime
        .prove_active_resource_revalidation_handle(active)
        .expect("active request should mint revalidation proof");

    let report = runtime
        .force_revalidate_resource_node(proof)
        .expect("policy-disabled force should still produce a report");
    let denied = report
        .denied_revalidation()
        .expect("explicit-intent-only policy must deny forced revalidation");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ForcedRevalidationPolicyDisabled
    );
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_forced_revalidation_policy_denial_count,
        1
    );
}

#[test]
fn resource_forced_revalidation_supersedes_proven_active_handle_when_policy_allows() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forced_revalidation_timeout_resource_declaration(node, 5))
        .expect("forced revalidation declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    let proof = runtime
        .prove_active_resource_revalidation_handle(first)
        .expect("active request should mint revalidation proof");

    let report = runtime
        .force_revalidate_resource_node(proof.clone())
        .expect("forced revalidation should admit");
    let revalidation = report
        .admitted_revalidation()
        .expect("forced revalidation should be admitted");
    let admitted = revalidation.admitted_request();
    let supersession = revalidation
        .supersession_record()
        .expect("forced revalidation should retain supersession lineage");
    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should remain visible");

    assert_eq!(revalidation.expected_active(), Some(first));
    assert_eq!(revalidation.forced_active_handle(), Some(first));
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), admitted.handle());
    assert_eq!(
        revalidation.decision_digest().as_str(),
        descriptor
            .revalidation_decision_plan()
            .decision_digest()
            .as_str()
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert!(runtime.promote_temporal_wake_ready(first_wake).is_err());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_forced_revalidation_count,
        1
    );
}

#[test]
fn resource_forced_revalidation_denies_stale_active_handle_proof_after_supersession() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(forced_revalidation_resource_declaration(node))
        .expect("forced revalidation declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let proof = runtime
        .prove_active_resource_revalidation_handle(first)
        .expect("first active request should mint proof");
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .force_revalidate_resource_node(proof)
        .expect("stale proof denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("stale active-handle proof must deny");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveHandleProofMismatch
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("newer active request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_active_handle_proof_mismatch_denial_count,
        1
    );
}
