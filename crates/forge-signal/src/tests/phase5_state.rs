use crate::facade::*;
use crate::tests::support::*;

#[test]
fn graph_snapshot_round_trip_restores_versions_and_emits_restore_replay_and_lineage() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("artifact-v1"))
        };
    let mut source_v2 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
                .with_output_identity("artifact-v2"))
        };
    let mut dependent_compute = |_id: NodeId, graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    let snapshot = graph.capture_snapshot();
    let replay_len_before = graph.replay_events().len();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    assert_eq!(
        graph
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );

    graph.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        graph
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1
    );
    assert!(
        graph.replay_events().len() > replay_len_before,
        "restore should append replay-visible state transitions"
    );
    assert!(
        graph.replay_events().iter().any(|event| {
            event.kind == ReplayEventKind::SnapshotRestored
                && event.snapshot_id == Some(snapshot.meta.snapshot_id)
        }),
        "restore should emit a snapshot-restored replay event"
    );
    assert!(
        graph.lineage_for_node(source).iter().any(|record| {
            record.event == LineageEvent::Restored
                && record.snapshot_id == Some(snapshot.meta.snapshot_id)
        }),
        "restore should emit restored lineage records for materialized artifacts"
    );
    let around_restore = graph.replay_around_snapshot(snapshot.meta.snapshot_id);
    assert!(
        around_restore
            .frames
            .iter()
            .any(|event| event.kind == ReplayEventKind::SnapshotRestored),
        "snapshot-centered replay inspection should include the restore event"
    );
}

#[test]
fn runtime_branches_keep_evaluation_state_isolated_across_switches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph.add_dependency(dependent, source, ASPECT_A).unwrap();

    let mut runtime = SignalRuntime::builder(graph).build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(dependent, &|node, view| {
                let result = if node == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("main-artifact"),
                    )
                } else {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    view.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature_branch = runtime.create_branch("feature-a").unwrap();

    runtime.switch_branch(feature_branch.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(dependent, &|node, view| {
                let result = if node == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(2, 0))
                            .with_output_identity("feature-artifact"),
                    )
                } else {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    view.finish(NodeEvaluationResult::from_version(version))
                };
                Ok(result)
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(runtime.current_branch().id, feature_branch.id);
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );

    runtime.switch_branch(main_branch.clone()).unwrap();

    assert_eq!(runtime.current_branch().id, main_branch.id);
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1
    );
    assert!(
        runtime
            .graph()
            .replay_events()
            .iter()
            .any(|event| event.kind == ReplayEventKind::BranchSwitched),
        "branch switching should emit replay events"
    );
    let ancestry = runtime.branch_ancestry(feature_branch.id);
    assert_eq!(ancestry.first().unwrap().id, main_branch.id);
    assert_eq!(ancestry.last().unwrap().id, feature_branch.id);
}

#[test]
fn lineage_distinguishes_replacement_refresh_and_memoized_reuse() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    let mut replaced = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("artifact-a"))
    };
    let mut refreshed = |_id: NodeId, _graph: &SignalGraph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("artifact-a")
            .with_output_change(OutputChange::Refreshed))
    };

    evaluate(&mut graph, source, &mut replaced).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut refreshed).unwrap();

    let lineage = graph.lineage_for_node(source);
    assert!(
        lineage
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "first materialized artifact should record replacement semantics"
    );
    assert!(
        lineage
            .iter()
            .any(|record| record.event == LineageEvent::Refreshed),
        "stable artifact continuity should record refresh semantics"
    );

    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let family = runtime.register_computation_family("projection");
    let keyed = runtime.keyed_node(&family, "bulkhead");
    let computation = KeyedComputation::new(family.clone(), "bulkhead").with_memo_key("shape-v1");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(keyed, &computation, &|_id, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("memo-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    mark_dirty(runtime.graph_mut(), keyed, ASPECT_A).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(keyed, &computation, &|_id, view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert!(
        runtime
            .graph()
            .lineage_for_node(keyed)
            .iter()
            .any(|record| record.event == LineageEvent::MemoizedFrom),
        "memoized reuse should emit memoized lineage semantics"
    );
}

#[test]
fn branch_snapshot_restore_rejects_incompatible_storage_profile_and_preserves_branch_head() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let branch = runtime.create_branch("analysis").unwrap();
    let snapshot = runtime.capture_branch_snapshot(branch.clone()).unwrap();
    let mut incompatible = snapshot.clone();
    incompatible.meta.core_storage_profile = "definitely-not-this-profile".to_string();

    let err = runtime.restore_branch_snapshot(branch.clone(), &incompatible);
    assert!(err.is_err());

    let restored = runtime.capture_branch_snapshot(branch).unwrap();
    assert_eq!(
        restored.meta.core_storage_profile,
        snapshot.meta.core_storage_profile
    );
}

#[test]
fn repeated_snapshot_restore_loops_do_not_leak_non_restore_lineage_or_branch_state() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("stable"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();
    let baseline_artifact = graph.current_lineage_artifact(source).unwrap();

    for _ in 0..8 {
        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
        graph.restore_snapshot(&snapshot).unwrap();
    }

    assert_eq!(graph.current_lineage_artifact(source), Some(baseline_artifact));
    let lineage = graph.lineage_for_node(source);
    assert!(
        lineage
            .iter()
            .filter(|record| record.event == LineageEvent::Restored)
            .count()
            == 1,
        "restore should replace branch-local history with the snapshot payload and append one restore event"
    );
    assert!(
        lineage.iter().all(|record| record.node == Some(source)),
        "restore churn should not create stray lineage ownership"
    );
    assert!(
        graph.replay_events()
            .iter()
            .filter(|event| event.kind == ReplayEventKind::SnapshotRestored)
            .count()
            == 1,
        "restore churn should not accumulate replay events outside the restored snapshot payload"
    );
}

#[test]
fn invalidation_emits_lineage_without_replacement_and_branch_restore_is_local() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("artifact-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let feature = runtime.create_branch("feature-b").unwrap();
    let main_snapshot = runtime.capture_snapshot();
    runtime.switch_branch(feature.clone()).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("artifact-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    assert!(
        runtime
            .graph()
            .lineage_for_node(source)
            .iter()
            .any(|record| record.event == LineageEvent::InvalidatedWithoutReplacement),
        "invalidation should record lineage even before the artifact is replaced"
    );

    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    let feature_replay = runtime.replay_for_branch(feature.id);
    assert!(
        feature_replay
            .frames
            .iter()
            .any(|event| event.kind == ReplayEventKind::SnapshotRestored),
        "branch-local restore should be visible in the restored branch replay stream"
    );

    runtime.switch_branch(main_branch).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "branch restore and branch-local churn must not contaminate main"
    );
    let main_replay = runtime.replay_around_snapshot(main_snapshot.meta.snapshot_id);
    assert!(
        main_replay
            .frames
            .iter()
            .all(|event| event.branch_id == runtime.current_branch().id),
        "snapshot inspection on main should stay branch-local"
    );
}
