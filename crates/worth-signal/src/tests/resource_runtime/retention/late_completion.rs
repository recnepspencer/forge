use super::*;

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
