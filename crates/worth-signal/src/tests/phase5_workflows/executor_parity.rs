#[cfg(feature = "parallel")]
use crate::facade::{
    lineage_records_equivalent, replay_slices_equivalent, EvaluationCondition, LineageRecord,
    NodeEvaluationResult, NodeExplanation, ReplaySlice, SignalGraph, SignalRuntime,
    SignalRuntimePolicy, StageExecutor,
};
#[cfg(feature = "parallel")]
use crate::tests::support::{mask_b, version_ab, DependencyBatchBuilder, ASPECT_A, ASPECT_B};

#[cfg(feature = "parallel")]
#[test]
fn long_session_replay_and_lineage_stay_equivalent_between_serial_and_parallel_executors() {
    fn run(executor: StageExecutor) -> (ReplaySlice, Vec<LineageRecord>, NodeExplanation) {
        let mut runtime = SignalRuntime::builder(SignalGraph::new())
            .with_kernel_defaults()
            .build();
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
        let mut dependencies = DependencyBatchBuilder::new(runtime.graph_mut());
        dependencies
            .append_dependency(a_gate, source, ASPECT_A)
            .unwrap()
            .append_dependency(b_gate, source, ASPECT_B)
            .unwrap()
            .append_dependency(sink, a_gate, ASPECT_A)
            .unwrap()
            .append_dependency(sink, b_gate, ASPECT_B)
            .unwrap();
        dependencies.commit().unwrap();
        let mut runtime_ctx = ();

        runtime
            .transaction(&mut runtime_ctx, |tx| {
                tx.read_with_executor(
                    source,
                    &|view| {
                        Ok(view.finish(
                            NodeEvaluationResult::from_version(version_ab(1, 10))
                                .with_output_identity("seed-source"),
                        ))
                    },
                    executor,
                )?;
                tx.read_with_executor(
                    a_gate,
                    &|view| {
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
                    &|view| {
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
                    &|view| {
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
        let snapshot = runtime
            .capture_snapshot()
            .expect("snapshot capture should succeed without managed queue bindings");
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
                        &|view| {
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
                        &|view| {
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
                        &|view| {
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
                        &|view| {
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
            runtime
                .observe()
                .lineage_chain_for_node(sink)
                .to_owned_records(),
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
