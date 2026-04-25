use std::sync::{Arc, Mutex};

use crate::facade::*;

type TestRuntime = SignalRuntime<(), (), (), (), ()>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResourceObservationRecord {
    observer_id: u64,
    handle_id: u64,
    matched_node_count: usize,
    touched: bool,
    recomputed: bool,
    meaningful_change: bool,
    trigger_matched: bool,
}

struct ResourceObservationListener {
    calls: Arc<Mutex<Vec<ResourceObservationRecord>>>,
}

impl ObservationListener<(), (), (), (), ()> for ResourceObservationListener {
    fn on_observation(
        &self,
        _ctx: ObservationReadContext<'_, (), (), (), (), ()>,
        notice: &ObservationNotice<'_>,
    ) {
        self.calls
            .lock()
            .expect("resource observation mutex poisoned")
            .push(ResourceObservationRecord {
                observer_id: notice.observer_id().get(),
                handle_id: notice.handle_id().get(),
                matched_node_count: notice.matched_nodes().len(),
                touched: notice.touched(),
                recomputed: notice.recomputed(),
                meaningful_change: notice.meaningful_change(),
                trigger_matched: notice.trigger_matched(),
            });
    }
}

fn resource_declaration(node: NodeId) -> ResourceNodeDeclaration {
    ResourceNodeDeclaration::new(
        ResourceNodeId::from_node(node),
        ResourcePayloadContract::new(ResourcePayloadContractId::new(7))
            .with_max_payload_bytes(1024),
    )
}

fn timeout_resource_declaration(node: NodeId, timeout_ms: u64) -> ResourceNodeDeclaration {
    resource_declaration(node).with_timeout_policy(
        ResourceTimeoutPolicyDeclaration::RuntimeTimeout {
            timeout: TemporalDuration::temporal_duration(timeout_ms).unwrap(),
        },
    )
}

fn retry_timeout_resource_declaration(
    node: NodeId,
    timeout_ms: u64,
    retry_delay_ms: u64,
) -> ResourceNodeDeclaration {
    timeout_resource_declaration(node, timeout_ms).with_retry_policy(
        ResourceRetryPolicyDeclaration::RuntimeBackoff {
            delay: TemporalDuration::temporal_duration(retry_delay_ms).unwrap(),
        },
    )
}

#[test]
fn resource_policy_lowering_records_built_in_descriptor_identity() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(retry_timeout_resource_declaration(first, 3, 7))
        .expect("first declaration should lower through built-in policy registry");
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(second, 5, 7))
        .expect("second declaration should lower through built-in policy registry");

    let first_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(first))
        .expect("first descriptor should exist");
    let second_descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(second))
        .expect("second descriptor should exist");

    assert_eq!(
        first_descriptor
            .resolved_policy_bundle()
            .retry()
            .descriptor()
            .semantic_name()
            .as_str(),
        "signal.resource.retry.runtime-backoff"
    );
    assert_eq!(
        first_descriptor
            .resolved_policy_bundle()
            .timeout()
            .parameter_digest()
            .as_str(),
        "timeout:runtime-timeout:3"
    );
    assert_ne!(
        first_descriptor
            .resolved_policy_bundle()
            .bundle_digest()
            .as_str(),
        second_descriptor
            .resolved_policy_bundle()
            .bundle_digest()
            .as_str()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_count,
        2
    );
}

#[test]
fn resource_policy_unknown_named_policy_denies_before_descriptor_allocation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let declaration =
        resource_declaration(node).with_retry_policy(ResourceRetryPolicyDeclaration::Named {
            name: ResourcePolicyName::new("example.resource.retry.unregistered"),
        });

    let err = runtime
        .declare_resource_node(declaration)
        .expect_err("unknown named retry policy should deny declaration");

    assert!(err
        .to_string()
        .contains("example.resource.retry.unregistered"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_policy_resolution_denial_count,
        1
    );
}

