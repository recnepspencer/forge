use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::*;
use crate::tests::support::*;

#[test]
fn branch_debug_session_mixed_churn_stays_forensically_coherent() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(SignalRuntimePolicy::development().with_history_limit(6));
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let family = define_keyed_computation(&mut runtime, "workflow-projection", ());
    let keyed_def = family.keyed("wing-left");
    let keyed = keyed_def.node(&mut runtime);
    let keyed_computation = keyed_def.memoized("shape-v1");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-source-v1"),
                ))
            })?;
            tx.read(dependent, &|view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("main-dependent-v1"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &keyed_computation, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("keyed-main-v1")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-debug").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-source-v2"),
                ))
            })?;
            tx.read(dependent, &|view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("feature-dependent-v2"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let analysis = runtime.create_branch("analysis-debug").unwrap();
    runtime.switch_branch(analysis.clone()).unwrap();
    let analysis_snapshot = runtime.capture_branch_snapshot(analysis.clone()).unwrap();

    for cycle in 0..10 {
        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(50 + cycle, 0))
                        .with_output_identity(format!("analysis-bad-{cycle}")),
                ))
            })?;
            Err(SignalError::invalid_input("synthetic analysis rollback"))
        });
        assert!(err.is_err());

        runtime
            .restore_branch_snapshot(analysis.clone(), &analysis_snapshot)
            .unwrap();
        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .restore_branch_snapshot(feature.clone(), &feature_snapshot)
            .unwrap();
        runtime.switch_branch(analysis.clone()).unwrap();
    }

    runtime.switch_branch(main.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1
    );
    let main_replay = runtime.observe().replay_for_branch(main.id);
    assert!(main_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == main.id));

    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );
    let feature_replay = runtime.observe().replay_for_branch(feature.id);
    assert!(feature_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == feature.id));
    assert!(
        feature_replay
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(feature_snapshot.meta.snapshot_id)),
        "feature replay should still point back to the saved branch head snapshot"
    );

    runtime.switch_branch(analysis.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1,
        "analysis restores should keep the branch aligned with the captured main snapshot"
    );
    let analysis_replay = runtime.observe().replay_for_branch(analysis.id);
    assert!(analysis_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == analysis.id));
    assert!(
        analysis_replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::TransactionRolledBack),
        "analysis debug session should preserve rollback evidence for post-hoc inspection"
    );
    assert!(
        analysis_replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::SnapshotRestored),
        "analysis debug session should preserve restore evidence for post-hoc inspection"
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id)
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(feature.id),
        Some(feature_snapshot.meta.snapshot_id)
    );
    assert_eq!(
        runtime.observe().branch_head_snapshot_id(analysis.id),
        Some(analysis_snapshot.meta.snapshot_id),
        "analysis branch should stay pinned to its own restored snapshot"
    );
}

#[test]
fn undo_redo_style_session_with_failures_and_memo_reuse_preserves_branch_local_truth() {
    let policy = SignalRuntimePolicy::development()
        .with_history_limit(4)
        .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode);
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(policy);
    let source = runtime.graph_mut().node().output_identity().build();
    let family = define_keyed_computation(&mut runtime, "undo-redo-session", ());
    let keyed_def = family.keyed("bulkhead");
    let keyed = keyed_def.node(&mut runtime);
    let computation = keyed_def.memoized("shape-v1");
    let compute_calls = AtomicU32::new(0);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-v1"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("memo-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-undo").unwrap();
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
            tx.evaluate_keyed(keyed, &computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("memo-artifact")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    for cycle in 0..20 {
        runtime.switch_branch(main.clone()).unwrap();
        runtime
            .restore_branch_snapshot(main.clone(), &main_snapshot)
            .unwrap();

        runtime.switch_branch(feature.clone()).unwrap();
        runtime
            .restore_branch_snapshot(feature.clone(), &feature_snapshot)
            .unwrap();

        let err = runtime.transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(100 + cycle, 0))
                        .with_output_identity(format!("bad-{cycle}")),
                ))
            })?;
            Err(SignalError::invalid_input("synthetic feature rollback"))
        });
        assert!(err.is_err());

        mark_dirty(runtime.graph_mut(), keyed, ASPECT_A).unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(keyed, &computation, &|view| {
                    compute_calls.fetch_add(1, Ordering::Relaxed);
                    Ok(view.finish(NodeEvaluationResult::from_version(version_ab(999, 0))))
                })?;
                Ok(())
            })
            .unwrap();
    }

    assert_eq!(
        compute_calls.load(Ordering::Relaxed),
        1,
        "long undo/redo churn should keep reusing the memoized artifact instead of recomputing every cycle"
    );

    runtime.switch_branch(main.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        1
    );

    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        2
    );
    let feature_lineage = runtime.observe().lineage_chain_for_node(source);
    assert!(
        feature_lineage
            .iter()
            .any(|record| record.event == LineageEvent::Restored),
        "feature workflow should preserve restore lineage under undo/redo churn"
    );
    assert!(
        runtime.observe().recent_execution_history_diagnostics().len() <= policy.history_limit,
        "history must stay bounded under long undo/redo churn"
    );
    assert!(
        runtime.graph().replay_events().len() <= policy.history_limit.max(1) * 32,
        "replay must stay bounded under long undo/redo churn"
    );
}

