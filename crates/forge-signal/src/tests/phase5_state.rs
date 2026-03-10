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
fn continuity_token_preserves_lineage_without_requiring_output_identity() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_continuity_token("stable-a"))
    })
    .unwrap();
    let first_artifact = graph.current_lineage_artifact(source).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_continuity_token("stable-a")
            .with_output_change(OutputChange::Refreshed))
    })
    .unwrap();

    assert_eq!(
        graph.current_lineage_artifact(source),
        Some(first_artifact),
        "matching continuity token should preserve lineage even without output identity"
    );
    assert!(
        graph
            .lineage_for_node(source)
            .iter()
            .any(|record| record.event == LineageEvent::Refreshed),
        "continuity-token continuity should record refresh semantics"
    );
}

#[test]
fn branch_snapshot_restore_rejects_incompatible_storage_profile_and_preserves_branch_head() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let branch = runtime.create_branch("analysis").unwrap();
    let snapshot = runtime.capture_branch_snapshot(branch.clone()).unwrap();
    assert_eq!(
        runtime.branch_handle(branch.id).unwrap().head_snapshot_id,
        Some(snapshot.meta.snapshot_id),
        "capturing a branch snapshot should advance that branch head metadata"
    );
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
fn restore_branch_snapshot_rejects_cross_branch_payloads_and_keeps_catalog_consistent() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let feature = runtime.create_branch("feature-cross").unwrap();
    let main = runtime.current_branch();
    let main_snapshot = runtime.capture_snapshot();

    let err = runtime.restore_branch_snapshot(feature.clone(), &main_snapshot);
    assert!(err.is_err(), "cross-branch restore should be rejected");

    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    assert_eq!(feature_snapshot.meta.branch_id, feature.id);
    assert_eq!(
        runtime.branch_handle(feature.id).unwrap().head_snapshot_id,
        Some(feature_snapshot.meta.snapshot_id),
        "non-active branch snapshot capture should update shared branch-head metadata"
    );
    assert_eq!(
        runtime.branch_handle(main.id).unwrap().head_snapshot_id,
        Some(main_snapshot.meta.snapshot_id),
        "capturing another branch snapshot should not erase the active branch head"
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

    assert_eq!(
        graph.current_lineage_artifact(source),
        Some(baseline_artifact)
    );
    let lineage = graph.lineage_for_node(source);
    assert!(
        lineage
            .iter()
            .filter(|record| record.event == LineageEvent::Restored)
            .count()
            == 8,
        "restore loops should preserve restore history instead of silently erasing it"
    );
    assert!(
        lineage.iter().all(|record| record.node == Some(source)),
        "restore churn should not create stray lineage ownership"
    );
    assert!(
        graph
            .replay_events()
            .iter()
            .filter(|event| event.kind == ReplayEventKind::SnapshotRestored)
            .count()
            == 8,
        "restore churn should preserve restore replay history instead of silently erasing it"
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

#[test]
fn replay_slices_and_lineage_chains_are_branch_and_snapshot_queryable() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("artifact-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main_branch = runtime.current_branch();
    let snapshot = runtime.capture_snapshot();
    let before_cursor = runtime
        .replay_for_branch(main_branch.id)
        .frames
        .last()
        .map(|frame| frame.cursor)
        .expect("main branch should have replay");

    let feature_branch = runtime.create_branch("feature-query").unwrap();
    runtime.switch_branch(feature_branch.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.read(node, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("artifact-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let feature_replay = runtime.replay_for_branch(feature_branch.id);
    assert!(
        feature_replay
            .frames
            .iter()
            .all(|frame| frame.branch_id == feature_branch.id),
        "branch replay slices should stay branch-local"
    );
    assert!(
        feature_replay
            .frames
            .iter()
            .any(|frame| frame.node == Some(node)),
        "branch replay should include node-local evaluation events"
    );

    let node_replay = runtime.replay_for_node(node);
    assert!(
        node_replay
            .frames
            .iter()
            .all(|frame| frame.node == Some(node)),
        "node replay slices should filter to the requested node"
    );

    let around_snapshot = runtime.replay_around_snapshot(snapshot.meta.snapshot_id);
    assert!(
        around_snapshot
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(snapshot.meta.snapshot_id)),
        "snapshot-centered replay queries should include the matching snapshot id"
    );

    runtime.switch_branch(main_branch).unwrap();
    let tail = runtime.replay_from_cursor(before_cursor);
    assert!(
        tail.frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::BranchSwitched),
        "cursor-based replay slices should include later branch transitions"
    );

    let artifact_id = runtime
        .current_lineage_artifact(node)
        .expect("node should have a current lineage artifact");
    let artifact_chain = runtime.lineage_chain_for_artifact(artifact_id);
    assert!(
        artifact_chain
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "artifact lineage chain should expose the replacement event that materialized it"
    );
    let artifact_replay = runtime.replay_for_artifact(artifact_id);
    assert!(
        artifact_replay
            .frames
            .iter()
            .all(|frame| frame.lineage_artifact_id == Some(artifact_id)),
        "artifact replay slices should filter to the requested lineage artifact"
    );
    let node_chain = runtime.lineage_chain_for_node(node);
    assert_eq!(
        node_chain.last().and_then(|record| record.artifact_id),
        Some(artifact_id),
        "node lineage chain should end at the current artifact"
    );
    let refreshed_feature_snapshot = runtime
        .capture_branch_snapshot(feature_branch.clone())
        .unwrap();
    assert_eq!(
        runtime.branch_head_snapshot_id(feature_branch.id),
        Some(refreshed_feature_snapshot.meta.snapshot_id),
        "capturing a non-active branch snapshot should keep the branch catalog in sync"
    );
}

#[test]
fn branch_switch_and_restore_churn_preserve_branch_local_heads_and_replay_isolation() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.current_branch();
    let feature = runtime.create_branch("feature-churn").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    for _ in 0..6 {
        runtime.switch_branch(main.clone()).unwrap();
        runtime
            .restore_branch_snapshot(main.clone(), &main_snapshot)
            .unwrap();
        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .restore_branch_snapshot(feature.clone(), &feature_snapshot)
            .unwrap();
    }

    assert_eq!(
        runtime.branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id),
        "main branch head should stay pinned to its own restored snapshot"
    );
    assert_eq!(
        runtime.branch_head_snapshot_id(feature.id),
        Some(feature_snapshot.meta.snapshot_id),
        "feature branch head should stay pinned to its own restored snapshot"
    );

    runtime.switch_branch(main.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "main branch state should survive churn"
    );
    let main_replay = runtime.replay_for_branch(main.id);
    assert!(
        main_replay
            .frames
            .iter()
            .all(|frame| frame.branch_id == main.id),
        "main branch replay should remain branch-local after churn"
    );

    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2,
        "feature branch state should survive churn"
    );
    let feature_replay = runtime.replay_for_branch(feature.id);
    assert!(
        feature_replay
            .frames
            .iter()
            .all(|frame| frame.branch_id == feature.id),
        "feature branch replay should remain branch-local after churn"
    );
}

