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

#[test]
fn resource_lifecycle_policy_initial_class_is_compile_time_constrained_to_unrequested() {
    let policy =
        ResourceLifecyclePolicyDeclaration::new(ResourceInitialLifecycleClass::unrequested());
    assert_eq!(policy.initial(), ResourceLifecycleClass::Unrequested);

    let encoded = serde_json::to_string(&ResourceInitialLifecycleClass::unrequested()).unwrap();
    assert_eq!(encoded, "\"Unrequested\"");
    let decoded: ResourceInitialLifecycleClass = serde_json::from_str(&encoded).unwrap();
    assert_eq!(decoded.lifecycle(), ResourceLifecycleClass::Unrequested);

    let rejected = serde_json::from_str::<ResourceInitialLifecycleClass>("\"Pending\"");
    assert!(rejected
        .expect_err("runtime lifecycle classes must not deserialize as initial policy")
        .to_string()
        .contains("Unrequested"));
    let policy_encoded = serde_json::to_string(&policy).unwrap();
    assert_eq!(policy_encoded, "{\"initial\":\"Unrequested\"}");
    let policy_decoded: ResourceLifecyclePolicyDeclaration =
        serde_json::from_str(&policy_encoded).unwrap();
    assert_eq!(
        policy_decoded.initial(),
        ResourceLifecycleClass::Unrequested
    );
    let rejected_policy =
        serde_json::from_str::<ResourceLifecyclePolicyDeclaration>("{\"initial\":\"Fulfilled\"}");
    assert!(rejected_policy
        .expect_err("policy declarations must reject terminal initial lifecycle data")
        .to_string()
        .contains("Unrequested"));
    let mut declaration_graph = SignalGraph::new();
    let declaration_node = declaration_graph.node().build();
    let declaration = resource_declaration(declaration_node);
    let mut declaration_value =
        serde_json::to_value(&declaration).expect("resource declaration should serialize");
    declaration_value["lifecycle_policy"]["initial"] = serde_json::json!("TimedOut");
    let rejected_declaration = serde_json::from_value::<ResourceNodeDeclaration>(declaration_value);
    assert!(rejected_declaration
        .expect_err("resource declarations must reject impossible initial lifecycle policy data")
        .to_string()
        .contains("Unrequested"));

    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    let report = runtime
        .declare_resource_node(resource_declaration(node).with_lifecycle_policy(policy))
        .expect("resource declaration should accept the constrained initial policy");

    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.transition().from(),
        ResourceLifecycleClass::Unrequested
    );
    assert_eq!(
        report.transition().to(),
        ResourceLifecycleClass::Unrequested
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
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::SparseIndexedLookup
    );

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
    assert_eq!(
        second.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
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
    let boundary_envelopes_at_snapshot = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should mutate resource state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate captured resource state");
    let restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish a report");

    assert!(
        runtime.in_flight_resource_request(pre_restore).is_none(),
        "pre-restore handles must not resolve after branch restore changes the resource epoch"
    );
    assert_eq!(
        restore_report.performance().boundary(),
        ResourceBoundaryKind::BranchRestore
    );
    assert_eq!(restore_report.performance().cost_contract().get(), 13);
    assert_eq!(
        restore_report.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(restore_report.restored_in_flight_width(), 1);
    assert_eq!(restore_report.retained_summary_width(), 1);
    assert_eq!(restore_report.broad_rebuild_denial_count(), 1);
    assert_eq!(restore_report.performance().broad_scan_denial_count(), 1);
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
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_retained_summary_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_broad_rebuild_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_at_snapshot + 1
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
fn resource_replay_reconstruction_digest_matches_after_snapshot_restore() {
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
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(9_999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce retained denial");
    let snapshot = runtime.capture_snapshot();
    let expected = runtime.reconstruct_resource_replay_summary();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot request should mutate resource state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate captured resource state");
    let boundary_envelopes_before_replay = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;
    let replayed = runtime.reconstruct_resource_replay_summary();

    assert_eq!(
        replayed.performance().boundary(),
        ResourceBoundaryKind::ReplayReconstruction
    );
    assert_eq!(replayed.performance().cost_contract().get(), 14);
    assert_eq!(
        replayed.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(replayed.descriptor_width(), 1);
    assert_eq!(replayed.lifecycle_summary_width(), 1);
    assert_eq!(replayed.denied_completion_width(), 1);
    assert_eq!(replayed.in_flight_width(), 0);
    assert_eq!(replayed.retained_history_unavailable_count(), 0);
    assert_eq!(replayed.descriptor_digest(), expected.descriptor_digest());
    assert_eq!(replayed.lifecycle_digest(), expected.lifecycle_digest());
    assert_eq!(
        replayed.denied_completion_digest(),
        expected.denied_completion_digest()
    );
    assert_eq!(replayed.in_flight_digest(), expected.in_flight_digest());
    assert_eq!(replayed.replay_digest(), expected.replay_digest());
    assert_eq!(replayed.performance().input_width(), 3);
    assert_eq!(replayed.performance().lifecycle_transition_count(), 1);
    assert_eq!(replayed.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_lifecycle_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_denial_width,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before_replay + 1
    );
}

#[test]
fn resource_certification_bundle_requires_all_named_phase10_families() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let first_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let first_request = first_admission.admitted_request();
    let second_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first request");
    let superseded_request = second_admission
        .superseded_request()
        .expect("second admission should retain supersession evidence");
    assert_eq!(superseded_request, first_request.handle());
    let second_request = second_admission.admitted_request();
    let snapshot = runtime.capture_snapshot();

    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            second_request.handle(),
            second_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("current completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("admitted completion should stage");
    let rollback = runtime.rollback_staged_resource_completion(staging.staged_effect());

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(&replay)
        .expect("lifecycle parity evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(rollback)
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(restore_report, &replay)
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            first_admission.performance(),
        )
        .expect("boundedness evidence should be accepted")
        .build()
        .expect("complete resource certification bundle should pass");

    assert!(bundle.passed());
    assert_eq!(
        bundle.schema_version(),
        RESOURCE_CERTIFICATION_BUNDLE_SCHEMA_VERSION
    );
    assert_eq!(
        bundle.records().len(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len()
    );
    assert_eq!(
        bundle.summary().passed_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(bundle.summary().missing_family_count(), 0);
    assert_eq!(bundle.summary().duplicate_family_count(), 0);
    assert!(bundle.failures().is_empty());
    assert!(bundle
        .records()
        .iter()
        .all(|record| record.performance().cost_contract().get() > 0));
}

#[test]
fn resource_certification_bundle_reports_missing_duplicate_and_parity_drift() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("request should admit");
    let performance = admission.performance();
    let missing_supersession = resource_certification_builder()
        .with_out_of_order_completion_supersession(admission)
        .expect_err("supersession family must require real supersession evidence");
    assert!(format!("{missing_supersession}")
        .contains("requires request admission with supersession evidence"));

    let lifecycle = ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncResourceLifecycleParity,
        "lifecycle",
        performance,
    )
    .expect("non-empty evidence digest should certify a record");
    let duplicate_lifecycle = ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncResourceLifecycleParity,
        "lifecycle-duplicate",
        performance,
    )
    .expect("duplicate family is reported at bundle construction");
    let partial = resource_certification_bundle([lifecycle.clone(), duplicate_lifecycle]);

    assert!(!partial.passed());
    assert_eq!(partial.summary().missing_family_count(), 4);
    assert_eq!(partial.summary().duplicate_family_count(), 1);
    assert!(partial.failures().iter().any(|failure| matches!(
        failure,
        ResourceCertificationFailure::DuplicateFamily {
            family: ResourceCertificationFamily::AsyncResourceLifecycleParity,
            count: 2
        }
    )));

    let complete = resource_certification_fixture_bundle(ResourceRequestId::new(9_999));
    let drifted = resource_certification_fixture_bundle(ResourceRequestId::new(9_998));
    let parity = resource_certification_bundle_parity_report(&complete, &drifted);

    assert!(!parity.parity());
    assert!(parity
        .mismatch_classes()
        .contains(&ResourceCertificationBundleMismatchClass::BundleDigestMismatch));
    assert!(parity
        .mismatch_classes()
        .contains(&ResourceCertificationBundleMismatchClass::RecordSetMismatch));
    assert!(ResourceCertificationRecord::passing(
        ResourceCertificationFamily::AsyncInflightBoundedness,
        "",
        performance,
    )
    .is_err());
    let builder_err = resource_certification_builder()
        .with_async_inflight_boundedness(runtime.resource_runtime_summary(), performance)
        .expect("first lifecycle record should be accepted")
        .with_async_inflight_boundedness(runtime.resource_runtime_summary(), performance)
        .expect_err("duplicate builder family must reject before bundle construction");
    assert!(format!("{builder_err}").contains("duplicate certification family evidence"));
}

#[test]
fn resource_milestone_b_certification_run_requires_complete_passing_bundle() {
    let (complete, hostile_evidence, summary_read, diagnostics_summary, diagnostics_denial) =
        resource_certification_fixture_artifacts(ResourceRequestId::new(9_999));
    let scenario_matrix = resource_milestone_b_scenario_matrix(&complete, &hostile_evidence)
        .expect("complete passing resource bundle should produce scenario matrix");
    let performance_closeout = resource_milestone_b_performance_closeout(
        &scenario_matrix,
        summary_read,
        diagnostics_summary,
        diagnostics_denial,
    )
    .expect("complete passing resource evidence should produce performance closeout");
    let run = resource_milestone_b_certification_run(
        complete.clone(),
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect("complete passing resource bundle should close milestone B certification");

    assert!(run.passed());
    assert!(scenario_matrix.passed());
    assert!(performance_closeout.passed());
    assert_eq!(
        run.schema_version(),
        RESOURCE_MILESTONE_B_CERTIFICATION_RUN_SCHEMA_VERSION
    );
    assert_eq!(run.bundle().bundle_digest(), complete.bundle_digest());
    assert_eq!(run.scenario_matrix(), &scenario_matrix);
    assert_eq!(run.performance_closeout(), &performance_closeout);
    assert_eq!(
        scenario_matrix.schema_version(),
        RESOURCE_MILESTONE_B_SCENARIO_MATRIX_SCHEMA_VERSION
    );
    assert_eq!(scenario_matrix.bundle_digest(), complete.bundle_digest());
    assert_eq!(
        scenario_matrix.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len()
    );
    assert_eq!(
        hostile_evidence.schema_version(),
        RESOURCE_MILESTONE_B_HOSTILE_SCENARIO_EVIDENCE_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.schema_version(),
        RESOURCE_MILESTONE_B_PERFORMANCE_CLOSEOUT_SCHEMA_VERSION
    );
    assert_eq!(
        performance_closeout.scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        performance_closeout.rows().len(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len()
    );
    assert!(REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS
        .iter()
        .all(|scenario| scenario_matrix
            .rows()
            .iter()
            .any(|row| row.id() == *scenario
                && row.certification_family() == scenario.certification_family()
                && row.completion_denial_class() == scenario.completion_denial_class()
                && row.passed())));
    assert!(REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS
        .iter()
        .all(|scenario| scenario_matrix
            .rows()
            .iter()
            .any(|row| row.id() == *scenario
                && row.evidence_kind()
                    == ResourceMilestoneBScenarioEvidenceKind::HostileCompletionDenial
                && row.completion_denial_class() == scenario.completion_denial_class()
                && row.certification_family().is_none()
                && row.passed())));
    assert!(REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS
        .iter()
        .all(|claim| performance_closeout
            .rows()
            .iter()
            .any(|row| row.id() == *claim && row.passed() && !row.evidence_digest().is_empty())));
    let replay_claim = performance_closeout
        .rows()
        .iter()
        .find(|row| {
            row.id() == ResourceMilestoneBPerformanceClaimId::LifecycleReplayParityDebtBounded
        })
        .expect("replay performance claim should be present");
    assert_eq!(
        replay_claim.performance().boundary(),
        ResourceBoundaryKind::ReplayReconstruction
    );
    assert_eq!(
        replay_claim.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        replay_claim.performance().diagnostics_allocation_count(),
        replay_claim.performance().input_width()
    );
    let supersession_claim = performance_closeout
        .rows()
        .iter()
        .find(|row| {
            row.id() == ResourceMilestoneBPerformanceClaimId::OutOfOrderSupersessionAdmissionBounded
        })
        .expect("supersession performance claim should be present");
    assert_eq!(
        supersession_claim.performance().boundary(),
        ResourceBoundaryKind::RequestAdmission
    );
    assert_eq!(
        supersession_claim.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(
        supersession_claim
            .performance()
            .lifecycle_transition_count(),
        2
    );
    let summary_read_claim = performance_closeout
        .rows()
        .iter()
        .find(|row| {
            row.id()
                == ResourceMilestoneBPerformanceClaimId::RuntimeSummaryReadZeroColdReconstruction
        })
        .expect("summary read claim should be present");
    assert_eq!(
        summary_read_claim.performance().boundary(),
        ResourceBoundaryKind::SummaryRead
    );
    assert_eq!(
        summary_read_claim
            .performance()
            .diagnostics_allocation_count(),
        0
    );
    let hostile_claim = performance_closeout
        .rows()
        .iter()
        .find(|row| {
            row.id() == ResourceMilestoneBPerformanceClaimId::HostileCompletionDenialsScalarBounded
        })
        .expect("hostile performance claim should be present");
    assert_eq!(
        hostile_claim.performance().denied_count(),
        REQUIRED_RESOURCE_MILESTONE_B_HOSTILE_SCENARIOS.len() as u32
    );
    assert_eq!(hostile_claim.performance().lifecycle_transition_count(), 0);
    assert_eq!(
        run.summary().required_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(
        run.summary().certified_family_count(),
        REQUIRED_RESOURCE_CERTIFICATION_FAMILIES.len() as u32
    );
    assert_eq!(run.summary().failed_family_count(), 0);
    assert_eq!(run.summary().bundle_digest(), complete.bundle_digest());
    assert_eq!(
        run.summary().required_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().certified_scenario_count(),
        REQUIRED_RESOURCE_MILESTONE_B_SCENARIOS.len() as u32
    );
    assert_eq!(
        run.summary().scenario_matrix_digest(),
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        run.summary().required_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().certified_performance_claim_count(),
        REQUIRED_RESOURCE_MILESTONE_B_PERFORMANCE_CLAIMS.len() as u32
    );
    assert_eq!(
        run.summary().performance_closeout_digest(),
        performance_closeout.closeout_digest()
    );
    assert!(!run.run_digest().is_empty());
    let serialized_run =
        serde_json::to_value(&run).expect("closeout certification run should serialize");
    assert_eq!(
        serialized_run["scenarioMatrix"]["matrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["scenarioMatrixDigest"],
        scenario_matrix.matrix_digest()
    );
    assert_eq!(
        serialized_run["summary"]["performanceCloseoutDigest"],
        performance_closeout.closeout_digest()
    );

    let incomplete = resource_certification_bundle([]);
    let err = resource_milestone_b_scenario_matrix(&incomplete, &hostile_evidence)
        .expect_err("incomplete certification bundle must not become scenario evidence");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let err = resource_milestone_b_certification_run(
        incomplete,
        scenario_matrix.clone(),
        performance_closeout.clone(),
    )
    .expect_err("incomplete certification bundle must not become a milestone run");
    assert!(format!("{err}").contains("resource certification bundle failed"));
    let misclassified_hostile = resource_milestone_b_hostile_scenario_evidence(
        resource_late_cancelled_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
    )
    .expect_err("hostile scenario evidence must reject the wrong denial class per row");
    assert!(format!("{misclassified_hostile}").contains("requires Superseded denial evidence"));

    let (
        drifted,
        drifted_hostile_evidence,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    ) = resource_certification_fixture_artifacts(ResourceRequestId::new(9_998));
    let drifted_matrix = resource_milestone_b_scenario_matrix(&drifted, &drifted_hostile_evidence)
        .expect("drifted but complete bundle should produce its own scenario matrix");
    let drifted_performance_closeout = resource_milestone_b_performance_closeout(
        &drifted_matrix,
        drifted_summary_read,
        drifted_diagnostics_summary,
        drifted_diagnostics_denial,
    )
    .expect("drifted but complete evidence should produce performance closeout");
    let wrong_matrix_err = resource_milestone_b_certification_run(
        complete,
        drifted_matrix.clone(),
        drifted_performance_closeout.clone(),
    )
    .expect_err("scenario matrix from a different bundle must not close the run");
    assert!(format!("{wrong_matrix_err}").contains("same bundle"));
    let wrong_performance_err = resource_milestone_b_certification_run(
        drifted.clone(),
        drifted_matrix.clone(),
        performance_closeout,
    )
    .expect_err("performance closeout from a different matrix must not close the run");
    assert!(format!("{wrong_performance_err}").contains("same scenario matrix"));
    let drifted_run = resource_milestone_b_certification_run(
        drifted,
        drifted_matrix,
        drifted_performance_closeout,
    )
    .expect("drifted but complete bundle should still produce its own run");
    assert_ne!(
        run.bundle().bundle_digest(),
        drifted_run.bundle().bundle_digest()
    );
    assert_ne!(
        run.scenario_matrix().matrix_digest(),
        drifted_run.scenario_matrix().matrix_digest()
    );
    assert_ne!(run.run_digest(), drifted_run.run_digest());
}

fn resource_certification_fixture_bundle(
    retained_denial_request_id: ResourceRequestId,
) -> ResourceCertificationBundle {
    resource_certification_fixture_artifacts(retained_denial_request_id).0
}

fn resource_certification_fixture_artifacts(
    retained_denial_request_id: ResourceRequestId,
) -> (
    ResourceCertificationBundle,
    ResourceMilestoneBHostileScenarioEvidence,
    ResourceRuntimeSummaryReadReport,
    ResourceDiagnosticsSummary,
    ResourceDiagnosticsExpansionDenial,
) {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            retained_denial_request_id,
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown request should produce retained denial evidence");
    let first_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit");
    let second_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first request");
    let second_request = second_admission.admitted_request();
    let snapshot = runtime.capture_snapshot();
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            second_request.handle(),
            second_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("current completion should admit");
    let staging = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("admitted completion should stage");
    let rollback = runtime.rollback_staged_resource_completion(staging.staged_effect());
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let restore = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(&replay)
        .expect("lifecycle evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(rollback)
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(restore, &replay)
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            runtime.resource_runtime_summary(),
            first_admission.performance(),
        )
        .expect("boundedness evidence should be accepted")
        .build()
        .expect("complete fixture bundle should pass");
    let summary_read = runtime.resource_runtime_summary_read_report();
    let diagnostics_summary =
        runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();
    let diagnostics_denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-only diagnostics budget should deny cold reconstruction");
    let hostile_evidence = resource_milestone_b_hostile_scenario_evidence(
        resource_late_superseded_completion_report(),
        resource_late_cancelled_completion_report(),
        resource_late_timed_out_completion_report(),
        resource_malformed_completion_report(),
    )
    .expect("hostile completion evidence should cover required denial lanes");
    (
        bundle,
        hostile_evidence,
        summary_read,
        diagnostics_summary,
        diagnostics_denial,
    )
}

fn resource_late_superseded_completion_report() -> ResourceCompletionAdmissionReport {
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
    runtime.admit_resource_completion(stale_first)
}

fn resource_late_cancelled_completion_report() -> ResourceCompletionAdmissionReport {
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
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should retire request");
    runtime.admit_resource_completion(late)
}

fn resource_late_timed_out_completion_report() -> ResourceCompletionAdmissionReport {
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
    runtime
        .admit_resource_timeout(admitted.handle(), ready)
        .expect("timeout admission should consume wake");
    runtime.admit_resource_completion(late)
}

fn resource_malformed_completion_report() -> ResourceCompletionAdmissionReport {
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
    runtime.admit_resource_completion(RawCompletionEnvelope::new(
        handle.request_id(),
        handle.generation(),
        handle.branch_epoch(),
        admitted.attempt(),
        ResourcePayloadContractDigest::new("payload-contract:999:1024"),
        64,
    ))
}

#[test]
fn resource_diagnostics_summary_preserves_truth_and_exposes_replay_debt() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            ResourceRequestId::new(9_999),
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    let runtime_summary_before = runtime.resource_runtime_summary();
    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let diagnostics = runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction();

    assert_eq!(
        diagnostics.schema_version(),
        RESOURCE_DIAGNOSTICS_SUMMARY_SCHEMA_VERSION
    );
    assert_eq!(diagnostics.runtime_summary(), runtime_summary_before);
    assert_eq!(runtime.resource_runtime_summary(), runtime_summary_before);
    assert!(diagnostics.latest_branch_restore_report().is_none());
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        diagnostics.performance().cost_posture(),
        ResourceCostPosture::Debt
    );
    assert_eq!(
        diagnostics
            .expansion_budget()
            .max_replay_reconstruction_width(),
        u32::MAX
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .denied_completion_width(),
        1
    );
    assert_eq!(
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().diagnostics_allocation_count(),
        diagnostics
            .replay_reconstruction()
            .performance()
            .input_width()
    );
    assert_eq!(
        diagnostics.performance().facade_report_allocation_count(),
        1
    );
    assert_eq!(diagnostics.performance().operational_allocation_count(), 0);
    assert_eq!(
        diagnostics
            .performance()
            .retained_history_allocation_count(),
        0
    );
    assert!(!diagnostics.provenance_digest().is_empty());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before + 1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before.resource_diagnostics_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .diagnostics_allocation_count() as u64
            + diagnostics.performance().diagnostics_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        diagnostics
            .replay_reconstruction()
            .performance()
            .facade_report_allocation_count() as u64
            + diagnostics.performance().facade_report_allocation_count() as u64
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
}

#[test]
fn resource_runtime_summary_read_report_is_zero_cold_reconstruction() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let replay_count_before = runtime
        .telemetry()
        .resource
        .resource_replay_reconstruction_count;
    let diagnostics_expansion_before = runtime
        .telemetry()
        .resource
        .resource_diagnostics_expansion_count;
    let allocation_telemetry_before = runtime.telemetry().resource;
    let report = runtime.resource_runtime_summary_read_report();

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::SummaryRead
    );
    assert_eq!(
        report.performance().cost_posture(),
        ResourceCostPosture::Verified
    );
    assert_eq!(report.performance().operational_allocation_count(), 0);
    assert_eq!(report.performance().retained_history_allocation_count(), 0);
    assert_eq!(report.performance().diagnostics_allocation_count(), 0);
    assert_eq!(report.performance().facade_report_allocation_count(), 1);
    assert_eq!(report.performance().broad_scan_denial_count(), 0);
    assert_eq!(report.summary(), runtime.resource_runtime_summary());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_summary_read_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        replay_count_before
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        diagnostics_expansion_before
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count,
        allocation_telemetry_before.resource_diagnostics_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        1
    );
}

#[test]
fn resource_diagnostics_summary_respects_cold_reconstruction_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let allocation_telemetry_before = runtime.telemetry().resource;

    let err = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::retained_summary_only(),
        )
        .expect_err("retained-summary-only diagnostics should deny replay reconstruction");

    assert_eq!(
        err.class(),
        ResourceDiagnosticsExpansionDenialClass::ColdReconstructionDisabled
    );
    assert_eq!(err.replay_reconstruction_width(), 2);
    assert_eq!(
        err.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(err.performance().denied_count(), 1);
    assert_eq!(
        err.performance().cost_posture(),
        ResourceCostPosture::DeniedFallback
    );
    assert_eq!(err.performance().operational_allocation_count(), 0);
    assert_eq!(err.performance().retained_history_allocation_count(), 0);
    assert_eq!(err.performance().diagnostics_allocation_count(), 0);
    assert_eq!(err.performance().facade_report_allocation_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_expansion_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_cold_reconstruction_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_operational_allocation_count,
        allocation_telemetry_before.resource_operational_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_allocation_count,
        allocation_telemetry_before.resource_retained_history_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count,
        allocation_telemetry_before.resource_diagnostics_allocation_count
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before.resource_facade_report_allocation_count,
        1
    );

    let allocation_telemetry_before_admission = runtime.telemetry().resource;
    let admitted = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
        )
        .expect("budget that admits descriptor plus lifecycle reconstruction should pass");

    assert_eq!(
        admitted.performance().boundary(),
        ResourceBoundaryKind::DiagnosticsExpansion
    );
    assert_eq!(
        admitted
            .expansion_budget()
            .max_replay_reconstruction_width(),
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        1
    );
    assert_eq!(admitted.performance().diagnostics_allocation_count(), 2);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_diagnostics_allocation_count
            - allocation_telemetry_before_admission.resource_diagnostics_allocation_count,
        4
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_facade_report_allocation_count
            - allocation_telemetry_before_admission.resource_facade_report_allocation_count,
        2
    );
}

