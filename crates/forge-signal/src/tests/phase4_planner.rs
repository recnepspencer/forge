use crate::facade::*;
use crate::tests::support::*;
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Tier {
    A,
}

#[test]
fn build_evaluation_plan_orders_chain_by_stage_depth() {
    let mut graph = SignalGraph::new();
    let a = graph.node().build();
    let b = graph.node().build();
    let c = graph.node().build();
    graph.append_dependency(b, a, ASPECT_A).unwrap();
    graph.append_dependency(c, b, ASPECT_A).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, a, &mut compute).unwrap();
    evaluate(&mut graph, b, &mut compute).unwrap();
    evaluate(&mut graph, c, &mut compute).unwrap();
    mark_dirty(&mut graph, a, ASPECT_A).unwrap();

    let plan = graph
        .build_evaluation_plan(&[c], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.stage_count, 3);
    assert_eq!(plan.stages[0].tasks[0].node, a);
    assert_eq!(plan.stages[1].tasks[0].node, b);
    assert_eq!(plan.stages[2].tasks[0].node, c);
    assert!(plan.stages[2].tasks[0].direct_request);
}

#[test]
fn build_evaluation_plan_omits_clean_target() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut compute).unwrap();

    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.task_count, 0);
    assert!(plan.stages.is_empty());
}

#[test]
fn build_evaluation_plan_prunes_dirty_target_when_contract_reads_do_not_intersect() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().reads_aspects(mask_a()).build();
    graph.append_dependency(dependent, source, ASPECT_B).unwrap();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, source, &mut compute).unwrap();
    evaluate(&mut graph, dependent, &mut compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_B).unwrap();

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.task_count, 0);
    assert!(plan.stages.is_empty());
}

#[test]
fn build_evaluation_plan_rejects_missing_relational_snapshot_context() {
    let mut graph = SignalGraph::new();
    let node = graph
        .node()
        .requires_context(ContextRequirement::RelationalSnapshot)
        .build();

    let err = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap_err();

    assert!(matches!(
        err,
        SignalError::ContractViolation {
            node: contract_node,
            requirement: ContextRequirement::RelationalSnapshot,
        } if contract_node == node
    ));
}

#[test]
fn force_on_demand_plans_and_executes_clean_target() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let mut bootstrap = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    evaluate(&mut graph, node, &mut bootstrap).unwrap();

    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::ForceOnDemand)
        .unwrap();

    assert_eq!(plan.summary.task_count, 1);
    assert!(matches!(
        plan.stages[0].tasks[0].reason,
        TaskReason::ConditionForced
    ));

    let calls = AtomicU32::new(0);
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_view| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(version_ab(2, 0)))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 1);
    assert_eq!(report.tasks_executed, 1, "{report:?}");
}

#[test]
fn execute_plan_returns_execution_report_and_updates_trace_record_id() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();
    mark_dirty(&mut graph, source, ASPECT_A).unwrap();

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &(), &|view| {
            let node = view.node();
            if node == source {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    source_v2(node, view.graph())?,
                ))
            } else {
                Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(
                    dependent_compute(node, view.graph())?,
                ))
            }
        })
        .unwrap();

    assert_eq!(report.stage_count, 2);
    assert_eq!(report.task_count, 2);
    assert!(report.tasks_executed >= 2);
    assert!(graph
        .get_entry(dependent)
        .unwrap()
        .get_trace_summary()
        .unwrap()
        .execution_record_id
        .is_some());
    assert_eq!(
        graph.observe().explain(dependent).unwrap().execution_record_id,
        graph
            .get_entry(dependent)
            .unwrap()
            .get_trace_summary()
            .unwrap()
            .execution_record_id
    );
}

