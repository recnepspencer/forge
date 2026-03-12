use crate::facade::*;
use crate::tests::support::*;

#[cfg(feature = "parallel")]
#[test]
fn transaction_parallel_execution_increments_parallel_not_serial_runtime_usage() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let nodes = (0..16)
        .map(|_| runtime.graph_mut().node().build())
        .collect::<Vec<_>>();
    let mut ctx = ();

    let mut tx = runtime.begin(&mut ctx);
    for &node in &nodes {
        tx.mark_dirty(node, ASPECT_A).unwrap();
    }
    let report = tx
        .evaluate_dirty_with_executor(&(), 
            &|_node, view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();
    tx.commit().unwrap();

    let metrics = runtime.observe().metrics();
    assert!(report
        .stages
        .iter()
        .any(|stage| matches!(stage.outcome, StageExecutionOutcome::CompletedParallel)));
    assert_eq!(metrics.execution.serial_executor_usage_count, 0);
    assert_eq!(metrics.execution.parallel_executor_usage_count, 1);
    assert!(metrics.execution.parallel_stage_dispatch_count >= 1);
}

#[test]
fn direct_whole_partition_changes_are_counted_as_partition_matches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let whole = graph.node().build();
    let detail = graph.node().build();
    graph
        .append_partition_dependency(whole, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_detail_dependency(detail, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[crate::data::output::ChangedRegion::new("wing")],
    )
    .unwrap();

    let metrics = graph.observe().metrics();
    assert_eq!(metrics.invalidation.partition_match_dirty_count, 1);
    assert_eq!(metrics.invalidation.detail_match_dirty_count, 1);
}