#[test]
fn posthoc_forensics_after_long_session_answers_branch_and_artifact_questions() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(SignalRuntimePolicy::forensic().with_history_limit(8));
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let family = define_keyed_computation(&mut runtime, "posthoc-session", ());
    let keyed_def = family.keyed("skin-panel");
    let keyed = keyed_def.node(&mut runtime);
    let computation = keyed_def.memoized("shape-a");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("source-main"),
                ))
            })?;
            tx.read(dependent, &|view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("dependent-main"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &computation, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("skin-a")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-posthoc").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("source-feature"),
                ))
            })?;
            tx.read(dependent, &|view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("dependent-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    let analysis = runtime.create_branch("analysis-posthoc").unwrap();
    runtime.switch_branch(analysis.clone()).unwrap();
    let analysis_snapshot = runtime.capture_branch_snapshot(analysis.clone()).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();
    runtime
        .restore_branch_snapshot(analysis.clone(), &analysis_snapshot)
        .unwrap();
    mark_dirty(runtime.graph_mut(), dependent, ASPECT_A).unwrap();
    runtime
        .restore_branch_snapshot(analysis.clone(), &analysis_snapshot)
        .unwrap();
    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.mark_dirty(source, ASPECT_A)?;
        tx.read(source, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(99, 0))
                    .with_output_identity("analysis-bad"),
            ))
        })?;
        Err(SignalError::invalid_input("analysis failure"))
    });
    assert!(err.is_err());

    runtime.switch_branch(main.clone()).unwrap();

    let main_head = runtime.observe().branch_head_snapshot_id(main.id);
    let feature_head = runtime.observe().branch_head_snapshot_id(feature.id);
    let analysis_head = runtime.observe().branch_head_snapshot_id(analysis.id);
    let main_replay = runtime.observe().replay_for_branch(main.id);
    let feature_replay = runtime.observe().replay_for_branch(feature.id);
    let analysis_replay = runtime.observe().replay_for_branch(analysis.id);
    let artifact = runtime
        .switch_branch(feature.clone())
        .map(|_| runtime.observe().current_lineage_artifact(source).unwrap())
        .unwrap();
    let artifact_replay = runtime.observe().replay_for_artifact(artifact);
    let artifact_lineage = runtime.observe().lineage_chain_for_artifact(artifact);
    let around_feature_snapshot = runtime.observe().replay_around_snapshot(feature_snapshot.meta.snapshot_id);

    assert_eq!(main_head, Some(main_snapshot.meta.snapshot_id));
    assert_eq!(feature_head, Some(feature_snapshot.meta.snapshot_id));
    assert_eq!(
        analysis_head,
        Some(analysis_snapshot.meta.snapshot_id),
        "analysis branch should point at its own snapshot after repeated local restores"
    );
    assert!(main_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == main.id));
    assert!(feature_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == feature.id));
    assert!(analysis_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == analysis.id));
    assert!(
        artifact_replay
            .frames
            .iter()
            .all(|frame| frame.lineage_artifact_id == Some(artifact)),
        "post-hoc artifact replay should isolate the requested artifact timeline"
    );
    assert!(
        artifact_lineage
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "post-hoc lineage should answer where the artifact came from"
    );
    assert!(
        around_feature_snapshot
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(feature_snapshot.meta.snapshot_id)),
        "post-hoc replay should answer which events surrounded the feature snapshot"
    );
    assert!(
        analysis_replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::TransactionRolledBack),
        "post-hoc replay should preserve the failed analysis transaction"
    );
}

#[test]
fn game_engine_frame_session_handles_threshold_flapping_branch_churn_and_posthoc_debugging() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::game_engine()
            .with_history_limit(6)
            .with_detail_limit(2)
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let source = runtime.graph_mut().node().output_identity().build();
    let culled = runtime
        .graph_mut()
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    let lod = runtime
        .graph_mut()
        .node()
        .aspect_filter(mask_b())
        .output_identity()
        .build();
    let render = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(culled, source, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(lod, source, ASPECT_B)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(render, culled, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(render, lod, ASPECT_B)
        .unwrap();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(10, 100))
                        .with_output_identity("source-frame-10-meta-100"),
                ))
            })?;
            tx.read(culled, &|view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("culled-frame-10"),
                ))
            })?;
            tx.read(lod, &|view| {
                let version = view.read_aspect_version(source, ASPECT_B)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("lod-meta-100"),
                ))
            })?;
            tx.read(render, &|view| {
                let geometry = view.read_aspect_version(culled, ASPECT_A)?;
                let lod_meta = view.read_aspect_version(lod, ASPECT_B)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(
                        geometry.get(ASPECT_A),
                        lod_meta.get(ASPECT_B),
                    ))
                    .with_output_identity("render-frame-10"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let editor = runtime.observe().current_branch();
    let editor_snapshot = runtime.capture_snapshot();
    let play = runtime.create_branch("play-session").unwrap();
    runtime.switch_branch(play.clone()).unwrap();

    for frame in [11_u64, 12, 13, 14, 15, 16, 17, 18] {
        let metadata_version = if frame % 3 == 0 { 100 + frame } else { 100 };
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                if frame % 3 == 0 {
                    tx.mark_dirty(source, ASPECT_B)?;
                }
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(frame, metadata_version))
                            .with_output_identity(format!(
                                "source-frame-{frame}-meta-{metadata_version}"
                            )),
                    ))
                })?;
                tx.read(culled, &|view| {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity(format!("culled-frame-{frame}")),
                    ))
                })?;
                tx.read(lod, &|view| {
                    let version = view.read_aspect_version(source, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity(format!("lod-meta-{metadata_version}")),
                    ))
                })?;
                tx.read(render, &|view| {
                    let geometry = view.read_aspect_version(culled, ASPECT_A)?;
                    let lod_meta = view.read_aspect_version(lod, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(
                            geometry.get(ASPECT_A),
                            lod_meta.get(ASPECT_B),
                        ))
                        .with_output_identity(format!(
                            "render-frame-{frame}-meta-{metadata_version}"
                        )),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        if frame % 2 == 0 {
            let err = runtime.transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.mark_dirty(source, ASPECT_B)?;
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(frame + 100, frame + 200))
                            .with_output_identity(format!("bad-frame-{frame}")),
                    ))
                })?;
                Err(SignalError::invalid_input(
                    "synthetic playtest frame rollback",
                ))
            });
            assert!(err.is_err());
        }
    }

    let play_snapshot = runtime.capture_branch_snapshot(play.clone()).unwrap();
    runtime.switch_branch(editor.clone()).unwrap();
    runtime
        .restore_branch_snapshot(editor.clone(), &editor_snapshot)
        .unwrap();
    runtime.switch_branch(play.clone()).unwrap();
    runtime
        .restore_branch_snapshot(play.clone(), &play_snapshot)
        .unwrap();

    let play_replay = runtime.observe().replay_for_branch(play.id);
    assert!(
        play_replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::TransactionRolledBack),
        "play-session replay should preserve failed-frame rollback evidence"
    );
    assert!(
        runtime
            .observe()
            .lineage_chain_for_node(render)
            .iter()
            .any(|record| record.event == LineageEvent::Restored),
        "render artifact lineage should expose restore transitions after editor/play churn"
    );
    assert!(
        runtime
            .observe()
            .lineage_chain_for_node(lod)
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "the aspect-filtered LOD node should participate in the same long workflow and keep its own lineage history"
    );

    runtime.switch_branch(editor.clone()).unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        10,
        "editor branch should recover the original frame state after play-session churn"
    );
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_B),
        100,
        "editor branch should also recover the untouched metadata aspect"
    );
    let editor_replay = runtime.observe().replay_for_branch(editor.id);
    assert!(editor_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == editor.id));
}

