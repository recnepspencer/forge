use crate::data::dependency::DependencySnapshot;
use crate::data::trace::{CausalityMetadata, RetainedDiagnosticArtifact};
use crate::diagnostics::{ExplanationFact, ProvenanceFact};
use crate::facade::*;
use crate::tests::support::*;

#[test]
fn graph_snapshot_round_trip_restores_versions_and_emits_restore_replay_and_lineage() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let dependent = graph.node().build();
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

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
        graph.observe().lineage_records().iter().any(|record| {
            matches!(
                record.kind,
                LineageRecordKind::SnapshotRestore { snapshot_id, .. }
                    if snapshot_id == snapshot.meta.snapshot_id
            )
        }),
        "restore should emit lineage-visible restore records under the active runtime policy"
    );
    let around_restore = graph
        .observe()
        .replay_around_snapshot(snapshot.meta.snapshot_id);
    assert!(
        around_restore
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
    graph
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(dependent, &|view| {
                let result = if view.node() == source {
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

    let main_branch = runtime.observe().current_branch();
    let feature_branch = runtime.create_branch("feature-a").unwrap();

    runtime.switch_branch(feature_branch.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(dependent, &|view| {
                let result = if view.node() == source {
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

    assert_eq!(runtime.observe().current_branch().id, feature_branch.id);
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

    assert_eq!(runtime.observe().current_branch().id, main_branch.id);
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
    let ancestry = runtime.observe().branch_ancestry(feature_branch.id);
    assert_eq!(ancestry.first().unwrap().id, main_branch.id);
    assert_eq!(ancestry.last().unwrap().id, feature_branch.id);
}

#[test]
fn switching_existing_branch_does_not_emit_branched_from_lineage() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();

    let main_branch = runtime.observe().current_branch();
    let feature_branch = runtime.create_branch("feature").unwrap();
    runtime.switch_branch(feature_branch.clone()).unwrap();
    let lineage_after_create = runtime.graph().observe().lineage_records().len();

    runtime.switch_branch(main_branch.clone()).unwrap();

    let switch_records = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .skip(lineage_after_create)
        .collect::<Vec<_>>();
    assert!(
        switch_records.iter().any(|record| {
            matches!(
                &record.kind,
                LineageRecordKind::BranchSwitch {
                    from_branch_id,
                    to_branch_id,
                    from_branch_display_name,
                    to_branch_display_name,
                } if *from_branch_id == feature_branch.id
                    && *to_branch_id == main_branch.id
                    && from_branch_display_name == "feature"
                    && to_branch_display_name == "main"
            )
        }),
        "branch switch should remain lineage-visible"
    );
    assert!(
        switch_records
            .iter()
            .all(|record| !matches!(record.kind, LineageRecordKind::BranchFork { .. })),
        "switching existing branches must not masquerade as branch creation"
    );
    assert_eq!(runtime.observe().current_branch().id, main_branch.id);
    assert_eq!(feature_branch.parent_branch_id, Some(main_branch.id));
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

    let lineage = graph.observe().lineage_for_node(source);
    assert!(
        lineage.iter().any(|record| {
            matches!(
                record.kind,
                LineageRecordKind::ArtifactTransition {
                    transition: ArtifactTransitionKind::Replaced,
                    ..
                }
            )
        }),
        "first materialized artifact should record replacement semantics"
    );
    assert!(
        lineage.iter().any(|record| {
            matches!(
                record.kind,
                LineageRecordKind::ArtifactTransition {
                    transition: ArtifactTransitionKind::Refreshed { .. },
                    ..
                }
            )
        }),
        "stable artifact continuity should record refresh semantics"
    );

    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let family = define_keyed_computation(&mut runtime, "projection", ());
    let bulkhead = family.keyed("bulkhead");
    let keyed = bulkhead.node(&mut runtime);
    let computation = bulkhead.memoized("shape-v1");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(keyed, &computation, &|view| {
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
            tx.evaluate_keyed(keyed, &computation, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert!(
        runtime
            .graph()
            .observe()
            .lineage_for_node(keyed)
            .iter()
            .any(|record| {
                matches!(
                    record.kind,
                    LineageRecordKind::ArtifactTransition {
                        transition: ArtifactTransitionKind::MemoizedReuse,
                        ..
                    }
                )
            }),
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
    let first_artifact = graph.observe().current_lineage_artifact(source).unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_continuity_token("stable-a")
            .with_output_change(OutputChange::Refreshed))
    })
    .unwrap();

    assert_eq!(
        graph.observe().current_lineage_artifact(source),
        Some(first_artifact),
        "matching continuity token should preserve lineage even without output identity"
    );
    assert!(
        graph
            .observe()
            .lineage_for_node(source)
            .iter()
            .any(|record| {
                matches!(
                    record.kind,
                    LineageRecordKind::ArtifactTransition {
                        transition: ArtifactTransitionKind::Refreshed { .. },
                        ..
                    }
                )
            }),
        "continuity-token continuity should record refresh semantics"
    );
}

#[test]
fn branch_snapshot_restore_rejects_incompatible_storage_profile_and_preserves_branch_head() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let branch = runtime.create_branch("analysis").unwrap();
    let snapshot = runtime.capture_branch_snapshot(branch.clone()).unwrap();
    assert_eq!(
        runtime
            .observe()
            .branch_handle(branch.id)
            .unwrap()
            .head_snapshot_id,
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
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let feature = runtime.create_branch("feature-cross").unwrap();
    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();

    let err = runtime.restore_branch_snapshot(feature.clone(), &main_snapshot);
    assert!(
        matches!(
            err,
            Err(SignalError::IncompatibleSnapshot { reason: _, .. })
        ),
        "cross-branch restore should be rejected"
    );

    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    assert_eq!(feature_snapshot.meta.branch_id, feature.id);
    assert_eq!(
        runtime
            .observe()
            .branch_handle(feature.id)
            .unwrap()
            .head_snapshot_id,
        Some(feature_snapshot.meta.snapshot_id),
        "non-active branch snapshot capture should update shared branch-head metadata"
    );
    assert_eq!(
        runtime
            .observe()
            .branch_handle(main.id)
            .unwrap()
            .head_snapshot_id,
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
    let baseline_artifact = graph.observe().current_lineage_artifact(source).unwrap();

    for _ in 0..8 {
        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
        graph.restore_snapshot(&snapshot).unwrap();
    }

    assert_eq!(
        graph.observe().current_lineage_artifact(source),
        Some(baseline_artifact)
    );
    let lineage = graph.observe().lineage_records();
    assert!(
        lineage
            .iter()
            .filter(|record| matches!(record.kind, LineageRecordKind::SnapshotRestore { .. }))
            .count()
            == 8,
        "restore loops should preserve restore history instead of silently erasing it"
    );
    assert!(
        lineage.iter().all(|record| matches!(
            record.kind,
            LineageRecordKind::SnapshotRestore { .. }
        ) || record.node() == Some(source)),
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
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("artifact-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main_branch = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-b").unwrap();
    let main_snapshot = runtime.capture_snapshot();
    runtime.switch_branch(feature.clone()).unwrap();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
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
            .observe()
            .lineage_for_node(source)
            .iter()
            .any(|record| {
                matches!(
                    &record.kind,
                    LineageRecordKind::Invalidation {
                        cause: InvalidationCause::SourceAspectChanged { .. }
                            | InvalidationCause::DirectDependencyChanged { .. }
                            | InvalidationCause::TransitiveDependencyChanged { .. },
                        ..
                    }
                )
            }),
        "invalidation should record lineage even before the artifact is replaced"
    );

    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    let feature_replay = runtime.observe().replay_for_branch(feature.id);
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
    let main_replay = runtime
        .observe()
        .replay_around_snapshot(main_snapshot.meta.snapshot_id);
    assert!(
        main_replay
            .frames
            .iter()
            .all(|event| event.branch_id == runtime.observe().current_branch().id),
        "snapshot inspection on main should stay branch-local"
    );
}

#[test]
fn snapshot_restore_lineage_records_restore_mode_structurally() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("stable"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();
    graph.restore_snapshot(&snapshot).unwrap();

    let compact_restore = graph
        .observe()
        .lineage_records()
        .iter()
        .find(|record| record.snapshot_id() == Some(snapshot.meta.snapshot_id))
        .expect("compact restore lineage should be recorded");
    assert!(matches!(
        compact_restore.kind,
        LineageRecordKind::SnapshotRestore {
            restore_kind: SnapshotRestoreKind::CompactGlobal,
            ..
        }
    ));

    let mut forensic = SignalGraph::new();
    forensic.set_runtime_policy(
        SignalRuntimePolicy::development()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let node = forensic.node().output_identity().build();
    evaluate(&mut forensic, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("stable"))
    })
    .unwrap();
    let per_node_snapshot = forensic.capture_snapshot();
    forensic.restore_snapshot(&per_node_snapshot).unwrap();

    let per_node_restore = forensic
        .observe()
        .lineage_records()
        .iter()
        .find(|record| {
            record.snapshot_id() == Some(per_node_snapshot.meta.snapshot_id)
                && record.node() == Some(node)
        })
        .expect("per-node restore lineage should be recorded");
    assert!(matches!(
        per_node_restore.kind,
        LineageRecordKind::SnapshotRestore {
            restore_kind: SnapshotRestoreKind::PerNodeArtifact,
            ..
        }
    ));
}

#[test]
fn replay_slices_and_lineage_chains_are_branch_and_snapshot_queryable() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("artifact-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main_branch = runtime.observe().current_branch();
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
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("artifact-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let feature_replay = runtime.observe().replay_for_branch(feature_branch.id);
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

    let node_replay = runtime.observe().replay_for_node(node);
    assert!(
        node_replay
            .frames
            .iter()
            .all(|frame| frame.node == Some(node)),
        "node replay slices should filter to the requested node"
    );

    let around_snapshot = runtime
        .observe()
        .replay_around_snapshot(snapshot.meta.snapshot_id);
    assert!(
        around_snapshot
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(snapshot.meta.snapshot_id)),
        "snapshot-centered replay queries should include the matching snapshot id"
    );

    runtime.switch_branch(main_branch).unwrap();
    let tail = runtime.observe().replay_from_cursor(before_cursor);
    assert!(
        tail.frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::BranchSwitched),
        "cursor-based replay slices should include later branch transitions"
    );

    let artifact_id = runtime
        .observe()
        .current_lineage_artifact(node)
        .expect("node should have a current lineage artifact");
    let artifact_chain = runtime.observe().lineage_chain_for_artifact(artifact_id);
    assert!(
        artifact_chain.iter().any(|record| matches!(
            record.kind,
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::Replaced,
                ..
            }
        )),
        "artifact lineage chain should expose the replacement event that materialized it"
    );
    let artifact_replay = runtime.observe().replay_for_artifact(artifact_id);
    assert!(
        artifact_replay
            .frames
            .iter()
            .all(|frame| frame.lineage_artifact_id == Some(artifact_id)),
        "artifact replay slices should filter to the requested lineage artifact"
    );
    let node_chain = runtime.observe().lineage_chain_for_node(node);
    assert_eq!(
        node_chain
            .last()
            .and_then(|record| record.subject_artifact_id()),
        Some(artifact_id),
        "node lineage chain should end at the current artifact"
    );
    let refreshed_feature_snapshot = runtime
        .capture_branch_snapshot(feature_branch.clone())
        .unwrap();
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(feature_branch.id),
        Some(refreshed_feature_snapshot.meta.snapshot_id),
        "capturing a non-active branch snapshot should keep the branch catalog in sync"
    );
}

#[test]
fn branched_runtime_preserves_unique_lineage_ids_and_sequences() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-v1"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let main_branch = runtime.observe().current_branch();
    let main_artifact = runtime.observe().current_lineage_artifact(source).unwrap();

    let feature = runtime.create_branch("feature-unique").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-v2"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_artifact = runtime.observe().current_lineage_artifact(source).unwrap();

    runtime.switch_branch(main_branch).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(3, 0))
                        .with_output_identity("main-v3"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let updated_main_artifact = runtime.observe().current_lineage_artifact(source).unwrap();

    assert_ne!(main_artifact, feature_artifact);
    assert_ne!(feature_artifact, updated_main_artifact);

    let sequences = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .map(|record| record.sequence)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        sequences.len(),
        runtime.graph().observe().lineage_records().len(),
        "active branch lineage history should not contain duplicate lineage sequence ids"
    );
}

#[test]
fn branched_runtime_preserves_unique_branch_and_snapshot_ids() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-runtime-ids").unwrap();

    runtime.switch_branch(feature.clone()).unwrap();
    let nested = runtime.create_branch("nested-runtime-ids").unwrap();
    let feature_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(main.clone()).unwrap();
    let sibling = runtime.create_branch("sibling-runtime-ids").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    assert!(
        sibling.id > nested.id,
        "restored main branch state must not reuse a branch id already allocated on another branch"
    );
    assert!(
        main_snapshot.meta.snapshot_id > feature_snapshot.meta.snapshot_id,
        "restored main branch state must not reuse a snapshot id already allocated on another branch"
    );
}

#[test]
fn branch_switch_and_restore_churn_preserve_branch_local_heads_and_replay_isolation() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-churn").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
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
        runtime.observe().branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id),
        "main branch head should stay pinned to its own restored snapshot"
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(feature.id),
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
    let main_replay = runtime.observe().replay_for_branch(main.id);
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
    let feature_replay = runtime.observe().replay_for_branch(feature.id);
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
    graph.set_runtime_policy(
        SignalRuntimePolicy::forensic()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let source = graph.node().output_identity().build();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("stable"))
    })
    .unwrap();
    let artifact_id = graph
        .observe()
        .current_lineage_artifact(source)
        .expect("materialized node should have lineage");

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    let snapshot = graph.capture_snapshot();
    graph.restore_snapshot(&snapshot).unwrap();

    let chain = graph.observe().lineage_chain_for_artifact(artifact_id);
    assert!(
        chain.iter().any(|record| matches!(
            record.kind,
            LineageRecordKind::ArtifactTransition {
                transition: ArtifactTransitionKind::Replaced,
                ..
            }
        )),
        "lineage chain should include the artifact's original materialization"
    );
    assert!(
        chain
            .iter()
            .any(|record| matches!(record.kind, LineageRecordKind::Invalidation { .. })),
        "lineage chain should retain invalidation history for the same artifact"
    );
    assert!(
        chain
            .iter()
            .any(|record| matches!(record.kind, LineageRecordKind::SnapshotRestore { .. })),
        "lineage chain should retain restore history for the same artifact"
    );
}

