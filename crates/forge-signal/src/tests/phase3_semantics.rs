use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::*;
use crate::tests::support::*;

#[test]
fn output_identity_unchanged_suppresses_downstream_propagation() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact"))
    };
    let mut source_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact"))
    };
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Dirty);

    evaluate(&mut graph, source, &mut source_v2_same_identity).unwrap();

    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
    let explanation = graph.explain(source).unwrap();
    assert_eq!(explanation.output_change, Some(OutputChange::Unchanged));
    assert!(explanation.propagation_suppressed);
    assert_eq!(graph.metrics().suppressed_downstream_propagations, 1);
}

#[test]
fn output_identity_suppression_does_not_hide_other_real_upstream_changes() {
    let mut graph = SignalGraph::new();
    let source_a = graph.node().output_identity().build();
    let source_b = graph.node().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source_a, ASPECT_A).unwrap();
    graph.add_dependency(dependent, source_b, ASPECT_B).unwrap();

    let mut source_a_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact-a"))
    };
    let mut source_a_v2_same_identity = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("artifact-a"))
    };
    let mut source_b_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut source_b_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 2));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 10)))
    };

    evaluate(&mut graph, source_a, &mut source_a_v1).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty(&mut graph, source_a, ASPECT_A).unwrap();
    mark_dirty(&mut graph, source_b, ASPECT_B).unwrap();
    evaluate(&mut graph, source_a, &mut source_a_v2_same_identity).unwrap();
    evaluate(&mut graph, source_b, &mut source_b_v2).unwrap();

    assert_ne!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn changed_regions_flow_into_trace_and_explanation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().partitioned_output().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing-panel").with_detail("rib-12")))
    };

    evaluate(&mut graph, node, &mut compute).unwrap();

    let explanation = graph.explain(node).unwrap();
    assert_eq!(explanation.changed_regions.len(), 1);
    assert_eq!(
        explanation
            .trace_summary
            .as_ref()
            .unwrap()
            .changed_partition_count,
        1
    );
    assert_eq!(graph.metrics().partition_aware_recomputations, 1);
}

#[test]
fn keyed_node_lookup_reuses_same_runtime_entry() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("fighter-projection");

    let node_a = runtime.keyed_node(&family, "left-wing");
    let node_b = runtime.keyed_node(&family, "left-wing");
    let node_c = runtime.keyed_node(&family, "right-wing");

    assert_eq!(node_a, node_b);
    assert_ne!(node_a, node_c);
}

#[test]
fn keyed_evaluation_can_reuse_memoized_result() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("projection");
    let node = runtime.keyed_node(&family, "bulkhead");
    let computation = KeyedComputation::new(family.clone(), "bulkhead").with_memo_key("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("bulkhead-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), node, ASPECT_A).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.explain(node).unwrap();
    assert_eq!(
        explanation.memoized_origin,
        Some(MemoizedResultOrigin::MemoizedFromCache)
    );
    let metrics = runtime.metrics();
    assert_eq!(metrics.keyed_evaluation_count, 2);
    assert_eq!(metrics.memoization_hits, 1);
    assert_eq!(metrics.memoization_misses, 1);
}

#[test]
fn memoization_is_scoped_by_family() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family_a = runtime.register_computation_family("projection-a");
    let family_b = runtime.register_computation_family("projection-b");
    let node_a = runtime.keyed_node(&family_a, "bulkhead");
    let node_b = runtime.keyed_node(&family_b, "bulkhead");
    let computation_a =
        KeyedComputation::new(family_a.clone(), "bulkhead").with_memo_key("shape-v1");
    let computation_b =
        KeyedComputation::new(family_b.clone(), "bulkhead").with_memo_key("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node_a, &computation_a, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("a"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node_b, &computation_b, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0)).with_output_identity("b"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);
}

