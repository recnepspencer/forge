#[cfg(feature = "parallel")]
use crate::facade::{
    EvaluationCondition, NodeEvaluationResult, SignalGraph, SignalRuntime, SignalRuntimePolicy,
    StageExecutor,
};
#[cfg(feature = "parallel")]
use crate::tests::support::{mask_b, version_ab, DependencyBatchBuilder, ASPECT_A, ASPECT_B};

#[cfg(feature = "parallel")]
#[test]
fn dynamic_rewire_threshold_session_with_parallel_restore_preserves_subscriber_sets() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
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
    let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
    dependencies
        .append_dependency(left_gate, left, ASPECT_A)
        .unwrap()
        .append_dependency(right_gate, right, ASPECT_B)
        .unwrap();
    dependencies.commit().unwrap();
    let executor = StageExecutor::aggressive_parallel();
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            tx.read_with_executor(
                selector,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(1, 0))
                            .with_output_identity("route-left"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                left,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 0))
                            .with_output_identity("left-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                right,
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(0, 20))
                            .with_output_identity("right-seed"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                left_gate,
                &|view| {
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
                &|view| {
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
                &|view| {
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
    let main_snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
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
                    &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(2 + step, 0))
                                .with_output_identity(format!("route-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    left,
                    &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(10 + step, 0))
                                .with_output_identity(format!("left-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    right,
                    &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(0, 20 + step))
                                .with_output_identity(format!("right-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    left_gate,
                    &|view| {
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
                    &|view| {
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
                    &|view| {
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