#[test]
fn resource_policy_registry_rejects_duplicate_descriptor_ids() {
    let first = ResourcePolicyDescriptor::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Retry,
        ResourcePolicyName::new("example.resource.retry.first"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(5),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );
    let second = ResourcePolicyDescriptor::new(
        ResourcePolicyDescriptorId::new(99),
        ResourcePolicyKind::Timeout,
        ResourcePolicyName::new("example.resource.timeout.second"),
        ResourcePolicyVersion::INITIAL,
        ResourceCostContractId::new(4),
        ResourcePolicyCompatibilityPosture::ExactDescriptorMatch,
    );

    let err = FrozenResourcePolicyRegistry::new(vec![
        ResourcePolicyRegistration::new(first),
        ResourcePolicyRegistration::new(second),
    ])
    .expect_err("duplicate policy descriptor ids must deny registry construction");

    assert_eq!(
        err,
        ResourcePolicyRegistryError::DuplicateId(ResourcePolicyDescriptorId::new(99))
    );
}

fn raw_completion(
    runtime: &TestRuntime,
    node: NodeId,
    handle: ResourceRequestHandle,
    attempt: ResourceAttemptId,
    payload_byte_len: u64,
) -> RawCompletionEnvelope {
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        attempt,
        digest,
        payload_byte_len,
    )
}

#[test]
fn resource_declaration_lowers_into_runtime_owned_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let report = runtime
        .declare_resource_node(resource_declaration(node))
        .expect("live node resource declaration should lower");

    assert_eq!(report.descriptor_id(), ResourceDescriptorId::new(0));
    assert_eq!(report.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.lifecycle().output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        report.transition().kind(),
        ResourceLifecycleTransitionKind::DeclarationInitialized
    );
    assert_eq!(
        report.lifecycle().lifecycle_ordinal(),
        report.transition().ordinal()
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(report.performance().broad_scan_denial_count(), 0);

    let summary = runtime.resource_runtime_summary();
    assert_eq!(summary.descriptor_count(), 1);
    assert_eq!(summary.declared_resource_node_count(), 1);
    assert_eq!(summary.next_descriptor_id(), ResourceDescriptorId::new(1));

    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should be indexed by resource node id");
    assert_eq!(descriptor.node(), ResourceNodeId::from_node(node));
    assert_eq!(descriptor.descriptor_id(), ResourceDescriptorId::new(0));
    assert_eq!(
        descriptor.payload_contract_digest().as_str(),
        "payload-contract:7:1024"
    );

    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_declaration_lowering_count,
        1
    );
    assert_eq!(runtime.telemetry().resource.resource_descriptor_count, 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        1
    );
}

#[test]
fn resource_declaration_rejects_non_live_node_owner() {
    let graph = SignalGraph::new();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .declare_resource_node(resource_declaration(NodeId::new(99, 0)))
        .expect_err("resource declarations must be owned by live signal nodes");

    assert!(err.to_string().contains("non-live owner"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_live_owner_denial_count,
        1
    );
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 0);
}

#[test]
fn resource_declaration_rejects_duplicate_node_without_relowering() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("first declaration should lower");
    let err = runtime
        .declare_resource_node(resource_declaration(node))
        .expect_err("duplicate resource declarations for one node should be denied");

    assert!(err
        .to_string()
        .contains("already has a lowered resource descriptor"));
    assert_eq!(runtime.resource_runtime_summary().descriptor_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_duplicate_declaration_denial_count,
        1
    );
}