#[test]
fn resource_diagnostics_summary_denies_when_replay_width_exceeds_budget() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");

    let denial = runtime
        .try_resource_diagnostics_summary(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(1),
        )
        .expect_err("descriptor plus lifecycle width should exceed budget one");

    assert_eq!(
        denial.class(),
        ResourceDiagnosticsExpansionDenialClass::ReplayReconstructionBudgetExceeded
    );
    assert_eq!(denial.budget().max_replay_reconstruction_width(), 1);
    assert_eq!(denial.replay_reconstruction_width(), 2);
    assert_eq!(denial.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_replay_reconstruction_count,
        0
    );
}

#[test]
fn resource_diagnostics_summary_digest_tracks_retained_denial_drift() {
    let left = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_999));
    let right = resource_diagnostics_summary_for_unknown_completion(ResourceRequestId::new(9_998));

    assert_ne!(left.provenance_digest(), right.provenance_digest());
    assert_ne!(
        left.replay_reconstruction().denied_completion_digest(),
        right.replay_reconstruction().denied_completion_digest()
    );
    assert_eq!(left.runtime_summary(), right.runtime_summary());
}

#[test]
fn resource_diagnostics_summary_digest_tracks_expansion_budget() {
    let strict = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(2),
    );
    let loose = resource_diagnostics_summary_for_budget(
        ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(8),
    );

    assert_ne!(strict.provenance_digest(), loose.provenance_digest());
    assert_eq!(
        strict.replay_reconstruction().replay_digest(),
        loose.replay_reconstruction().replay_digest()
    );
    assert_eq!(strict.runtime_summary(), loose.runtime_summary());
}

