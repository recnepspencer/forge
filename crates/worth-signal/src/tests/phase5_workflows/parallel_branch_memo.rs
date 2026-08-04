#[cfg(feature = "parallel")]
use crate::facade::{
    mark_dirty, ArtifactTransitionKind, EvaluationCondition, LineageRecordKind,
    NodeEvaluationResult, OutputChange, SignalError, SignalGraph, SignalRuntime,
    SignalRuntimePolicy, StageExecutor,
};
#[cfg(feature = "parallel")]
use crate::tests::support::{
    define_keyed_computation, mask_b, version_ab, DependencyBatchBuilder, ASPECT_A, ASPECT_B,
};
#[cfg(feature = "parallel")]
use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "parallel")]
#[test]
fn parallel_branch_memo_rollback_session_preserves_branch_local_replay_and_cache_truth() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
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
    let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
    dependencies
        .append_dependency(gated, source, ASPECT_A)
        .unwrap()
        .append_dependency(filtered, source, ASPECT_B)
        .unwrap()
        .append_dependency(fused, gated, ASPECT_A)
        .unwrap()
        .append_dependency(fused, filtered, ASPECT_B)
        .unwrap();
    dependencies.commit().unwrap();
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
                &|view| {
                    Ok(view.finish(
                        NodeEvaluationResult::from_version(version_ab(10, 100))
                            .with_output_identity("seed-ab"),
                    ))
                },
                executor,
            )?;
            tx.read_with_executor(
                gated,
                &|view| {
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
                &|view| {
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
                &|view| {
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
    let main_snapshot = runtime
        .capture_snapshot()
        .expect("snapshot capture should succeed without managed queue bindings");
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
                    &|view| {
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
                    &|view| {
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
                    &|view| {
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
                    &|view| {
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
                    &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(500 + step, 900))
                                .with_output_identity(format!("bad-{step}")),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    fused,
                    &|view| {
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
                    record.kind,
                    LineageRecordKind::SnapshotRestore { .. }
                        | LineageRecordKind::ArtifactTransition {
                            transition: ArtifactTransitionKind::Replaced
                                | ArtifactTransitionKind::Refreshed { .. },
                            ..
                        }
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