#[test]
fn fintech_tick_correction_session_preserves_auditability_under_branching_replay_and_memo_reuse() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::fintech()
            .with_history_limit(8)
            .with_detail_limit(3),
    );
    let ticks = runtime.graph_mut().node().output_identity().build();
    let price = runtime.graph_mut().node().output_identity().build();
    let alert = runtime
        .graph_mut()
        .node()
        .aspect_filter(mask_b())
        .output_identity()
        .build();
    let throttle = runtime
        .graph_mut()
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    runtime
        .graph_mut()
        .add_dependency(price, ticks, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(alert, ticks, ASPECT_B)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(throttle, ticks, ASPECT_A)
        .unwrap();
    let family = define_keyed_computation(&mut runtime, "pricing-book", ());
    let risk_def = family.keyed("book-a");
    let risk = risk_def.node(&mut runtime);
    let risk_computation = risk_def.memoized("book-a-day-1");
    let compute_calls = AtomicU32::new(0);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(ticks, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(100, 5))
                        .with_output_identity("ticks-100-vol-5"),
                ))
            })?;
            tx.read(price, &|view| {
                let version = view.read_aspect_version(ticks, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version).with_output_identity("price-100"),
                ))
            })?;
            tx.read(alert, &|view| {
                let version = view.read_aspect_version(ticks, ASPECT_B)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version).with_output_identity("alert-vol-5"),
                ))
            })?;
            tx.read(throttle, &|view| {
                let version = view.read_aspect_version(ticks, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("throttle-100"),
                ))
            })?;
            tx.evaluate_keyed(risk, &risk_computation, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(100, 0))
                        .with_output_identity("risk-100")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let audit_snapshot = runtime.capture_snapshot();
    let what_if = runtime.create_branch("what-if-shock").unwrap();
    runtime.switch_branch(what_if.clone()).unwrap();

    for tick in [101_u64, 102, 103, 104] {
        let volatility = if tick % 2 == 0 { 9 } else { 5 };
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(ticks, ASPECT_A)?;
                if tick % 2 == 0 {
                    tx.mark_dirty(ticks, ASPECT_B)?;
                }
                tx.read(ticks, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(tick, volatility))
                            .with_output_identity(format!("ticks-{tick}-vol-{volatility}")),
                    ))
                })?;
                tx.read(price, &|view| {
                    let version = view.read_aspect_version(ticks, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity(format!("price-{tick}")),
                    ))
                })?;
                tx.read(alert, &|view| {
                    let version = view.read_aspect_version(ticks, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity(format!("alert-vol-{volatility}")),
                    ))
                })?;
                tx.read(throttle, &|view| {
                    let version = view.read_aspect_version(ticks, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity(format!("throttle-{tick}")),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        mark_dirty(runtime.graph_mut(), risk, ASPECT_A).unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(risk, &risk_computation, &|view| {
                    compute_calls.fetch_add(1, Ordering::Relaxed);
                    Ok(view.finish(NodeEvaluationResult::from_version(version_ab(9999, 0))))
                })?;
                Ok(())
            })
            .unwrap();
    }

    let what_if_snapshot = runtime.capture_branch_snapshot(what_if.clone()).unwrap();
    let correction = runtime.create_branch("late-tick-correction").unwrap();
    runtime.switch_branch(correction.clone()).unwrap();
    let correction_snapshot = runtime.capture_branch_snapshot(correction.clone()).unwrap();

    let err = runtime.transaction(&mut runtime_ctx, |tx| {
        tx.mark_dirty(ticks, ASPECT_A)?;
        tx.read(ticks, &|view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(80, 0))
                    .with_output_identity("bad-correction"),
            ))
        })?;
        Err(SignalError::invalid_input("synthetic late-tick rollback"))
    });
    assert!(err.is_err());

    runtime
        .restore_branch_snapshot(correction.clone(), &correction_snapshot)
        .unwrap();
    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .restore_branch_snapshot(main.clone(), &audit_snapshot)
        .unwrap();
    runtime.switch_branch(what_if.clone()).unwrap();
    runtime
        .restore_branch_snapshot(what_if.clone(), &what_if_snapshot)
        .unwrap();

    assert_eq!(
        compute_calls.load(Ordering::Relaxed),
        1,
        "risk memoization should survive tick-session churn without recomputing every audit pass"
    );

    let correction_replay = runtime.observe().replay_for_branch(correction.id);
    assert!(
        correction_replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::TransactionRolledBack),
        "correction branch replay should preserve rollback evidence for audit"
    );
    runtime.switch_branch(what_if.clone()).unwrap();
    let risk_artifact = runtime.observe().current_lineage_artifact(risk).unwrap();
    let risk_replay = runtime.observe().replay_for_artifact(risk_artifact);
    assert!(
        risk_replay
            .frames
            .iter()
            .all(|frame| frame.lineage_artifact_id == Some(risk_artifact)),
        "artifact replay should isolate the memoized risk artifact timeline"
    );
    assert!(
        runtime
            .observe()
            .lineage_chain_for_artifact(risk_artifact)
            .iter()
            .any(|record| record.event == LineageEvent::MemoizedFrom),
        "artifact lineage should explain memoized reuse in the audit workflow"
    );
    assert!(
        runtime
            .observe()
            .lineage_chain_for_node(alert)
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "the aspect-filtered alert node should keep its own branch-local lineage under the same workflow"
    );
    let around_snapshot = runtime.observe().replay_around_snapshot(what_if_snapshot.meta.snapshot_id);
    assert!(
        around_snapshot
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(what_if_snapshot.meta.snapshot_id)),
        "audit replay should answer what happened around the what-if snapshot"
    );

    runtime.switch_branch(main).unwrap();
    assert_eq!(
        runtime.graph().get_entry(ticks).unwrap().get_aspect_version().get(ASPECT_A),
        100,
        "main branch should remain on the authoritative baseline after what-if and correction churn"
    );
    assert_eq!(
        runtime
            .graph()
            .get_entry(ticks)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_B),
        5,
        "main branch should also preserve the baseline volatility aspect after churn"
    );
}

