use super::*;

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
    assert!(
        supersession.overlap_admission().is_none(),
        "default newest-generation-wins supersession should not claim explicit overlap admission"
    );
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
fn resource_overlap_supersession_retains_old_host_work_evidence_and_denies_late_completion() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_retained_host_work_resource_declaration(node))
        .expect("overlap-retained-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and supersede the first as runtime authority");

    let supersession = second
        .supersession_record()
        .expect("overlap supersession should retain explicit lineage");
    let overlap = supersession
        .overlap_admission()
        .expect("overlap policy should emit an explicit overlap admission artifact");
    assert_eq!(overlap.previous(), first);
    assert_eq!(overlap.replacing(), second.admitted_request().handle());
    assert_eq!(
        overlap.policy_decision_digest().as_str(),
        supersession.policy_decision_digest().as_str()
    );
    assert!(
        overlap.old_host_work_cancellation_advisory().is_none(),
        "retained-host-work overlap should not claim old-host-work cancellation advisory"
    );

    let denied = runtime.admit_resource_completion(RawCompletionEnvelope::new(
        first.request_id(),
        first.generation(),
        first.branch_epoch(),
        ResourceAttemptId::ZERO,
        ResourcePayloadContractDigest::new("payload-contract:7:1024"),
        8,
    ));
    let denied = denied
        .denied_completion()
        .expect("late completion for overlap-retained loser should be denied");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);

    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_overlapping_generation_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_retained_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_advisory_cancelled_count,
        0
    );
}

#[test]
fn resource_overlap_supersession_can_request_old_host_work_advisory_cancel() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_cancelled_host_work_resource_declaration(node))
        .expect("overlap-cancelled-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and emit overlap advisory evidence");

    let supersession = second
        .supersession_record()
        .expect("supersession should be explicit");
    let overlap = supersession
        .overlap_admission()
        .expect("overlap-cancel policy should retain overlap admission evidence");
    let advisory = overlap
        .old_host_work_cancellation_advisory()
        .expect("overlap-cancel policy should emit old-host-work advisory evidence");
    assert_eq!(
        advisory.policy_decision_digest().as_str(),
        supersession.policy_decision_digest().as_str()
    );
    assert_eq!(overlap.previous(), first);
    assert_eq!(overlap.replacing(), second.admitted_request().handle());
    assert_ne!(
        first.request_id(),
        second.admitted_request().handle().request_id()
    );
    assert_ne!(
        first.generation(),
        second.admitted_request().handle().generation()
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(first)
            .expect("older overlapping request should remain retained as superseded")
            .status(),
        ResourceInFlightStatus::Superseded
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(second.admitted_request().handle())
            .expect("winner should remain the only active authority")
            .status(),
        ResourceInFlightStatus::Active
    );
    let denied = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        first,
        ResourceAttemptId::ZERO,
        8,
    ));
    let denied = denied
        .denied_completion()
        .expect("late completion must still deny even if old host work kept running");
    assert_eq!(denied.class(), CompletionDenialClass::Superseded);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_overlapping_generation_admission_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_retained_count,
        0
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_old_host_work_advisory_cancelled_count,
        1
    );
}

#[test]
fn resource_overlap_supersession_replay_retains_superseded_denial_evidence_after_restore() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(overlap_retained_host_work_resource_declaration(node))
        .expect("overlap-retained-host-work declaration should lower");

    let first = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("first request should admit")
        .admitted_request()
        .handle();
    let second = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(node)))
        .expect("second request should admit and supersede first")
        .admitted_request()
        .handle();

    let denied = runtime.admit_resource_completion(raw_completion(
        &runtime,
        node,
        first,
        ResourceAttemptId::ZERO,
        32,
    ));
    assert_eq!(
        denied
            .denied_completion()
            .expect("superseded completion should be retained as denial evidence")
            .class(),
        CompletionDenialClass::Superseded
    );

    let snapshot = runtime.capture_snapshot();
    let expected = runtime.reconstruct_resource_replay_summary();

    runtime
        .cancel_resource_request(second, ResourceCancellationReason::HostRequested)
        .expect("post-snapshot mutation should change the replay surface");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate overlap supersession state");
    let replayed = runtime.reconstruct_resource_replay_summary();

    assert_eq!(replayed.denied_completion_width(), 1);
    assert_eq!(replayed.descriptor_digest(), expected.descriptor_digest());
    assert_eq!(
        replayed.lifecycle_summary_width(),
        expected.lifecycle_summary_width()
    );
    assert_eq!(replayed.in_flight_width(), expected.in_flight_width());
    assert_eq!(
        replayed.denied_completion_digest(),
        expected.denied_completion_digest(),
        "superseded completion denial evidence must survive replay/restore unchanged"
    );
}
