use crate::facade::*;
use crate::tests::support::*;

#[test]
fn unscoped_dependency_removal_removes_partition_scoped_edges() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    graph.drop_dependency(dependent, source, ASPECT_A).unwrap();

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
        NodeState::Clean,
        "source seeds must not assign subscriber state before a committed delta exists"
    );

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    })
    .unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Dirty);
}

#[test]
fn source_frontier_summary_excludes_uncommitted_subscriber_classifications() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct_dirty = graph.node().build();
    let maybe_stale = graph.node().build();

    graph
        .append_partition_detail_dependency(direct_dirty, source, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .append_partition_detail_dependency(maybe_stale, source, ASPECT_A, "wing", "rib-13")
        .unwrap();
    let direct_dirty_before = graph.get_state(direct_dirty).unwrap();
    let maybe_stale_before = graph.get_state(maybe_stale).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let estimate = graph
        .observe()
        .latest_invalidation_planning_estimate()
        .cloned()
        .expect("the runtime should retain its latest caller-visible planning estimate");
    assert_eq!(estimate.seed_count(), 1);
    assert_eq!(estimate.direct_candidate_count(), 0);
    assert_eq!(estimate.partition_scoped_check_count(), 0);
    assert!(summary.direct_waves.is_empty());
    assert!(summary.transitive_waves.is_empty());
    assert_eq!(
        summary.touched_scope_summary.touched_nodes.as_slice(),
        &[source]
    );
    assert_eq!(graph.get_state(direct_dirty).unwrap(), direct_dirty_before);
    assert_eq!(graph.get_state(maybe_stale).unwrap(), maybe_stale_before);
}

#[test]
fn frontier_runtime_counters_are_derived_from_execution_summary() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let whole = graph.node().build();
    let detail = graph.node().build();

    graph
        .append_partition_dependency(whole, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_detail_dependency(detail, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let metrics = graph.observe().metrics();
    assert_eq!(
        metrics.invalidation.frontier_seed_count,
        summary.counters.frontier_seed_count
    );
    assert_eq!(
        metrics.invalidation.frontier_direct_wave_count,
        summary.counters.frontier_direct_wave_count
    );
    assert_eq!(
        metrics.invalidation.frontier_transitive_wave_count,
        summary.counters.frontier_transitive_wave_count
    );
    assert_eq!(
        metrics.invalidation.frontier_cycle_check_candidate_count,
        summary.counters.frontier_cycle_check_candidate_count
    );
}

#[test]
fn frontier_transitive_wave_count_stays_zero_when_no_transitive_entries_realize() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let direct = graph.node().build();

    graph
        .append_partition_detail_dependency(direct, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let summary = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should be retained");
    let metrics = graph.observe().metrics();

    assert!(summary
        .transitive_waves
        .iter()
        .all(|wave| wave.entries.is_empty()));
    assert_eq!(summary.counters.frontier_transitive_wave_count, 0);
    assert_eq!(metrics.invalidation.frontier_transitive_wave_count, 0);
}

#[test]
fn frontier_tracing_policy_changes_retained_richness_not_invalidation_truth() {
    let mut operational = SignalGraph::new();
    operational.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = operational.node().partitioned_output().build();
    let dependent = operational.node().build();
    operational
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut development = operational.clone();
    development.set_runtime_policy(SignalRuntimePolicy::development());

    let changed = &[ChangedRegion::new("wing").with_detail("rib-12")];
    mark_dirty_with_regions(&mut operational, source, ASPECT_A, changed).unwrap();
    mark_dirty_with_regions(&mut development, source, ASPECT_A, changed).unwrap();

    let operational_summary = operational
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("operational summary should exist");
    let development_summary = development
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("development summary should exist");

    assert_eq!(
        operational_summary.seed_count,
        development_summary.seed_count
    );
    assert_eq!(
        operational_summary.direct_waves,
        development_summary.direct_waves
    );
    assert_eq!(
        operational_summary.transitive_waves,
        development_summary.transitive_waves
    );
    assert_eq!(
        operational_summary.touched_scope_summary,
        development_summary.touched_scope_summary
    );
    assert_eq!(
        operational_summary.counters.frontier_trace_retained_count,
        0
    );
    assert_eq!(
        development_summary.counters.frontier_trace_retained_count,
        0
    );
    assert!(operational
        .observe()
        .latest_invalidation_trace_records()
        .is_empty());
    assert!(development
        .observe()
        .latest_invalidation_trace_records()
        .is_empty());
}