#[test]
fn resource_request_admission_creates_pending_in_flight_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("declared resource node should admit a request");
    let admitted = report.admitted_request();
    let handle = admitted.handle();

    assert_eq!(handle.request_id(), ResourceRequestId::new(0));
    assert_eq!(handle.generation(), ResourceGeneration::new(1));
    assert_eq!(admitted.attempt(), ResourceAttemptId::ZERO);
    assert_eq!(report.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        report.lifecycle().output_continuity(),
        ResourceOutputContinuity::NoPriorOutput
    );
    assert_eq!(
        report.transition().kind(),
        ResourceLifecycleTransitionKind::RequestAdmitted
    );
    assert_eq!(
        report.transition().from(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(report.transition().to(), ResourceLifecycleClass::Pending);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RequestAdmission
    );
    assert_eq!(report.performance().lifecycle_transition_count(), 1);

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("request handle should resolve through hot in-flight lookup");
    assert_eq!(in_flight.handle(), handle);
    assert_eq!(in_flight.node(), ResourceNodeId::from_node(node));
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Pending);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Active);

    let summary = runtime.resource_runtime_summary();
    assert_eq!(summary.in_flight_request_count(), 1);
    assert_eq!(summary.active_in_flight_node_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_request_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_hot_in_flight_lookup_count,
        1
    );
}

#[test]
fn resource_request_admission_denies_undeclared_resource_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect_err("request admission must require a lowered descriptor");

    assert!(err.to_string().contains("undeclared resource node"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_undeclared_owner_denial_count,
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}

#[test]
fn resource_request_admission_supersedes_prior_active_request_for_node() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit as the active generation");
    let second_handle = second.admitted_request().handle();

    assert_eq!(second.superseded_request(), Some(first));
    let supersession = second
        .supersession_record()
        .expect("second admission should return explicit supersession lineage");
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), second_handle);
    assert_eq!(
        supersession.supersession_ordinal(),
        ResourceSupersessionOrdinal::new(1)
    );
    let superseded_transition = second
        .superseded_transition()
        .expect("second admission should return the supersession transition");
    assert_eq!(supersession.lifecycle_transition(), superseded_transition);
    assert_eq!(
        superseded_transition.kind(),
        ResourceLifecycleTransitionKind::RequestSuperseded
    );
    assert_eq!(
        superseded_transition.from(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        superseded_transition.to(),
        ResourceLifecycleClass::Superseded
    );
    assert_eq!(second_handle.request_id(), ResourceRequestId::new(1));
    assert_eq!(second.performance().lifecycle_transition_count(), 2);
    let superseded = runtime
        .in_flight_resource_request(first)
        .expect("superseded request remains retained for later denial");
    assert_eq!(superseded.status(), ResourceInFlightStatus::Superseded);
    assert_eq!(superseded.superseded_by(), Some(second_handle));
    assert_eq!(
        runtime
            .in_flight_resource_request(second_handle)
            .expect("new request is active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        2
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_superseded_in_flight_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_supersession_record_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_supersession_lineage_width,
        2
    );
}

#[test]
fn resource_cancellation_marks_request_cancelled_and_removes_active_frontier() {
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

    let report = runtime
        .cancel_resource_request(handle, ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire timeout side effects cleanly");

    let cancelled = report
        .cancelled_request()
        .expect("active pending request should cancel");
    let lifecycle = report
        .lifecycle()
        .expect("admitted cancellation should report lifecycle truth");
    let transition = report
        .transition()
        .expect("admitted cancellation should report transition truth");
    assert!(report.denied_cancellation().is_none());

    assert_eq!(cancelled.handle(), handle);
    assert_eq!(
        cancelled.reason(),
        ResourceCancellationReason::HostRequested
    );
    assert_eq!(
        cancelled.cancellation_ordinal(),
        ResourceCancellationOrdinal::new(1)
    );
    assert_eq!(lifecycle.node(), ResourceNodeId::from_node(node));
    assert_eq!(lifecycle.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        transition.kind(),
        ResourceLifecycleTransitionKind::RequestCancelled
    );
    assert_eq!(transition.from(), ResourceLifecycleClass::Pending);
    assert_eq!(transition.to(), ResourceLifecycleClass::Cancelled);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::Cancellation
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("cancelled request remains retained for late completion denial");
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Cancelled);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Cancelled);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(runtime.telemetry().resource.resource_cancellation_count, 1);
}

