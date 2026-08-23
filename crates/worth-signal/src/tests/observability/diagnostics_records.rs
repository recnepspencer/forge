use crate::facade::{
    mark_dirty_with_regions, ChangedRegion, DiagnosticsTier, SignalGraph, SignalRuntimePolicy,
};
use crate::tests::support::{GraphDependencyBatchExt, ASPECT_A};

#[test]
fn diagnostics_access_exposes_source_only_planning_estimate() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let observation = graph.observe();
    let estimate = observation
        .latest_invalidation_planning_estimate()
        .expect("source-only invalidation estimate should be available");
    assert_eq!(estimate.seed_count(), 1);
    assert_eq!(estimate.direct_candidate_count(), 0);
}

#[test]
fn pending_invalidation_summary_is_retained_and_served_without_cold_work() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let before = graph
        .observe()
        .metrics()
        .storage
        .explicit_cold_materialization_request_count;
    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let pending = graph
        .diagnostics_state()
        .pending_graph_summary()
        .cloned()
        .expect("pending invalidation should retain a graph summary");
    let summary = graph
        .observe()
        .diagnostics_summary(DiagnosticsTier::Development);
    let after = graph
        .observe()
        .metrics()
        .storage
        .explicit_cold_materialization_request_count;

    assert_eq!(summary, pending.with_profile(DiagnosticsTier::Development));
    assert!(summary.dirty_node_count >= 1);
    assert_eq!(
        before, after,
        "ordinary dirty-state summary reads must not trigger cold materialization work"
    );
}

#[test]
fn operational_diagnostics_do_not_retain_frontier_trace_records_by_default() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let observation = graph
        .begin_observation_session(crate::facade::SignalObservationRequest::frontier())
        .unwrap();
    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();
    graph
        .finish_optional_invalidation_execution_observation(&observation)
        .unwrap();

    let diagnostics = graph.observe().diagnostics();
    assert!(diagnostics.latest_frontier_execution().is_some());
    assert!(diagnostics.latest_invalidation_trace_records().is_empty());
}

#[test]
fn observer_reads_do_not_mutate_frontier_truth_or_retain_extra_trace_records() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().partitioned_output().build();
    let dependent = graph.node().build();
    graph
        .append_partition_detail_dependency(dependent, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-12")],
    )
    .unwrap();

    let summary_before = graph
        .observe()
        .latest_frontier_execution_summary()
        .cloned()
        .expect("frontier execution summary should exist");
    let traces_before = graph.observe().latest_invalidation_trace_records().to_vec();
    let metrics_before = graph
        .observe()
        .metrics()
        .invalidation
        .frontier_trace_retained_count;

    let diagnostics = graph.observe().diagnostics();
    let summary_after = diagnostics
        .latest_frontier_execution()
        .cloned()
        .expect("frontier execution summary should remain available");
    let traces_after = diagnostics.latest_invalidation_trace_records().to_vec();
    let metrics_after = graph
        .observe()
        .metrics()
        .invalidation
        .frontier_trace_retained_count;

    assert_eq!(summary_before, summary_after);
    assert_eq!(traces_before, traces_after);
    assert_eq!(metrics_before, metrics_after);
}
