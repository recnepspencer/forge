use crate::facade::*;
use crate::tests::support::*;

#[test]
fn partition_scoped_dependencies_on_same_source_check_all_matching_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();

    assert_eq!(
        graph.get_state(dependent).unwrap(),
        NodeState::Dirty,
        "later partition-scoped dependency edges on the same source/aspect must still be checked"
    );
}

#[test]
fn repeated_partition_invalidations_union_dirty_scopes_until_evaluation() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .add_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();
    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();

    let entry = graph.get_entry(dependent).unwrap();
    let scopes = entry.get_dirty_partition_scopes();
    assert_eq!(entry.get_state(), &NodeState::Dirty);
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "wing" && scope.detail.as_deref() == Some("rib-12")
        }),
        "the first invalidation scope should not be erased by a later wave"
    );
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "wing" && scope.detail.as_deref() == Some("rib-13")
        }),
        "the second invalidation scope should be merged with earlier scopes"
    );
}
