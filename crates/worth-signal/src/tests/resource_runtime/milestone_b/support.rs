use super::super::declaration_and_visibility::raw_completion;
use super::*;

pub(super) fn resource_certification_fixture_bundle(
    retained_denial_request_id: ResourceRequestId,
) -> ResourceCertificationBundle {
    resource_certification_fixture_artifacts(retained_denial_request_id).0
}

pub(super) fn resource_certification_fixture_artifacts(
    retained_denial_request_id: ResourceRequestId,
) -> (
    ResourceCertificationBundle,
    ResourceMilestoneBHostileScenarioEvidence,
    ResourceRuntimeSummaryReadReport,
    ResourceDiagnosticsSummary,
    ResourceDiagnosticsExpansionDenial,
) {
    let lifecycle_rollback = resource_async_lifecycle_rollback_workload();
    let inflight_pressure = resource_async_inflight_pressure_workload();
    let mut graph = SignalGraph::new();
    let lifecycle_node = graph.node().build();
    let cancel_node = graph.node().build();
    let timeout_node = graph.node().build();
    let malformed_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(resource_declaration(lifecycle_node))
        .expect("lifecycle declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(cancel_node))
        .expect("cancel declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(timeout_node, 3))
        .expect("timeout declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(malformed_node))
        .expect("malformed declaration should lower");
    let digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(lifecycle_node))
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
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            lifecycle_node,
        )))
        .expect("first request should admit");
    let stale_first = raw_completion(
        &runtime,
        lifecycle_node,
        first_admission.admitted_request().handle(),
        first_admission.admitted_request().attempt(),
        64,
    );
    let second_admission = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            lifecycle_node,
        )))
        .expect("second request should supersede first request");
    let superseded_completion_report = runtime.admit_resource_completion(stale_first);
    assert_eq!(
        superseded_completion_report
            .denied_completion()
            .expect("late superseded completion should deny explicitly")
            .class(),
        CompletionDenialClass::Superseded
    );

    let cancel_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancel_node,
        )))
        .expect("cancel request should admit")
        .admitted_request();
    let late_cancelled = raw_completion(
        &runtime,
        cancel_node,
        cancel_request.handle(),
        cancel_request.attempt(),
        64,
    );
    runtime
        .cancel_resource_request(
            cancel_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should retire the active request");
    let cancelled_completion_report = runtime.admit_resource_completion(late_cancelled);
    assert_eq!(
        cancelled_completion_report
            .denied_completion()
            .expect("late cancelled completion should deny explicitly")
            .class(),
        CompletionDenialClass::Cancelled
    );

    let timeout_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request();
    let late_timed_out = raw_completion(
        &runtime,
        timeout_node,
        timeout_request.handle(),
        timeout_request.attempt(),
        64,
    );
    let timeout_wake = runtime
        .in_flight_resource_request(timeout_request.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("authoritative clock should advance");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(timeout_request.handle(), ready_timeout)
        .expect("timeout admission should consume the wake");
    let timed_out_completion_report = runtime.admit_resource_completion(late_timed_out);
    assert_eq!(
        timed_out_completion_report
            .denied_completion()
            .expect("late timed out completion should deny explicitly")
            .class(),
        CompletionDenialClass::TimedOut
    );

    let malformed_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            malformed_node,
        )))
        .expect("malformed request should admit")
        .admitted_request();
    let malformed_completion_report =
        runtime.admit_resource_completion(RawCompletionEnvelope::new(
            malformed_request.handle().request_id(),
            malformed_request.handle().generation(),
            malformed_request.handle().branch_epoch(),
            malformed_request.attempt(),
            ResourcePayloadContractDigest::new("payload-contract:999:1024"),
            64,
        ));
    assert_eq!(
        malformed_completion_report
            .denied_completion()
            .expect("malformed completion should deny explicitly")
            .class(),
        CompletionDenialClass::Malformed
    );

    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let replay_before_restore = runtime.reconstruct_resource_replay_summary();
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            lifecycle_node,
        )))
        .expect("post-snapshot request should mutate state before restore");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate resource state");
    let _restore = runtime
        .latest_resource_branch_restore_report()
        .expect("resource restore should publish branch evidence");
    let replay = runtime.reconstruct_resource_replay_summary();
    assert_eq!(
        replay.descriptor_digest(),
        replay_before_restore.descriptor_digest(),
        "restore must preserve descriptor truth"
    );
    assert_eq!(
        replay.lifecycle_digest(),
        replay_before_restore.lifecycle_digest(),
        "restore must preserve lifecycle truth"
    );
    assert_eq!(
        replay.denied_completion_digest(),
        replay_before_restore.denied_completion_digest(),
        "restore must not invent or erase retained denial history"
    );
    assert_eq!(
        replay.in_flight_digest(),
        replay_before_restore.in_flight_digest(),
        "restore must reconstruct the same in-flight story"
    );
    assert_eq!(
        replay.replay_digest(),
        replay_before_restore.replay_digest(),
        "equivalent suffix after restore must preserve replay truth: lifecycle={} vs {}, denial={} vs {}, inflight={} vs {}",
        replay.lifecycle_digest(),
        replay_before_restore.lifecycle_digest(),
        replay.denied_completion_digest(),
        replay_before_restore.denied_completion_digest(),
        replay.in_flight_digest(),
        replay_before_restore.in_flight_digest()
    );
    let branch_replay_outcome = resource_branch_replay_workload(retained_denial_request_id);

    let bundle = resource_certification_builder()
        .with_async_resource_lifecycle_parity(
            &branch_replay_outcome.feature.replay_after_restore,
            &branch_replay_outcome.sibling.replay_after_restore,
            &branch_replay_outcome.feature.diagnostics_after_restore,
            &branch_replay_outcome.sibling.diagnostics_after_restore,
        )
        .expect("lifecycle evidence should be accepted")
        .with_out_of_order_completion_supersession(second_admission)
        .expect("supersession evidence should be accepted")
        .with_async_rollback_observation_equivalence(
            lifecycle_rollback.rollback_report,
            lifecycle_rollback.rollback_observation,
            lifecycle_rollback.control_commit_observation,
            &lifecycle_rollback.pre_rollback_replay,
            &lifecycle_rollback.post_rollback_replay,
            &lifecycle_rollback.diagnostics_after_rollback,
        )
        .expect("rollback evidence should be accepted")
        .with_async_branch_restore_replay_equivalence(
            branch_replay_outcome.feature.restore_report,
            &branch_replay_outcome.feature.replay_after_restore,
        )
        .expect("branch/replay evidence should be accepted")
        .with_async_inflight_boundedness(
            inflight_pressure.runtime_summary,
            &inflight_pressure.replay_after_restore,
            inflight_pressure.telemetry,
            inflight_pressure.pressure_performance,
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
        superseded_completion_report,
        cancelled_completion_report,
        timed_out_completion_report,
        malformed_completion_report,
        &inflight_pressure.pressure_batch,
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

pub(super) fn resource_late_cancelled_completion_report() -> ResourceCompletionAdmissionReport {
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

pub(super) fn resource_late_superseded_completion_report() -> ResourceCompletionAdmissionReport {
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
    let late = raw_completion(&runtime, node, first.handle(), first.attempt(), 64);
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should supersede first");
    runtime.admit_resource_completion(late)
}

pub(super) fn resource_late_timed_out_completion_report() -> ResourceCompletionAdmissionReport {
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
