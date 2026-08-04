use crate::facade::{
    lineage_records_equivalent, replay_slices_equivalent, LineageRecord, NodeEvaluationResult,
    ReplaySlice, SignalGraph, SignalRuntime, SignalRuntimePolicy, SnapshotRestoreLineageMode,
};
use crate::tests::support::{version_ab, ASPECT_A};

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
        let main_snapshot = runtime
            .capture_snapshot()
            .expect("snapshot capture should succeed without managed queue bindings");

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
