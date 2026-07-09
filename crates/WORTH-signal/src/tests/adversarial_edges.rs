use crate::data::dependency::{DependencyEdge, DependencySnapshot};
use crate::facade::*;
use crate::tests::support::{version_ab, DependencyBatchBuilder, ASPECT_A, ASPECT_B};

#[test]
fn repeated_edge_churn_preserves_dependency_and_subscriber_integrity() {
    let mut graph = SignalGraph::new();
    let hub = graph.node().build();
    let leaves: Vec<_> = (0..48).map(|_| graph.node().build()).collect();

    for round in 0..40 {
        for &leaf in &leaves {
            let aspect = if round % 2 == 0 { ASPECT_A } else { ASPECT_B };
            graph.append_dependency(leaf, hub, aspect).unwrap();
            assert!(graph
                .dependencies_of(leaf)
                .unwrap()
                .iter()
                .any(|dependency| dependency.source() == hub && dependency.aspect() == aspect));
        }

        for &leaf in leaves.iter().step_by(3) {
            let aspect = if round % 2 == 0 { ASPECT_A } else { ASPECT_B };
            graph.drop_dependency(leaf, hub, aspect).unwrap();
            graph.append_dependency(leaf, hub, aspect).unwrap();
        }

        let subscribers = graph.subscribers_of(hub).unwrap();
        assert_eq!(subscribers.len(), leaves.len());
        for &leaf in &leaves {
            let deps = graph.dependencies_of(leaf).unwrap();
            assert!(!deps.is_empty());
            assert!(deps.iter().all(|dependency| dependency.source() == hub));
            let distinct_aspects = deps
                .iter()
                .map(|dependency| dependency.aspect().id())
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(distinct_aspects.len(), deps.len());
        }
    }
}

#[test]
fn rollback_after_dynamic_dependency_churn_restores_original_dependencies() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source_a = runtime.graph_mut().node().build();
    let source_b = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    let mut ctx = ();

    runtime
        .transaction(&mut ctx, |tx| {
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let result = if view.node() == source_a {
                        crate::logic::evaluation::EvaluationOutput::from_result(version_ab(1, 0))
                    } else if view.node() == source_b {
                        crate::logic::evaluation::EvaluationOutput::from_result(version_ab(2, 0))
                    } else {
                        let version = view.read_aspect_version(source_a, ASPECT_A)?;
                        view.finish(version)
                    };
                    Ok(result)
                },
                EvaluationRequestMode::ForceOnDemand,
            )?;
            Ok(())
        })
        .unwrap();

    let err = runtime.transaction(&mut ctx, |tx| {
        tx.mark_dirty(source_b, ASPECT_A)?;
        tx.evaluate_with_plan(
            dependent,
            &|view| {
                let result = if view.node() == source_a {
                    crate::logic::evaluation::EvaluationOutput::from_result(version_ab(1, 0))
                } else if view.node() == source_b {
                    crate::logic::evaluation::EvaluationOutput::from_result(version_ab(3, 0))
                } else {
                    let version = view.read_aspect_version(source_b, ASPECT_A)?;
                    view.finish(version)
                };
                Ok(result)
            },
            EvaluationRequestMode::Default,
        )?;
        Err(SignalError::invalid_input(
            "force rollback after dependency churn",
        ))
    });
    assert!(err.is_err());

    let dependencies = runtime.graph().dependencies_of(dependent).unwrap();
    assert_eq!(dependencies.len(), 1);
    assert_eq!(dependencies[0].source(), source_a);
    let explanation = runtime.observe().explain(dependent).unwrap();
    assert!(explanation
        .upstream
        .iter()
        .all(|cause| format!("{cause:?}").contains(&source_a.to_string())
            || !format!("{cause:?}").contains(&source_b.to_string())));
}

#[test]
fn unregister_and_slot_reuse_after_churn_leave_no_ghost_edges() {
    let mut graph = SignalGraph::new();
    let upstream = graph.node().build();
    let middle = graph.node().build();
    let downstreams: Vec<_> = (0..24).map(|_| graph.node().build()).collect();

    graph.append_dependency(middle, upstream, ASPECT_A).unwrap();
    for &downstream in &downstreams {
        graph
            .append_dependency(downstream, middle, ASPECT_B)
            .unwrap();
    }

    for &downstream in downstreams.iter().step_by(2) {
        graph.drop_dependency(downstream, middle, ASPECT_B).unwrap();
        graph
            .append_dependency(downstream, middle, ASPECT_B)
            .unwrap();
    }

    graph.unregister_node(middle).unwrap();
    let replacement = graph.node().build();

    assert!(graph.subscribers_of(upstream).unwrap().is_empty());
    for &downstream in &downstreams {
        assert!(
            graph
                .runtime_dependencies_of(downstream)
                .unwrap()
                .is_empty(),
            "runtime cleanup should clear stale edges left behind by retirement"
        );
    }
    assert_ne!(replacement, middle);
}

