use crate::facade::{
    mark_dirty, EvaluationRequestMode, ExecutionReport, NodeEvaluationResult, NodeId,
    ParallelApplyMode, SignalError, SignalGraph, StageExecutor,
};
use crate::tests::support::{version_ab, ASPECT_A};

use super::executor_policy::aggressive_parallel_runtime_policy;

#[test]
fn full_parallel_rewires_dynamic_dependencies_without_losing_parity() {
    fn bootstrap_graph() -> Result<(SignalGraph, NodeId, NodeId, NodeId, [NodeId; 2]), SignalError>
    {
        let mut graph = SignalGraph::new();
        graph.set_runtime_policy(aggressive_parallel_runtime_policy());
        let selector = graph.node().build();
        let left = graph.node().build();
        let right = graph.node().build();
        let target_a = graph.node().build();
        let target_b = graph.node().build();
        let targets = [target_a, target_b];

        let bootstrap = graph.build_evaluation_plan(
            &[selector, left, right, target_a, target_b],
            EvaluationRequestMode::ForceOnDemand,
        )?;
        graph.execute_prepared_plan(&bootstrap, &(), &move |ctx| {
            let node = ctx.node();
            let result = if node == selector {
                ctx.finish(version_ab(0, 0))
            } else if node == left {
                ctx.finish(version_ab(10, 0))
            } else if node == right {
                ctx.finish(version_ab(20, 0))
            } else {
                let selector_version = ctx.read_aspect_version(selector, ASPECT_A)?;
                let source = if selector_version.get(ASPECT_A) == 0 {
                    left
                } else {
                    right
                };
                let chosen = ctx.read_aspect_version(source, ASPECT_A)?;
                ctx.finish(NodeEvaluationResult::from_version(chosen))
            };
            Ok(result)
        })?;

        Ok((graph, selector, left, right, targets))
    }

    fn rewire_targets(
        graph: &mut SignalGraph,
        selector: NodeId,
        right: NodeId,
        targets: &[NodeId; 2],
        executor: StageExecutor,
    ) -> Result<ExecutionReport, SignalError> {
        mark_dirty(&mut *graph, selector, ASPECT_A)?;
        let plan = graph.build_evaluation_plan(targets, EvaluationRequestMode::Default)?;
        graph.execute_prepared_plan_with_executor(
            &plan,
            &(),
            &move |ctx| {
                let node = ctx.node();
                let result = if node == selector {
                    ctx.finish(version_ab(1, 0))
                } else {
                    let selector_version = ctx.read_aspect_version(selector, ASPECT_A)?;
                    let source = if selector_version.get(ASPECT_A) == 0 {
                        unreachable!("selector should have recomputed before dynamic targets")
                    } else {
                        right
                    };
                    let chosen = ctx.read_aspect_version(source, ASPECT_A)?;
                    ctx.finish(NodeEvaluationResult::from_version(chosen))
                };
                Ok(result)
            },
            executor,
        )
    }

    let (base_graph, selector, left, right, targets) = bootstrap_graph().unwrap();
    let mut serial_graph = base_graph.clone();
    let mut parallel_graph = base_graph;

    let serial_report = rewire_targets(
        &mut serial_graph,
        selector,
        right,
        &targets,
        StageExecutor::Serial,
    )
    .unwrap();
    let parallel_report = rewire_targets(
        &mut parallel_graph,
        selector,
        right,
        &targets,
        StageExecutor::full_parallel(2),
    )
    .unwrap();

    for target in targets {
        assert_eq!(
            serial_graph.dependencies_of(target).unwrap(),
            parallel_graph.dependencies_of(target).unwrap()
        );
        assert!(parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == right));
        assert!(!parallel_graph
            .dependencies_of(target)
            .unwrap()
            .iter()
            .any(|edge| edge.source() == left));
        assert_eq!(
            serial_graph.get_entry(target).unwrap().get_aspect_version(),
            parallel_graph
                .get_entry(target)
                .unwrap()
                .get_aspect_version()
        );
    }

    assert_eq!(
        serial_graph.subscribers_of(left).unwrap(),
        parallel_graph.subscribers_of(left).unwrap()
    );
    assert_eq!(
        serial_graph.subscribers_of(right).unwrap(),
        parallel_graph.subscribers_of(right).unwrap()
    );
    assert!(parallel_graph.subscribers_of(left).unwrap().is_empty());
    assert_eq!(
        parallel_graph.subscribers_of(right).unwrap().len(),
        targets.len()
    );
    assert!(parallel_report
        .stages
        .iter()
        .any(|stage| { stage.apply_mode == Some(ParallelApplyMode::SerialApply) }));
    assert_eq!(serial_report.tasks_executed, parallel_report.tasks_executed);
}
