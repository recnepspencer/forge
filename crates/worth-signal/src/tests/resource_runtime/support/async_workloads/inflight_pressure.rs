use super::*;

pub(in crate::tests::resource_runtime) fn resource_async_inflight_pressure_workload(
) -> ResourceAsyncInflightPressureWorkloadOutcome {
    let mut graph = SignalGraph::new();
    let retry_node = graph.node().build();
    let supersede_node = graph.node().build();
    let batch_node = graph.node().build();
    let cancel_node = graph.node().build();
    let branch_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retry_timeout_resource_declaration(retry_node, 3, 7))
        .expect("retry node should lower");
    runtime
        .declare_resource_node(resource_declaration(supersede_node))
        .expect("supersede node should lower");
    runtime
        .declare_resource_node(resource_declaration(batch_node))
        .expect("batch node should lower");
    runtime
        .declare_resource_node(resource_declaration(cancel_node))
        .expect("cancel node should lower");
    runtime
        .declare_resource_node(resource_declaration(branch_node))
        .expect("branch node should lower");

    let retry_admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            retry_node,
        )))
        .expect("retry request should admit")
        .admitted_request();
    let retry_timeout_wake = runtime
        .in_flight_resource_request(retry_admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("retry timeout wake should attach");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(3),
        ))
        .expect("clock should reach retry timeout");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(retry_timeout_wake)
        .expect("retry timeout wake should become ready");
    runtime
        .admit_resource_timeout(retry_admitted.handle(), ready_timeout)
        .expect("retry timeout admission should consume the wake");
    let first_retry_schedule = runtime
        .schedule_resource_retry(retry_admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("first retry scheduling should admit");
    let scheduled_retry = first_retry_schedule
        .scheduled_retry()
        .expect("retry policy should schedule a backoff wake");
    let duplicate_retry_schedule = runtime
        .schedule_resource_retry(retry_admitted.handle(), ResourceRetryReason::TimedOut)
        .expect("duplicate retry scheduling should stay report-shaped");
    assert_eq!(
        duplicate_retry_schedule
            .denied_retry()
            .expect("duplicate retry should deny explicitly")
            .class(),
        ResourceRetryDenialClass::RetryAlreadyScheduled
    );
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should reach retry backoff");
    let ready_retry = runtime
        .promote_temporal_wake_ready(scheduled_retry.backoff_wake_id())
        .expect("scheduled retry wake should become ready");
    runtime
        .admit_scheduled_resource_retry(retry_admitted.handle(), ready_retry)
        .expect("scheduled retry admission should consume the backoff wake");

    let first_superseded = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            supersede_node,
        )))
        .expect("first supersession request should admit")
        .admitted_request();
    let stale_superseded = raw_completion(
        &runtime,
        supersede_node,
        first_superseded.handle(),
        first_superseded.attempt(),
        64,
    );
    runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            supersede_node,
        )))
        .expect("second supersession request should admit");
    let stale_supersession_report = runtime.admit_resource_completion(stale_superseded);
    assert_eq!(
        stale_supersession_report
            .denied_completion()
            .expect("late superseded completion should deny explicitly")
            .class(),
        CompletionDenialClass::Superseded
    );

    let batch_admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            batch_node,
        )))
        .expect("batch request should admit")
        .admitted_request();
    let accepted_completion = raw_completion(
        &runtime,
        batch_node,
        batch_admitted.handle(),
        batch_admitted.attempt(),
        64,
    );
    let contradictory_completion = raw_completion(
        &runtime,
        batch_node,
        batch_admitted.handle(),
        batch_admitted.attempt(),
        96,
    );
    let batch_digest = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(batch_node))
        .expect("batch descriptor should exist")
        .payload_contract_digest()
        .clone();
    let unknown_completion = RawCompletionEnvelope::new(
        ResourceRequestId::new(88_001),
        ResourceGeneration::new(1),
        ResourceBranchEpoch::new(runtime.graph().current_branch().id, 0),
        ResourceAttemptId::ZERO,
        batch_digest,
        32,
    );
    let pressure_batch = runtime.admit_resource_completion_batch([
        contradictory_completion,
        accepted_completion.clone(),
        accepted_completion,
        unknown_completion,
    ]);
    assert_eq!(pressure_batch.input_width(), 4);
    assert_eq!(pressure_batch.admitted_completions().len(), 1);
    assert_eq!(pressure_batch.denied_completions().len(), 3);

    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancel_node,
        )))
        .expect("cancel request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should retire active request");

    let branch_admitted = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            branch_node,
        )))
        .expect("branch request should admit")
        .admitted_request();
    let retained_branch_completion = raw_completion(
        &runtime,
        branch_node,
        branch_admitted.handle(),
        branch_admitted.attempt(),
        64,
    );
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let drifted_branch_request = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            branch_node,
        )))
        .expect("post-snapshot drift should mutate branch inflight state")
        .admitted_request();
    let zombie_branch_completion = raw_completion(
        &runtime,
        branch_node,
        drifted_branch_request.handle(),
        drifted_branch_request.attempt(),
        64,
    );
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate the original branch inflight story");
    let branch_restore_report = runtime
        .latest_resource_branch_restore_report()
        .expect("restore should publish branch restore evidence");
    let drifted_branch_handle_live_after_restore = runtime
        .in_flight_resource_request(drifted_branch_request.handle())
        .is_some();
    let zombie_completion_after_restore =
        runtime.admit_resource_completion(zombie_branch_completion);
    let pre_restore_completion_after_restore =
        runtime.admit_resource_completion(retained_branch_completion);

    let runtime_summary = runtime.resource_runtime_summary();
    let replay_after_restore = runtime.reconstruct_resource_replay_summary();
    let telemetry = runtime.telemetry().resource;

    ResourceAsyncInflightPressureWorkloadOutcome {
        runtime_summary,
        replay_after_restore,
        telemetry,
        pressure_performance: pressure_batch.performance(),
        pressure_batch,
        branch_restore_report,
        drifted_branch_handle_live_after_restore,
        zombie_completion_after_restore,
        pre_restore_completion_after_restore,
    }
}
