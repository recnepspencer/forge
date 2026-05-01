use super::*;

#[test]
fn resource_lifecycle_retention_compaction_moves_terminal_records_out_of_hot_lookup() {
    let mut graph = SignalGraph::new();
    let cancelled_node = graph.node().build();
    let fulfilled_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(cancelled_node))
        .expect("cancelled resource declaration should lower");
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(fulfilled_node))
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
    assert_eq!(report.retained_history_unavailable_count(), 0);
    assert_eq!(report.retained_history_width(), 1);
    assert_eq!(report.hot_in_flight_width(), 1);
    assert_eq!(report.compacted_terminal_summary_count(), 0);
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
fn resource_terminal_summaries_only_compaction_emits_typed_unavailable_history_artifact() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(terminal_summaries_only_resource_declaration(node))
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
    assert_eq!(report.retained_history_write_count(), 0);
    assert_eq!(report.retained_history_pruned_count(), 0);
    assert_eq!(report.retained_history_unavailable_count(), 1);
    assert_eq!(report.compacted_terminal_summary_count(), 1);
    assert_eq!(report.retained_history_width(), 0);
    let availability = runtime
        .retained_history_availability_for_request(admitted.handle().request_id())
        .expect("terminal-summary compaction should retain typed availability evidence");
    assert_eq!(
        availability.class(),
        ResourceRetainedHistoryAvailabilityClass::TerminalSummaryOnly
    );
    assert_eq!(
        availability.retention_decision_class(),
        ResourceRetentionDecisionClass::TerminalSummariesOnly
    );
    let descriptor = runtime
        .resource_descriptor_for_node(ResourceNodeId::from_node(node))
        .expect("descriptor should remain available for the node");
    assert_eq!(
        availability.retention_descriptor_id(),
        descriptor.retention_decision_plan().descriptor_id()
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            node,
            admitted.handle(),
            admitted.attempt(),
            64,
        ))
        .denied_completion()
        .expect("late completion should still deny from compacted terminal summary");
    assert_eq!(denied.class(), CompletionDenialClass::Cancelled);
}

#[test]
fn resource_targeted_retention_compaction_only_reclaims_matching_lifecycle_policy() {
    let mut graph = SignalGraph::new();
    let cancelled_node = graph.node().build();
    let mismatched_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_cancelled_resource_declaration(cancelled_node))
        .expect("cancelled compaction declaration should lower");
    runtime
        .declare_resource_node(compact_superseded_resource_declaration(mismatched_node))
        .expect("mismatched compaction declaration should lower");
    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    let mismatched = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            mismatched_node,
        )))
        .expect("mismatched request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancelled request should terminate");
    runtime
        .cancel_resource_request(
            mismatched.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("mismatched request should also terminate");

    let report = runtime.compact_resource_lifecycle_history(2);

    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.compacted_cancelled_count(), 1);
    assert_eq!(report.compacted_superseded_count(), 0);
    assert!(
        runtime
            .retained_history_availability_for_request(cancelled.handle().request_id())
            .is_some(),
        "matching cancelled policy should produce availability artifact"
    );
    assert!(
        runtime
            .in_flight_resource_request(mismatched.handle())
            .is_some(),
        "non-matching supersession policy should not compact cancelled lifecycle"
    );
}

#[test]
fn resource_timed_out_retention_compaction_only_reclaims_matching_timeout_policy() {
    let mut graph = SignalGraph::new();
    let timed_out_node = graph.node().build();
    let cancelled_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(compact_timed_out_resource_declaration(timed_out_node))
        .expect("timed-out compaction declaration should lower");
    runtime
        .declare_resource_node(compact_timed_out_resource_declaration(cancelled_node))
        .expect("cancelled declaration should still lower");

    let timed_out = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timed_out_node,
        )))
        .expect("timed-out request should admit")
        .admitted_request();
    let timeout_wake = runtime
        .in_flight_resource_request(timed_out.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach for timed-out policy");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(runtime.clock_basis().current_tick().get().saturating_add(3)),
        ))
        .expect("clock should advance to timeout boundary");
    let ready_timeout = runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    runtime
        .admit_resource_timeout(timed_out.handle(), ready_timeout)
        .expect("timed-out request should transition to timed-out lifecycle");

    let cancelled = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancelled_node,
        )))
        .expect("cancelled request should admit")
        .admitted_request();
    runtime
        .cancel_resource_request(
            cancelled.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancelled request should terminate");

    let report = runtime.compact_resource_lifecycle_history(2);

    assert_eq!(report.selected_terminal_count(), 1);
    assert_eq!(report.reclaimed_in_flight_count(), 1);
    assert_eq!(report.compacted_timed_out_count(), 1);
    assert_eq!(report.compacted_cancelled_count(), 0);
    let availability = runtime
        .retained_history_availability_for_request(timed_out.handle().request_id())
        .expect("matching timed-out policy should produce availability artifact");
    assert_eq!(
        availability.class(),
        ResourceRetainedHistoryAvailabilityClass::CompactTimedOut
    );
    assert_eq!(
        availability.retention_decision_class(),
        ResourceRetentionDecisionClass::CompactTimedOut
    );
    let denied = runtime
        .admit_resource_completion(raw_completion(
            &runtime,
            timed_out_node,
            timed_out.handle(),
            timed_out.attempt(),
            64,
        ))
        .denied_completion()
        .expect("late completion after timed-out compaction should still deny as timed out");
    assert_eq!(denied.class(), CompletionDenialClass::TimedOut);
    assert!(
        runtime
            .in_flight_resource_request(cancelled.handle())
            .is_some(),
        "timed-out-only compaction should not reclaim cancelled lifecycle"
    );
}

#[test]
fn resource_lifecycle_retention_compaction_prunes_retained_history_by_explicit_limit() {
    let mut graph = SignalGraph::new();
    let first_node = graph.node().build();
    let second_node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(first_node))
        .expect("first resource declaration should lower");
    runtime
        .declare_resource_node(retain_all_transitions_resource_declaration(second_node))
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
    assert_eq!(report.retained_history_unavailable_count(), 1);
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