#[test]
fn snapshot_metadata_and_replay_ranges_are_inspectable_without_restore() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("range-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let before = runtime
        .replay_for_branch(runtime.observe().current_branch().id)
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
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("range-artifact-2"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let current_branch = runtime.observe().current_branch().id;
    let branch_replay = runtime.observe().replay_for_branch(current_branch);
    let end = branch_replay
        .frames
        .last()
        .map(|frame| frame.cursor)
        .expect("replay tail should exist");
    let ranged = runtime.observe().replay_between(before, end);
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
    let left_replay = graph.observe().replay_for_node(source).to_owned_slice();
    let left_lineage = graph.observe().lineage_for_node(source).to_owned_records();

    let right_replay = graph.observe().replay_for_node(source).to_owned_slice();
    let right_lineage = graph.observe().lineage_for_node(source).to_owned_records();

    assert!(replay_slices_equivalent(&left_replay, &right_replay));
    assert!(compare_replay_slices(&left_replay, &right_replay).is_empty());
    assert!(lineage_records_equivalent(&left_lineage, &right_lineage));
    assert!(compare_lineage_records(&left_lineage, &right_lineage).is_empty());

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    graph.capture_snapshot();
    let changed_replay = graph
        .observe()
        .replay_for_branch(graph.observe().current_branch().id)
        .to_owned_slice();
    let changed_lineage = graph.observe().lineage_for_node(source).to_owned_records();
    assert!(!compare_replay_slices(&left_replay, &changed_replay).is_empty());
    assert!(!compare_lineage_records(&left_lineage, &changed_lineage).is_empty());
}

#[test]
fn branch_local_transaction_failure_does_not_advance_heads_or_leak_committed_artifacts() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-artifact"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-failure").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-stable"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();
    let feature_head_before = runtime.observe().branch_head_snapshot_id(feature.id);
    let feature_artifact_before = runtime.observe().current_lineage_artifact(source);
    let feature_lineage_before = runtime
        .graph()
        .observe()
        .lineage_for_node(source)
        .to_owned_records();
    let feature_replay_before = runtime.observe().replay_for_branch(feature.id);

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.read(source, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(3, 0))
                    .with_output_identity("feature-bad"),
            ))
        })?;
        Err(SignalError::invalid_input("force branch-local rollback"))
    });
    assert!(err.is_err(), "failing transaction should surface an error");

    assert_eq!(
        runtime.observe().branch_head_snapshot_id(feature.id),
        feature_head_before,
        "failed branch-local work must not advance the branch head"
    );
    assert_eq!(
        runtime.observe().current_lineage_artifact(source),
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
        runtime
            .graph()
            .observe()
            .lineage_for_node(source)
            .to_owned_records(),
        feature_lineage_before,
        "failed branch-local work must not leak committed lineage transitions for the node"
    );
    let feature_replay_after = runtime.observe().replay_for_branch(feature.id);
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
    let main_replay = runtime.observe().replay_for_branch(main.id);
    assert!(
        main_replay
            .frames
            .iter()
            .all(|frame| frame.branch_id == main.id),
        "sibling branch replay should remain branch-local after feature failure"
    );

    runtime.switch_branch(feature).unwrap();
    runtime
        .restore_branch_snapshot(runtime.observe().current_branch(), &feature_snapshot)
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
        let mut runtime = SignalRuntime::builder(SignalGraph::new())
            .with_kernel_defaults()
            .build();
        runtime.set_runtime_policy(policy);
        let source = runtime.graph_mut().node().output_identity().build();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("artifact-main"),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        let main = runtime.observe().current_branch();
        let feature = runtime.create_branch("feature-policy").unwrap();
        let main_snapshot = runtime.capture_snapshot();

        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.read(source, &|view| {
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
            runtime
                .observe()
                .replay_for_branch(runtime.observe().current_branch().id),
            runtime.observe().replay_for_branch(feature.id),
            runtime
                .graph()
                .observe()
                .lineage_for_node(source)
                .to_owned_records(),
        )
    }

    let operational = run_workload(
        SignalRuntimePolicy::operational()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::CompactGlobal),
    );
    let development = run_workload(
        SignalRuntimePolicy::development()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::CompactGlobal),
    );
    let forensic = run_workload(
        SignalRuntimePolicy::forensic()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::CompactGlobal),
    );

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
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
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
fn graph_restore_uses_checkpoint_image_not_raw_snapshot_graph_bundle() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    let mut source_v1 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("artifact-v1"))
        };
    let mut source_v2 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(9, 0))
                .with_output_identity("artifact-v9"))
        };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    let mut snapshot = graph.capture_snapshot();

    let mut tampered_graph = snapshot.diagnostic_graph.clone();
    tampered_graph
        .get_entry_mut(source)
        .unwrap()
        .set_aspect_version(version_ab(77, 0));
    snapshot.diagnostic_graph = tampered_graph;

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();
    evaluate(&mut graph, source, &mut source_v2).unwrap();
    assert_eq!(
        graph
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        9
    );

    graph.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        graph.get_entry(source).unwrap().get_aspect_version().get(ASPECT_A),
        1,
        "restore should follow the checkpoint image authority carrier rather than the raw snapshot graph bundle"
    );
}

