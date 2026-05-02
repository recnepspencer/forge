use super::*;

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