#[test]
fn resource_cancellation_denies_stale_handle_without_mutating_active_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .cancel_resource_request(first, ResourceCancellationReason::HostRequested)
        .expect("denied cancellation should not trip temporal cleanup");

    let denied = report
        .denied_cancellation()
        .expect("stale superseded handle should be denied");
    assert_eq!(
        denied.class(),
        ResourceCancellationDenialClass::NonActiveRequest
    );
    assert!(report.cancelled_request().is_none());
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_active_cancellation_denial_count,
        1
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second)
            .expect("current request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
}

#[test]
fn resource_request_admission_with_timeout_policy_schedules_temporal_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    let report = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("timeout policy should schedule a runtime-owned wake");
    let handle = report.admitted_request().handle();

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("admitted request should be retained in flight");
    assert_eq!(in_flight.timeout_wake_id(), Some(TemporalWakeId::new(0)));
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_temporal_wake_footprint,
        1
    );
}

#[test]
fn resource_timeout_wake_owner_does_not_alias_node_temporal_owner() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .after(5)
        .expect("temporal evaluation condition should be valid")
        .build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("resource timeout policy should schedule resource-owned wake");
    let node_wake = runtime
        .admit_node_temporal_wake(node)
        .expect("node temporal wake admission should remain independent");

    assert!(node_wake.is_some());
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
}

#[test]
fn resource_timeout_admission_requires_ready_temporal_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let wake_id = runtime
        .in_flight_resource_request(handle)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached to request");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance to timeout tick");
    let ready = runtime
        .promote_temporal_wake_ready(wake_id)
        .expect("timeout wake should promote when due");
    let report = runtime
        .admit_resource_timeout(handle, ready)
        .expect("timeout admission should consume temporal wake cleanly");

    let timed_out = report
        .timed_out_request()
        .expect("matching ready wake should admit timeout");
    assert_eq!(timed_out.handle(), handle);
    assert_eq!(
        timed_out.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::RequestTimedOut
    );
    assert_eq!(
        report
            .lifecycle()
            .expect("timeout should report lifecycle")
            .lifecycle(),
        ResourceLifecycleClass::TimedOut
    );
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::TimeoutAdmission
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("timed out request remains retained for late completion denial")
            .status(),
        ResourceInFlightStatus::TimedOut
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_admission_count,
        1
    );
}

#[test]
fn resource_timeout_admission_denies_wrong_ready_wake_without_mutation() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(first_node, 5))
        .expect("first declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(second_node, 5))
        .expect("second declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request()
        .handle();
    let second_wake = runtime
        .in_flight_resource_request(second)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("second timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance");
    let wrong_ready = runtime
        .promote_temporal_wake_ready(second_wake)
        .expect("second wake should promote");
    let report = runtime
        .admit_resource_timeout(first, wrong_ready)
        .expect("wrong wake denial should not trip temporal cleanup");

    let denied = report
        .denied_timeout()
        .expect("wrong ready wake should be denied");
    assert_eq!(denied.class(), ResourceTimeoutDenialClass::WakeMismatch);
    assert!(report.timed_out_request().is_none());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("first request should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_timeout_wake_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_supersession_retires_prior_timeout_wake_before_it_can_drive_timeout() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("first timeout wake should be attached");

    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");

    let supersession = second
        .supersession_record()
        .expect("supersession should be explicit");
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), second.admitted_request().handle());
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("authoritative clock should advance");
    let err = runtime
        .promote_temporal_wake_ready(first_wake)
        .expect_err("superseded timeout wake must not become ready truth");
    assert!(!err.to_string().is_empty());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("first request should remain retained as superseded")
            .superseded_by(),
        Some(second.admitted_request().handle())
    );
}