#[test]
fn checkpoint_image_omits_diagnostic_richness_while_snapshot_bundle_retains_it() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();

    let mut source_v1 =
        |_id: NodeId, _graph: &SignalGraph| {
            Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("artifact-v1"))
        };

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    let snapshot = graph.capture_snapshot();

    assert!(
        snapshot
            .authority_graph()
            .observe()
            .replay_events()
            .is_empty(),
        "checkpoint image should not carry retained replay richness"
    );
    assert!(
        snapshot
            .authority_graph()
            .observe()
            .lineage_records()
            .is_empty(),
        "checkpoint image should not carry retained lineage richness"
    );
    assert!(
        snapshot
            .authority_graph()
            .diagnostics_state()
            .explanation_facts()
            .is_empty(),
        "checkpoint image should not carry retained explanation richness"
    );
    assert!(
        !snapshot.diagnostics.replay_frames.is_empty()
            || !snapshot
                .diagnostic_graph
                .observe()
                .replay_events()
                .is_empty(),
        "rich snapshot bundle should still carry explicit diagnostics/replay payloads"
    );
}

#[test]
fn snapshot_restore_preserves_advanced_reuse_history_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(SignalRuntimePolicy::development());
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "phase5-advanced-reuse".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching()
                .with_partial_artifact_splicing()
                .with_partition_scope(PartitionSubscription::whole_partition("wing")),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("phase5-advanced-artifact")
                        .with_output_change(OutputChange::Refreshed)
                        .with_changed_region(ChangedRegion::new("wing")),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias");
    let wing = projection.keyed("wing");
    let alias_node = alias.node(&mut runtime);
    let wing_node = wing.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            source.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity_with_lineage_mapping(
                tx,
                "source",
                "shape-v1",
                "lineage-map:mesh-42->mesh-77",
            )
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    mark_dirty(runtime.graph_mut(), wing_node, ASPECT_A).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_partial_splice(
                tx,
                "shape-v1",
                [PartitionSubscription::whole_partition("wing")],
            )
        })
        .unwrap();

    let snapshot = runtime.capture_snapshot();

    mark_dirty(runtime.graph_mut(), alias_node, ASPECT_A).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_memoized(tx, "shape-v2")
        })
        .unwrap();
    mark_dirty(runtime.graph_mut(), wing_node, ASPECT_A).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_memoized(tx, "shape-v2")
        })
        .unwrap();

    runtime.restore_snapshot(&snapshot).unwrap();

    let history = runtime
        .observe()
        .execution_history_summary(DiagnosticsTier::Development);
    let alias_summary = history
        .nodes
        .iter()
        .find(|node| node.node == alias_node)
        .expect("alias history summary");
    assert_eq!(
        alias_summary.reuse_origin,
        Some(ReuseOrigin::CrossIdentityPersistentReuse)
    );
    assert_eq!(
        alias_summary.persistent_correspondence_kind,
        Some(crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping)
    );

    let wing_summary = history
        .nodes
        .iter()
        .find(|node| node.node == wing_node)
        .expect("wing history summary");
    assert_eq!(
        wing_summary.reuse_origin,
        Some(ReuseOrigin::PartialArtifactSplice)
    );
    assert_eq!(wing_summary.composition_region_count, 1);

    let alias_explain = runtime.observe().explain(alias_node).unwrap();
    assert_eq!(
        alias_explain.reuse_origin,
        Some(ReuseOrigin::CrossIdentityPersistentReuse)
    );
    let wing_explain = runtime.observe().explain(wing_node).unwrap();
    assert_eq!(
        wing_explain.reuse_origin,
        Some(ReuseOrigin::PartialArtifactSplice)
    );

    assert!(runtime.graph().replay_events().iter().any(|event| {
        event.kind == ReplayEventKind::SnapshotRestored
            && event.snapshot_id == Some(snapshot.meta.snapshot_id)
    }));
    let branch_replay = runtime
        .observe()
        .replay_for_branch(runtime.observe().current_branch().id);
    assert!(branch_replay.frames.iter().any(|event| {
        event.kind == ReplayEventKind::SnapshotRestored
            && event.snapshot_id == Some(snapshot.meta.snapshot_id)
    }));

    let alias_lineage = runtime.observe().lineage_chain_for_node(alias_node);
    assert!(alias_lineage.iter().any(|record| matches!(
        &record.kind,
        LineageRecordKind::ArtifactTransition {
            transition: ArtifactTransitionKind::CrossIdentityPersistentReuse {
                correspondence_kind:
                    crate::data::reuse::PersistentCorrespondenceKind::LineageBackedMapping
            },
            ..
        }
    )));
    let wing_lineage = runtime.observe().lineage_chain_for_node(wing_node);
    assert!(wing_lineage.iter().any(|record| matches!(
        &record.kind,
        LineageRecordKind::ArtifactTransition {
            transition: ArtifactTransitionKind::PartialArtifactSplice {
                composition_region_count: 1,
                recomputed_region_count: 1
            },
            ..
        }
    )));
}