#[test]
fn snapshot_churn_reorders_dependencies_without_ghost_snapshots() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let a = runtime.graph_mut().node().build();
    let b = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let result = if view.node() == a {
                        view.finish(version_ab(1, 0))
                    } else if view.node() == b {
                        view.finish(version_ab(2, 0))
                    } else {
                        let version = view.read_aspect_version(a, ASPECT_A)?;
                        view.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                EvaluationRequestMode::ForceOnDemand,
            )?;
            Ok(())
        })
        .unwrap();

    assert_eq!(
        runtime
            .graph()
            .get_dep_snapshot(dependent)
            .unwrap()
            .entries()
            .len(),
        1
    );

    runtime
        .graph_mut()
        .drop_dependency(dependent, a, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .append_dependency(dependent, b, ASPECT_A)
        .unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(b, ASPECT_A)?;
            tx.evaluate_with_plan(
                dependent,
                &|view| {
                    let result = if view.node() == a {
                        view.finish(version_ab(1, 0))
                    } else if view.node() == b {
                        view.finish(version_ab(3, 0))
                    } else {
                        let version = view.read_aspect_version(b, ASPECT_A)?;
                        view.finish(NodeEvaluationResult::from_version(version))
                    };
                    Ok(result)
                },
                EvaluationRequestMode::Default,
            )?;
            Ok(())
        })
        .unwrap();

    let snapshot = runtime.graph().get_dep_snapshot(dependent).unwrap();
    assert_eq!(snapshot.entries().len(), 1);
    assert_eq!(snapshot.entries()[0].source, b);

    let explanation = runtime.observe().explain(dependent).unwrap();
    assert!(!format!("{:?}", explanation).contains(&a.to_string()));
}

#[test]
fn reconverging_invalidation_path_is_not_reported_as_a_cycle() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let direct_b = graph.node().build();
    let direct_c = graph.node().build();
    let direct_d = graph.node().build();
    let direct_e = graph.node().build();

    let mut dependencies = DependencyBatchBuilder::new(&mut graph);
    dependencies
        .append_dependency(direct_b, source, ASPECT_A)
        .unwrap()
        .append_dependency(direct_c, source, ASPECT_A)
        .unwrap()
        .append_dependency(direct_d, source, ASPECT_A)
        .unwrap()
        .append_dependency(direct_e, source, ASPECT_A)
        .unwrap()
        .append_dependency(direct_d, direct_b, ASPECT_A)
        .unwrap()
        .append_dependency(direct_d, direct_c, ASPECT_A)
        .unwrap()
        .append_dependency(direct_e, direct_d, ASPECT_A)
        .unwrap();
    dependencies.commit().unwrap();

    let result = mark_dirty(&mut graph, source, ASPECT_A);

    assert!(
        result.is_ok(),
        "reconverging DAG invalidation should not be treated as a circular reference: {result:?}"
    );
}

#[test]
fn gc_epoch_compacts_edge_and_snapshot_storage_after_churn() {
    let mut runtime = SignalRuntime::builder(SignalGraph::with_gc_threshold(1))
        .with_kernel_defaults()
        .build();
    let source_a = runtime.graph_mut().node().build();
    let source_b = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    let mut runtime_ctx = ();

    for round in 0..24 {
        runtime
            .graph_mut()
            .set_dependencies(
                dependent,
                [DependencyEdge::new(
                    if round % 2 == 0 { source_a } else { source_b },
                    ASPECT_A,
                )],
            )
            .unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(if round % 2 == 0 { source_a } else { source_b }, ASPECT_A)?;
                tx.evaluate_with_plan(
                    dependent,
                    &|view| {
                        let result = if view.node() == source_a {
                            view.finish(version_ab(round as u64 + 1, 0))
                        } else if view.node() == source_b {
                            view.finish(version_ab(round as u64 + 100, 0))
                        } else {
                            let source = if round % 2 == 0 { source_a } else { source_b };
                            let version = view.read_aspect_version(source, ASPECT_A)?;
                            view.finish(NodeEvaluationResult::from_version(version))
                        };
                        Ok(result)
                    },
                    EvaluationRequestMode::Default,
                )?;
                Ok(())
            })
            .unwrap();
    }

    let before = runtime.graph().test_storage_counts();
    runtime.graph_mut().run_gc_epoch();
    let after = runtime.graph().test_storage_counts();

    assert!(
        after.0 .1 <= 4,
        "dependency edge segments should compact back near live-node cardinality after GC: before={before:?} after={after:?}"
    );
    assert!(
        after.1 .1 <= 4,
        "subscriber edge segments should compact back near live-node cardinality after GC: before={before:?} after={after:?}"
    );
    assert!(
        after.2 <= 2,
        "dependency snapshots should compact back near live snapshot count after GC: before={before:?} after={after:?}"
    );
}