#[test]
fn alternating_dynamic_rewire_across_branches_preserves_subscriber_integrity() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    let selector = runtime.graph_mut().node().output_identity().build();
    let left = runtime.graph_mut().node().output_identity().build();
    let right = runtime.graph_mut().node().output_identity().build();
    let target = runtime.graph_mut().node().output_identity().build();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(selector, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("route-left"),
                ))
            })?;
            tx.read(left, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(10, 0))
                        .with_output_identity("left-v1"),
                ))
            })?;
            tx.read(right, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(20, 0))
                        .with_output_identity("right-v1"),
                ))
            })?;
            tx.read(target, &|view| {
                let route = view.read_aspect_version(selector, ASPECT_A)?;
                let upstream = if route.get(ASPECT_A) % 2 == 1 {
                    left
                } else {
                    right
                };
                let version = view.read_aspect_version(upstream, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity(format!("target-from-{}", upstream.index())),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-rewire").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(selector, ASPECT_A)?;
            tx.read(selector, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("route-right"),
                ))
            })?;
            tx.read(target, &|view| {
                let route = view.read_aspect_version(selector, ASPECT_A)?;
                let upstream = if route.get(ASPECT_A) % 2 == 1 {
                    left
                } else {
                    right
                };
                let version = view.read_aspect_version(upstream, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity(format!("target-from-{}", upstream.index())),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    assert!(runtime.graph().depends_on(target, right, ASPECT_A).unwrap());
    assert!(!runtime.graph().depends_on(target, left, ASPECT_A).unwrap());

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .restore_branch_snapshot(main.clone(), &main_snapshot)
        .unwrap();
    assert!(runtime.graph().depends_on(target, left, ASPECT_A).unwrap());
    assert!(!runtime.graph().depends_on(target, right, ASPECT_A).unwrap());
    assert!(runtime
        .graph()
        .subscribers_of(left)
        .unwrap()
        .contains(&target));
    assert!(!runtime
        .graph()
        .subscribers_of(right)
        .unwrap()
        .contains(&target));

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    assert!(runtime.graph().depends_on(target, right, ASPECT_A).unwrap());
    assert!(!runtime.graph().depends_on(target, left, ASPECT_A).unwrap());
    assert!(runtime
        .graph()
        .subscribers_of(right)
        .unwrap()
        .contains(&target));
    assert!(!runtime
        .graph()
        .subscribers_of(left)
        .unwrap()
        .contains(&target));
}

#[test]
fn retained_vs_reconstructed_artifacts_match_after_long_churn() {
    fn run(policy: SignalRuntimePolicy) -> (ReplaySlice, Vec<LineageRecord>, NodeExplanation) {
        let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
        runtime.set_runtime_policy(policy);
        let source = runtime.graph_mut().node().output_identity().build();
        let dependent = runtime.graph_mut().node().output_identity().build();
        runtime
            .graph_mut()
            .add_dependency(dependent, source, ASPECT_A)
            .unwrap();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("seed"),
                    ))
                })?;
                tx.read(dependent, &|view| {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("dependent-seed"),
                    ))
                })?;
                Ok(())
            })
            .unwrap();

        let main = runtime.observe().current_branch();
        let snapshot = runtime.capture_snapshot();
        let feature = runtime.create_branch("feature-retention").unwrap();
        runtime.switch_branch(feature.clone()).unwrap();

        for step in 0..12 {
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    tx.read(source, &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(2 + step, 0))
                                .with_output_identity(format!("source-{step}")),
                        ))
                    })?;
                    tx.read(dependent, &|view| {
                        let version = view.read_aspect_version(source, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("dependent-{step}")),
                        ))
                    })?;
                    Ok(())
                })
                .unwrap();
            if step % 3 == 2 {
                runtime.switch_branch(main.clone()).unwrap();
                runtime
                    .restore_branch_snapshot(main.clone(), &snapshot)
                    .unwrap();
                runtime.switch_branch(feature.clone()).unwrap();
            }
        }

        (
            runtime.observe().replay_for_branch(feature.id),
            runtime.observe().lineage_chain_for_node(dependent),
            runtime.observe().explain(dependent).unwrap(),
        )
    }

    let development = run(SignalRuntimePolicy::development());
    let operational = run(SignalRuntimePolicy::operational());

    assert!(replay_slices_equivalent(&development.0, &operational.0));
    assert!(lineage_records_equivalent(&development.1, &operational.1));
    assert_eq!(development.2.state, operational.2.state);
    assert_eq!(development.2.output_change, operational.2.output_change);
    assert_eq!(development.2.upstream.len(), operational.2.upstream.len());
}

