use crate::facade::*;
use crate::tests::async_node_support::{
    async_node_capability_declaration, async_node_capability_with_dependents,
    AsyncNodeTestRuntime as TestRuntime,
};

#[test]
fn async_node_hierarchy_cancellation_propagates_and_replay_summary_restores_identically() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("child should depend on parent");
    graph
        .append_dependency(grandchild, child, Aspect::new(0))
        .expect("grandchild should depend on child");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("parent capability should lower");
    runtime
        .declare_async_node_capability(async_node_capability_with_dependents(child, [grandchild]))
        .expect("child capability should lower");
    runtime
        .declare_async_node_capability(async_node_capability_declaration(grandchild))
        .expect("grandchild capability should lower");

    let parent_handle = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(parent))
        .expect("parent should admit")
        .resource_admission()
        .expect("parent should expose resource admission")
        .admitted_request()
        .handle();
    let child_handle = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(child))
        .expect("child should admit")
        .resource_admission()
        .expect("child should expose resource admission")
        .admitted_request()
        .handle();
    let grandchild_handle = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(grandchild))
        .expect("grandchild should admit")
        .resource_admission()
        .expect("grandchild should expose resource admission")
        .admitted_request()
        .handle();

    let baseline = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("hierarchy replay summary should materialize");
    let snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");

    assert_eq!(baseline.root_node(), parent);
    assert_eq!(baseline.hierarchy_nodes(), &[parent, child, grandchild]);
    assert_eq!(baseline.active_request_handles().len(), 3);
    assert_eq!(baseline.hierarchy_depth(), 2);
    assert_eq!(
        baseline.performance().boundary(),
        ResourceBoundaryKind::AsyncNodeHierarchyReplay
    );

    let cancellation = runtime
        .cancel_async_node_request(parent_handle, ResourceCancellationReason::HostRequested)
        .expect("hierarchical cancellation should succeed");

    assert_eq!(cancellation.root_node(), parent);
    assert_eq!(cancellation.affected_nodes(), &[parent, child, grandchild]);
    assert_eq!(cancellation.propagated_hierarchy_width(), 2);
    let propagation = cancellation
        .cancellation()
        .dependent_propagation()
        .expect("declared dependent cancellation footprint should propagate");
    assert_eq!(propagation.cancelled_dependent_width(), 2);
    assert_eq!(
        propagation
            .cancelled_dependents()
            .iter()
            .map(CancelledResourceRequest::handle)
            .collect::<Vec<_>>(),
        vec![child_handle, grandchild_handle]
    );
    assert_eq!(
        runtime
            .telemetry()
            .resource
            .async_node_hierarchical_propagation_count,
        2
    );

    runtime
        .restore_snapshot(&snapshot)
        .expect("restore should reinstate hierarchy truth");
    let restored = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("restored hierarchy replay summary should materialize");

    assert_eq!(restored.hierarchy_nodes(), baseline.hierarchy_nodes());
    assert_eq!(
        restored
            .active_request_handles()
            .iter()
            .map(|handle| (handle.request_id(), handle.generation()))
            .collect::<Vec<_>>(),
        vec![
            (parent_handle.request_id(), parent_handle.generation()),
            (child_handle.request_id(), child_handle.generation()),
            (
                grandchild_handle.request_id(),
                grandchild_handle.generation()
            ),
        ]
    );
    assert_eq!(restored.lifecycle_digest(), baseline.lifecycle_digest());
    assert_eq!(restored.in_flight_digest(), baseline.in_flight_digest());
    assert_eq!(restored.replay_digest(), baseline.replay_digest());
}