#[test]
fn runtime_plan_keeps_requested_maybe_stale_validation_task() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().with_tiers::<Tier>().build();
    runtime.set_node_tier(dependent, Tier::A);
    runtime.set_tier_policy(
        TierPolicy::new(
            Tier::A,
            DependencyMode::AutoDiscovered,
            DirtyPropagation::Immediate,
            EvaluationTrigger::LazyPull,
        )
        .with_default_comparator(VersionComparatorPolicy::Tolerance { epsilon: 2 }),
    );

    let mut source_v10 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 10));
    let mut source_v12 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 12));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(0, 100));
    evaluate(runtime.graph_mut(), source, &mut source_v10).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_compute).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_B).unwrap();
    evaluate(runtime.graph_mut(), source, &mut source_v12).unwrap();

    let plan = runtime
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.task_count, 1);
    assert!(matches!(
        plan.stages[0].tasks[0].reason,
        TaskReason::RequestedTarget
    ));
}

#[test]
fn public_evaluate_routes_through_planner_and_records_execution_metadata() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();

    let mut compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(3, 0));
    evaluate(&mut graph, node, &mut compute).unwrap();

    let trace = graph
        .get_entry(node)
        .unwrap()
        .get_trace_summary()
        .cloned()
        .unwrap();
    assert!(trace.execution_record_id.is_some());

    let metrics = graph.observe().metrics();
    assert!(metrics.planner.plans_built >= 1);
    assert!(metrics.planner.tasks_scheduled >= 1);
    assert!(metrics.execution.serial_executor_usage_count >= 1);
}

#[test]
fn runtime_execute_plan_with_executor_serial_matches_default_path() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut runtime = SignalRuntime::builder(graph).with_kernel_defaults().build();
    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let source_v2 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(2, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));

    evaluate(runtime.graph_mut(), source, &mut source_v1).unwrap();
    evaluate(runtime.graph_mut(), dependent, &mut dependent_compute).unwrap();
    mark_dirty(runtime.graph_mut(), source, ASPECT_A).unwrap();

    let plan = runtime
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = runtime
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &|view| {
                let node = view.node();
                if node == source {
                    Ok(view.finish(source_v2(node, view.graph())?))
                } else {
                    Ok(view.finish(dependent_compute(node, view.graph())?))
                }
            },
            StageExecutor::Serial,
        )
        .unwrap();

    assert_eq!(report.stage_count, 2);
    assert_eq!(
        runtime.graph().get_state(dependent).unwrap(),
        NodeState::Clean
    );
}

#[test]
fn execution_report_marks_requested_maybe_stale_validation_as_validated_clean() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_compute = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_compute).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_compute).unwrap();

    {
        let entry = graph.get_entry_mut(dependent).unwrap();
        entry.set_state(NodeState::MaybeStale);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
    }

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::validated_clean())
        })
        .unwrap();

    assert_eq!(report.task_count, 1);
    assert_eq!(report.tasks_validated_clean, 1, "{report:?}");
    assert_eq!(report.tasks_pruned, 1, "{report:?}");
    assert!(matches!(
        report.stages[0].task_records[0].outcome,
        TaskExecutionOutcome::ValidatedClean
    ));
}

#[test]
fn maybe_stale_requested_target_validates_clean_without_running_compute() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut source_v2_same_aspect_a = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 1));
    let mut dependent_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_v1).unwrap();

    mark_dirty(&mut graph, source, ASPECT_B).unwrap();
    evaluate(&mut graph, source, &mut source_v2_same_aspect_a).unwrap();
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::MaybeStale);

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();

    let calls = AtomicU32::new(0);
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_view| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(version_ab(99, 0)))
        })
        .unwrap();

    assert_eq!(
        calls.load(Ordering::Relaxed),
        0,
        "MaybeStale validation should skip user compute when cached inputs are still meaningful"
    );
    assert_eq!(report.tasks_validated_clean, 1, "{report:?}");
    assert_eq!(graph.get_state(dependent).unwrap(), NodeState::Clean);
}

