use super::super::*;

pub(super) struct ResourceMilestoneCTimeoutEvidence {
    pub(super) timeout_report: ResourceTimeoutReport,
    pub(super) heartbeat_denial_report: ResourceTimeoutHeartbeatExtensionReport,
}

pub(super) fn resource_milestone_c_timeout_evidence() -> ResourceMilestoneCTimeoutEvidence {
    let mut timeout_graph = SignalGraph::new();
    let timeout_node = timeout_graph.node().build();
    let mut timeout_runtime = TestRuntime::build(timeout_graph);
    timeout_runtime
        .declare_resource_node(heartbeat_extension_timeout_resource_declaration(
            timeout_node,
            5,
            2,
        ))
        .expect("timeout declaration should lower");
    let timeout_admitted = timeout_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            timeout_node,
        )))
        .expect("timeout request should admit")
        .admitted_request();
    let timeout_wake = timeout_runtime
        .in_flight_resource_request(timeout_admitted.handle())
        .and_then(|in_flight| in_flight.timeout_wake_id())
        .expect("timeout wake should attach");
    timeout_runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(5),
        ))
        .expect("clock should reach timeout");
    let ready_timeout = timeout_runtime
        .promote_temporal_wake_ready(timeout_wake)
        .expect("timeout wake should become ready");
    let timeout_report = timeout_runtime
        .admit_resource_timeout(timeout_admitted.handle(), ready_timeout)
        .expect("timeout admission should succeed");
    let heartbeat_denial_report = timeout_runtime
        .extend_resource_timeout_heartbeat(timeout_admitted.handle())
        .expect("terminal heartbeat extension should still report denial");

    ResourceMilestoneCTimeoutEvidence {
        timeout_report,
        heartbeat_denial_report,
    }
}