#[test]
fn memoization_write_is_discarded_on_rollback() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("projection");
    let node = runtime.keyed_node(&family, "bulkhead");
    let computation = KeyedComputation::new(family.clone(), "bulkhead").with_memo_key("shape-v1");
    let mut runtime_ctx = ();
    let compute_calls = AtomicU32::new(0);

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.evaluate_keyed(node, &computation, &|_id, view| {
            compute_calls.fetch_add(1, Ordering::Relaxed);
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("cached"),
            ))
        })?;
        Err(SignalError::invalid_input("force rollback"))
    });
    assert!(err.is_err());

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(node, &computation, &|_id, view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("fresh"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(compute_calls.load(Ordering::Relaxed), 2);
    let metrics = runtime.metrics();
    assert_eq!(metrics.memoization_hits, 0);
    assert_eq!(metrics.memoization_misses, 2);
}

#[test]
fn aborted_keyed_evaluation_does_not_leak_key_registry_growth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let node = runtime.graph_mut().node().build();
    let family = ComputationFamily::from("fresh-family");
    let computation = KeyedComputation::new(family.clone(), "fresh-key").with_memo_key("fresh-v1");
    let before = runtime.config().test_registry_counts();
    let mut runtime_ctx = ();

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.evaluate_keyed(node, &computation, &|_id, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("cached"),
            ))
        })?;
        Err(SignalError::invalid_input("force rollback"))
    });
    assert!(err.is_err());

    assert_eq!(
        runtime.config().test_registry_counts(),
        before,
        "aborted keyed evaluation must not leak family/key/memo registry entries"
    );
}

#[test]
fn partition_subscribers_only_dirty_on_matching_partition() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let wing_subscriber = graph.node().build();
    let tail_subscriber = graph.node().build();
    graph
        .add_partition_dependency(wing_subscriber, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .add_partition_dependency(tail_subscriber, source, ASPECT_A, "tail")
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, wing_subscriber, &mut subscriber_compute).unwrap();
    evaluate(&mut graph, tail_subscriber, &mut subscriber_compute).unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(graph.get_state(wing_subscriber).unwrap(), NodeState::Dirty);
    assert_eq!(
        graph.get_state(tail_subscriber).unwrap(),
        NodeState::MaybeStale
    );
    assert_eq!(graph.metrics().partition_match_dirty_count, 1);
    assert_eq!(graph.metrics().partition_scoped_invalidation_checks, 2);
}

#[test]
fn detail_sensitive_partition_subscriber_reverts_clean_when_detail_does_not_match() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let subscriber = graph.node().build();
    graph
        .add_partition_detail_dependency(subscriber, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    let mut source_rib_12 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut source_rib_13 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_rib_12).unwrap();
    evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();
    assert_eq!(graph.get_state(subscriber).unwrap(), NodeState::MaybeStale);

    evaluate(&mut graph, source, &mut source_rib_13).unwrap();
    evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();

    assert_eq!(graph.get_state(subscriber).unwrap(), NodeState::Clean);
    let explanation = graph.explain(subscriber).unwrap();
    assert!(matches!(
        explanation.upstream.as_slice(),
        [UpstreamCause::Clean { subscription: Some(subscription), .. }]
        if subscription.partition == PartitionToken::new("wing")
            && subscription.detail.as_deref() == Some("rib-12")
    ));
    assert_eq!(graph.metrics().partition_scope_revert_clean_count, 1);
}