#[test]
fn lineage_chain_preserves_invalidation_and_restore_events_for_the_same_artifact() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("stable"))
    })
    .unwrap();
    let artifact_id = graph
        .current_lineage_artifact(source)
        .expect("materialized node should have lineage");

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let snapshot = graph.capture_snapshot();
    graph.restore_snapshot(&snapshot).unwrap();

    let chain = graph.lineage_chain_for_artifact(artifact_id);
    assert!(
        chain
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "lineage chain should include the artifact's original materialization"
    );
    assert!(
        chain
            .iter()
            .any(|record| record.event == LineageEvent::InvalidatedWithoutReplacement),
        "lineage chain should retain invalidation history for the same artifact"
    );
    assert!(
        chain
            .iter()
            .any(|record| record.event == LineageEvent::Restored),
        "lineage chain should retain restore history for the same artifact"
    );
}

#[test]
fn snapshot_metadata_and_replay_ranges_are_inspectable_without_restore() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("range-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let before = runtime
        .replay_for_branch(runtime.current_branch().id)
        .frames
        .last()
        .map(|frame| frame.cursor)
        .expect("replay should exist after first transaction");

    let snapshot = runtime.capture_snapshot();
    assert_eq!(snapshot.meta().snapshot_id, snapshot.snapshot_id());
    assert_eq!(snapshot.meta().branch_id, snapshot.branch_id());
    assert_eq!(
        snapshot.meta().schema_version,
        SignalSnapshotMeta::SCHEMA_VERSION
    );

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.read(node, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("range-artifact-2"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let current_branch = runtime.current_branch().id;
    let branch_replay = runtime.replay_for_branch(current_branch);
    let end = branch_replay
        .frames
        .last()
        .map(|frame| frame.cursor)
        .expect("replay tail should exist");
    let ranged = runtime.replay_between(before, end);
    assert!(
        !ranged.frames.is_empty(),
        "bounded replay range should return frames within the requested cursor window"
    );
    assert!(
        ranged
            .frames
            .iter()
            .all(|frame| frame.cursor >= before && frame.cursor <= end),
        "bounded replay ranges should respect both cursor endpoints"
    );
}

#[test]
fn replay_and_lineage_diff_helpers_compare_generic_phase5_artifacts() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("diff-a"))
    })
    .unwrap();
    let left_replay = graph.replay_for_node(source);
    let left_lineage = graph.lineage_for_node(source);

    let right_replay = graph.replay_for_node(source);
    let right_lineage = graph.lineage_for_node(source);

    assert!(replay_slices_equivalent(&left_replay, &right_replay));
    assert!(compare_replay_slices(&left_replay, &right_replay).is_empty());
    assert!(lineage_records_equivalent(&left_lineage, &right_lineage));
    assert!(compare_lineage_records(&left_lineage, &right_lineage).is_empty());

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    graph.capture_snapshot();
    let changed_replay = graph.replay_for_branch(graph.current_branch().id);
    let changed_lineage = graph.lineage_for_node(source);
    assert!(!compare_replay_slices(&left_replay, &changed_replay).is_empty());
    assert!(!compare_lineage_records(&left_lineage, &changed_lineage).is_empty());
}