#[test]
fn snapshot_artifact_retention_policy_changes_richness_not_restore_truth() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let node = graph.node().output_identity().build();

    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("snapshot-richness")
            .with_label("retained"))
    })
    .unwrap();
    let retained_explanation = graph
        .observe()
        .materialize()
        .materialize_explanation_artifact(node)
        .unwrap()
        .0
        .expect("development policy should materialize explanation artifacts");
    graph
        .diagnostics_state_mut()
        .record_explanation_fact(ExplanationFact::from_explanation(&retained_explanation));
    graph
        .diagnostics_state_mut()
        .record_provenance_fact(ProvenanceFact::from_explanation(&retained_explanation));

    let retained_snapshot = graph.capture_snapshot();
    assert_eq!(
        retained_snapshot
            .meta
            .artifact_retention
            .explanation_retention,
        ArtifactRetentionPolicy::Retain
    );
    assert_eq!(
        retained_snapshot
            .meta
            .artifact_retention
            .provenance_retention,
        ArtifactRetentionPolicy::Retain
    );
    assert!(
        retained_snapshot
            .diagnostics
            .explanation_facts
            .contains_key(&node),
        "development snapshot capture should retain explanation facts eagerly"
    );
    assert!(
        retained_snapshot
            .diagnostics
            .provenance_facts
            .contains_key(&node),
        "development snapshot capture should retain provenance facts eagerly"
    );

    graph.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );
    let omitted_snapshot = graph.capture_snapshot();
    assert_eq!(
        omitted_snapshot
            .meta
            .artifact_retention
            .explanation_retention,
        ArtifactRetentionPolicy::Omit
    );
    assert_eq!(
        omitted_snapshot
            .meta
            .artifact_retention
            .provenance_retention,
        ArtifactRetentionPolicy::Omit
    );
    assert!(
        omitted_snapshot.diagnostics.explanation_facts.is_empty(),
        "snapshot capture should omit cold explanation richness under an omit policy"
    );
    assert!(
        omitted_snapshot.diagnostics.provenance_facts.is_empty(),
        "snapshot capture should omit cold provenance richness under an omit policy"
    );

    mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("snapshot-richness-2"))
    })
    .unwrap();
    assert_eq!(
        graph
            .get_entry(node)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );

    graph.restore_snapshot(&omitted_snapshot).unwrap();

    assert_eq!(
        graph
            .get_entry(node)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "snapshot restore should rewind operational truth even when cold artifact richness was omitted"
    );
    let (explanation, materialization_mode) = graph
        .observe()
        .materialize()
        .materialize_explanation_artifact(node)
        .unwrap();
    assert!(
        explanation.is_none(),
        "omitted snapshot richness should remain absent after restore under the active runtime policy"
    );
    assert_eq!(materialization_mode, DiagnosticsAvailability::OmittedByTier);
}