#[test]
fn resource_retry_after_timeout_preserves_attempt_lineage_and_backoff_wake_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake")
        .timed_out_request()
        .expect("timeout should admit");

    let schedule = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff");
    let scheduled = schedule
        .scheduled_retry()
        .expect("timed-out request with retry policy should schedule retry");
    assert_eq!(scheduled.previous(), admitted.handle());
    assert_eq!(scheduled.next_attempt(), ResourceAttemptId::new(1));
    assert_eq!(
        schedule.performance().boundary(),
        ResourceBoundaryKind::RetrySchedule
    );
    assert_eq!(schedule.performance().temporal_wake_footprint(), 1);

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("retry backoff wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("retry admission should consume backoff wake");
    let retry = report
        .admitted_retry()
        .expect("matching backoff wake should admit retry");
    let retry_request = retry.admitted_request();

    assert_eq!(retry.scheduled().previous(), admitted.handle());
    assert_eq!(retry_request.attempt(), ResourceAttemptId::new(1));
    assert_eq!(
        retry_request.handle().generation(),
        admitted.handle().generation()
    );
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RetryAdmission
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(retry_request.handle())
            .expect("retry request should be retained")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        1
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_temporal_wake_footprint,
        2
    );
}

#[test]
fn resource_retry_schedule_denies_disabled_policy_without_temporal_wake() {
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
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let report = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("disabled policy denial should stay report-shaped");
    let denied = report
        .denied_retry()
        .expect("retry policy disabled should deny retry scheduling");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryPolicyDisabled
    );
    assert_eq!(report.performance().temporal_wake_footprint(), 0);
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_policy_disabled_denial_count,
        1
    );
}

#[test]
fn resource_retry_schedule_denies_duplicate_without_allocating_second_wake() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");

    let first = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry scheduling should admit");
    let scheduled = first
        .scheduled_retry()
        .expect("first retry should carry a pending backoff wake");
    let second = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("duplicate retry scheduling should stay report-shaped");
    let denied = second
        .denied_retry()
        .expect("duplicate retry should be denied before temporal allocation");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::RetryAlreadyScheduled
    );
    assert_eq!(second.performance().temporal_wake_footprint(), 0);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach original retry backoff");
    assert_eq!(
        runtime
            .promote_temporal_wake_ready(scheduled.backoff_wake_id())
            .expect("original retry wake should remain the only schedulable wake")
            .id(),
        scheduled.backoff_wake_id()
    );
    assert_eq!(
        runtime.telemetry().resource.resource_retry_schedule_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_already_scheduled_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_temporal_wake_footprint,
        1
    );
}

#[test]
fn resource_retry_admission_denies_if_newer_request_wins_before_backoff_ready() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");
    let scheduled = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff")
        .scheduled_retry()
        .expect("retry should be scheduled");

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("fresh admission should win before retry backoff fires");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("retry backoff wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("superseded retry denial should remain report-shaped");
    let denied = report
        .denied_retry()
        .expect("newer request should deny stale retry admission");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::SupersededByNewerRequest
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retry_superseded_denial_count,
        1
    );
}

#[test]
fn resource_pending_retry_handle_is_rekeyed_across_restore_epoch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(node, 3, 7))
        .expect("resource declaration should lower");
    let admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should be ready");
    runtime
        .admit_resource_timeout(admitted.handle(), ready_timeout)
        .expect("timeout admission should consume temporal wake");
    let scheduled = runtime
        .schedule_resource_retry(admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("retry scheduling should use runtime backoff")
        .scheduled_retry()
        .expect("retry should be scheduled");
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("state should mutate after snapshot");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should rekey pending retry handle identity");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled.backoff_wake_id())
        .expect("restored retry backoff wake should become ready");
    let report = runtime
        .admit_scheduled_resource_retry(admitted.handle(), ready_retry)
        .expect("stale retry handle denial should be report-shaped");
    let denied = report
        .denied_retry()
        .expect("pre-restore retry handle must not admit after restore");

    assert_eq!(
        denied.class(),
        ResourceRetryDenialClass::UnknownOrStaleRequest
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_stale_retry_denial_count,
        1
    );
}

