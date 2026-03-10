use std::sync::atomic::{AtomicU32, Ordering};

use crate::facade::*;
use crate::tests::support::*;

#[test]
fn branch_debug_session_mixed_churn_stays_forensically_coherent() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(SignalRuntimePolicy::development().with_history_limit(6));
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let family = runtime.register_computation_family("workflow-projection");
    let keyed = runtime.keyed_node(&family, "wing-left");
    let keyed_computation =
        KeyedComputation::new(family.clone(), "wing-left").with_memo_key("shape-v1");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-source-v1"),
                ))
            })?;
            tx.read(dependent, &|_node, view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("main-dependent-v1"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &keyed_computation, &|_id, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("keyed-main-v1")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-debug").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-source-v2"),
                ))
            })?;
            tx.read(dependent, &|_node, view| {
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
            tx.read(source, &|_node, view| {
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
    let main_replay = runtime.replay_for_branch(main.id);
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
    let feature_replay = runtime.replay_for_branch(feature.id);
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
    let analysis_replay = runtime.replay_for_branch(analysis.id);
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
        runtime.branch_head_snapshot_id(main.id),
        Some(main_snapshot.meta.snapshot_id)
    );
    assert_eq!(
        runtime.branch_head_snapshot_id(feature.id),
        Some(feature_snapshot.meta.snapshot_id)
    );
    assert_eq!(
        runtime.branch_head_snapshot_id(analysis.id),
        Some(analysis_snapshot.meta.snapshot_id),
        "analysis branch should stay pinned to its own restored snapshot"
    );
}

#[test]
fn undo_redo_style_session_with_failures_and_memo_reuse_preserves_branch_local_truth() {
    let policy = SignalRuntimePolicy::development().with_history_limit(4);
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(policy);
    let source = runtime.graph_mut().node().output_identity().build();
    let family = runtime.register_computation_family("undo-redo-session");
    let keyed = runtime.keyed_node(&family, "bulkhead");
    let computation = KeyedComputation::new(family.clone(), "bulkhead").with_memo_key("shape-v1");
    let compute_calls = AtomicU32::new(0);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("main-v1"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &computation, &|_id, view| {
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

    let main = runtime.current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-undo").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("feature-v2"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &computation, &|_id, view| {
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
            tx.read(source, &|_node, view| {
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
                tx.evaluate_keyed(keyed, &computation, &|_id, view| {
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
    let feature_lineage = runtime.lineage_chain_for_node(source);
    assert!(
        feature_lineage
            .iter()
            .any(|record| record.event == LineageEvent::Restored),
        "feature workflow should preserve restore lineage under undo/redo churn"
    );
    assert!(
        runtime.recent_execution_history_diagnostics().len() <= policy.history_limit,
        "history must stay bounded under long undo/redo churn"
    );
    assert!(
        runtime.graph().replay_events().len() <= policy.history_limit.max(1) * 32,
        "replay must stay bounded under long undo/redo churn"
    );
}

#[test]
fn posthoc_forensics_after_long_session_answers_branch_and_artifact_questions() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new()).build();
    runtime.set_runtime_policy(SignalRuntimePolicy::forensic().with_history_limit(8));
    let source = runtime.graph_mut().node().output_identity().build();
    let dependent = runtime.graph_mut().node().output_identity().build();
    runtime
        .graph_mut()
        .add_dependency(dependent, source, ASPECT_A)
        .unwrap();
    let family = runtime.register_computation_family("posthoc-session");
    let keyed = runtime.keyed_node(&family, "skin-panel");
    let computation = KeyedComputation::new(family.clone(), "skin-panel").with_memo_key("shape-a");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("source-main"),
                ))
            })?;
            tx.read(dependent, &|_node, view| {
                let version = view.read_aspect_version(source, ASPECT_A)?;
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version)
                        .with_output_identity("dependent-main"),
                ))
            })?;
            tx.evaluate_keyed(keyed, &computation, &|_id, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("skin-a")
                        .with_output_change(OutputChange::Refreshed),
                ))
            })?;
            Ok(())
        })
        .unwrap();

    let main = runtime.current_branch();
    let main_snapshot = runtime.capture_snapshot();
    let feature = runtime.create_branch("feature-posthoc").unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.mark_dirty(source, ASPECT_A)?;
            tx.read(source, &|_node, view| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(2, 0))
                        .with_output_identity("source-feature"),
                ))
            })?;
            tx.read(dependent, &|_node, view| {
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
        tx.read(source, &|_node, view| {
            Ok(view.finish(
                NodeEvaluationResult::from_version(version_ab(99, 0))
                    .with_output_identity("analysis-bad"),
            ))
        })?;
        Err(SignalError::invalid_input("analysis failure"))
    });
    assert!(err.is_err());

    runtime.switch_branch(main.clone()).unwrap();

    let main_head = runtime.branch_head_snapshot_id(main.id);
    let feature_head = runtime.branch_head_snapshot_id(feature.id);
    let analysis_head = runtime.branch_head_snapshot_id(analysis.id);
    let main_replay = runtime.replay_for_branch(main.id);
    let feature_replay = runtime.replay_for_branch(feature.id);
    let analysis_replay = runtime.replay_for_branch(analysis.id);
    let artifact = runtime
        .switch_branch(feature.clone())
        .map(|_| runtime.current_lineage_artifact(source).unwrap())
        .unwrap();
    let artifact_replay = runtime.replay_for_artifact(artifact);
    let artifact_lineage = runtime.lineage_chain_for_artifact(artifact);
    let around_feature_snapshot = runtime.replay_around_snapshot(feature_snapshot.meta.snapshot_id);

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