#[test]
fn threshold_flap_storm_with_on_demand_and_restore_keeps_replay_coherent() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(
        SignalRuntimePolicy::development()
            .with_snapshot_restore_lineage_mode(SnapshotRestoreLineageMode::PerNode),
    );
    let source = graph.node().output_identity().build();
    let threshold = graph
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    let deferred = graph.node().on_demand().output_identity().build();
    graph.add_dependency(threshold, source, ASPECT_A).unwrap();
    graph.add_dependency(deferred, threshold, ASPECT_A).unwrap();

    evaluate(&mut graph, source, &mut |_id, _graph| {
        Ok(NodeEvaluationResult::from_version(version_ab(10, 0)).with_output_identity("base"))
    })
    .unwrap();
    evaluate(&mut graph, threshold, &mut |_id, graph| {
        let version = graph.get_entry(source).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version).with_output_identity("threshold-base"))
    })
    .unwrap();
    evaluate_on_demand(&mut graph, deferred, &mut |_id, graph| {
        let version = graph.get_entry(threshold).unwrap().get_aspect_version();
        Ok(NodeEvaluationResult::from_version(version).with_output_identity("deferred-base"))
    })
    .unwrap();
    let snapshot = graph.capture_snapshot();

    for version in [11_u64, 12, 11, 13, 12, 14, 13, 15] {
        mark_dirty(&mut graph, source, ASPECT_A).unwrap();
        evaluate(&mut graph, source, &mut |_id, _graph| {
            Ok(NodeEvaluationResult::from_version(version_ab(version, 0))
                .with_output_identity(format!("source-{version}")))
        })
        .unwrap();
        evaluate(&mut graph, threshold, &mut |_id, graph| {
            let current = graph.get_entry(source).unwrap().get_aspect_version();
            Ok(NodeEvaluationResult::from_version(current)
                .with_output_identity(format!("threshold-{version}")))
        })
        .unwrap();
        if version % 2 == 0 {
            evaluate_on_demand(&mut graph, deferred, &mut |_id, graph| {
                let current = graph.get_entry(threshold).unwrap().get_aspect_version();
                Ok(NodeEvaluationResult::from_version(current)
                    .with_output_identity(format!("deferred-{version}")))
            })
            .unwrap();
        }
    }

    graph.restore_snapshot(&snapshot).unwrap();
    assert!(
        graph.replay_events().iter().any(|frame| {
            frame.kind == ReplayEventKind::SnapshotRestored
                && frame.snapshot_id == Some(snapshot.meta.snapshot_id)
        }),
        "restore should append a snapshot-restored replay event after threshold flap churn"
    );
    let explanation = graph.observe().explain(deferred).unwrap();
    assert_eq!(explanation.state, NodeState::Clean);
    assert!(
        graph.observe()
            .lineage_for_node(deferred)
            .iter()
            .any(|record| record.event == LineageEvent::Restored),
        "on-demand node should preserve restore lineage after threshold flap storm"
    );
}

