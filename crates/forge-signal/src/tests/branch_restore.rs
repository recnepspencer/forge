use std::sync::atomic::{AtomicU32, Ordering};

use crate::data::output::MemoizedResultOrigin;
use crate::facade::*;
use crate::tests::support::*;

#[test]
fn restore_branch_snapshot_uses_captured_branch_semantic_state_not_active_branch_config() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature").unwrap();
    let family = define_keyed_computation(&mut runtime, "shared-family", ());
    let keyed = family.keyed("shared-key");
    let computation = keyed.memoized("shared");

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_node = keyed.node(&mut runtime);
    let feature_compute_calls = AtomicU32::new(0);
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(feature_node, &computation, &|view| {
                feature_compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0))))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_counts = runtime.config().test_registry_counts();
    let feature_snapshot = runtime.capture_snapshot();
    let feature_record = feature_snapshot
        .reconstructability
        .clone()
        .expect("runtime snapshot should carry reconstructability record");
    assert_eq!(feature_record.authority_branch_id, feature.id);
    assert_eq!(
        feature_record.authority_snapshot_id,
        Some(feature_snapshot.meta.snapshot_id)
    );
    assert_eq!(feature_record.replay_head, feature_snapshot.meta.replay_head);
    assert_eq!(
        feature_record.checkpoint.checkpoint_size,
        feature_snapshot
            .runtime_telemetry
            .as_ref()
            .map(|telemetry| telemetry.checkpoint.checkpoint_size)
            .unwrap_or(0)
    );
    assert_eq!(feature_record.checkpoint.journal_replay_span, 0);
    assert!(feature_record.journal.is_none());

    runtime.switch_branch(main).unwrap();
    let main_node = keyed.node(&mut runtime);
    let main_compute_calls = AtomicU32::new(0);
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(main_node, &computation, &|view| {
                main_compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(9, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    mark_dirty(runtime.graph_mut(), feature_node, ASPECT_A).unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(feature_node, &computation, &|view| {
                feature_compute_calls.fetch_add(1, Ordering::Relaxed);
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(99, 0))))
            })?;
            Ok(())
        })
        .unwrap();

    assert_eq!(runtime.config().test_registry_counts(), feature_counts);
    assert_eq!(feature_compute_calls.load(Ordering::Relaxed), 1);
    assert_eq!(main_compute_calls.load(Ordering::Relaxed), 1);
    let explanation = runtime.observe().explain(feature_node).unwrap();
    assert_eq!(
        explanation.reuse_basis,
        Some(ReuseBasis::Reused {
            source: ReuseSource::MemoizedArtifact,
            crossing: ReuseCrossing::None,
        })
    );
    assert_eq!(
        explanation.memoized_origin,
        Some(MemoizedResultOrigin::MemoizedFromCache)
    );
}

#[test]
fn restore_branch_snapshot_keeps_sibling_branch_keyed_bindings_isolated() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let mut runtime_ctx = ();
    let main = runtime.observe().current_branch();
    let feature = runtime.create_branch("feature").unwrap();
    let sibling = runtime.create_branch("sibling").unwrap();
    let family = define_keyed_computation(&mut runtime, "shared-family", ());
    let keyed = family.keyed("shared-key");
    let computation = keyed.memoized("shared");

    runtime.switch_branch(feature.clone()).unwrap();
    let feature_node = keyed.node(&mut runtime);
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(feature_node, &computation, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(3, 0))))
            })?;
            Ok(())
        })
        .unwrap();
    let feature_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(sibling.clone()).unwrap();
    let sibling_node = keyed.node(&mut runtime);
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.evaluate_keyed(sibling_node, &computation, &|view| {
                Ok(view.finish(NodeEvaluationResult::from_version(version_ab(8, 0))))
            })?;
            Ok(())
        })
        .unwrap();
    let sibling_snapshot = runtime.capture_snapshot();

    runtime.switch_branch(main).unwrap();
    runtime
        .restore_branch_snapshot(feature.clone(), &feature_snapshot)
        .unwrap();
    runtime.switch_branch(feature.clone()).unwrap();
    assert_eq!(keyed.node(&mut runtime), feature_node);
    assert_eq!(
        runtime
            .graph()
            .get_entry(feature_node)
            .unwrap()
            .get_aspect_version(),
        version_ab(3, 0),
        "feature restore must retain its own keyed binding and semantic state"
    );

    runtime
        .restore_branch_snapshot(sibling.clone(), &sibling_snapshot)
        .unwrap();
    runtime.switch_branch(sibling).unwrap();
    assert_eq!(keyed.node(&mut runtime), sibling_node);
    assert_eq!(
        runtime
            .graph()
            .get_entry(sibling_node)
            .unwrap()
            .get_aspect_version(),
        version_ab(8, 0),
        "sibling restore must keep its own keyed binding rather than reusing feature state"
    );
}
