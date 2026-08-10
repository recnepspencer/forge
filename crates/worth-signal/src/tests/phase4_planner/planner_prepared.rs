#[cfg(feature = "parallel")]
use crate::data::trace::assemble_trace_summary;
use crate::facade::{EvaluationContext, EvaluationRequestMode, NodeEvaluationResult, SignalGraph};
#[cfg(feature = "parallel")]
use crate::facade::{StageExecutionOutcome, StageExecutor};
#[cfg(feature = "parallel")]
use crate::logic::planner::model::{
    ParallelAdmissionReason, ParallelApplyMode, ParallelExecutionKind,
};
use crate::tests::support::{version_ab, GraphDependencyBatchExt, ASPECT_A};

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
        assemble_trace_summary(
            serial_graph
                .get_entry(a)
                .unwrap()
                .get_runtime_artifact_state(),
            serial_graph
                .get_entry(a)
                .unwrap()
                .retained_diagnostic_artifact(),
        )
        .unwrap()
        .output_hash,
        assemble_trace_summary(
            parallel_graph
                .get_entry(parallel_a)
                .unwrap()
                .get_runtime_artifact_state(),
            parallel_graph
                .get_entry(parallel_a)
                .unwrap()
                .retained_diagnostic_artifact(),
        )
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
        graph
            .append_dependency(current, previous, ASPECT_A)
            .unwrap();
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
        .execute_prepared_plan_with_executor(&plan, &(), &evaluator, StageExecutor::parallel(2))
        .unwrap();

    assert!(matches!(
        report.stages[0].outcome,
        StageExecutionOutcome::CompletedSerial
    ));
}

#[cfg(feature = "parallel")]
#[test]
fn full_parallel_executor_falls_back_honestly_when_mutable_apply_is_unavailable() {
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
    assert!(parallel_report
        .stages
        .iter()
        .all(|stage| match stage.parallel_admission_reason {
            Some(ParallelAdmissionReason::FullParallelUnsupportedByMutableEngine) => {
                stage.parallel_kind.is_none()
                    && stage.apply_mode == Some(ParallelApplyMode::SerialApply)
            }
            Some(ParallelAdmissionReason::AdmittedProofSafeGroupedConcurrent) => {
                stage.parallel_kind == Some(ParallelExecutionKind::FullParallel)
                    && stage.apply_mode == Some(ParallelApplyMode::GroupedConcurrentApply)
            }
            _ => false,
        }));
}