#[test]
fn inspect_only_at_end_after_50_step_session_preserves_forensic_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(SignalRuntimePolicy::forensic().with_history_limit(10));
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let mut saved = None;

    for step in 0..50_u64 {
        if step == 0 {
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.read(source, &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(1, 0))
                                .with_output_identity("seed"),
                        ))
                    })?;
                    tx.read(dependent, &|view| {
                        let version = view.read_aspect_version(source, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity("dependent-seed"),
                        ))
                    })?;
                    Ok(())
                })
                .unwrap();
            saved = Some(runtime.capture_snapshot());
            continue;
        }

        if step % 10 == 0 {
            let branch = runtime.create_branch(format!("branch-{step}")).unwrap();
            runtime.switch_branch(branch).unwrap();
        }
        if step % 7 == 0 {
            let err = runtime.transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1000 + step, 0))
                            .with_output_identity(format!("bad-{step}")),
                    ))
                })?;
                Err(SignalError::invalid_input(
                    "synthetic long-session rollback",
                ))
            });
            assert!(err.is_err());
        } else {
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    tx.read(source, &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(1 + step, 0))
                                .with_output_identity(format!("source-{step}")),
                        ))
                    })?;
                    tx.read(dependent, &|view| {
                        let version = view.read_aspect_version(source, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("dependent-{step}")),
                        ))
                    })?;
                    Ok(())
                })
                .unwrap();
        }

        if step % 13 == 0 {
            if runtime.observe().current_branch().id == main.id {
                runtime
                    .restore_branch_snapshot(main.clone(), saved.as_ref().unwrap())
                    .unwrap();
            }
        }
    }

    let replay = runtime.observe().replay_for_branch(runtime.observe().current_branch().id);
    let lineage = runtime.observe().lineage_chain_for_node(dependent);
    let explanation = runtime.observe().explain(dependent).unwrap();

    assert!(
        !replay.frames.is_empty(),
        "end-of-session replay should still be queryable"
    );
    assert!(
        !lineage.is_empty(),
        "end-of-session lineage should still be queryable"
    );
    assert!(
        replay
            .frames
            .iter()
            .any(|frame| frame.kind == ReplayEventKind::TransactionRolledBack),
        "end-of-session replay should retain rollback evidence"
    );
    assert!(
        lineage
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "end-of-session lineage should retain materialization history"
    );
    assert!(
        explanation.execution_record_id.is_some(),
        "end-of-session explanation should still resolve to a trace-bearing artifact"
    );
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_branch_memo_rollback_session_preserves_branch_local_replay_and_cache_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(
        SignalRuntimePolicy::game_engine()
            .with_history_limit(8)
            .with_detail_limit(4),
    );
    let source = runtime.graph_mut().node().output_identity().build();
    let gated = runtime
        .graph_mut()
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    let filtered = runtime
        .graph_mut()
        .node()
        .aspect_filter(mask_b())
        .output_identity()
        .build();
    let fused = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(gated, source, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(filtered, source, ASPECT_B)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(fused, gated, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(fused, filtered, ASPECT_B)
        .unwrap();
    let family = define_keyed_computation(&mut runtime, "parallel-branch-memo", ());
    let keyed_def = family.keyed("mesh-cache");
    let keyed = keyed_def.node(&mut runtime);
    let memo = keyed_def.memoized("lod-0");
    let compute_calls = AtomicU32::new(0);
    let executor = StageExecutor::aggressive_parallel();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read_with_executor(
                source,
                &|_node, view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 100))
                            .with_output_identity("seed-ab"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                gated,
                &|_node, view| {
                    let version = view.read_aspect_version(source, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("gated-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                filtered,
                &|_node, view| {
                    let version = view.read_aspect_version(source, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("filtered-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                fused,
                &|_node, view| {
                    let a = view.read_aspect_version(gated, ASPECT_A)?;
                    let b = view.read_aspect_version(filtered, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(
                            a.get(ASPECT_A),
                            b.get(ASPECT_B),
                        ))
                        .with_output_identity("fused-seed")
                        .with_continuity_token("mesh-continuity"),
                    ))
                },
                executor,
            )?;
            tx.evaluate_keyed(keyed, &memo, &|view| {
                compute_calls.fetch_add(1, Ordering::Relaxed);
                let version = view.read_aspect_version(fused, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("memo-seed")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("parallel-feature").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    for step in 0..12_u64 {
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                if step % 3 == 0 {
                    tx.mark_dirty(source, ASPECT_B)?;
                }
                tx.read_with_executor(
                    source,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(
                                20 + step,
                                100 + (step % 2),
                            ))
                            .with_output_identity(format!("source-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    gated,
                    &|_node, view| {
                        let version = view.read_aspect_version(source, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("gated-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    filtered,
                    &|_node, view| {
                        let version = view.read_aspect_version(source, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("filtered-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    fused,
                    &|_node, view| {
                        let a = view.read_aspect_version(gated, ASPECT_A)?;
                        let b = view.read_aspect_version(filtered, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(
                                a.get(ASPECT_A),
                                b.get(ASPECT_B),
                            ))
                            .with_output_identity(format!("fused-{step}"))
                            .with_continuity_token("mesh-continuity"),
                        ))
                    },
                    executor,
                )?;
                Ok(())
            })
            .unwrap();

        mark_dirty(runtime.graph_mut(), keyed, ASPECT_A).unwrap();
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.evaluate_keyed(keyed, &memo, &|view| {
                    compute_calls.fetch_add(1, Ordering::Relaxed);
                    let version = view.read_aspect_version(fused, ASPECT_A)?;
                    Ok(view.finish(NodeEvaluationResult::from_version(version)))
                })?;
                Ok(())
            })
            .unwrap();

        if step % 2 == 1 {
            let analysis = runtime.create_branch(format!("analysis-{step}")).unwrap();
            runtime.switch_branch(analysis.clone()).unwrap();
            let err = runtime.transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_A)?;
                tx.read_with_executor(
                    source,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(500 + step, 900))
                                .with_output_identity(format!("bad-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    fused,
                    &|_node, view| {
                        let a = view.read_aspect_version(gated, ASPECT_A)?;
                        let b = view.read_aspect_version(filtered, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(
                                a.get(ASPECT_A),
                                b.get(ASPECT_B),
                            ))
                            .with_output_identity(format!("bad-fused-{step}")),
                        ))
                    },
                    executor,
                )?;
                Err(SignalError::invalid_input(
                    "synthetic parallel analysis rollback",
                ))
            });
            assert!(err.is_err());
            runtime.switch_branch(feature.clone()).unwrap();
        }
    }

    assert_eq!(
        compute_calls.load(Ordering::Relaxed),
        1,
        "memoized keyed artifact should stay hot through parallel branch churn instead of recomputing each cycle"
    );

    let feature_replay = runtime.observe().replay_for_branch(feature.id);
    assert!(feature_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == feature.id));
    let fused_lineage = runtime.observe().lineage_chain_for_node(fused);
    assert!(
        fused_lineage.len() >= 2
            && fused_lineage.iter().any(|record| {
                matches!(
                    record.event,
                    LineageEvent::Restored | LineageEvent::Replaced | LineageEvent::Refreshed
                )
            }),
        "fused node should retain a real lineage history through parallel branch churn"
    );

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .restore_branch_snapshot(main.clone(), &main_snapshot)
        .unwrap();
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_A),
        10
    );
    assert_eq!(
        runtime
            .graph()
            .get_entry(source)
            .unwrap()
            .get_aspect_version()
            .get(ASPECT_B),
        100
    );
}

#[cfg(feature = "parallel")]
#[test]
fn long_session_replay_and_lineage_stay_equivalent_between_serial_and_parallel_executors() {
    fn run(executor: StageExecutor) -> (ReplaySlice, Vec<LineageRecord>, NodeExplanation) {
        let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
        runtime.set_runtime_policy(SignalRuntimePolicy::kernel().with_history_limit(8));
        let source = runtime.graph_mut().node().output_identity().build();
        let a_gate = runtime
            .graph_mut()
            .node()
            .condition(EvaluationCondition::DeltaThreshold(2.0))
            .output_identity()
            .build();
        let b_gate = runtime
            .graph_mut()
            .node()
            .aspect_filter(mask_b())
            .output_identity()
            .build();
        let sink = runtime.graph_mut().node().output_identity().build();
        runtime
            .graph_mut()
            .add_dependency(a_gate, source, ASPECT_A)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(b_gate, source, ASPECT_B)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(sink, a_gate, ASPECT_A)
            .unwrap();
        runtime
            .graph_mut()
            .add_dependency(sink, b_gate, ASPECT_B)
            .unwrap();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read_with_executor(
                    source,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(1, 10))
                                .with_output_identity("seed-source"),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    a_gate,
                    &|_node, view| {
                        let version = view.read_aspect_version(source, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity("seed-a"),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    b_gate,
                    &|_node, view| {
                        let version = view.read_aspect_version(source, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity("seed-b"),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    sink,
                    &|_node, view| {
                        let a = view.read_aspect_version(a_gate, ASPECT_A)?;
                        let b = view.read_aspect_version(b_gate, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(
                                a.get(ASPECT_A),
                                b.get(ASPECT_B),
                            ))
                            .with_output_identity("seed-sink")
                            .with_continuity_token("surface"),
                        ))
                    },
                    executor,
                )?;
                Ok(())
            })
            .unwrap();

        let main = runtime.observe().current_branch();
        let snapshot = runtime.capture_snapshot();
        let feature = runtime.create_branch("executor-feature").unwrap();
        runtime.switch_branch(feature.clone()).unwrap();

        for step in 0..20_u64 {
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    if step % 4 == 0 {
                        tx.mark_dirty(source, ASPECT_B)?;
                    }
                    tx.read_with_executor(
                        source,
                        &|_node, view| {
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version_ab(
                                    2 + step,
                                    10 + (step % 3),
                                ))
                                .with_output_identity(format!("source-{step}")),
                            ))
                        },
                        executor,
                    )?;
                    tx.read_with_executor(
                        a_gate,
                        &|_node, view| {
                            let version = view.read_aspect_version(source, ASPECT_A)?;
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version)
                                    .with_output_identity(format!("a-{step}")),
                            ))
                        },
                        executor,
                    )?;
                    tx.read_with_executor(
                        b_gate,
                        &|_node, view| {
                            let version = view.read_aspect_version(source, ASPECT_B)?;
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version)
                                    .with_output_identity(format!("b-{step}")),
                            ))
                        },
                        executor,
                    )?;
                    tx.read_with_executor(
                        sink,
                        &|_node, view| {
                            let a = view.read_aspect_version(a_gate, ASPECT_A)?;
                            let b = view.read_aspect_version(b_gate, ASPECT_B)?;
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version_ab(
                                    a.get(ASPECT_A),
                                    b.get(ASPECT_B),
                                ))
                                .with_output_identity(format!("sink-{step}"))
                                .with_continuity_token("surface"),
                            ))
                        },
                        executor,
                    )?;
                    Ok(())
                })
                .unwrap();

            if step % 5 == 4 {
                runtime.switch_branch(main.clone()).unwrap();
                runtime
                    .restore_branch_snapshot(main.clone(), &snapshot)
                    .unwrap();
                runtime.switch_branch(feature.clone()).unwrap();
            }
        }

        (
            runtime.observe().replay_for_branch(feature.id),
            runtime.observe().lineage_chain_for_node(sink),
            runtime.observe().explain(sink).unwrap(),
        )
    }

    let serial = run(StageExecutor::Serial);
    let parallel = run(StageExecutor::aggressive_parallel());

    assert!(
        replay_slices_equivalent(&serial.0, &parallel.0),
        "serial and parallel long sessions should preserve the same replay truth surface"
    );
    assert!(
        lineage_records_equivalent(&serial.1, &parallel.1),
        "serial and parallel long sessions should preserve the same lineage history"
    );
    assert_eq!(serial.2.state, parallel.2.state);
    assert_eq!(serial.2.output_change, parallel.2.output_change);
    assert_eq!(serial.2.upstream.len(), parallel.2.upstream.len());
}

