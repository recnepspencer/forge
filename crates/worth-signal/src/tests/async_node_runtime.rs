use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{evaluate, version_ab, GraphDependencyBatchExt};

#[test]
fn async_node_capability_declaration_lowers_into_runtime_owned_descriptor() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let report = runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("live async-capable node should lower");

    assert_eq!(report.descriptor_id(), ResourceDescriptorId::new(0));
    assert_eq!(report.lifecycle().node(), ResourceNodeId::from_node(node));
    assert_eq!(
        report.lifecycle().lifecycle(),
        ResourceLifecycleClass::Unrequested
    );
    let lowered = runtime
        .async_node_capability_bundle_for_node(node)
        .expect("lowered async capability bundle should exist");
    assert_eq!(lowered.node(), node);
    assert_eq!(
        lowered.payload_contract_digest().as_str(),
        "payload-contract:7:1024"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_attachment_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_declaration_lowering_count,
        1
    );
}

#[test]
fn async_node_capability_declaration_rejects_non_live_owner() {
    let graph = SignalGraph::new();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .declare_async_node_capability(async_node_capability_declaration(NodeId::new(99, 0)))
        .expect_err("async-capability declarations must be owned by live nodes");

    assert!(err.to_string().contains("non-live owner"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_live_owner_denial_count,
        1
    );
}

#[test]
fn async_node_capability_validation_rejects_non_live_owner() {
    let graph = SignalGraph::new();
    let mut runtime = TestRuntime::build(graph);

    let err = runtime
        .validate_async_node_capability_declaration(&async_node_capability_declaration(
            NodeId::new(99, 0),
        ))
        .expect_err("validated async capability proof must not exist for a non-live owner");

    assert!(err.to_string().contains("non-live owner"));
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .resource_non_live_owner_denial_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_validation_count,
        0
    );
}

#[test]
fn async_node_capability_alias_lowering_matches_legacy_resource_truth() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);

    let declaration = async_node_capability_declaration(node);
    let validated = runtime
        .validate_async_node_capability_declaration(&declaration)
        .expect("async capability declaration should validate");
    let frozen = runtime
        .freeze_async_node_capability_descriptor(&validated)
        .expect("validated async capability should freeze");
    let lowered = runtime.lower_async_node_capability_bundle(&frozen);
    let proof = runtime
        .prove_async_node_capability_alias_lowering(&declaration)
        .expect("capability-first and legacy resource-shaped lowering should match");

    assert_eq!(validated.node(), node);
    assert_eq!(frozen.node(), node);
    assert_eq!(lowered.node(), node);
    assert_eq!(
        proof.capability_registry_digest().as_str(),
        proof.legacy_registry_digest().as_str()
    );
    assert_eq!(
        proof.capability_bundle_digest().as_str(),
        proof.legacy_bundle_digest().as_str()
    );
    assert_eq!(
        proof.capability_payload_contract_digest().as_str(),
        proof.legacy_payload_contract_digest().as_str()
    );
    assert_eq!(proof.compared_width(), 3);
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_validation_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_freeze_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_bundle_lowering_count,
        2
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_capability_alias_lowering_count,
        1
    );
}

#[test]
fn async_node_request_admission_flows_through_existing_lifecycle_substrate() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");

    let report = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("declared async-capable node should admit request");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::AdmittedNewLineage
    );
    let resource_admission = report
        .resource_admission()
        .expect("admitted request should expose underlying resource report");
    assert_eq!(
        resource_admission.lifecycle().node(),
        ResourceNodeId::from_node(node)
    );
    assert_eq!(
        resource_admission.lifecycle().lifecycle(),
        ResourceLifecycleClass::Pending
    );
    assert_eq!(
        resource_admission.admitted_request().handle().generation(),
        ResourceGeneration::new(1)
    );
}

#[test]
fn async_node_condition_gated_request_blocks_without_mutating_lifecycle_truth() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph
        .node()
        .aspect_filter(AspectMask::from_aspect(Aspect::new(0)))
        .build();
    graph
        .append_dependency(node, source, Aspect::new(1))
        .expect("dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("dependent should evaluate");
    mark_dirty(&mut graph, source, Aspect::new(1)).expect("dirty source should propagate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");

    let report = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("blocked async admission should still return a report");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::BlockedByCondition
    );
    assert_eq!(
        report.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::AspectFilterMismatch)
    );
    assert!(report.resource_admission().is_none());
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}