#[test]
fn maybe_stale_validation_prunes_retired_runtime_dependencies_before_recapture() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let retired = graph.node().build();
    let dependent = graph.node().build();
    graph.append_dependency(dependent, source, ASPECT_A).unwrap();

    let mut source_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(1, 0));
    let mut dependent_v1 = |_id: NodeId, _graph: &SignalGraph| Ok(version_ab(10, 0));
    evaluate(&mut graph, source, &mut source_v1).unwrap();
    evaluate(&mut graph, dependent, &mut dependent_v1).unwrap();

    graph.unregister_node(retired).unwrap();
    graph
        .inject_retired_dependency_for_test(dependent, retired, ASPECT_A)
        .unwrap();
    {
        let entry = graph.get_entry_mut(dependent).unwrap();
        entry.set_state(NodeState::MaybeStale);
        entry.set_dirty_aspects(AspectMask::from_aspect(ASPECT_A));
    }

    let plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();

    let calls = AtomicU32::new(0);
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_view| {
            calls.fetch_add(1, Ordering::Relaxed);
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::from_result(version_ab(99, 0)))
        })
        .unwrap();

    assert_eq!(calls.load(Ordering::Relaxed), 0);
    assert_eq!(report.tasks_validated_clean, 1, "{report:?}");
    assert!(
        graph.dependencies_of(dependent).unwrap().is_empty(),
        "validated-clean recapture should prune retired runtime edges before persisting dependencies"
    );
}

#[test]
fn execution_report_marks_on_demand_deferral_explicitly() {
    let mut graph = SignalGraph::new();
    let node = graph.node().on_demand().build();

    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();
    let report = graph
        .execute_prepared_plan(&plan, &(), &|_ctx| {
            Ok::<EvaluationOutput, SignalError>(EvaluationOutput::deferred_by_condition())
        })
        .unwrap();

    assert_eq!(report.task_count, 1);
    assert_eq!(report.tasks_deferred_by_condition, 1);
    assert!(matches!(
        report.stages[0].task_records[0].outcome,
        TaskExecutionOutcome::ConditionDeferred
    ));
    assert_eq!(graph.get_state(node).unwrap(), NodeState::MaybeStale);
}

#[test]
fn prepared_plan_captures_dependencies_without_manual_graph_wiring() {
    let mut graph = SignalGraph::new();
    let source = graph.node().build();
    let dependent = graph.node().build();

    let source_compute = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));
    let dependent_compute = |ctx: &mut EvaluationContext<'_, ()>| {
        let version = ctx.read_aspect_version(source, ASPECT_A)?;
        Ok(ctx.finish(NodeEvaluationResult::from_version(version)))
    };

    let source_plan = graph
        .build_evaluation_plan(&[source], EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan(&source_plan, &(), &source_compute)
        .unwrap();

    let dependent_plan = graph
        .build_evaluation_plan(&[dependent], EvaluationRequestMode::Default)
        .unwrap();
    graph
        .execute_prepared_plan(&dependent_plan, &(), &dependent_compute)
        .unwrap();

    let dependencies = graph.dependencies_of(dependent).unwrap();
    assert_eq!(dependencies.len(), 1);
    assert_eq!(dependencies[0].source(), source);
    assert_eq!(dependencies[0].aspect(), ASPECT_A);
}

#[cfg(feature = "parallel")]
#[test]
fn prepared_parallel_precompute_matches_serial_results() {
    let mut serial_graph = SignalGraph::new();
    let a = serial_graph.node().build();
    let b = serial_graph.node().build();

    let mut parallel_graph = serial_graph.clone();
    let parallel_a = a;
    let parallel_b = b;

    let plan = serial_graph
        .build_evaluation_plan(&[a, b], EvaluationRequestMode::Default)
        .unwrap();
    let parallel_plan = parallel_graph
        .build_evaluation_plan(&[parallel_a, parallel_b], EvaluationRequestMode::Default)
        .unwrap();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(7, 0)));

    let serial_report = serial_graph
        .execute_prepared_plan_with_executor(&plan, &(), &evaluator, StageExecutor::Serial)
        .unwrap();
    let parallel_report = parallel_graph
        .execute_prepared_plan_with_executor(
            &parallel_plan,
            &(),
            &evaluator,
            StageExecutor::parallel(1),
        )
        .unwrap();

    assert_eq!(
        serial_graph.get_state(a).unwrap(),
        parallel_graph.get_state(parallel_a).unwrap()
    );
    assert_eq!(
        serial_graph.get_state(b).unwrap(),
        parallel_graph.get_state(parallel_b).unwrap()
    );
    assert_eq!(
        serial_graph
            .get_entry(a)
            .unwrap()
            .get_trace_summary()
            .unwrap()
            .output_hash,
        parallel_graph
            .get_entry(parallel_a)
            .unwrap()
            .get_trace_summary()
            .unwrap()
            .output_hash
    );
    assert_eq!(serial_report.task_count, parallel_report.task_count);
    assert_eq!(serial_report.tasks_executed, parallel_report.tasks_executed);
}