#[test]
fn branch_local_transaction_failure_does_not_advance_heads_or_leak_committed_artifacts() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.current_branch();
    let feature = runtime.create_branch("feature-failure").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-stable"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    let feature_head_before = runtime.branch_head_snapshot_id(feature.id);
    let feature_artifact_before = runtime.current_lineage_artifact(source);
    let feature_lineage_before = runtime.graph().lineage_for_node(source);
    let feature_replay_before = runtime.replay_for_branch(feature.id);

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.read(source, &|_node, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(3, 0))
                    .with_output_identity("feature-bad"),
            ))
        })?;
        Err(SignalError::invalid_input("force branch-local rollback"))
    });
    assert!(err.is_err(), "failing transaction should surface an error");

    assert_eq!(
        runtime.branch_head_snapshot_id(feature.id),
        feature_head_before,
        "failed branch-local work must not advance the branch head"
    );
    assert_eq!(
        runtime.current_lineage_artifact(source),
        feature_artifact_before,
        "failed branch-local work must not replace the committed lineage artifact"
    );
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2,
        "failed branch-local work must rewind the active branch graph state"
    );
    assert_eq!(
        runtime.graph().lineage_for_node(source),
        feature_lineage_before,
        "failed branch-local work must not leak committed lineage transitions for the node"
    );
    let feature_replay_after = runtime.replay_for_branch(feature.id);
    assert!(
        feature_replay_after.frames.len() >= feature_replay_before.frames.len(),
        "failed branch-local work may append rollback/failure events, but it must not erase prior replay"
    );
    assert!(
        feature_replay_after
            .frames
            .iter()
            .all(|frame| frame.branch_id == feature.id),
        "failed branch-local work must keep replay isolation inside the active branch"
    );
    assert!(
        feature_replay_after
            .frames
            .iter()
            .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
            .count()
            == feature_replay_before
                .frames
                .iter()
                .filter(|frame| frame.kind == ReplayEventKind::TransactionCommitted)
                .count(),
        "failed branch-local work must not leak a committed replay outcome"
    );

    runtime.switch_branch(main.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "branch-local rollback must not contaminate sibling branches"
    );
    let main_replay = runtime.replay_for_branch(main.id);
    assert!(
        main_replay
            .frames
            .iter()
            .all(|frame| frame.branch_id == main.id),
        "sibling branch replay should remain branch-local after feature failure"
    );

    runtime.switch_branch(feature).unwrap();
    runtime
        .restore_branch_snapshot(runtime.current_branch(), &feature_snapshot)
        .unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2,
        "restoring the saved feature snapshot should still be possible after failure churn"
    );
}