#[test]
fn non_active_branch_inspection_after_heavy_foreground_churn_uses_stored_branch_state() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(SignalRuntimePolicy::forensic().with_history_limit(8));
    let source = runtime.graph_mut().node().output_identity().build();
    let filtered = runtime
        .graph_mut()
        .node()
        .aspect_filter(mask_b())
        .output_identity()
        .build();
    runtime
        .graph_mut()
        .add_dependency(filtered, source, ASPECT_B)
        .unwrap();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 10))
                        .with_output_identity("seed"),
                ))
            })?;
            tx.read(filtered, &|view| {
                let version = view.read_aspect_version(source, ASPECT_B)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("filtered-seed"),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature-inspect").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_B)?;
            tx.read(source, &|view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 20))
                        .with_output_identity("feature"),
                ))
            })?;
            tx.read(filtered, &|view| {
                let version = view.read_aspect_version(source, ASPECT_B)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("filtered-feature"),
                ))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    let analysis = runtime.create_branch("analysis-foreground").unwrap();
    runtime.switch_branch(analysis.clone()).unwrap();
    for step in 0..25_u64 {
        if step % 4 == 0 {
            let err = runtime.transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(source, ASPECT_B)?;
                tx.read(source, &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(100 + step, 1000 + step))
                            .with_output_identity(format!("bad-{step}")),
                    ))
                })?;
                Err(SignalError::invalid_input("synthetic analysis rollback"))
            });
            assert!(err.is_err());
        } else {
            runtime
                .transaction(&mut runtime_ctx, |tx| {
                    tx.mark_dirty(source, ASPECT_A)?;
                    tx.read(source, &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(50 + step, 10))
                                .with_output_identity(format!("analysis-{step}")),
                        ))
                    })?;
                    Ok(())
                })
                .unwrap();
        }
    }

    let feature_replay = runtime.observe().replay_for_branch(feature.id);
    let feature_lineage = runtime.observe().lineage_chain_for_node(filtered);
    let feature_head = runtime.observe().branch_head_snapshot_id(feature.id);
    assert_eq!(feature_head, Some(feature_snapshot.meta.snapshot_id));
    assert!(feature_replay
        .frames
        .iter()
        .all(|frame| frame.branch_id == feature.id));
    assert!(
        feature_replay
            .frames
            .iter()
            .any(|frame| frame.snapshot_id == Some(feature_snapshot.meta.snapshot_id)),
        "non-active branch inspection should still see the stored feature snapshot head"
    );
    assert!(
        feature_lineage
            .iter()
            .any(|record| record.event == LineageEvent::Replaced),
        "non-active branch inspection should read stored lineage instead of the active branch state"
    );
}