#[test]
fn resource_revalidation_admits_new_generation_when_no_request_is_active() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("live declared resource should admit revalidation");
    let revalidation = report
        .admitted_revalidation()
        .expect("no-active revalidation should admit");
    let admitted = revalidation.admitted_request();

    assert_eq!(revalidation.expected_active(), None);
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(1));
    assert_eq!(admitted.attempt(), ResourceAttemptId::ZERO);
    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::RevalidationAdmission
    );
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    assert_eq!(
        runtime
            .in_flight_resource_request(admitted.handle())
            .expect("revalidation request should be retained")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_admission_count,
        1
    );
}

#[test]
fn resource_revalidation_requires_expected_handle_when_active_request_exists() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            node,
        )))
        .expect("expected-handle denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("ambient active request should require explicit expected handle");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ActiveRequestRequiresExpectedHandle
    );
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_revalidation_active_requires_expected_denial_count,
        1
    );
}

#[test]
fn resource_revalidation_supersedes_only_the_expected_active_handle() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request()
        .handle();
    let first_wake = runtime
        .in_flight_resource_request(first)
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should be attached");

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("expected active request should revalidate");
    let revalidation = report
        .admitted_revalidation()
        .expect("expected active revalidation should admit");
    let admitted = revalidation.admitted_request();
    let supersession = revalidation
        .supersession_record()
        .expect("revalidation should retain explicit supersession lineage");

    assert_eq!(revalidation.expected_active(), Some(first));
    assert_eq!(supersession.previous(), first);
    assert_eq!(supersession.replacing(), admitted.handle());
    assert_eq!(admitted.handle().generation(), ResourceGeneration::new(2));
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert!(runtime.promote_temporal_wake_ready(first_wake).is_err());
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("prior request should be retained")
            .status(),
        ResourceInFlightStatus::Superseded
    );
}

#[test]
fn resource_revalidation_denies_stale_expected_handle_after_newer_generation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first")
        .admitted_request()
        .handle();

    let report = runtime
        .revalidate_resource_node(ResourceRevalidationIntent::with_expected_active(
            ResourceNodeId::from_node(node),
            first,
        ))
        .expect("stale expected denial should be report-shaped");
    let denied = report
        .denied_revalidation()
        .expect("stale expected handle must not overwrite newer active request");

    assert_eq!(
        denied.class(),
        ResourceRevalidationDenialClass::ExpectedActiveRequestMismatch
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
            .resource_revalidation_expected_mismatch_denial_count,
        1
    );
}

#[test]
fn resource_snapshot_restore_rekeys_in_flight_handles_to_new_restore_epoch() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let pre_restore = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit before snapshot")
        .admitted_request()
        .handle();
    assert_eq!(pre_restore.branch_epoch().restore_epoch(), 0);
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate resource state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate captured resource state");

    assert!(
        runtime.in_flight_resource_request(pre_restore).is_none(),
        "pre-restore handles must not resolve after branch restore changes the resource epoch"
    );
    assert_eq!(
        runtime.telemetry().resource.resource_branch_restore_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_in_flight_width,
        1
    );

    let post_restore = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("restored resource state should admit a new epoch-safe request");
    assert_eq!(
        post_restore
            .superseded_request()
            .expect("restored in-flight request should be superseded")
            .branch_epoch()
            .restore_epoch(),
        1
    );
    assert_eq!(
        post_restore
            .admitted_request()
            .handle()
            .branch_epoch()
            .restore_epoch(),
        1
    );
}

#[test]
fn resource_request_identity_is_not_node_identity() {
    let node = NodeId::new(3, 0);
    let resource_node = ResourceNodeId::from_node(node);
    let request = ResourceRequestId::new(3);

    assert_eq!(resource_node.node(), node);
    assert_eq!(request.get(), node.index() as u64);
}

