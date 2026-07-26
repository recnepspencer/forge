use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{evaluate, version_ab};

#[test]
fn async_node_interior_gate_report_tracks_dependency_shape_and_restores_identically() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let gate = graph.node().build();
    let sink = graph.node().build();
    graph
        .append_dependency(gate, source, Aspect::new(0))
        .expect("source should feed gate");
    graph
        .append_dependency(sink, gate, Aspect::new(0))
        .expect("gate should feed sink");
    let mut source_eval = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut gate_eval = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("gate-v1"))
    };
    let mut sink_eval = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(3, 0));
    evaluate(&mut graph, source, &mut source_eval).expect("source should evaluate");
    evaluate(&mut graph, gate, &mut gate_eval).expect("gate should evaluate");
    evaluate(&mut graph, sink, &mut sink_eval).expect("sink should evaluate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(gate))
        .expect("gate capability should lower");
    let admitted = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(gate))
        .expect("interior gate should admit async work")
        .resource_admission()
        .expect("admitted gate should expose resource admission")
        .admitted_request();
    let baseline = runtime
        .async_node_gate_state_report(gate)
        .expect("interior async gate report should materialize");
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    assert_eq!(baseline.node(), gate);
    assert_eq!(baseline.upstream_dependency_count(), 1);
    assert_eq!(baseline.downstream_subscriber_count(), 1);
    assert_eq!(baseline.lifecycle_class(), ResourceLifecycleClass::Pending);
    assert_eq!(baseline.active_request_handle(), Some(admitted.handle()));
    assert_eq!(
        baseline
            .committed_output_identity()
            .expect("gate output should be visible")
            .as_str(),
        "gate-v1"
    );
    assert_eq!(baseline.latest_observation_match_count(), 0);
    assert_eq!(
        baseline.performance().boundary(),
        ResourceBoundaryKind::AsyncNodeGateState
    );
    assert_eq!(
        baseline.downstream_dependence_facts(),
        &[
            AsyncNodeDownstreamDependenceFact::LifecycleClass,
            AsyncNodeDownstreamDependenceFact::CommittedOutput,
            AsyncNodeDownstreamDependenceFact::OutputContinuity,
            AsyncNodeDownstreamDependenceFact::ObservationBoundary,
        ]
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_interior_gate_admission_count,
        1
    );

    runtime
        .cancel_async_node_request(admitted.handle(), ResourceCancellationReason::HostRequested)
        .expect("post-snapshot mutation should succeed");
    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate interior gate truth");
    let restored = runtime
        .async_node_gate_state_report(gate)
        .expect("restored gate report should materialize");

    assert_eq!(restored.gate_digest(), baseline.gate_digest());
    assert_eq!(restored.lifecycle_class(), baseline.lifecycle_class());
    assert_eq!(
        restored
            .active_request_handle()
            .map(|handle| (handle.request_id(), handle.generation())),
        baseline
            .active_request_handle()
            .map(|handle| (handle.request_id(), handle.generation()))
    );
    assert_eq!(
        restored
            .committed_output_identity()
            .map(OutputIdentity::as_str),
        baseline
            .committed_output_identity()
            .map(OutputIdentity::as_str)
    );
}

#[test]
fn async_node_gate_state_report_rejects_undeclared_owner() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .async_node_gate_state_report(node)
        .expect_err("undeclared node must not pretend to have async gate state");

    assert!(err.to_string().contains("undeclared node"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_undeclared_owner_denial_count,
        1
    );
}