#[test]
fn branch_snapshot_records_explicit_artifact_retention_for_non_active_branches() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime.set_runtime_policy(SignalRuntimePolicy::development());
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("branch-retain")
                        .with_label("retain"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-retention").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("branch-omit"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    runtime.switch_branch(main).unwrap();

    let feature_snapshot = runtime.capture_branch_snapshot(feature).unwrap();
    assert_eq!(
        feature_snapshot
            .meta
            .artifact_retention
            .explanation_retention,
        ArtifactRetentionPolicy::Omit
    );
    assert_eq!(
        feature_snapshot
            .meta
            .artifact_retention
            .provenance_retention,
        ArtifactRetentionPolicy::Omit
    );
    assert!(
        feature_snapshot.diagnostics.explanation_facts.is_empty(),
        "non-active branch snapshots should respect the branch-local snapshot artifact retention contract"
    );
    assert!(
        feature_snapshot.diagnostics.provenance_facts.is_empty(),
        "non-active branch snapshots should not retain omitted provenance richness"
    );
}

#[test]
fn checkpoint_image_strips_node_local_cold_payloads_while_snapshot_bundle_retains_them() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let node = graph.node().output_identity().build();

    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("checkpoint-cold"))
    })
    .unwrap();

    {
        let mut entry = graph.get_entry_mut(node).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::from(vec![ChangedRegion::new("wing")]),
            labels: vec!["retained".to_string()],
            keyed_family: Some("airframe".to_string()),
            keyed_key: Some("wing".to_string()),
            reuse_certification: None,
            reuse_boundary_context: None,
        }));
        entry.set_causality(Some(CausalityMetadata {
            kind: "bridge".to_string(),
            fields: [("patch".to_string(), "s9-12".to_string())]
                .into_iter()
                .collect(),
        }));
    }

    let snapshot = graph.capture_snapshot();

    let checkpoint_graph = snapshot.authority_graph();
    let checkpoint_entry = checkpoint_graph
        .get_entry(node)
        .expect("checkpoint node entry");
    assert!(
        checkpoint_entry.retained_diagnostic_artifact().is_none(),
        "checkpoint image must not carry retained node-local cold artifacts"
    );
    assert!(
        checkpoint_entry.get_causality().is_none(),
        "checkpoint image must not carry causality metadata through the authority lane"
    );

    let rich_entry = snapshot
        .diagnostic_graph
        .get_entry(node)
        .expect("rich snapshot node entry");
    assert!(
        rich_entry.retained_diagnostic_artifact().is_some(),
        "rich snapshot bundle should still retain node-local cold artifacts for diagnostics"
    );
    assert!(
        rich_entry.get_causality().is_some(),
        "rich snapshot bundle should still retain node-local causality for diagnostics"
    );
}