fn resource_diagnostics_summary_for_budget(
    budget: ResourceDiagnosticsExpansionBudget,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    runtime
        .try_resource_diagnostics_summary(budget)
        .expect("budget should admit descriptor plus lifecycle reconstruction")
}

fn resource_diagnostics_summary_for_unknown_completion(
    request_id: ResourceRequestId,
) -> ResourceDiagnosticsSummary {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(node))
        .expect("resource declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("resource descriptor should exist")
        .payload_contract_digest()
        .clone();
    runtime
        .admit_resource_completion(RawCompletionEnvelope::new(
            request_id,
            ResourceGeneration::new(1),
            ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
            ResourceAttemptId::ZERO,
            digest,
            32,
        ))
        .denied_completion()
        .expect("unknown completion should retain denial provenance");

    runtime.resource_diagnostics_summary_with_unbounded_cold_reconstruction()
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
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::SparseIndexedLookup
    );
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
    assert_eq!(staging.performance().cost_contract().get(), 9);
    assert_eq!(
        staging.performance().cost_posture(),
        ResourceCostPosture::Verified
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
fn resource_lifecycle_retention_compaction_moves_terminal_records_out_of_hot_lookup() {
    let mut graph = SignalGraph::new();
    let cancelled_node = graph.node().build();
    let fulfilled_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(cancelled_node))
        .expect("cancelled resource declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(fulfilled_node))
        .expect("fulfilled resource declaration should lower");
    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    let fulfilled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            fulfilled_node,
        )))
        .expect("fulfilled request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should admit");
    let admitted_completion = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            fulfilled_node,
            fulfilled.handle(),
            fulfilled.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("completion should admit");
    let staged = runtime
        .stage_admitted_resource_completion(admitted_completion)
        .expect("completion should stage")
        .staged_effect();
    runtime
        .commit_staged_resource_completion(staged)
        .expect("completion should commit");
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        2
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_lifecycle_history_count(),
        0
    );

    let report = runtime.compact_resource_lifecycle_history(1);

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::LifecycleRetentionCompaction
    );
    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.retained_history_write_count(), 1);
    assert_eq!(report.retained_history_pruned_count(), 0);
    assert_eq!(report.retained_history_width(), 1);
    assert_eq!(report.hot_in_flight_width(), 1);
    assert_eq!(report.performance().input_width(), 1);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().retained_history_allocation_count(), 1);
    assert!(runtime
        .in_flight_resource_request(cancelled.handle())
        .is_none());
    assert!(runtime
        .in_flight_resource_request(fulfilled.handle())
        .is_some());
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        1
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .retained_lifecycle_history_count(),
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_hot_in_flight_compaction_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_in_flight_reclaimed_record_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_lifecycle_history_write_count,
        1
    );
}

