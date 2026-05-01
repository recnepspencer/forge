use super::*;

#[test]
fn resource_cancellation_only_propagates_across_declared_dependent_footprint() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    let sibling = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(dependent_cancellation_resource_declaration(parent, [child]))
        .expect("parent declaration should lower");
    runtime
        .declare_resource_node(dependent_cancellation_resource_declaration(
            child,
            [grandchild],
        ))
        .expect("child declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(grandchild))
        .expect("grandchild declaration should lower");
    runtime
        .declare_resource_node(resource_declaration(sibling))
        .expect("sibling declaration should lower");

    let parent_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            parent,
        )))
        .expect("parent request should admit")
        .admitted_request()
        .handle();
    let child_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(child)))
        .expect("child request should admit")
        .admitted_request()
        .handle();
    let grandchild_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            grandchild,
        )))
        .expect("grandchild request should admit")
        .admitted_request()
        .handle();
    let sibling_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            sibling,
        )))
        .expect("sibling request should admit")
        .admitted_request()
        .handle();

    let report = runtime
        .cancel_resource_request(parent_handle, ResourceCancellationReason::HostRequested)
        .expect("parent cancellation should admit");
    let propagation = report
        .dependent_propagation()
        .expect("declared dependent footprint should emit propagation evidence");
    let propagated_handles = propagation
        .cancelled_dependents()
        .iter()
        .map(CancelledResourceRequest::handle)
        .collect::<Vec<_>>();
    let propagated_reasons = propagation
        .cancelled_dependents()
        .iter()
        .map(CancelledResourceRequest::reason)
        .collect::<Vec<_>>();

    assert_eq!(propagation.parent(), parent_handle);
    assert_eq!(propagation.cancelled_dependent_width(), 2);
    assert_eq!(propagated_handles, vec![child_handle, grandchild_handle]);
    assert_eq!(
        propagated_reasons,
        vec![
            ResourceCancellationReason::RuntimePolicy,
            ResourceCancellationReason::RuntimePolicy,
        ]
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(parent_handle)
            .expect("parent should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(child_handle)
            .expect("child should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(grandchild_handle)
            .expect("grandchild should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(
        runtime
            .in_flight_resource_request(sibling_handle)
            .expect("undeclared sibling should remain active")
            .status(),
        ResourceInFlightStatus::Active
    );
    assert_eq!(
        runtime
            .resource_runtime_summary()
            .active_in_flight_node_count(),
        1
    );
    assert_eq!(runtime.telemetry().resource.resource_cancellation_count, 3);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_dependent_cancellation_propagation_count,
        2
    );
    assert_eq!(report.performance().input_width(), 3);
    assert_eq!(report.performance().admitted_count(), 3);
    assert_eq!(report.performance().lifecycle_transition_count(), 3);
}

#[test]
fn resource_dependent_cancellation_retires_child_timeout_wakes_with_the_cancelled_footprint() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_resource_node(
            timeout_resource_declaration(parent, 10)
                .with_declared_dependent_cancellation_nodes([ResourceNodeId::from_node(child)]),
        )
        .expect("parent declaration should lower");
    runtime
        .declare_resource_node(timeout_resource_declaration(child, 10))
        .expect("child declaration should lower");

    let parent_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            parent,
        )))
        .expect("parent request should admit")
        .admitted_request()
        .handle();
    let child_handle = runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(child)))
        .expect("child request should admit")
        .admitted_request()
        .handle();

    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 2);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 0);

    let report = runtime
        .cancel_resource_request(parent_handle, ResourceCancellationReason::HostRequested)
        .expect("parent cancellation should retire timeout wakes across the footprint");

    assert!(report.dependent_propagation().is_some());
    assert_eq!(
        runtime
            .in_flight_resource_request(child_handle)
            .expect("child should remain retained")
            .status(),
        ResourceInFlightStatus::Cancelled
    );
    assert_eq!(runtime.temporal_wake_summary().scheduled_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().ready_count(), 0);
    assert_eq!(runtime.temporal_wake_summary().retired_count(), 2);
}