#[test]
fn replay_and_lineage_overlap_stay_equivalent_across_runtime_policy_matrix() {
    fn run_workload(policy: SignalRuntimePolicy) -> (ReplaySlice, ReplaySlice, Vec<LineageRecord>) {
        let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
        runtime.set_runtime_policy(policy);
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

        let main = runtime.current_branch();
        let feature = runtime.create_branch("feature-policy").unwrap();
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
        runtime
            .restore_branch_snapshot(feature.clone(), &feature_snapshot)
            .unwrap();
        runtime.switch_branch(main).unwrap();
        runtime.restore_snapshot(&main_snapshot).unwrap();

        (
            runtime.replay_for_branch(runtime.current_branch().id),
            runtime.replay_for_branch(feature.id),
            runtime.graph().lineage_for_node(source),
        )
    }

    let operational = run_workload(SignalRuntimePolicy::operational());
    let development = run_workload(SignalRuntimePolicy::development());
    let forensic = run_workload(SignalRuntimePolicy::forensic());

    for (left_main, left_feature, left_lineage, right_main, right_feature, right_lineage) in [
        (
            &operational.0,
            &operational.1,
            &operational.2,
            &development.0,
            &development.1,
            &development.2,
        ),
        (
            &development.0,
            &development.1,
            &development.2,
            &forensic.0,
            &forensic.1,
            &forensic.2,
        ),
        (
            &operational.0,
            &operational.1,
            &operational.2,
            &forensic.0,
            &forensic.1,
            &forensic.2,
        ),
    ] {
        assert!(
            replay_slices_equivalent(left_main, right_main),
            "main-branch replay should remain equivalent across runtime-policy richness changes"
        );
        assert!(
            replay_slices_equivalent(left_feature, right_feature),
            "feature-branch replay should remain equivalent across runtime-policy richness changes"
        );
        assert!(
            lineage_records_equivalent(left_lineage, right_lineage),
            "lineage on the overlapping guaranteed surface should remain equivalent across runtime-policy richness changes"
        );
    }
}

#[test]
fn snapshot_contract_accepts_matching_schema_and_rejects_profile_or_schema_mismatch() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("snapshot-contract"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let snapshot = runtime.capture_snapshot();
    runtime.restore_snapshot(&snapshot).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "matching schema/profile snapshots should restore successfully"
    );

    let mut wrong_profile = snapshot.clone();
    wrong_profile.meta.core_storage_profile = "definitely-wrong-profile".to_string();
    let wrong_profile_err = runtime.restore_snapshot(&wrong_profile).unwrap_err();
    assert!(
        wrong_profile_err
            .to_string()
            .contains("core storage profile"),
        "profile mismatch should fail explicitly"
    );

    let mut wrong_schema = snapshot.clone();
    wrong_schema.meta.schema_version += 1;
    let wrong_schema_err = runtime.restore_snapshot(&wrong_schema).unwrap_err();
    assert!(
        wrong_schema_err.to_string().contains("schema version"),
        "schema mismatch should fail explicitly"
    );
}

#[test]
fn branch_churn_respects_history_and_replay_budgets_under_tight_policy() {
    let policy = SignalRuntimePolicy::operational()
        .with_history_limit(2)
        .with_detail_limit(1)
        .with_history_details(false);
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(policy);
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("bounded-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.current_branch();
    let feature = runtime.create_branch("feature-budget").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("bounded-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    for _ in 0..20 {
        runtime.switch_branch(main.clone()).unwrap();
        runtime
            .restore_branch_snapshot(main.clone(), &main_snapshot)
            .unwrap();
        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .restore_branch_snapshot(feature.clone(), &feature_snapshot)
            .unwrap();
    }

    assert!(
        runtime.recent_execution_history_diagnostics().len() <= policy.history_limit,
        "execution history should stay within the configured history budget under branch churn"
    );
    assert!(
        runtime.graph().replay_events().len() <= policy.history_limit.max(1) * 32,
        "replay retention should stay within the policy-derived bound under branch churn"
    );
    assert!(
        runtime.graph().lineage_records().len() <= policy.history_limit.max(1) * 32,
        "lineage retention should stay within the policy-derived bound under branch churn"
    );
    assert_eq!(
        runtime.known_branches().len(),
        2,
        "branch churn should not fabricate extra branch catalog entries"
    );
    assert_eq!(
        runtime.branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id),
        "main head should remain pinned to its snapshot under churn"
    );
    assert_eq!(
        runtime.branch_head_snapshot_id(feature.id),
        Some(feature_snapshot.meta.snapshot_id),
        "feature head should remain pinned to its snapshot under churn"
    );
}
