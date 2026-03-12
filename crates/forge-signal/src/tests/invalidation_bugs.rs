use crate::facade::*;
use crate::tests::support::*;

#[test]
fn partition_scoped_dependencies_on_same_source_check_all_matching_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
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
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-13")
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

#[test]
fn whole_aspect_invalidation_does_not_erase_other_aspects_partition_precision() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_B, "tail", "panel-7")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 1))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12"))
            .with_changed_region(ChangedRegion::new("tail").with_detail("panel-7")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_B,
        &[ChangedRegion::new("tail").with_detail("panel-7")],
    )
    .unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let entry = graph.get_entry(dependent).unwrap();
    let scopes = entry.get_dirty_partition_scopes();
    assert!(
        scopes.iter().any(|scope| {
            scope.partition.0.as_str() == "tail" && scope.detail.as_deref() == Some("panel-7")
        }),
        "whole-aspect invalidation on aspect A must not erase scoped dirtiness retained for aspect B"
    );
}

#[test]
fn reconverging_frontier_does_not_revisit_already_visited_nodes() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    let d = graph.node().build();
    let e = graph.node().build();

    graph.append_dependency(b, source, ASPECT_A).unwrap();
    graph.append_dependency(c, source, ASPECT_A).unwrap();
    graph.append_dependency(e, source, ASPECT_A).unwrap();
    graph.append_dependency(d, b, ASPECT_A).unwrap();
    graph.append_dependency(d, c, ASPECT_A).unwrap();
    graph.append_dependency(e, d, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)))
    })
    .unwrap();
    for node in [b, c, d, e] {
        evaluate(&mut graph, node, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0)))
        })
        .unwrap();
    }

    graph.reset_telemetry();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    assert_eq!(
        graph.telemetry().invalidation.invalidation_nodes_visited,
        4,
        "reconverging downstream nodes should only count once during transitive invalidation"
    );
}

#[test]
fn deep_invalidation_chain_completes_without_recursive_cycle_detection() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let mut previous = root;
    for _ in 0..20_000 {
        let next = graph.node().build();
        graph.append_dependency(next, previous, ASPECT_A).unwrap();
        previous = next;
    }

    let result = mark_dirty(&mut graph, root, ASPECT_A);
    assert!(
        result.is_ok(),
        "deep invalidation chains should not overflow recursive cycle detection: {result:?}"
    );
}

#[test]
fn unscoped_dependency_removal_removes_partition_scoped_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    graph
        .drop_dependency(dependent, source, ASPECT_A)
        .unwrap();

    assert!(
        graph.dependencies_of(dependent).unwrap().is_empty(),
        "unscoped dependency removal should remove matching scoped edges too"
    );
}

#[test]
fn whole_partition_invalidates_partition_detail_subscribers() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    })
    .unwrap();
    evaluate(&mut graph, dependent, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(
        graph.get_state(dependent).unwrap(),
        NodeState::Dirty,
        "whole-partition changes must invalidate detail subscribers on the same partition"
    );
}