#[test]
fn build_evaluation_plan_handles_deep_linear_chain_without_recursion() {
    let mut graph = SignalGraph::new();
    let root = graph.node().build();
    let mut previous = root;
    let depth = 2_048;

    for _ in 0..depth {
        let current = graph.node().build();
        graph.append_dependency(current, previous, ASPECT_A).unwrap();
        previous = current;
    }

    let plan = graph
        .build_evaluation_plan(&[previous], EvaluationRequestMode::Default)
        .unwrap();

    assert_eq!(plan.summary.stage_count, (depth + 1) as u32);
    assert_eq!(plan.stages.first().unwrap().tasks[0].node, root);
    assert_eq!(plan.stages.last().unwrap().tasks[0].node, previous);
}

#[cfg(feature = "parallel")]
#[test]
fn parallel_executor_threshold_keeps_narrow_stage_serial() {
    let mut graph = SignalGraph::new();
    let node = graph.node().build();
    let plan = graph
        .build_evaluation_plan(&[node], EvaluationRequestMode::Default)
        .unwrap();
    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(1, 0)));

    let report = graph
        .execute_prepared_plan_with_executor(
            &plan,
            &(),
            &evaluator,
            StageExecutor::parallel(2),
        )
        .unwrap();

    assert!(matches!(
        report.stages[0].outcome,
        StageExecutionOutcome::CompletedSerial
    ));
}

#[cfg(feature = "parallel")]
#[test]
fn full_parallel_executor_matches_serial_results() {
    let mut serial_graph = SignalGraph::new();
    let serial_nodes = (0..12)
        .map(|_| serial_graph.node().build())
        .collect::<Vec<_>>();

    let mut parallel_graph = serial_graph.clone();
    let parallel_nodes = serial_nodes.clone();

    let plan = serial_graph
        .build_evaluation_plan(&serial_nodes, EvaluationRequestMode::Default)
        .unwrap();
    let parallel_plan = parallel_graph
        .build_evaluation_plan(&parallel_nodes, EvaluationRequestMode::Default)
        .unwrap();

    let evaluator = |ctx: &mut EvaluationContext<'_, ()>| Ok(ctx.finish(version_ab(11, 0)));

    let serial_report = serial_graph
        .execute_prepared_plan_with_executor(&plan, &(), &evaluator, StageExecutor::Serial)
        .unwrap();
    let parallel_report = parallel_graph
        .execute_prepared_plan_with_executor(
            &parallel_plan,
            &(),
            &evaluator,
            StageExecutor::aggressive_parallel(),
        )
        .unwrap();

    for (serial_node, parallel_node) in serial_nodes.iter().zip(parallel_nodes.iter()) {
        assert_eq!(
            serial_graph.get_state(*serial_node).unwrap(),
            parallel_graph.get_state(*parallel_node).unwrap()
        );
    }
    assert_eq!(serial_report.task_count, parallel_report.task_count);
    assert_eq!(serial_report.tasks_executed, parallel_report.tasks_executed);
    assert!(parallel_report.stages.iter().all(|stage| matches!(
        stage.parallel_kind,
        Some(crate::logic::planner::ParallelExecutionKind::FullParallel)
    )));
}
