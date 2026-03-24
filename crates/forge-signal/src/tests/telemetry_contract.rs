use crate::facade::*;
#[cfg(feature = "parallel")]
use crate::logic::planner::model::ParallelAdmissionReason;
use crate::tests::support::*;

#[cfg(feature = "parallel")]
#[test]
fn transaction_full_parallel_executor_usage_is_recorded_across_honest_apply_modes() {
    let graph = SignalGraph::new();
    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let nodes = (0..16)
        .map(|_| runtime.graph_mut().node().build())
        .collect::<Vec<_>>();
    let mut ctx = ();

    let mut tx = runtime.begin(&mut ctx);
    for &node in &nodes {
        tx.mark_dirty(node, ASPECT_A).unwrap();
    }
    let report = tx
        .evaluate_dirty_with_executor(
            &|view| Ok(view.finish(NodeEvaluationResult::from_version(version_ab(1, 0)))),
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();
    tx.commit().unwrap();

    let metrics = runtime.observe().metrics();
    assert!(report.stages.iter().all(|stage| matches!(
        stage.outcome,
        StageExecutionOutcome::CompletedSerial | StageExecutionOutcome::CompletedParallel
    )));
    assert!(report.stages.iter().all(|stage| {
        matches!(
            stage.parallel_admission_reason,
            Some(ParallelAdmissionReason::FullParallelUnsupportedByMutableEngine)
                | Some(ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent)
        )
    }));
    assert_eq!(metrics.execution.serial_executor_usage_count, 0);
    assert_eq!(metrics.execution.parallel_executor_usage_count, 1);
}

#[cfg(feature = "parallel")]
#[test]
fn full_parallel_honest_serial_apply_emits_group_local_packet_and_reduction_counters() {
    use std::num::NonZeroUsize;

    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::operational().with_parallel_admission(
        ParallelAdmissionPolicy {
            operational_min_parallel_tasks: 1,
            development_min_parallel_tasks: 1,
            forensic_min_parallel_tasks: 1,
            full_parallel_min_tasks: 1,
        },
    ));
    let requested: Vec<_> = (0..4).map(|_| graph.node().build()).collect();

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| Ok(ctx.finish(version_ab(1, 0))))
        .unwrap();

    for &node in &requested {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    }

    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let before = graph.observe().metrics().execution;
    graph
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &|ctx| Ok(ctx.finish(version_ab(2, 0))),
            StageExecutor::full_parallel(1).with_parallel_policy(
                ParallelExecutionPolicy::new(NonZeroUsize::new(1).unwrap())
                    .with_apply_group_min_width(2)
                    .with_max_concurrent_apply_groups(2),
            ),
        )
        .unwrap();

    let metrics = graph.observe().metrics();
    let execution = metrics.execution;
    assert_eq!(
        execution.group_local_packet_breadth - before.group_local_packet_breadth,
        4
    );
    assert_eq!(
        execution.reduction_packet_breadth - before.reduction_packet_breadth,
        2
    );
    assert_eq!(
        execution.reduction_group_count - before.reduction_group_count,
        2
    );
    assert!(
        execution.shared_surface_publication_breadth - before.shared_surface_publication_breadth
            >= 4,
        "reducer publication breadth should at least cover one semantic publication per task"
    );
}

#[cfg(feature = "parallel")]
#[test]
fn serial_staged_apply_telemetry_tracks_executed_batch_width_not_planned_groups() {
    let mut graph = SignalGraph::new();
    let requested: Vec<_> = (0..4).map(|_| graph.node().build()).collect();

    let bootstrap = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::ForceOnDemand)
        .unwrap();
    graph
        .execute_prepared_plan(&bootstrap, &(), &|ctx| Ok(ctx.finish(version_ab(1, 0))))
        .unwrap();

    for &node in &requested {
        mark_dirty(&mut graph, node, ASPECT_A).unwrap();
    }

    let plan = graph
        .build_evaluation_plan(&requested, EvaluationRequestMode::Default)
        .unwrap();
    let before = graph.observe().metrics().execution;
    let report = graph
        .execute_prepared_plan(&plan, &(), &|ctx| Ok(ctx.finish(version_ab(2, 0))))
        .unwrap();

    let serial_stages = report
        .stages
        .iter()
        .filter(|stage| matches!(stage.outcome, StageExecutionOutcome::CompletedSerial))
        .collect::<Vec<_>>();
    assert!(!serial_stages.is_empty());
    assert!(serial_stages
        .iter()
        .all(|stage| stage.apply_group_count == 1));

    let executed_serial_width = serial_stages
        .iter()
        .map(|stage| u64::from(stage.serial_apply_task_count))
        .sum::<u64>();
    let max_serial_batch_width = serial_stages
        .iter()
        .map(|stage| u64::from(stage.serial_apply_task_count))
        .max()
        .unwrap_or(0);

    let after = graph.observe().metrics().execution;
    assert_eq!(
        after.apply_group_width_total - before.apply_group_width_total,
        executed_serial_width
    );
    assert_eq!(
        after.apply_group_disjoint_count - before.apply_group_disjoint_count,
        0
    );
    assert!(
        after.max_apply_group_width >= max_serial_batch_width,
        "global max apply-group width must cover the executed serial batch width"
    );
}

#[test]
fn direct_whole_partition_changes_are_counted_as_partition_matches() {
    let mut graph = SignalGraph::new();
    let source = graph.node().partitioned_output().build();
    let whole = graph.node().build();
    let detail = graph.node().build();
    graph
        .append_partition_dependency(whole, source, ASPECT_A, "wing")
        .unwrap();
    graph
        .append_partition_detail_dependency(detail, source, ASPECT_A, "wing", "rib-12")
        .unwrap();

    mark_dirty_with_regions(
        &mut graph,
        source,
        ASPECT_A,
        &[crate::data::output::ChangedRegion::new("wing")],
    )
    .unwrap();

    let metrics = graph.observe().metrics();
    assert_eq!(metrics.invalidation.partition_match_dirty_count, 1);
    assert_eq!(metrics.invalidation.detail_match_dirty_count, 1);
}

