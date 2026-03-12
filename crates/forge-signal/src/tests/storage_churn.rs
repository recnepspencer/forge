use crate::facade::*;
use crate::tests::support::*;

#[test]
fn compaction_and_slot_reuse_are_observable_through_storage_metrics() {
    let mut runtime = SignalRuntime::builder(SignalGraph::with_gc_threshold(1)).with_kernel_defaults().build();
    let root = runtime.graph_mut().node().build();
    let partitioned = runtime.graph_mut().node().partitioned_output().build();
    let localized = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_partition_detail_dependency(localized, partitioned, ASPECT_A, "desk-0", "book-0")
        .unwrap();
    let leaves = (0..8)
        .map(|_| runtime.graph_mut().node().build())
        .collect::<Vec<_>>();

    for round in 0..8 {
        for (index, &leaf) in leaves.iter().enumerate() {
            let aspect = if (round + index) % 2 == 0 {
                ASPECT_A
            } else {
                ASPECT_B
            };
            let _ = runtime.graph_mut().drop_dependency(leaf, root, ASPECT_A);
            let _ = runtime.graph_mut().drop_dependency(leaf, root, ASPECT_B);
            runtime
                .graph_mut()
                .append_dependency(leaf, root, aspect)
                .unwrap();
        }
        runtime.graph_mut().run_gc_epoch();
    }
    runtime
        .graph_mut()
        .rebuild_subscriber_index_from_dependencies()
        .unwrap();

    let metrics = runtime.graph().observe().metrics();
    assert!(metrics.storage.graph_storage_compaction_count >= 1);
    assert!(metrics.storage.graph_storage_dependency_segments_rewritten >= 1);
    assert!(metrics.storage.graph_storage_subscriber_segments_rewritten >= 1);
    assert!(metrics.invalidation.partition_interner_growth_delta >= 1);
    assert!(metrics.storage.subscriber_index_rebuild_count >= 1);
}