#[test]
fn resource_completion_admission_accepts_matching_active_request_without_committing_lifecycle() {
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
    let handle = admitted_request.handle();

    let report = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        handle,
        admitted_request.attempt(),
        64,
    ));

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::CompletionAdmission
    );
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(report.performance().lifecycle_transition_count(), 1);
    let completion = report
        .admitted_completion()
        .expect("matching envelope should admit");
    assert_eq!(completion.handle(), handle);
    assert_eq!(completion.node(), ResourceNodeId::from_node(node));
    assert_eq!(completion.payload_byte_len(), 64);
    assert_eq!(
        completion.lifecycle_transition().kind(),
        ResourceLifecycleTransitionKind::CompletionAdmitted
    );
    assert_eq!(
        completion.lifecycle_transition().from(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        completion.lifecycle_transition().to(),
        ResourceLifecycleClass::Fulfilled
    );
    assert_eq!(
        completion.completion_ordinal(),
        ResourceCompletionOrdinal::new(1)
    );
    assert!(report.denied_completion().is_none());

    let in_flight = runtime
        .in_flight_resource_request(handle)
        .expect("admission must not retire or mutate in-flight state before apply");
    assert_eq!(in_flight.lifecycle(), ResourceLifecycleClass::Pending);
    assert_eq!(in_flight.status(), ResourceInFlightStatus::Active);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_validation_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_denial_count,
        0
    );
}

#[test]
fn resource_completion_stage_and_commit_apply_lifecycle_once() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");

    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage");
    assert_eq!(
        staging.performance().boundary(),
        ResourceBoundaryKind::CompletionStaging
    );
    assert_eq!(staging.performance().lifecycle_transition_count(), 0);
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("staging must not mutate request lifecycle")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);

    let commit = runtime
        .commit_staged_resource_completion(staging.staged_effect())
        .expect("staged completion should commit exactly once");

    assert_eq!(
        commit.performance().boundary(),
        ResourceBoundaryKind::CompletionCommit
    );
    assert_eq!(commit.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        commit.lifecycle().lifecycle(),
        ResourceLifecycleClass::Fulfilled
    );
    assert_eq!(
        commit.transition().kind(),
        ResourceLifecycleTransitionKind::CompletionAdmitted
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("fulfilled request remains retained for audit")
            .status(),
        ResourceInFlightStatus::Fulfilled
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        0
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_staging_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        1
    );
}

#[test]
fn resource_completion_rollback_of_staged_admitted_preserves_pending_request() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");

    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("active admitted completion should stage")
        .staged_effect();
    let rollback = runtime.rollback_staged_resource_completion(staged);

    assert_eq!(
        rollback.performance().boundary(),
        ResourceBoundaryKind::CompletionRollback
    );
    assert_eq!(rollback.performance().admitted_count(), 1);
    assert_eq!(rollback.performance().denied_count(), 0);
    assert_eq!(rollback.performance().lifecycle_transition_count(), 0);
    assert_eq!(
        rollback.rolled_back_completion().subject(),
        ResourceCompletionRollbackSubject::Admitted {
            handle,
            node: ResourceNodeId::from_node(node),
            completion_ordinal: ResourceCompletionOrdinal::new(1),
        }
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("rollback must leave request available for a later commit")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_rollback_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        0
    );
}