#[test]
fn checkpoint_image_omits_dependency_snapshots_and_restore_rebuilds_them_from_explicit_batch() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let target = graph.node().build();
    graph.append_dependency(target, source, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("checkpoint-deps"))
    })
    .unwrap();
    evaluate(&mut graph, target, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    let snapshot = graph.capture_snapshot();
    let authority_graph = snapshot.authority_graph();
    assert!(
        authority_graph
            .get_dep_snapshot(target)
            .unwrap()
            .entries()
            .is_empty(),
        "checkpoint authority lane must not carry dependency snapshot state"
    );
    assert_eq!(
        snapshot
            .checkpoint_image
            .dependency_snapshot_batch
            .target_nodes()
            .as_slice(),
        &[target],
        "checkpoint image should carry dependency snapshot rebuild work explicitly"
    );

    let mut overwritten = DependencySnapshot::empty();
    overwritten.record(source, ASPECT_A, 9, None);
    graph.set_dep_snapshot(target, overwritten).unwrap();
    assert_eq!(
        graph.get_dep_snapshot(target).unwrap().entries()[0].cached_version,
        9
    );

    graph.restore_snapshot(&snapshot).unwrap();

    assert_eq!(
        graph.get_dep_snapshot(target).unwrap().entries()[0].cached_version,
        1,
        "restore must rebuild dependency snapshots from the explicit checkpoint batch"
    );
}