#[test]
fn async_node_clean_dependency_request_blocks_dirty_lineage_before_resource_admission() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph.node().build();
    graph
        .append_dependency(node, source, Aspect::new(0))
        .expect("dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("dependent should evaluate");
    mark_dirty(&mut graph, source, Aspect::new(0)).expect("source should invalidate dependent");
    let mut runtime = TestRuntime::build(graph);
    let capability = runtime
        .attach_async_capability(async_node_capability_declaration(node))
        .expect("capability should attach");

    let report = runtime
        .admit_async_node_request(capability.request_intent_requiring_clean_dependencies())
        .expect("dependency block should remain report-shaped");

    assert_eq!(
        report.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::DependencyNotReady)
    );
    assert!(report.classification().requires_clean_dependencies());
    assert!(report.resource_admission().is_none());
    assert_eq!(
        runtime.resource_runtime_summary().in_flight_request_count(),
        0
    );
}

#[test]
fn async_node_temporal_request_admission_blocks_until_clock_reaches_ready_tick() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .after(10)
        .expect("valid temporal condition")
        .build();
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");

    let blocked = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("temporal block should still return a report");
    assert_eq!(
        blocked.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::TemporalConditionNotReady)
    );

    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(10),
        ))
        .expect("clock should advance");
    runtime
        .promote_due_temporal_wakes_ready()
        .expect("due temporal wake should promote");

    let admitted = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("ready temporal condition should admit");
    assert_eq!(
        admitted.classification().class(),
        AsyncNodeAdmissionClass::AdmittedNewLineage
    );
    assert!(
        admitted.resource_admission().is_some(),
        "ready temporal admission should reach the resource substrate"
    );
}

#[test]
fn async_node_previous_value_drift_blocks_request_admission() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut node_v1).expect("node should evaluate");
    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");

    let wake = runtime
        .schedule_owned_temporal_wake(
            TemporalWakeOwner::Node(node),
            TemporalCondition::after(1).expect("valid delay"),
            ClockTick::new(1),
        )
        .expect("wake should schedule");
    runtime
        .advance_clock(ClockAdvanceRequest::new(
            ClockDomain::MonotonicExecution,
            ClockTick::new(1),
        ))
        .expect("clock should advance");
    runtime
        .promote_temporal_wake_ready(wake.id())
        .expect("wake should become ready");
    let access = runtime
        .grant_temporal_previous_value_access(wake.id())
        .expect("ready wake should grant previous-value access");
    let reference = runtime
        .previous_temporal_value(&access, node)
        .expect("previous-value reference should capture");
    mark_dirty(runtime.graph_mut(), node, Aspect::new(0)).expect("dirty node should change");
    let mut node_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    evaluate(runtime.graph_mut(), node, &mut node_v2).expect("node should commit drift");

    let report = runtime
        .admit_async_node_request(
            AsyncNodeRequestIntent::new(node).with_previous_value_reference(reference),
        )
        .expect("drifted previous-value gate should still return a report");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::BlockedByCondition
    );
    assert_eq!(
        report.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::PreviousValueReferenceDrifted)
    );
}

#[test]
fn async_node_revalidation_can_refresh_when_new_lineage_condition_is_blocked() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let node = graph
        .node()
        .aspect_filter(AspectMask::from_aspect(Aspect::new(0)))
        .build();
    graph
        .append_dependency(node, source, Aspect::new(1))
        .expect("dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("dependent should evaluate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("initial request should admit");
    mark_dirty(runtime.graph_mut(), source, Aspect::new(1)).expect("dirty source should propagate");

    let report = runtime
        .revalidate_async_node(AsyncNodeRevalidationIntent::new(node))
        .expect("refresh-eligible path should still return a report");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::RefreshEligibleNoNewLineage
    );
    assert_eq!(
        report.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::AspectFilterMismatch)
    );
    assert!(
        report.resource_revalidation().is_some(),
        "refresh-eligible classification should still drive revalidation truth"
    );
}