#[test]
fn frontier_cycle_check_counters_reflect_planned_roots_and_reachability() {
    let mut graph = SignalGraph::new();
    graph.set_runtime_policy(SignalRuntimePolicy::development());
    let source = graph.node().build();
    let left = graph.node().build();
    let right = graph.node().build();
    let downstream = graph.node().build();

    graph.append_dependency(left, source, ASPECT_A).unwrap();
    graph.append_dependency(right, source, ASPECT_A).unwrap();
    graph.append_dependency(downstream, left, ASPECT_A).unwrap();
    graph
        .append_dependency(downstream, right, ASPECT_A)
        .unwrap();

    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let frontier = graph
        .observe()
        .latest_frontier_execution_summary()
        .expect("frontier execution summary should be retained");
    let metrics = graph.observe().metrics();

    assert_eq!(frontier.counters.frontier_cycle_check_candidate_count, 2);
    assert_eq!(
        metrics.invalidation.frontier_cycle_check_candidate_count,
        frontier.counters.frontier_cycle_check_candidate_count
    );
    assert!(
        frontier.counters.frontier_cycle_check_visited_count
            >= frontier.counters.frontier_cycle_check_candidate_count,
        "cycle preflight visited breadth should cover at least the planned roots"
    );
    assert_eq!(
        metrics.invalidation.frontier_cycle_check_visited_count,
        frontier.counters.frontier_cycle_check_visited_count
    );
}

#[test]
fn runtime_metrics_surface_typed_reuse_family_counters() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let compute_calls = std::sync::atomic::AtomicU32::new(0);
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_cross_identity_persistent_matching()
                .with_partial_artifact_splicing(),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                compute_calls.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("typed-reuse")
                        .with_output_change(OutputChange::Refreshed),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias");
    let splice = projection.keyed("splice");
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            source.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-telemetry")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            splice.evaluate_partial_splice(
                tx,
                "shape-v1",
                [PartitionSubscription::whole_partition("wing")],
            )
        })
        .unwrap();

    let metrics = runtime.observe().metrics();
    assert_eq!(metrics.evaluation.memoization_hits, 1);
    assert_eq!(metrics.evaluation.cross_identity_reuse_count, 1);
    assert_eq!(metrics.evaluation.partial_artifact_splice_count, 1);
}

#[test]
fn typed_rejection_counters_match_runtime_reuse_failures() {
    let mut runtime = SignalRuntime::builder(SignalGraph::new())
        .with_kernel_defaults()
        .build();
    let projection = runtime
        .define_computation(ComputationSpec {
            family: "restricted-projection".into(),
            contract: NodeContract::reads([ASPECT_A])
                .with_produces([ASPECT_B])
                .with_partition_scope(PartitionSubscription::whole_partition("wing"))
                .with_reuse_contract(NodeReuseContract {
                    equivalence: ArtifactEquivalenceContract {
                        required_boundaries: vec![
                            ArtifactSemanticBoundary::TopologyRegime,
                            ArtifactSemanticBoundary::ToleranceRegime,
                            ArtifactSemanticBoundary::SemanticRegionIdentity,
                            ArtifactSemanticBoundary::ArtifactFamilyBasis,
                            ArtifactSemanticBoundary::StructuralDependencyBasis,
                            ArtifactSemanticBoundary::PartitionRegionBasis,
                        ],
                        supported_strategies: vec![ReuseStrategy::MemoizedArtifactReuse],
                        allows_snapshot_restore_reuse: false,
                        allows_authority_reconciliation_reuse: false,
                    },
                    retain_certification: true,
                }),
            tier: (),
            comparator: VersionComparatorPolicy::OutputIdentity,
            evaluator: |view: &mut EvaluationContext<'_, ()>| {
                Ok(view.finish(
                    NodeEvaluationResult::from_version(version_ab(1, 0))
                        .with_output_identity("restricted-artifact")
                        .with_output_change(OutputChange::Refreshed)
                        .with_changed_region(ChangedRegion::new("wing")),
                ))
            },
        })
        .unwrap();
    let source = projection.keyed("source");
    let alias = projection.keyed("alias");
    let wing = projection.keyed("wing");
    let wing_node = wing.node(&mut runtime);
    let mut runtime_ctx = ();

    runtime
        .transaction(&mut runtime_ctx, |tx| {
            source.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_memoized(tx, "shape-v1")
        })
        .unwrap();
    mark_dirty(runtime.graph_mut(), wing_node, ASPECT_A).unwrap();

    let cross_identity_err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            alias.evaluate_cross_identity(tx, "source", "shape-v1", "mesh-reject")
        })
        .expect_err("cross-identity should be rejected by the reuse contract");
    let partial_splice_err = runtime
        .transaction(&mut runtime_ctx, |tx| {
            wing.evaluate_partial_splice(
                tx,
                "shape-v1",
                [PartitionSubscription::whole_partition("wing")],
            )
        })
        .expect_err("partial splice should be rejected by the reuse contract");

    assert!(cross_identity_err
        .to_string()
        .contains("reuse certification failed"));
    assert!(partial_splice_err
        .to_string()
        .contains("reuse certification failed"));

    let metrics = runtime.observe().metrics();
    assert_eq!(metrics.evaluation.reuse_rejected_contract_strategy_count, 2);
    assert_eq!(metrics.evaluation.cross_identity_reuse_count, 0);
    assert_eq!(metrics.evaluation.partial_artifact_splice_count, 0);
}