#[cfg(feature = "parallel")]
#[test]
fn dynamic_rewire_threshold_session_with_parallel_restore_preserves_subscriber_sets() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).with_kernel_defaults().build();
    runtime.set_runtime_policy(SignalRuntimePolicy::development().with_history_limit(8));
    let selector = runtime.graph_mut().node().output_identity().build();
    let left = runtime.graph_mut().node().output_identity().build();
    let right = runtime.graph_mut().node().output_identity().build();
    let left_gate = runtime
        .graph_mut()
        .node()
        .condition(EvaluationCondition::DeltaThreshold(2.0))
        .output_identity()
        .build();
    let right_gate = runtime
        .graph_mut()
        .node()
        .aspect_filter(mask_b())
        .output_identity()
        .build();
    let target = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(left_gate, left, ASPECT_A)
        .unwrap();
    runtime
        .graph_mut()
        .add_dependency(right_gate, right, ASPECT_B)
        .unwrap();
    let executor = StageExecutor::aggressive_parallel();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read_with_executor(
                selector,
                &|_node, view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("route-left"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                left,
                &|_node, view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 0))
                            .with_output_identity("left-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                right,
                &|_node, view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(0, 20))
                            .with_output_identity("right-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                left_gate,
                &|_node, view| {
                    let version = view.read_aspect_version(left, ASPECT_A)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("left-gate"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                right_gate,
                &|_node, view| {
                    let version = view.read_aspect_version(right, ASPECT_B)?;
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version)
                            .with_output_identity("right-gate"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                target,
                &|_node, view| {
                    let route = view.read_aspect_version(selector, ASPECT_A)?;
                    if route.get(ASPECT_A) % 2 == 1 {
                        let version = view.read_aspect_version(left_gate, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity("target-left"),
                        ))
                    } else {
                        let version = view.read_aspect_version(right_gate, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity("target-right"),
                        ))
                    }
                },
                executor,
            )?;
            Ok(())
        })
        .unwrap();

    let main = runtime.observe().current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-rewire-parallel").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();

    for step in 0..10_u64 {
        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.mark_dirty(selector, ASPECT_A)?;
                if step % 2 == 0 {
                    tx.mark_dirty(right, ASPECT_B)?;
                } else {
                    tx.mark_dirty(left, ASPECT_A)?;
                }
                tx.read_with_executor(
                    selector,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(2 + step, 0))
                                .with_output_identity(format!("route-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    left,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(10 + step, 0))
                                .with_output_identity(format!("left-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    right,
                    &|_node, view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(0, 20 + step))
                                .with_output_identity(format!("right-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    left_gate,
                    &|_node, view| {
                        let version = view.read_aspect_version(left, ASPECT_A)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("left-gate-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    right_gate,
                    &|_node, view| {
                        let version = view.read_aspect_version(right, ASPECT_B)?;
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version)
                                .with_output_identity(format!("right-gate-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    target,
                    &|_node, view| {
                        let route = view.read_aspect_version(selector, ASPECT_A)?;
                        if route.get(ASPECT_A) % 2 == 1 {
                            let version = view.read_aspect_version(left_gate, ASPECT_A)?;
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version)
                                    .with_output_identity(format!("target-left-{step}")),
                            ))
                        } else {
                            let version = view.read_aspect_version(right_gate, ASPECT_B)?;
                            Ok(view.finish(
                                NodeEvaluationResult::from_version(version)
                                    .with_output_identity(format!("target-right-{step}")),
                            ))
                        }
                    },
                    executor,
                )?;
                Ok(())
            })
            .unwrap();
    }
    let feature_snapshot = runtime.capture_branch_snapshot(feature.clone()).unwrap();

    runtime.switch_branch(main.clone()).unwrap();
    runtime
        .restore_branch_snapshot(main.clone(), &main_snapshot)
        .unwrap();
    assert!(runtime
        .graph()
        .depends_on(target, left_gate, ASPECT_A)
        .unwrap());
    assert!(!runtime
        .graph()
        .depends_on(target, right_gate, ASPECT_B)
        .unwrap());
    assert!(runtime
        .graph()
        .subscribers_of(left_gate)
        .unwrap()
        .contains(&target));

    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    let feature_depends_left = runtime
        .graph()
        .depends_on(target, left_gate, ASPECT_A)
        .unwrap();
    let feature_depends_right = runtime
        .graph()
        .depends_on(target, right_gate, ASPECT_B)
        .unwrap();
    assert_ne!(
        feature_depends_left, feature_depends_right,
        "parallel rewires should restore exactly one active upstream path for the target"
    );
    if feature_depends_left {
        assert!(runtime
            .graph()
            .subscribers_of(left_gate)
            .unwrap()
            .contains(&target));
        assert!(!runtime
            .graph()
            .subscribers_of(right_gate)
            .unwrap()
            .contains(&target));
    } else {
        assert!(runtime
            .graph()
            .subscribers_of(right_gate)
            .unwrap()
            .contains(&target));
        assert!(!runtime
            .graph()
            .subscribers_of(left_gate)
            .unwrap()
            .contains(&target));
    }
}