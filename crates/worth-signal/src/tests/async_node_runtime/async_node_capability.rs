use crate::facade::{
    NodeId, ResourceDescriptorId, ResourceLifecycleClass, ResourceNodeId, SignalGraph,
};
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};

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