#[test]
fn semantically_identical_dependency_snapshots_deduplicate_even_if_recorded_in_different_orders() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let dependent = graph.node().build();

    let mut left = DependencySnapshot::empty();
    left.record(a, ASPECT_A, 1, None);
    left.record(b, ASPECT_B, 2, None);

    let mut right = DependencySnapshot::empty();
    right.record(b, ASPECT_B, 2, None);
    right.record(a, ASPECT_A, 1, None);

    graph.set_dep_snapshot(dependent, left).unwrap();
    let first = graph.get_entry(dependent).unwrap().get_dep_snapshot_id();
    graph.set_dep_snapshot(dependent, right).unwrap();
    let second = graph.get_entry(dependent).unwrap().get_dep_snapshot_id();

    assert_eq!(
        first, second,
        "snapshot storage should deduplicate canonical-equal snapshots regardless of record order"
    );
}

#[test]
fn dependency_snapshot_growth_returns_near_live_state_after_gc() {
    let mut runtime = SignalRuntime::builder(SignalGraph::with_gc_threshold(1))
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().build();
    let dependent = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime_ctx = ();

    for round in 0..64 {
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.evaluate_with_plan(
                    dependent,
                    &|view| {
                        let result = if view.node() == source {
                            view.finish(version_ab(round as u64 + 1, 0))
                        } else {
                            let version = view.read_aspect_version(source, ASPECT_A)?;
                            view.finish(NodeEvaluationResult::from_version(version))
                        };
                        Ok(result)
                    },
                    EvaluationRequestMode::Default,
                )?;
                Ok(())
            })
            .unwrap();
    }

    let before = runtime.graph().test_storage_counts();
    runtime.graph_mut().run_gc_epoch();
    let after = runtime.graph().test_storage_counts();
    assert!(
        after.2 <= 2,
        "dependency snapshot storage should compact back near live snapshot count after churn: before={before:?} after={after:?}"
    );
}

#[test]
fn identical_dependency_snapshots_are_deduplicated_before_gc() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut snapshot = DependencySnapshot::empty();
    snapshot.record(source, ASPECT_A, 1, None);
    graph.set_dep_snapshot(dependent, snapshot.clone()).unwrap();
    let first = graph.get_entry(dependent).unwrap().get_dep_snapshot_id();
    graph.set_dep_snapshot(dependent, snapshot).unwrap();
    let second = graph.get_entry(dependent).unwrap().get_dep_snapshot_id();

    assert_eq!(first, second);
    assert_eq!(graph.test_storage_counts().2, 1);
}

#[test]
#[ignore = "stress coverage for large edge rewrites and slot reuse"]
fn stress_edge_rewrites_across_reused_nodes() {
    let mut graph = SignalGraph::new();
    let roots: Vec<_> = (0..128).map(|_| graph.node().build()).collect();
    let leaves: Vec<_> = (0..512).map(|_| graph.node().build()).collect();

    for round in 0..200 {
        for (index, &leaf) in leaves.iter().enumerate() {
            let root = roots[(index + round) % roots.len()];
            let aspect = if (index + round) % 2 == 0 {
                ASPECT_A
            } else {
                ASPECT_B
            };
            let _ = graph.drop_dependency(leaf, roots[index % roots.len()], ASPECT_A);
            let _ = graph.drop_dependency(leaf, roots[index % roots.len()], ASPECT_B);
            graph.append_dependency(leaf, root, aspect).unwrap();
        }
    }

    for &root in &roots {
        for &subscriber in graph.subscribers_of(root).unwrap() {
            assert!(graph
                .dependencies_of(subscriber)
                .unwrap()
                .iter()
                .any(|edge| edge.source() == root));
        }
    }
}