#[test]
fn restore_uses_checkpoint_authority_even_when_rich_snapshot_node_cold_payloads_are_tampered() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let node = graph.node().output_identity().build();

    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0))
            .with_output_identity("checkpoint-restore"))
    })
    .unwrap();

    {
        let mut entry = graph.get_entry_mut(node).unwrap();
        entry.set_retained_diagnostic_artifact(Some(RetainedDiagnosticArtifact {
            changed_regions: CanonicalChangedRegions::from(vec![ChangedRegion::new("fuselage")]),
            labels: vec!["captured".to_string()],
            keyed_family: Some("airframe".to_string()),
            keyed_key: Some("fuselage".to_string()),
            reuse_certification: None,
            reuse_boundary_context: None,
        }));
        entry.set_causality(Some(CausalityMetadata {
            kind: "capture".to_string(),
            fields: [("rev".to_string(), "1".to_string())].into_iter().collect(),
        }));
    }

    let snapshot = graph.capture_snapshot();

    {
        let mut entry = graph.get_entry_mut(node).unwrap();
        entry.set_retained_diagnostic_artifact(None);
        entry.set_causality(None);
    }
    mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(2, 0))
            .with_output_identity("checkpoint-restore-updated"))
    })
    .unwrap();

    let mut tampered = snapshot.clone();
    {
        let mut entry = tampered.diagnostic_graph.get_entry_mut(node).unwrap();
        entry.set_retained_diagnostic_artifact(None);
        entry.set_causality(None);
    }

    graph.restore_snapshot(&tampered).unwrap();

    assert_eq!(
        graph
            .get_entry(node)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "restore must still follow checkpoint authority for operational state"
    );
    assert!(
        graph
            .get_entry(node)
            .unwrap()
            .retained_diagnostic_artifact()
            .is_none(),
        "restored authority lane must not rehydrate node-local retained artifacts from the checkpoint image"
    );
    assert!(
        graph.get_entry(node).unwrap().get_causality().is_none(),
        "restored authority lane must not rehydrate causality from the checkpoint image"
    );
}

