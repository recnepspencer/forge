use super::*;

#[test]
fn resource_completion_admission_denies_pre_restore_epoch_as_stale() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit before snapshot")
        .admitted_request();
    let stale = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey in-flight handle epochs");

    let report = runtime.admit_resource_completion(stale);

    let denied = report
        .denied_completion()
        .expect("pre-restore completion should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Stale);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_superseded_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request();
    let stale_first = raw_completion(&runtime, node, first.handle(), first.attempt(), 64);
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");

    let report = runtime.admit_resource_completion(stale_first);

    let denied = report
        .denied_completion()
        .expect("superseded completion should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_cancellation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let cancellation = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");
    assert!(cancellation.cancelled_request().is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after cancellation should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("cancelled request remains retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_rejection() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let rejection = runtime
        .reject_resource_request(admitted.handle(), ResourceRejectionReason::SemanticFailure)
        .expect("rejection should retire timeout side effects cleanly");
    assert!(rejection.rejected_request().is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after rejection should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Rejected);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_rejected_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("rejected request remains retained")
            .status(),
        ResourceInFlightStatus::Rejected
    );
}

#[test]
fn resource_completion_identity_staleness_dominates_cancelled_lifecycle_after_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let pre_restore_completion =
        raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let cancellation = runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");
    assert!(cancellation.cancelled_request().is_some());
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey retained cancelled in-flight handles");

    let report = runtime.admit_resource_completion(pre_restore_completion);

    let denied = report
        .denied_completion()
        .expect("pre-restore completion should be stale even when retained request is cancelled");
    assert_eq!(denied.class(), CompletionDenialClass::Stale);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        0
    );
}

#[test]
fn resource_completion_admission_denies_late_success_after_timeout() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 3))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let late = raw_completion(&runtime, node, admitted.handle(), admitted.attempt(), 64);
    let wake_id = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("authoritative clock should advance");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should promote");
    assert!(runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should consume temporal wake cleanly")
        .timed_out_request()
        .is_some());

    let report = runtime.admit_resource_completion(late);

    let denied = report
        .denied_completion()
        .expect("late completion after timeout should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::TimedOut);
    assert!(report.admitted_completion().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timed_out_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_payload_contract_mismatch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted.handle();

    let report = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        admitted.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    ));

    let denied = report
        .denied_completion()
        .expect("wrong payload contract should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Malformed);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_malformed_completion_denial_count,
        1
    );
}

#[test]
fn resource_completion_admission_denies_payload_above_declared_limit() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();

    let report = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        admitted.handle(),
        admitted.attempt(),
        2048,
    ));

    let denied = report
        .denied_completion()
        .expect("oversized payload should be denied before apply");
    assert_eq!(denied.class(), CompletionDenialClass::Partial);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_partial_completion_denial_count,
        1
    );
}