#[test]
fn resource_completion_transaction_commit_delivers_lifecycle_observation_once() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    let observation_handle = runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );

    let mut ctx = ();
    let result = runtime
        .transaction(&mut ctx, |tx| {
            let staging = tx.stage_admitted_resource_completion(admitted_completion)?;
            tx.commit_staged_resource_completion(staging.staged_effect())?;
            Ok(())
        })
        .expect("completion transaction should commit");

    let recorded = calls
        .lock()
        .expect("resource observation mutex poisoned")
        .clone();
    assert_eq!(
        recorded,
        vec![ResourceObservationRecord {
            observer_id: observation_handle.observer_id().get(),
            handle_id: observation_handle.handle_id().get(),
            matched_node_count: 1,
            touched: true,
            recomputed: false,
            meaningful_change: true,
            trigger_matched: true,
        }]
    );
    assert_eq!(result.observation.classified_event_count, 1);
    assert_eq!(result.observation.trigger_matched_event_count, 1);
    assert_eq!(result.observation.delivered_event_count, 1);
    assert_eq!(result.observation.rollback_suppressed_event_count, 0);
    assert_eq!(
        result.observation.boundary_events[0]
            .matched_nodes
            .iter()
            .collect::<Vec<_>>(),
        vec![node]
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("fulfilled request remains retained for audit")
            .status(),
        ResourceInFlightStatus::Fulfilled
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 1);
    assert_eq!(
        runtime.telemetry().transaction.delivered_observation_count,
        1
    );
}

#[test]
fn resource_completion_transaction_rollback_suppresses_observation_and_restores_state() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(timeout_resource_declaration(node, 5))
        .expect("resource declaration should lower");
    let admitted_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit")
        .admitted_request();
    let handle = admitted_request.handle();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            handle,
            admitted_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("matching completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );

    let mut ctx = ();
    let mut tx = runtime.begin(&mut ctx);
    let staging = tx
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage inside transaction");
    tx.commit_staged_resource_completion(staging.staged_effect())
        .expect("completion should mutate transaction-local resource state");
    let result = tx
        .rollback()
        .expect("rollback should restore resource and temporal state");

    assert!(
        calls
            .lock()
            .expect("resource observation mutex poisoned")
            .is_empty(),
        "rollback must suppress completion-driven observation delivery"
    );
    assert_eq!(result.observation.classified_event_count, 1);
    assert_eq!(result.observation.trigger_matched_event_count, 1);
    assert_eq!(result.observation.delivered_event_count, 0);
    assert_eq!(result.observation.rollback_suppressed_event_count, 1);
    assert!(matches!(
        result.observation.boundary_events[0].outcome,
        ObservationBoundaryOutcome::RollbackSuppressed
    ));
    assert_eq!(
        runtime
            .in_flight_resource_request(handle)
            .expect("rollback must restore active request")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 1);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_resource_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_packet_temporal_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .transaction
            .rollback_suppressed_observation_count,
        1
    );
}

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

#[test]
fn resource_completion_rollback_of_staged_denied_preserves_retained_denial_without_mutation() {
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

    let denied = runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce a retained denial");
    let denial_id = denied.denial_id();
    let request_id = denied.request_id();

    let staging = runtime
        .stage_denied_resource_completion(denied)
        .expect("retained denied completion should stage");
    assert_eq!(
        staging.performance().boundary(),
        ResourceBoundaryKind::CompletionDenialStaging
    );
    assert_eq!(staging.performance().admitted_count(), 0);
    assert_eq!(staging.performance().denied_count(), 1);

    let rollback =
        runtime.rollback_staged_denied_resource_completion(staging.staged_denial_effect());
    assert_eq!(
        rollback.performance().boundary(),
        ResourceBoundaryKind::CompletionRollback
    );
    assert_eq!(rollback.performance().admitted_count(), 0);
    assert_eq!(rollback.performance().denied_count(), 1);
    assert_eq!(
        rollback.rolled_back_completion().subject(),
        ResourceCompletionRollbackSubject::Denied {
            denial_id,
            class: CompletionDenialClass::UnknownRequest,
            request_id,
        }
    );
    assert_eq!(
        runtime.resource_runtime_summary().denied_completion_count(),
        1
    );
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_denial_staging_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_rollback_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_commit_count,
        0
    );
}

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
    let snapshot = runtime.capture_snapshot();

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
    let snapshot = runtime.capture_snapshot();

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