#[test]
fn resource_lifecycle_retention_compaction_prunes_retained_history_by_explicit_limit() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(first_node))
        .expect("first resource declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(second_node))
        .expect("second resource declaration should lower");
    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            first_node,
        )))
        .expect("first request should admit")
        .admitted_request();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second_node,
        )))
        .expect("second request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(first.handle(), ResourceCancellationReason::HostRequested)
        .expect("first cancellation should admit");
    runtime
        .cancel_resource_request(second.handle(), ResourceCancellationReason::HostRequested)
        .expect("second cancellation should admit");

    let report = runtime.compact_resource_lifecycle_history_with_retained_limit(2, 1);

    assert_eq!(report.selected_terminal_count(), 2);
    assert_eq!(report.reclaimed_in_flight_count(), 2);
    assert_eq!(report.retained_history_write_count(), 2);
    assert_eq!(report.retained_history_pruned_count(), 1);
    assert_eq!(report.retained_history_width(), 1);
    assert_eq!(report.hot_in_flight_width(), 0);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_lifecycle_history_pruned_count,
        1
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            first_node,
            first.handle(),
            first.attempt(),
            64,
        ))
        .denied_completion()
        .expect("pruned retained history completion should deny explicitly");
    assert_eq!(
        denied.class(),
        CompletionDenialClass::RetainedHistoryUnavailable
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_retained_history_unavailable_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        0
    );
}