#[test]
fn restore_snapshot_with_active_policy_prunes_cold_richness_without_changing_operational_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let node = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime.set_runtime_policy(SignalRuntimePolicy::development());
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("restore-policy")
                        .with_label("retained"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let explanation = runtime
        .observe()
        .materialize()
        .materialize_explanation_artifact(node)
        .unwrap()
        .0
        .expect("development policy should materialize explanation");
    runtime
        .graph_mut()
        .diagnostics_state_mut()
        .record_explanation_fact(ExplanationFact::from_explanation(&explanation));
    runtime
        .graph_mut()
        .diagnostics_state_mut()
        .record_provenance_fact(ProvenanceFact::from_explanation(&explanation));

    let snapshot = runtime.capture_snapshot();
    assert!(
        snapshot.diagnostics.explanation_facts.contains_key(&node),
        "captured snapshot should include retained explanation richness"
    );

    runtime.set_runtime_policy(
        SignalRuntimePolicy::operational()
            .with_explanation_retention(ArtifactRetentionPolicy::Omit)
            .with_provenance_retention(ArtifactRetentionPolicy::Omit),
    );
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(node, ASPECT_A)?;
            tx.read(node, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("restore-policy-updated"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let restore_plan = runtime
        .graph()
        .plan_snapshot_restore(
            &snapshot,
            SnapshotRestoreIntent::restore_runtime_truth_with_active_policy(),
        )
        .unwrap();
    runtime
        .restore_snapshot_with_intent(
            &snapshot,
            SnapshotRestoreIntent::restore_runtime_truth_with_active_policy(),
        )
        .unwrap();

    assert_eq!(
        runtime
            .graph()
            .get_entry(node)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "active-policy restore should still rewind operational state"
    );
    let (artifact, materialization_mode) = runtime
        .observe()
        .materialize()
        .materialize_explanation_artifact(node)
        .unwrap();
    assert!(artifact.is_none());
    assert_eq!(materialization_mode, DiagnosticsAvailability::OmittedByTier);
    assert!(
        runtime
            .observe()
            .metrics()
            .checkpoint
            .snapshot_restore_count
            >= 1,
        "restore intent should be visible in checkpoint telemetry"
    );
    assert!(
        runtime
            .observe()
            .metrics()
            .checkpoint
            .snapshot_restore_apply_active_policy_count
            >= 1,
        "active-policy restore should be counted explicitly for certification"
    );
    assert_eq!(
        runtime
            .observe()
            .metrics()
            .checkpoint
            .snapshot_restore_shared_delta_node_count,
        restore_plan.dependency_snapshot_delta_node_count(),
        "runtime restore counters should report the same shared-node delta breadth as the canonical restore plan"
    );
    assert_eq!(
        runtime
            .observe()
            .metrics()
            .checkpoint
            .snapshot_restore_coarse_reason_count,
        restore_plan.coarse_reasons().len() as u64,
        "runtime restore counters should report the same coarse restore reason count as the canonical restore plan"
    );
}

#[test]
fn restore_snapshot_rejects_seed_recomputation_intent_before_mutation() {
    let mut graph = SignalGraph::new();
    let node = graph.node().output_identity().build();

    evaluate(&mut graph, node, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(1, 0)).with_output_identity("seed-test"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();

    mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    let err = graph
        .restore_snapshot_with_intent(
            &snapshot,
            SnapshotRestoreIntent {
                state: SnapshotStateRestoreMode::RewindActiveState,
                artifacts: SnapshotArtifactRestoreMode::RestoreCapturedRetention,
                dependency_state: SnapshotDependencyRestoreMode::SeedRecomputationFromSnapshot,
            },
        )
        .unwrap_err();

    assert!(
        err.to_string().contains("SeedRecomputationFromSnapshot"),
        "unsupported recomputation-seed restore intent should fail explicitly"
    );
    assert_eq!(
        graph.get_state(node).unwrap(),
        NodeState::Dirty,
        "unsupported restore intent must fail before mutating operational graph state"
    );
}

#[test]
fn snapshot_restore_plan_reports_shared_delta_and_coarse_requirements() {
    let mut graph = SignalGraph::new();
    let source = graph.node().output_identity().build();
    let target = graph.node().build();
    graph.append_dependency(target, source, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(
            NodeEvaluationResult::from_version(version_ab(1, 0))
                .with_output_identity("plan-source"),
        )
    })
    .unwrap();
    evaluate(&mut graph, target, &mut |_id, graph| {
        Ok(NodeEvaluationResult::from_version(
            graph.get_entry(source).unwrap().get_aspect_version(),
        ))
    })
    .unwrap();

    let snapshot = graph.capture_snapshot();

    let mut updated_snapshot = DependencySnapshot::empty();
    updated_snapshot.record(source, ASPECT_A, 9, None);
    graph.set_dep_snapshot(target, updated_snapshot).unwrap();

    let plan = graph
        .plan_snapshot_restore(&snapshot, SnapshotRestoreIntent::restore_runtime_truth())
        .unwrap();

    assert_eq!(plan.shared_node_count(), 2);
    assert_eq!(plan.current_only_node_count(), 0);
    assert_eq!(plan.snapshot_only_node_count(), 0);
    assert_eq!(plan.dependency_snapshot_delta_node_count(), 1);
    assert_eq!(
        plan.checkpoint_restore_batch()
            .classified()
            .target_nodes()
            .as_slice()
            .len(),
        1
    );
    assert!(plan.coarse_replacement_required());
    assert!(plan
        .coarse_reasons()
        .contains(&SnapshotRestoreCoarseReason::EntryStateRewind));
    assert!(plan
        .coarse_reasons()
        .contains(&SnapshotRestoreCoarseReason::DiagnosticsHistoryRestore));
    assert!(
        !plan
            .coarse_reasons()
            .contains(&SnapshotRestoreCoarseReason::NodeSetDifference),
        "shared-node-only restore planning should not claim node-set mismatch when node sets still align"
    );
}

#[test]
fn snapshot_restore_lineage_defaults_to_compact_global_but_forensic_can_emit_per_node() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().build();
    runtime
        .graph_mut()
        .append_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(dependent, &|view| {
                let result = if view.node() == source {
                    view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("compact-restore"),
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

    let snapshot = runtime.capture_snapshot();
    runtime.restore_snapshot(&snapshot).unwrap();
    let compact_restores = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .filter(|record| {
            matches!(
                record.kind,
                LineageRecordKind::SnapshotRestore { snapshot_id, .. }
                    if snapshot_id == snapshot.meta.snapshot_id
            )
        })
        .count();
    assert_eq!(
        compact_restores, 1,
        "operational/development restore lineage should default to one compact global restore event"
    );

    runtime.set_runtime_policy(
        SignalRuntimePolicy::forensic()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let forensic_snapshot = runtime.capture_snapshot();
    runtime.restore_snapshot(&forensic_snapshot).unwrap();
    let forensic_restores = runtime
        .graph()
        .observe()
        .lineage_records()
        .iter()
        .filter(|record| {
            matches!(
                record.kind,
                LineageRecordKind::SnapshotRestore { snapshot_id, .. }
                    if snapshot_id == forensic_snapshot.meta.snapshot_id
            )
        })
        .count();
    assert!(
        forensic_restores >= 2,
        "forensic per-node restore lineage should emit restored entries for materialized artifacts"
    );
}

#[test]
fn branch_churn_respects_history_and_replay_budgets_under_tight_policy() {
    let policy = SignalRuntimePolicy::operational()
        .with_history_limit(2)
        .with_detail_limit(1)
        .with_history_details(false);
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    runtime.set_runtime_policy(policy);
    let source = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("bounded-main"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-budget").unwrap();
    let main_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
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
        runtime
            .observe()
            .recent_execution_history_diagnostics()
            .len()
            <= policy.retention_budget.history_limit,
        "execution history should stay within the configured history budget under branch churn"
    );
    assert!(
        runtime.graph().replay_events().len() <= policy.retention_budget.history_limit.max(1) * 32,
        "replay retention should stay within the policy-derived bound under branch churn"
    );
    assert!(
        runtime.graph().observe().lineage_records().len()
            <= policy.retention_budget.history_limit.max(1) * 32,
        "lineage retention should stay within the policy-derived bound under branch churn"
    );
    assert_eq!(
        runtime.observe().known_branches().len(),
        2,
        "branch churn should not fabricate extra branch catalog entries"
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id),
        "main head should remain pinned to its snapshot under churn"
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(feature.id),
        Some(feature_snapshot.meta.snapshot_id),
        "feature head should remain pinned to its snapshot under churn"
    );
}