#[test]
fn async_node_hierarchy_restore_is_branch_local_and_checkpoint_honest() {
    let mut graph = SignalGraph::new();
    let parent = graph.node().build();
    let child = graph.node().build();
    let grandchild = graph.node().build();
    graph
        .append_dependency(child, parent, Aspect::new(0))
        .expect("child should depend on parent");
    graph
        .append_dependency(grandchild, child, Aspect::new(0))
        .expect("grandchild should depend on child");

    let mut runtime = TestRuntime::build(graph);
    runtime
        .declare_async_node_capability(async_node_capability_with_dependents(parent, [child]))
        .expect("parent capability should lower");
    runtime
        .declare_async_node_capability(async_node_capability_with_dependents(child, [grandchild]))
        .expect("child capability should lower");
    runtime
        .declare_async_node_capability(async_node_capability_declaration(grandchild))
        .expect("grandchild capability should lower");

    let main = runtime.observe().current_branch();
    let feature = runtime
        .create_branch("async-hierarchy-feature")
        .expect("feature branch should create");
    let sibling = runtime
        .create_branch("async-hierarchy-sibling")
        .expect("sibling branch should create");

    runtime
        .switch_branch(feature.clone())
        .expect("feature branch should activate");
    let feature_parent_handle = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(parent))
        .expect("feature parent should admit")
        .resource_admission()
        .expect("feature parent should expose resource admission")
        .admitted_request()
        .handle();
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(child))
        .expect("feature child should admit");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(grandchild))
        .expect("feature grandchild should admit");
    let feature_snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let feature_head_before_restore = runtime.observe().branch_head_snapshot_id(feature.id);
    let feature_before_restore = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("feature hierarchy summary should materialize");
    runtime
        .cancel_async_node_request(
            feature_parent_handle,
            ResourceCancellationReason::HostRequested,
        )
        .expect("feature hierarchy cancellation should succeed");
    let feature_after_snapshot_drift = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("feature drifted hierarchy summary should materialize");

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should activate");
    let sibling_parent_handle = runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(parent))
        .expect("sibling parent should admit")
        .resource_admission()
        .expect("sibling parent should expose resource admission")
        .admitted_request()
        .handle();
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(child))
        .expect("sibling child should admit");
    runtime
        .admit_async_node_request(AsyncNodeRequestIntent::new(grandchild))
        .expect("sibling grandchild should admit");
    let sibling_snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
    let sibling_head_before_restore = runtime.observe().branch_head_snapshot_id(sibling.id);
    let sibling_before_restore = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("sibling hierarchy summary should materialize");
    runtime
        .cancel_async_node_request(
            sibling_parent_handle,
            ResourceCancellationReason::HostRequested,
        )
        .expect("sibling hierarchy cancellation should succeed");
    let sibling_after_snapshot_drift = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("sibling drifted hierarchy summary should materialize");

    runtime
        .switch_branch(main)
        .expect("main branch should reactivate before inactive restore");
    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .expect("inactive feature restore should succeed");

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should still be independently accessible");
    let sibling_still_drifted = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("sibling hierarchy summary should remain drifted");
    assert_eq!(
        sibling_still_drifted.replay_digest(),
        sibling_after_snapshot_drift.replay_digest(),
        "restoring feature must not mutate sibling branch-local hierarchy truth"
    );

    runtime
        .switch_branch(feature.clone())
        .expect("feature branch should activate after restore");
    let feature_head_after_restore = runtime.observe().branch_head_snapshot_id(feature.id);
    let feature_after_restore = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("feature restored hierarchy summary should materialize");
    assert_eq!(feature_head_after_restore, feature_head_before_restore);
    assert_eq!(
        feature_after_restore.replay_digest(),
        feature_before_restore.replay_digest(),
        "feature restore must reinstate the captured hierarchy story exactly"
    );
    assert_ne!(
        feature_after_restore.replay_digest(),
        feature_after_snapshot_drift.replay_digest(),
        "feature restore must erase post-snapshot drift rather than preserving it accidentally"
    );

    runtime
        .restore_branch_snapshot(sibling.clone(), &sibling_snapshot)
        .expect("inactive sibling restore should succeed");
    let feature_still_restored = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("feature hierarchy summary should remain restored");
    assert_eq!(
        feature_still_restored.replay_digest(),
        feature_after_restore.replay_digest(),
        "restoring sibling must not perturb already-restored feature hierarchy truth"
    );

    runtime
        .switch_branch(sibling.clone())
        .expect("sibling branch should activate after restore");
    let sibling_head_after_restore = runtime.observe().branch_head_snapshot_id(sibling.id);
    let sibling_after_restore = runtime
        .async_node_hierarchy_replay_summary(parent)
        .expect("sibling restored hierarchy summary should materialize");
    assert_eq!(sibling_head_after_restore, sibling_head_before_restore);
    assert_eq!(
        sibling_after_restore.replay_digest(),
        sibling_before_restore.replay_digest(),
        "sibling restore must reinstate the captured hierarchy story exactly"
    );
    assert_eq!(
        sibling_after_restore.replay_digest(),
        feature_after_restore.replay_digest(),
        "equivalent restored hierarchy suffixes must converge across sibling branches"
    );
}