#[test]
fn resource_branch_restore_accounts_for_retained_lifecycle_history_width() {
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
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should admit");
    let compaction = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(compaction.retained_history_width(), 1);
    let snapshot = runtime.capture_snapshot();

    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("post-snapshot request should mutate resource state");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate retained lifecycle history");
    let restore = runtime
        .latest_resource_branch_restore_report()
        .expect("restore should publish resource branch evidence");

    assert_eq!(restore.restored_in_flight_width(), 0);
    assert_eq!(restore.retained_summary_width(), 2);
    assert_eq!(restore.performance().input_width(), 2);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_branch_restore_retained_summary_width,
        2
    );
}

#[test]
fn resource_lifecycle_retention_compaction_preserves_late_completion_denial_class() {
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
    runtime
        .cancel_resource_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("cancellation should admit");
    let report = runtime.compact_resource_lifecycle_history(1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);

    let late = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        admitted.handle(),
        admitted.attempt(),
        64,
    ));

    let denied = late
        .denied_completion()
        .expect("late compacted cancelled completion should deny");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_cancelled_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_unknown_request_completion_denial_count,
        0
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
fn resource_completion_batch_admission_canonicalizes_out_of_order_inputs() {
    let mut graph = SignalGraph::new();
    let first = graph.node().build();
    let second = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(first))
        .expect("first resource declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(second))
        .expect("second resource declaration should lower");
    let first_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(first)))
        .expect("first request should admit")
        .admitted_request();
    let second_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            second,
        )))
        .expect("second request should admit")
        .admitted_request();
    let first_raw = raw_completion(
        &runtime,
        first,
        first_request.handle(),
        first_request.attempt(),
        64,
    );
    let second_raw = raw_completion(
        &runtime,
        second,
        second_request.handle(),
        second_request.attempt(),
        96,
    );
    let boundary_envelopes_before = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;

    let report = runtime.admit_resource_completion_batch([second_raw, first_raw]);

    assert_eq!(
        report.performance().boundary(),
        ResourceBoundaryKind::CompletionBatchAdmission
    );
    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 2);
    assert_eq!(report.duplicate_width(), 0);
    assert_eq!(report.admitted_completions().len(), 2);
    assert!(report.denied_completions().is_empty());
    assert_eq!(
        report.admitted_completions()[0].handle(),
        first_request.handle()
    );
    assert_eq!(
        report.admitted_completions()[1].handle(),
        second_request.handle()
    );
    assert_eq!(report.performance().admitted_count(), 2);
    assert_eq!(report.performance().denied_count(), 0);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_batch_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_reports_dense_strategy_without_truth_drift() {
    let mut graph = SignalGraph::new();
    let nodes = [
        graph.node().build(),
        graph.node().build(),
        graph.node().build(),
        graph.node().build(),
    ];
    let mut runtime = TestRuntime::build(graph);
    for node in nodes {
        runtime
            .declare_resource_node(resource_declaration(node))
            .expect("resource declaration should lower");
    }
    let admitted = nodes.map(|node| {
        runtime
            .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
            .expect("request should admit")
            .admitted_request()
    });
    let mut completions = admitted
        .iter()
        .zip(nodes)
        .map(|(request, node)| {
            raw_completion(&runtime, node, request.handle(), request.attempt(), 64)
        })
        .collect::<Vec<_>>();
    completions.reverse();
    let density_before = runtime
        .telemetry()
        .resource
        .resource_density_strategy_selection_count;
    let dense_before = runtime
        .telemetry()
        .resource
        .resource_dense_density_strategy_count;

    let report = runtime.admit_resource_completion_batch(completions);

    assert_eq!(report.input_width(), 4);
    assert_eq!(report.deduplicated_width(), 4);
    assert_eq!(report.admitted_completions().len(), 4);
    assert!(report.denied_completions().is_empty());
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::DenseSortedDeduplicated
    );
    assert_eq!(
        report
            .admitted_completions()
            .iter()
            .map(|completion| completion.handle())
            .collect::<Vec<_>>(),
        admitted
            .iter()
            .map(|request| request.handle())
            .collect::<Vec<_>>()
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_density_strategy_selection_count,
        density_before + 1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_dense_density_strategy_count,
        dense_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_denies_in_batch_duplicate_without_second_admitted_proof() {
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
    let boundary_envelopes_before = runtime
        .telemetry()
        .resource
        .resource_boundary_performance_envelope_count;

    let report = runtime.admit_resource_completion_batch([raw.clone(), raw]);

    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 1);
    assert_eq!(report.duplicate_width(), 1);
    assert_eq!(report.admitted_completions().len(), 1);
    assert_eq!(
        report.admitted_completions()[0].handle(),
        admitted_request.handle()
    );
    assert_eq!(report.denied_completions().len(), 1);
    assert_eq!(
        report.denied_completions()[0].class(),
        CompletionDenialClass::Duplicate
    );
    assert_eq!(report.performance().input_width(), 2);
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        report.performance().density_strategy(),
        ResourceDensityStrategy::BurstySortedDeduplicated
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_duplicate_completion_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_validation_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_completion_admission_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_boundary_performance_envelope_count,
        boundary_envelopes_before + 1
    );
}

#[test]
fn resource_completion_batch_admission_denies_contradictory_duplicate_identity() {
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
    let accepted = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        64,
    );
    let contradictory = raw_completion(
        &runtime,
        node,
        admitted_request.handle(),
        admitted_request.attempt(),
        96,
    );

    let report = runtime.admit_resource_completion_batch([contradictory, accepted]);

    assert_eq!(report.input_width(), 2);
    assert_eq!(report.deduplicated_width(), 1);
    assert_eq!(report.duplicate_width(), 1);
    assert_eq!(report.admitted_completions().len(), 1);
    assert_eq!(
        report.admitted_completions()[0].handle(),
        admitted_request.handle()
    );
    assert_eq!(report.denied_completions().len(), 1);
    assert_eq!(
        report.denied_completions()[0].class(),
        CompletionDenialClass::Contradictory
    );
    assert_eq!(report.performance().admitted_count(), 1);
    assert_eq!(report.performance().denied_count(), 1);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_contradictory_completion_denial_count,
        1
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