#[test]
fn mixed_whole_aspect_and_partition_subscribers_behave_deterministically() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let whole_aspect_subscriber = graph.node().build();
    let matching_partition_subscriber = graph.node().build();
    let non_matching_partition_subscriber = graph.node().build();
    graph
        .add_dependency(whole_aspect_subscriber, source, ASPECT_A)
        .unwrap();
    graph
        .add_partition_dependency(matching_partition_subscriber, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .add_partition_dependency(non_matching_partition_subscriber, source, ASPECT_A, "tail")
        .unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut source_v2 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, whole_aspect_subscriber, &mut subscriber_compute).unwrap();
    evaluate(
        &mut graph,
        matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();
    evaluate(
        &mut graph,
        non_matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();

    mark_dirty_with_regions(&mut graph, source, ASPECT_A, &[ChangedRegion::new("wing")]).unwrap();

    assert_eq!(
        graph.get_state(whole_aspect_subscriber).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        graph.get_state(matching_partition_subscriber).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        graph.get_state(non_matching_partition_subscriber).unwrap(),
        NodeState::MaybeStale
    );

    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(
        &mut graph,
        non_matching_partition_subscriber,
        &mut subscriber_compute,
    )
    .unwrap();
    assert_eq!(
        graph.get_state(non_matching_partition_subscriber).unwrap(),
        NodeState::Clean
    );
}

#[test]
fn partition_scoped_cleanup_does_not_hide_other_dirty_upstreams() {
    let mut graph = SignalGraph::new();
    let source_partitioned = graph.node().partitioned_output().build();
    let source_other = graph.node().build();
    let dependent = graph.node().build();
    graph
        .add_partition_detail_dependency(dependent, source_partitioned, ASPECT_A, "wing", "rib-12")
        .unwrap();
    graph
        .add_dependency(dependent, source_other, ASPECT_B)
        .unwrap();

    let mut partitioned_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-12")))
    };
    let mut partitioned_v2_other_detail = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_changed_region(ChangedRegion::new("wing").with_detail("rib-13")))
    };
    let mut other_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 1));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 10)))
    };

    evaluate(&mut graph, source_partitioned, &mut partitioned_v1).unwrap();
    evaluate(&mut graph, source_other, &mut other_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source_partitioned,
        ASPECT_A,
        &[ChangedRegion::new("wing").with_detail("rib-13")],
    )
    .unwrap();
    mark_dirty(&mut graph, source_other, ASPECT_B).unwrap();
    evaluate(
        &mut graph,
        source_partitioned,
        &mut partitioned_v2_other_detail,
    )
    .unwrap();

    assert_ne!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn transaction_mark_dirty_with_regions_routes_partition_matches() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let source = runtime.graph_mut().node().partitioned_output().build();
    let matching = runtime.graph_mut().node().build();
    let non_matching = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .add_partition_dependency(matching, source, ASPECT_A, "wing")
        .unwrap();
    runtime
        .graph_mut()
        .add_partition_dependency(non_matching, source, ASPECT_A, "tail")
        .unwrap();

    runtime
        .transaction(&mut (), |tx| {
            tx.mark_dirty_with_regions(source, ASPECT_A, &[ChangedRegion::new("wing")])?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime.graph().get_state(matching).unwrap(),
        NodeState::Dirty
    );
    assert_eq!(
        runtime.graph().get_state(non_matching).unwrap(),
        NodeState::MaybeStale
    );
}

#[test]
fn sparse_partition_fanout_keeps_most_subscribers_out_of_dirty_state() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let mut subscribers = Vec::new();
    for index in 0..128 {
        let subscriber = graph.node().build();
        graph
            .add_partition_dependency(subscriber, source, ASPECT_A, format!("partition-{index}"))
            .unwrap();
        subscribers.push(subscriber);
    }

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_changed_region(ChangedRegion::new("partition-7")))
    };
    let mut subscriber_compute = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    for &subscriber in &subscribers {
        evaluate(&mut graph, subscriber, &mut subscriber_compute).unwrap();
    }

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[ChangedRegion::new("partition-7")],
    )
    .unwrap();

    let dirty_count = subscribers
        .iter()
        .filter(|&&subscriber| graph.get_state(subscriber).unwrap() == NodeState::Dirty)
        .count();
    let maybe_stale_count = subscribers
        .iter()
        .filter(|&&subscriber| graph.get_state(subscriber).unwrap() == NodeState::MaybeStale)
        .count();

    assert_eq!(dirty_count, 1);
    assert_eq!(maybe_stale_count, 127);
    assert_eq!(graph.metrics().partition_scoped_invalidation_checks, 128);
    assert_eq!(graph.metrics().partition_match_dirty_count, 1);
}
