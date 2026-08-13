use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, AsyncNodeTestRuntime as TestRuntime,
};
use crate::tests::support::{
    define_keyed_computation, evaluate, version_ab, GraphDependencyBatchExt, ASPECT_A,
};

#[test]
fn async_keyed_node_capability_binding_preserves_family_identity_and_node_local_truth() {
    let mut runtime = TestRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "async-projection", ());
    let left = family.keyed("left-wing");
    let right = family.keyed("right-wing");

    let left_binding = left
        .declare_async_capability(
            &mut runtime,
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(9))
                .with_max_payload_bytes(2048),
        )
        .expect("left keyed node should attach async capability");
    let right_binding = right
        .declare_async_capability(
            &mut runtime,
            AsyncNodePayloadContract::new(AsyncNodePayloadContractId::new(9))
                .with_max_payload_bytes(2048),
        )
        .expect("right keyed node should attach async capability");

    assert_eq!(left_binding.family(), right_binding.family());
    assert_ne!(left_binding.key(), right_binding.key());
    assert_ne!(left_binding.node(), right_binding.node());
    assert_eq!(
        left_binding.bundle_digest().as_str(),
        right_binding.bundle_digest().as_str()
    );

    let left_intent = left.async_request_intent(&mut runtime);
    let right_intent = right.async_request_intent(&mut runtime);
    let left_request = runtime
        .admit_async_node_request(left_intent)
        .expect("left keyed async request should admit");
    let right_request = runtime
        .admit_async_node_request(right_intent)
        .expect("right keyed async request should admit");

    let left_resource = left_request
        .resource_admission()
        .expect("left keyed request should reach resource substrate");
    let right_resource = right_request
        .resource_admission()
        .expect("right keyed request should reach resource substrate");
    assert_ne!(
        left_resource.lifecycle().node(),
        right_resource.lifecycle().node()
    );
    assert_eq!(
        runtime.telemetry().resource.async_node_family_binding_count,
        2
    );
}

#[test]
fn async_node_partition_local_revalidation_blocks_when_changed_region_misses_contract_scope() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let node = graph
        .node()
        .reads_aspects([ASPECT_A])
        .with_partition_scope(PartitionSubscription::partition_and_detail(
            "wing", "rib-12",
        ))
        .build();
    graph
        .append_partition_detail_dependency(node, source, ASPECT_A, "wing", "rib-12")
        .expect("partition dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("dependent should evaluate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("initial request should admit");

    mark_dirty_with_regions(
        runtime.graph_mut(),
        node,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .expect("node-local dirty scope should capture the non-overlapping region");

    let report = runtime
        .revalidate_async_node(AsyncNodeRevalidationIntent::new(node))
        .expect("partition mismatch should still return a report");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::BlockedByCondition
    );
    assert_eq!(
        report.classification().condition_block_class(),
        Some(AsyncNodeConditionBlockClass::PartitionScopeMismatch)
    );
    assert_eq!(report.classification().dirty_partition_scope_count(), 1);
    assert_eq!(report.classification().contract_partition_scope_count(), 1);
    assert!(report.resource_revalidation().is_none());
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_partition_local_refresh_count,
        0
    );
}

#[test]
fn async_node_partition_local_revalidation_matches_contract_scope_and_records_locality() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let node = graph
        .node()
        .reads_aspects([ASPECT_A])
        .with_partition_scope(PartitionSubscription::partition_and_detail(
            "wing", "rib-12",
        ))
        .build();
    graph
        .append_partition_detail_dependency(node, source, ASPECT_A, "wing", "rib-12")
        .expect("partition dependency should wire");
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut node_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    evaluate(&mut graph, source, &mut source_v1).expect("source should evaluate");
    evaluate(&mut graph, node, &mut node_v1).expect("dependent should evaluate");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_declaration(node))
        .expect("async capability declaration should lower");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(node))
        .expect("initial request should admit");

    mark_dirty_with_regions(
        runtime.graph_mut(),
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .expect("matching changed region should mark dirty");
    evaluate(runtime.graph_mut(), source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    })
    .expect("producer commit should resolve the consumer-local scoped cause");

    let report = runtime
        .revalidate_async_node(AsyncNodeRevalidationIntent::new(node))
        .expect("matching partition-local change should revalidate");

    assert_eq!(
        report.classification().class(),
        AsyncNodeAdmissionClass::AdmittedNewLineage
    );
    assert_eq!(report.classification().dirty_partition_scope_count(), 1);
    assert_eq!(report.classification().contract_partition_scope_count(), 1);
    assert!(
        report.resource_revalidation().is_some(),
        "matching partition-local revalidation should drive resource truth"
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_aspect_local_refresh_count,
        1
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_partition_local_refresh_count,
        1
    );
}
